use async_trait::async_trait;
use mongodb::{
    bson::{doc, self},
    options::ClientOptions,
    Client, Collection,
};

use crate::domain::{
    ports::PedidoRepository,
    entities::{Pedido, DominioError}
};

// --- DTO para MongoDB ---
// Mapea la entidad Pedido, añadiendo Serialize/Deserialize para BSON/JSON
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct PedidoDTO {
    pub id: u32,
    pub status: String,
    pub amount: f64,
}

// --- El Adaptador de MongoDB ---
pub struct MongoDbPedidoRepository {
    collection: Collection<PedidoDTO>,
}

impl MongoDbPedidoRepository {
    pub async fn new(uri: &str, db_name: &str, collection_name: &str) -> Result<Self, DominioError> {
        let client_options = ClientOptions::parse(uri)
            .await
            .map_err(|_e| DominioError::InternalError)?;

        let client = Client::with_options(client_options)
            .map_err(|_e| DominioError::InternalError)?;

        // Pings the server to see if the connection was successful
        client
            .database(db_name)
            .run_command(doc! {"ping": 1}, None)
            .await
            .map_err(|_e| DominioError::InternalError)?;

        let collection = client
            .database(db_name)
            .collection::<PedidoDTO>(collection_name);

        Ok(MongoDbPedidoRepository { collection })
    }
}

// --- Implementación del Puerto (El Contrato) ---
#[async_trait]
impl PedidoRepository for MongoDbPedidoRepository {
    async fn registrar_o_actualizar(&self, pedido: Pedido) -> Result<(), DominioError> {
        let pedido_dto = PedidoDTO {
            id: pedido.id,
            status: pedido.status,
            amount: pedido.amount,
        };

        // El filtro para encontrar el pedido por ID
        let filter = doc! { "id": pedido_dto.id };

        // El documento para la actualización (usando $set) o inserción (upsert)
        let update_doc = doc! { "$set": bson::to_document(&pedido_dto).map_err(|_| DominioError::InternalError)? };

        self.collection
            .update_one(filter, update_doc, mongodb::options::UpdateOptions::builder().upsert(true).build())
            .await
            .map_err(|_e| DominioError::InternalError)?;

        println!("[MONGO] Pedido registrado/actualizado. ID: {}", pedido_dto.id);
        Ok(())
    }
}