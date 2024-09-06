#![allow(unused)]

pub mod request {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct LoginRequest {
        #[serde(rename = "Login")]
        pub login: Login,
    }

    #[derive(Debug, Serialize)]
    pub struct Login {
        pub username: String,
        pub password: String,
    }
}

pub mod response {
    use std::collections::HashMap;

    use serde::Deserialize;
    use serde_json::Value;

    #[derive(Debug, Deserialize)]
    pub struct SuccessResponse<T> {
        pub success: bool,
        pub data: T,
    }

    #[derive(Debug, Deserialize)]
    pub struct Login {
        pub ucr: Vec<LoginUCR>,
        pub user: LoginUser,
    }

    #[derive(Debug, Deserialize)]
    pub struct LoginUCR {
        pub id: i32,
        pub name: String,
        pub shortname: String,
        pub usergroup_id: i64,
    }

    #[derive(Debug, Deserialize)]
    pub struct LoginUser {
        pub access_token: String,
        pub auth_key: String,
        pub autologin: bool,
        pub default_user_cluster_relation: i64,
    }

    #[derive(Debug, Deserialize)]
    pub struct ReportTypes {
        items: HashMap<i64, ReportTypesItem>,
        sorting: Vec<i32>,
    }

    #[derive(Debug, Deserialize)]
    pub struct ReportTypesItem {
        pub id: i64,
        pub name: String,
        pub description: String,
        pub fields: Vec<Value>,
    }
}
