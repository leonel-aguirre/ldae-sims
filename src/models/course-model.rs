use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize, Default)]
#[allow(non_snake_case)]
pub struct CourseModel {
    pub course_code: String,
    pub display_name: String,
    pub program_code: String,
    pub specialization_code: Option<String>,
}
