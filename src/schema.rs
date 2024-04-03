use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDegreeSchema {
    pub name: String,
}

pub struct SelectDegreeSchema {
    pub id: String,
    pub name: String,
}
