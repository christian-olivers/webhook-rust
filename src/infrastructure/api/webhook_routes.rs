// src/infrastructure/api/webhook_routes.rs

use warp::Filter;
use std::sync::Arc;
use crate::application::ProcessWebhookService;
use crate::domain::{
    entities::{WebhookEvent, DominioError}, 
    ports::PedidoRepository,
};

// 1. La función de manejo (Handler)
// Recibe el WebhookEvent ya deserializado por Warp
async fn webhook_handler<R: PedidoRepository>(
    event: WebhookEvent,
    service: Arc<ProcessWebhookService<R>>, // El servicio ya viene inyectado
) -> Result<impl warp::Reply, warp::Rejection> {
    
    // Llamar al Caso de Uso (Lógica de Negocio)
    match service.execute(event).await {
        Ok(()) => {
            // Éxito: devolver respuesta 200 OK inmediatamente (vital para webhooks)
            Ok(warp::reply::with_status("OK", warp::http::StatusCode::OK))
        },
        Err(DominioError::PayloadFormatError(msg)) => {
            eprintln!("Error de formato de payload: {}", msg);
            // Error de cliente: devolver 400 Bad Request
            Err(warp::reject::custom(ApiError::BadRequest(msg)))
        },
        Err(e) => {
            eprintln!("Error interno al procesar: {:?}", e);
            // Cualquier otro error: devolver 500 Internal Server Error
            Err(warp::reject::custom(ApiError::Internal(e.to_string())))
        }
    }
}

// 2. Definición de la inyección de dependencias (DI)
// Crea un filtro de Warp que contiene una referencia al Service
fn with_service<R: PedidoRepository>(
    service: Arc<ProcessWebhookService<R>>,
) -> impl Filter<Extract = (Arc<ProcessWebhookService<R>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || service.clone())
}

// 3. Definición de la ruta principal del webhook
pub fn webhook_routes<R: PedidoRepository>(
    service: Arc<ProcessWebhookService<R>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    
    // Filtro para manejar la deserialización del JSON entrante en WebhookEvent
    let json_body = warp::body::content_length_limit(1024 * 16) // Límite de 16kb
        .and(warp::body::json());

    warp::path!("webhook")
        .and(warp::post())
        .and(json_body) // 1. Extrae el WebhookEvent del cuerpo
        .and(with_service(service)) // 2. Inyecta el Service
        .and_then(webhook_handler) // 3. Llama al Handler
}


// --- Manejo de Errores Específico para Warp ---
// Define errores personalizados para Warp (para mapear DominioError a HTTP)
#[derive(Debug)]
enum ApiError {
    BadRequest(String),
    Internal(String),
}
impl warp::reject::Reject for ApiError {}

// Esto es necesario para que Warp traduzca nuestros errores DominioError en HTTP
pub async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(e) = err.find::<ApiError>() {
        let (code, message) = match e {
            ApiError::BadRequest(msg) => (warp::http::StatusCode::BAD_REQUEST, format!("Bad Request: {}", msg)),
            ApiError::Internal(msg) => (warp::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Internal Error: {}", msg)),
        };
        return Ok(warp::reply::with_status(message, code));
    }
    Err(err)
}// src/infrastructure/api/webhook_routes.rs