pub mod shared;

#[path = "./schemas/program-schemas.rs"]
pub mod program_schemas;
#[path = "./schemas/specialization-schemas.rs"]
pub mod specialization_schemas;

#[path = "./schemas/course-schemas.rs"]
pub mod course_schemas;

#[path = "./models/program-model.rs"]
pub mod program_model;
#[path = "./models/specialization-model.rs"]
pub mod specialization_model;

#[path = "./models/course-model.rs"]
pub mod course_model;

#[path = "./handlers/program-handlers.rs"]
pub mod program_handlers;
#[path = "./handlers/specialization-handlers.rs"]
pub mod specialization_handlers;

#[path = "./handlers/course-handlers.rs"]
pub mod course_handlers;

#[path = "./routes/program-routes.rs"]
pub mod program_routes;
#[path = "./routes/specialization-routes.rs"]
pub mod specialization_routes;

#[path = "./routes/course-routes.rs"]
pub mod course_routes;
