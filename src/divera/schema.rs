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
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct SuccessResponse<T> {
        pub success: bool,
        pub data: T,
    }

    #[derive(Debug, Deserialize)]
    pub struct Login {
        pub ucr: Vec<UCR>,
        pub user: User,
    }

    #[derive(Debug, Deserialize)]
    pub struct UCR {
        pub id: i32,
        pub name: String,
        pub shortname: String,
        pub usergroup_id: i32,
    }

    #[derive(Debug, Deserialize)]
    pub struct User {
        pub access_token: String,
        pub auth_key: String,
        pub autologin: bool,
        pub default_user_cluster_relation: i32,
    }
}
