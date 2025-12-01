// src/main.rs

use std::sync::Arc;

use dotenv::dotenv;
// Importar todas las capas
mod domain;
mod application;
mod infrastructure;

use warp::Filter;


use infrastructure::persistence::MongoDbPedidoRepository;

// use crate::infrastructure::persistence::InMemoryPedidoRepository;
use crate::infrastructure::api::{webhook_routes, handle_rejection};
use application::ProcessWebhookService;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // 1. Infraestructura: Crear la implementación de la BD (el Adaptador)
    let mongo_uri = std::env::var("MONGO_URI").expect("MONGO_URI must be set");
    let db_name = std::env::var("MONGO_DB_NAME").unwrap_or_else(|_| "webhook_db".to_string());
    let collection_name = "pedidos";
    // let repo_impl = Arc::new(InMemoryPedidoRepository::new());
    // let service = Arc::new(ProcessWebhookService::new(repo_impl));

    let repo_impl = Arc::new(
        MongoDbPedidoRepository::new(&mongo_uri, &db_name, collection_name)
            .await
            .expect("Failed to connect to MongoDB")
    );
    
    let service = Arc::new(ProcessWebhookService::new(repo_impl));

    let routes = webhook_routes(service)
        .recover(handle_rejection);

    println!("🚀 Servidor de Webhook escuchando en http://127.0.0.1:8080/webhook");
    
    // Iniciar el servidor
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}