# Webhook Listener (Rust + Warp)

Servicio para recibir webhooks y persistir actualizaciones de pedidos siguiendo Arquitectura Limpia (Dominio → Aplicación → Infraestructura). Implementado con `warp`, `tokio` y MongoDB.

## Requisitos

- `Rust` 1.70+
- `MongoDB` local o Atlas
- `cargo` y `tokio`
- Opcional: `Docker` y `docker-compose`

## Configuración

Crea un archivo `.env` en la raíz con:

```env
MONGO_URI=mongodb://localhost:27017
MONGO_DB_NAME=webhook_db
```

## Ejecutar

```bash
cargo run
```

- Levanta el servidor en `http://127.0.0.1:8080/webhook`.

## Endpoint

- `POST /webhook`
- Body JSON debe seguir el modelo `WebhookEvent`:

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "event_type": "pedido.actualizado",
  "payload": {
    "pedido_id": 101,
    "monto": 50.0,
    "estado_actual": "PAGADO"
  }
}
```

## Ejemplo con curl

```bash
curl -X POST http://127.0.0.1:8080/webhook \
  -H "Content-Type: application/json" \
  -d '{
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "event_type": "pedido.actualizado",
    "payload": {"pedido_id": 101, "monto": 50.0, "estado_actual": "PAGADO"}
  }'
```

## Respuestas

- `200 OK` → "OK" cuando se persiste el pedido
- `400 Bad Request` → Error de formato del `payload`
- `500 Internal Server Error` → Error interno (por ejemplo, MongoDB)

## Arquitectura

```
src
├── main.rs                 # Inicializa app y servidor Warp
├── domain/                 # Lógica pura
│   ├── entities.rs         # WebhookEvent, Pedido, errores
│   └── ports.rs            # Trait PedidoRepository
├── application/
│   └── service.rs          # ProcessWebhookService (usa repo)
└── infrastructure/
    ├── api/webhook_routes.rs # Handler /webhook y errores
    └── persistence/
        ├── in_memory_repo.rs # Repositorio en memoria
        └── mongodb_repo.rs   # Repositorio MongoDB
```

- El handler (`webhook_routes`) deserializa el cuerpo a `WebhookEvent` y delega en `ProcessWebhookService`.
- El service extrae del `payload` los campos `pedido_id`, `monto`, `estado_actual` y llama al `PedidoRepository`.

## Elegir repositorio

Por defecto se usa MongoDB (`MongoDbPedidoRepository`). Para probar sin BD, cambia a `InMemoryPedidoRepository` en `src/main.rs` comentando/activando las líneas indicadas en ese archivo.

## Docker (opcional)

- Inicia MongoDB con `docker-compose.yml`:

```bash
docker-compose up -d
```

- Ejecuta la app:

```bash
cargo run
```

## Dependencias clave

- `warp`, `tokio`, `serde/serde_json`, `dotenv`, `async-trait`, `uuid`, `thiserror`, `mongodb`
