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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct PedidoDTO {
    pub id: u32,
    pub status: String,
    pub amount: f64,
}

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

#[async_trait]
impl PedidoRepository for MongoDbPedidoRepository {
    async fn registrar_o_actualizar(&self, pedido: Pedido) -> Result<(), DominioError> {
        let pedido_dto = PedidoDTO {
            id: pedido.id,
            status: pedido.status,
            amount: pedido.amount,
        };

        let filter = doc! { "id": pedido_dto.id };

        let update_doc = doc! { "$set": bson::to_document(&pedido_dto).map_err(|_| DominioError::InternalError)? };

        self.collection
            .update_one(filter, update_doc, mongodb::options::UpdateOptions::builder().upsert(true).build())
            .await
            .map_err(|_e| DominioError::InternalError)?;

        println!("[MONGO] Pedido registrado/actualizado. ID: {}", pedido_dto.id);
        Ok(())
    }
}