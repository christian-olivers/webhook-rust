use serde::{Deserialize, Serialize};
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub id: Uuid,
    pub event_type: String,
    pub payload: serde_json::Value,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Pedido {
    pub id: u32,
    pub status: String, // e.g., "PENDIENTE", "PAGADO"
    pub amount: f64,
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum DominioError {
    #[error("Recurso no encontrado")]
    NotFound,
    #[error("Error interno del repositorio")]
    InternalError,
    #[error("Error de formato del payload: {0}")] 
    PayloadFormatError(String),
}