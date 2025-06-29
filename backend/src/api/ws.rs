use std::collections::HashMap;

use actix_web::{HttpRequest, Responder, get, web};
use actix_ws::Message;
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Action {
    Mount {
        plugin_name: String,
    },
    Event {
        plugin_name: String,
        event: String,
        data: ActionData,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ActionData {
    text_inputs: HashMap<String, String>,
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
                    if let Ok(action) = serde_json::from_str::<Action>(&msg.to_string()) {
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
    action: Action,
    session: &mut actix_ws::Session,
    state: &AppState,
) -> anyhow::Result<()> {
    match action {
        Action::Mount { plugin_name } => {
            log::info!("Mounting {}", plugin_name);
            let plugin = state
                .plugins
                .lock()
                .unwrap()
                .iter()
                .find(|&plugin| plugin.metadata.name == plugin_name)
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
        Action::Event {
            plugin_name,
            event,
            data,
        } => {
            log::info!(
                "Event: {} from {} with data {:#?}",
                event,
                plugin_name,
                data
            );

            Ok(())
        }
    }
}
