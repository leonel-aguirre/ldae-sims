use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct ParamOptions {
    pub program_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateProgramSchema {
    pub program_code: String,
    pub display_name: String,
    pub program_type: String,
    pub duration_type: String,
    pub duration: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateProgramSchema {
    pub display_name: Option<String>,
    pub program_type: Option<String>,
    pub duration_type: Option<String>,
    pub duration: Option<i32>,
}
