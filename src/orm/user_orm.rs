use diesel::result::Error;
use diesel::{ExpressionMethods, Insertable, Queryable, Selectable, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::response::Response;
use crate::DBPooledConnection;

pub type Users = Response<User>;

#[derive(Debug, Deserialize, Serialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub login: String,
    pub name: String,
    pub password: String,
}

pub fn get_users(total_users: i64, conn: &mut DBPooledConnection) -> Result<Users, Error> {
    use crate::schema::users::dsl::*;

    let _users = match users
        .order(login.desc())
        .limit(total_users)
        .load::<User>(conn)
    {
        Ok(usr) => usr,
        Err(_) => vec![],
    };

    Ok(Users {
        results: _users,
    })
}

pub fn get_user(login_input: String, conn: &mut DBPooledConnection) -> Result<User, Error> {
    use crate::schema::users::dsl::*;

    users
        .filter(login.eq(login_input))
        .first::<User>(conn)
}

pub fn create_user(user: User, conn: &mut DBPooledConnection) -> Result<User, Error> {
    use crate::schema::users::dsl::*;
    
    diesel::insert_into(users).values(&user).get_result::<User>(conn)
}
