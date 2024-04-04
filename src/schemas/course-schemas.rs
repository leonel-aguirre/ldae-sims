use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct ParamOptions {
    pub course_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCourseSchema {
    pub course_code: String,
    pub display_name: String,
    pub program_code: String,
    pub specialization_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCourseSchema {
    pub display_name: Option<String>,
    pub program_code: Option<String>,
    pub specialization_code: Option<String>,
}
