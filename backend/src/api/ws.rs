use std::collections::HashMap;

use actix_web::{HttpRequest, Responder, web};
use actix_ws::Message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Action {
    Mount,
    Event {
        plugin_name: String,
        event: String,
        data: HashMap<String, String>,
    },
}

pub async fn ws_handler(req: HttpRequest, body: web::Payload) -> actix_web::Result<impl Responder> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.recv().await {
            match msg {
                Message::Text(msg) => {
                    if let Ok(action) = serde_json::from_str::<Action>(&msg.to_string()) {
                        log::info!("Got action: {:#?}", action);
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
