use super::Database;
use crate::models::User;

type Result<T> = crate::Result<T>;

macro_rules! mock_users {
    ($(($username:expr, $password_hash: expr)),*) => {
        vec![
            $( User::new(String::from($username), String::from($password_hash))),*
        ]
    };
}

pub struct MockDatabase {}

impl MockDatabase {
    pub fn new() -> MockDatabase {
        MockDatabase {}
    }

    pub fn users_vec() -> Vec<User> {
        mock_users!(
            ("user1", "pass1"),
            ("user2", "pass2"),
            ("user3", "pass3"),
            ("user4", "pass4")
        )
    }
}

impl Database for MockDatabase {
    fn insert_user(&self, _user: &User) -> Result<()> {
        Ok(())
    }

    fn get_user(&self, username: String) -> Result<User> {
        Ok(User::new(username, String::new()))
    }

    fn get_users(&self) -> Result<Vec<User>> {
        Ok(MockDatabase::users_vec())
    }

    fn is_admin(&self) -> bool {
        true
    }
}
