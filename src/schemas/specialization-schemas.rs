use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct ParamOptions {
    pub specialization_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSpecializationSchema {
    pub specialization_code: String,
    pub display_name: String,
    pub program_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateSpecializationSchema {
    pub display_name: Option<String>,
    pub program_code: Option<String>,
}
