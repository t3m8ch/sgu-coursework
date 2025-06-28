use actix_web::{HttpResponse, Responder, get, web};

use crate::state::AppState;

#[get("/plugins")]
async fn list_plugins(data: web::Data<AppState>) -> impl Responder {
    let plugins = data.plugins.lock().unwrap();
    HttpResponse::Ok().json(
        plugins
            .iter()
            .map(|p| p.metadata.clone())
            .collect::<Vec<_>>(),
    )
}

#[get("/plugins/{name}/metadata")]
async fn plugin_metadata(name: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let name = name.into_inner();
    log::info!("Requesting metadata for plugin: {}", name);

    let plugins = data.plugins.lock().unwrap();
    let plugin = plugins.iter().filter(|p| p.metadata.name == name).next();
    match plugin {
        Some(plugin) => {
            log::debug!("Found plugin manifest for: {}", name);
            HttpResponse::Ok().json(plugin.metadata.clone())
        }
        None => {
            log::warn!("Plugin not found: {}", name);
            HttpResponse::NotFound().body("Plugin not found")
        }
    }
}
