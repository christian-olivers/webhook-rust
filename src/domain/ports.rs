use async_trait::async_trait;
use super::entities::{Pedido, DominioError};

#[async_trait]
pub trait PedidoRepository: Send + Sync {
    async fn registrar_o_actualizar(&self, pedido: Pedido) -> Result<(), DominioError>;
}