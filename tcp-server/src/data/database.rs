use crate::models::User;

type Result<T> = crate::Result<T>;

pub trait Database {
    //TODO: add query parameters
    fn get_session_user(&self, session_id: &str) -> Result<String>;
    fn is_admin(&self) -> bool;
    fn insert_user(&self, user: &User) -> Result<()>;
    fn get_users(&self) -> Result<Vec<User>>;
    fn get_user(&self, username: String) -> Result<User>;
}
