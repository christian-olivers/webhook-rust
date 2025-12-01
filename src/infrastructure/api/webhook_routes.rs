use warp::Filter;
use std::sync::Arc;
use crate::application::ProcessWebhookService;
use crate::domain::{
    entities::{WebhookEvent, DominioError}, 
    ports::PedidoRepository,
};

async fn webhook_handler<R: PedidoRepository>(
    event: WebhookEvent,
    service: Arc<ProcessWebhookService<R>>
) -> Result<impl warp::Reply, warp::Rejection> {
 
    match service.execute(event).await {
        Ok(()) => {
            Ok(warp::reply::with_status("OK", warp::http::StatusCode::OK))
        },
        Err(DominioError::PayloadFormatError(msg)) => {
            eprintln!("Error de formato de payload: {}", msg);
            Err(warp::reject::custom(ApiError::BadRequest(msg)))
        },
        Err(e) => {
            eprintln!("Error interno al procesar: {:?}", e);
            Err(warp::reject::custom(ApiError::Internal(e.to_string())))
        }
    }
}

fn with_service<R: PedidoRepository>(
    service: Arc<ProcessWebhookService<R>>,
) -> impl Filter<Extract = (Arc<ProcessWebhookService<R>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || service.clone())
}

pub fn webhook_routes<R: PedidoRepository>(
    service: Arc<ProcessWebhookService<R>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

    let json_body = warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json());

    warp::path!("webhook")
        .and(warp::post())
        .and(json_body) // 1. Extrae el WebhookEvent del cuerpo
        .and(with_service(service)) // 2. Inyecta el Service
        .and_then(webhook_handler)
}


#[derive(Debug)]
enum ApiError {
    BadRequest(String),
    Internal(String),
}
impl warp::reject::Reject for ApiError {}

pub async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(e) = err.find::<ApiError>() {
        let (code, message) = match e {
            ApiError::BadRequest(msg) => (warp::http::StatusCode::BAD_REQUEST, format!("Bad Request: {}", msg)),
            ApiError::Internal(msg) => (warp::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Internal Error: {}", msg)),
        };
        return Ok(warp::reply::with_status(message, code));
    }
    Err(err)
}