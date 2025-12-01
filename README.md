```
src
├── main.rs                 # Inicializa la aplicación y el servidor Warp
├── domain/                 # 💖 Capa Central: Lógica Pura del Negocio
│   ├── mod.rs              # Exporta los módulos del dominio
│   ├── entities.rs         # Modelos de datos (structs)
│   └── ports.rs            # Traits (Interfaces) para Infraestructura
├── application/            # 🚀 Capa de Casos de Uso
│   ├── mod.rs
│   └── service.rs          # Implementa la lógica que usa los puertos
└── infrastructure/         # ⚙️ Capa Externa: Adaptadores y Frameworks
    ├── mod.rs
    ├── persistence/        # Implementación de la BD o Colas (Adapters)
    │   ├── mod.rs
    │   └── in_memory_repo.rs # Repositorio de prueba (fácil de cambiar)
    └── api/                # Adaptador de entrada (Warp Handler)
        ├── mod.rs
        └── webhook_routes.rs
```
