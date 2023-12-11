use serde::{Deserialize, Serialize};

#[derive(Deserialize,Debug,Serialize)]
pub struct MyJsonData {
    pub page: i32,
    pub size: i32,
}

