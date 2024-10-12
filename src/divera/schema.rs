#![allow(unused)]

use std::collections::HashMap;

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
        pub jwt: bool,
    }
}

pub mod response {
    use std::collections::HashMap;

    use serde::Deserialize;
    use serde_json::Value;

    #[derive(Clone, Debug, Deserialize)]
    pub struct SuccessResponse<T> {
        pub success: bool,
        pub data: T,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Login {
        pub ucr: Vec<LoginUCR>,
        pub user: LoginUser,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Jwt {
        pub jwt: String,
        pub jwt_api: String,
        pub jwt_ws: String,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct All {
        pub cluster: Cluster,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Cluster {
        pub consumer: HashMap<String, Consumer>,
        pub reporttypes: ReportTypes,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Consumer {
        pub firstname: String,
        pub lastname: String,
        pub stdformat_name: String,
        // groups: ,
        // qualifications:
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct LoginUCR {
        pub id: i32,
        pub name: String,
        pub shortname: String,
        pub usergroup_id: i64,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct LoginUser {
        pub access_token: String,
        pub auth_key: String,
        pub autologin: bool,
        pub default_user_cluster_relation: i64,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct ReportTypes {
        pub items: HashMap<i64, ReportTypesItem>,
        pub sorting: Vec<i32>,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct ReportTypesItem {
        pub id: i64,
        pub name: String,
        pub description: String,
        pub fields: Vec<ReportTypesItemFields>,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct ReportTypesItemFields {
        pub id: String,
        pub name: String,
        pub r#type: ReportTypesItemFieldsType,
        pub options: Option<Vec<ReportTypesItemFieldOption>>,
    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum ReportTypesItemFieldsType {
        Checkbox,
        Date,
        Number,
        Radio,
        SelectBox,
        String,
        TextArea,
        TextInput,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct ReportTypesItemFieldOption {
        pub id: String,
        pub name: String,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Reports {
        pub items: Vec<Report>,
        pub itemcount: u64,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Report {
        pub id: i64,
        pub cluster_id: i64,
        pub user_cluster_relation_id: i64,
        pub status: i64,
        pub lat: i64,
        pub lng: i64,
        pub address: String,
        pub fields: Vec<Value>,
    }

    // #[derive(Clone, Debug, Deserialize)]
    // pub struct User {
    //     pub id: i64,
    //     pub cluster_id: i64,
    //     pub has_multiple_user_cluster_relations: bool,
    //     pub is_default_user_cluster_relation: bool,
    //     pub foreign_id: Value,
    //     pub firstname: String,
    //     pub lastname: String,
    //     pub username: String,
    //     pub email: Option<UserEmail>,
    //     pub phonenumbers: Option<Vec<UserPhonenumber>>,
    // }
    // #[derive(Clone, Debug, Deserialize)]
    // pub struct UserEmail {
    //     pub email: String,
    //     pub confirmed: bool,
    // }
    // #[derive(Clone, Debug, Deserialize)]
    // pub struct UserPhonenumber {
    //     pub phonenumber: String,
    //     pub receive_call: bool,
    //     pub receive_sms: bool,
    // }
}
