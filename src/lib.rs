pub mod shared;

#[path = "./schemas/program-schemas.rs"]
pub mod program_schemas;
#[path = "./schemas/specialization-schemas.rs"]
pub mod specialization_schemas;

#[path = "./models/program-model.rs"]
pub mod program_model;
#[path = "./models/specialization-model.rs"]
pub mod specialization_model;

#[path = "./handlers/program-handlers.rs"]
pub mod program_handlers;
#[path = "./handlers/specialization-handlers.rs"]
pub mod specialization_handlers;

#[path = "./routes/program-routes.rs"]
pub mod program_routes;
#[path = "./routes/specialization-routes.rs"]
pub mod specialization_routes;
