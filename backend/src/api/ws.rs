use actix_web::{HttpRequest, Responder, get, web};
use actix_ws::Message;
use plugin_sdk::{Action, StateInput, UINode};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ActionReq {
    pub plugin_name: String,
    pub action: Action,
    pub session_id: Option<uuid::Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ActionRes {
    UITree(UINode),
    Session(uuid::Uuid),
    Error(String),
}

#[get("/ws")]
pub async fn ws(
    req: HttpRequest,
    body: web::Payload,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.recv().await {
            match msg {
                Message::Text(msg) => {
                    log::info!("Received message: {}", msg);
                    if let Ok(action) = serde_json::from_str::<ActionReq>(&msg.to_string()) {
                        if let Err(err) = handle_action(action, &mut session, &app_state).await {
                            log::error!("Handle action error: {:#?}", err);
                        }
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
        }

        let _ = session.close(None).await;
    });

    Ok(response)
}

async fn handle_action(
    action_req: ActionReq,
    ws_session: &mut actix_ws::Session,
    app_state: &AppState,
) -> anyhow::Result<()> {
    log::info!("Action {:#?}", action_req);
    let plugin = app_state
        .plugins
        .lock()
        .unwrap()
        .iter()
        .find(|&plugin| plugin.metadata.name == action_req.plugin_name)
        .cloned();

    let not_found = ActionRes::Error("Plugin not found".to_string());
    match plugin {
        Some(mut plugin) => {
            let ui_state = match action_req.action {
                Action::Mount => {
                    let session_id = uuid::Uuid::new_v4();
                    let session = ActionRes::Session(session_id);
                    ws_session.text(serde_json::to_string(&session)?).await?;

                    let init_state = plugin.state(StateInput {
                        action: action_req.action,
                        old_state: None,
                    })?;

                    log::info!("Init state: {:#?}", init_state);

                    app_state
                        .sessions
                        .lock()
                        .unwrap()
                        .insert(session_id, init_state.clone());

                    serde_json::to_value(init_state)
                }
                Action::Event { event: _, data: _ } => {
                    if let Some(session_id) = action_req.session_id {
                        let old_state =
                            app_state.sessions.lock().unwrap().get(&session_id).cloned();
                        log::info!("Old state: {:#?}", old_state);

                        let new_state = plugin.state(StateInput {
                            action: action_req.action,
                            old_state: old_state,
                        })?;
                        log::info!("New state: {:#?}", new_state);

                        app_state
                            .sessions
                            .lock()
                            .unwrap()
                            .insert(session_id, new_state.clone());

                        serde_json::to_value(new_state)
                    } else {
                        serde_json::to_value(ActionRes::Error("No session ID provided".to_string()))
                    }
                }
            };

            let ui = ActionRes::UITree(plugin.ui(ui_state?)?);
            ws_session.text(serde_json::to_string(&ui)?).await?;
        }
        None => ws_session.text(serde_json::to_string(&not_found)?).await?,
    };

    Ok(())
}
