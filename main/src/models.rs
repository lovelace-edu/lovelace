use chrono::NaiveDateTime;

use crate::schema::users;

#[derive(Queryable, Identifiable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    username: &'a str,
    email: &'a str,
    password: &'a str,
    created: NaiveDateTime,
}

impl<'a> NewUser<'a> {
    pub fn new(
        username: &'a str,
        email: &'a str,
        password: &'a str,
        created: NaiveDateTime,
    ) -> Self {
        NewUser {
            username,
            email,
            password,
            created,
        }
    }
}
