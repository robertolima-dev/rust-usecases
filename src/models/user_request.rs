use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProfileRequest {
    pub bio: Option<String>,
    pub birth_date: Option<String>,
    pub phone: Option<String>,
    pub document: Option<String>,
    pub profession: Option<String>,
    pub avatar: Option<String>,
    pub confirm_email: Option<bool>,
    pub unsubscribe: Option<bool>,
    pub access_level: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserRequest {
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub profile: Option<ProfileRequest>,
}
