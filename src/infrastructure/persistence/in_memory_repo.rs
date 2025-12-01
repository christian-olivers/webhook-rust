use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;

// Importar el Dominio: ¡Solo importamos el Puerto y las Entidades, no el Service!
use crate::domain::{ports::PedidoRepository, entities::{Pedido, DominioError}}; 

// El adaptador: Implementación del Repositorio
#[derive(Clone)]
pub struct InMemoryPedidoRepository {
    // Usamos Arc<Mutex<...>> para poder compartir y modificar el HashMap 
    // de forma segura entre múltiples hilos (como los hilos de Warp).
    storage: Arc<Mutex<HashMap<u32, Pedido>>>, 
}

impl InMemoryPedidoRepository {
    pub fn new() -> Self {
        // Inicializa el almacenamiento con algunos datos de ejemplo si quieres.
        let initial_storage = {
            let mut map = HashMap::new();
            map.insert(101, Pedido {
                id: 101,
                status: "PENDIENTE".to_string(),
                amount: 50.00,
            });
            map
        };

        InMemoryPedidoRepository {
            storage: Arc::new(Mutex::new(initial_storage)),
        }
    }
}

// Implementar el Puerto: La clave de la Arquitectura Limpia
#[async_trait]
impl PedidoRepository for InMemoryPedidoRepository {
    async fn registrar_o_actualizar(&self, pedido: Pedido) -> Result<(), DominioError> {
        let mut storage = self.storage.lock()
            .map_err(|_| DominioError::InternalError)?; // Manejo de error de Mutex

        // Insertar o actualizar el pedido. Si ya existe, se sobrescribe.
        storage.insert(pedido.id, pedido);

        println!("[REPO] Pedido registrado/actualizado. ID: {}", storage.keys().last().unwrap_or(&0));
        Ok(())
    }
}