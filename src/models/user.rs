#[derive(Debug)]
pub struct user_name {
    pub ID: Option<i64>,
    pub UserName: String,
    pub UserCode: Option<String>,
    pub Password: Option<String>,
    pub Position: Option<String>,
    pub Telephone: Option<String>,
    pub Email: Option<String>,
    pub Handphone: Option<String>,
    pub GroupID: Option<i64>,
    pub LogIn: Option<i32>,
    pub SecurityCode: Option<String>,
    pub Status: Option<i32>,
    pub UserID: Option<i64>,
}
