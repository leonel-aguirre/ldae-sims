use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct ParamOptions {
    pub id: Option<i32>,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct CreateDegreeSchema {
//     pub name: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct SelectDegreeSchema {
//     pub id: i32,
//     pub name: String,
// }