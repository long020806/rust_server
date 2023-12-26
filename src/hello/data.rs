use serde::{Deserialize, Serialize};

#[derive(Deserialize,Debug,Serialize)]
pub struct MyJsonData {
    pub page: i32,
    pub size: i32,
}


#[derive(Deserialize,Debug,Serialize)]
pub struct MyDetailQuery {
    pub id: String
}

#[derive(Debug, Serialize)]
pub struct MyData {
    pub key: String,
    pub value: i32,
}

#[derive(Deserialize, Debug)]
pub struct MyQuery {
    pub value: i32,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct UserVo{
    pub id: i32,
    pub username: String,
    pub created_at: Option<String>,
}

#[derive(Deserialize,Debug,Serialize)]
pub struct TestJsonData {
    pub value: String,
}