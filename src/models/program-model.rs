use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize, Default)]
#[allow(non_snake_case)]
pub struct ProgramModel {
    pub program_code: String,
    pub display_name: String,
    pub program_type: String,
    pub duration_type: String,
    pub duration: i32,
}
