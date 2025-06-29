use actix_web::{HttpRequest, Responder, get, web};
use actix_ws::Message;
use plugin_sdk::Action;
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ActionReq {
    pub plugin_name: String,
    pub action: Action,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ActionErrorResponse {
    Error(String),
}

#[get("/ws")]
pub async fn ws(
    req: HttpRequest,
    body: web::Payload,
    state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.recv().await {
            match msg {
                Message::Text(msg) => {
                    if let Ok(action) = serde_json::from_str::<ActionReq>(&msg.to_string()) {
                        if let Err(err) = handle_action(action, &mut session, &state).await {
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
    session: &mut actix_ws::Session,
    state: &AppState,
) -> anyhow::Result<()> {
    match action_req.action {
        Action::Mount => {
            log::info!("Mounting {}", action_req.plugin_name);
            let plugin = state
                .plugins
                .lock()
                .unwrap()
                .iter()
                .find(|&plugin| plugin.metadata.name == action_req.plugin_name)
                .cloned();

            let not_found = ActionErrorResponse::Error("Plugin not found".to_string());
            match plugin {
                Some(mut plugin) => {
                    let ui = plugin.ui()?;
                    session.text(serde_json::to_string(&ui)?).await?
                }
                None => session.text(serde_json::to_string(&not_found)?).await?,
            };

            Ok(())
        }
        Action::Event { event, data } => {
            log::info!(
                "Event: {} from {} with data {:#?}",
                event,
                action_req.plugin_name,
                data
            );

            Ok(())
        }
    }
}
