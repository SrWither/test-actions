use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SignupParams<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Serialize)]
pub struct SigninParams<'a> {
    pub email: &'a str,
    pub password: &'a str,
}
