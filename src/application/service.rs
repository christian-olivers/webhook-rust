use std::sync::Arc;
use crate::domain::{
    entities::{WebhookEvent, Pedido}, 
    ports::PedidoRepository, 
    DominioError
};
use serde::Deserialize;

pub struct ProcessWebhookService<R: PedidoRepository> {
    repo: Arc<R>, 
}

impl<R: PedidoRepository> ProcessWebhookService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, event: WebhookEvent) -> Result<(), DominioError> {
        #[derive(Deserialize)]
        struct WebhookData {
            pedido_id: u32,
            monto: f64,
            estado_actual: String,
        }

        let data: WebhookData = serde_json::from_value(event.payload)
            .map_err(|e| DominioError::PayloadFormatError(format!("Error en payload del webhook: {}", e)))?;

        let pedido = Pedido {
            id: data.pedido_id,
            status: data.estado_actual,
            amount: data.monto,
        };

        self.repo.registrar_o_actualizar(pedido).await
    }
}