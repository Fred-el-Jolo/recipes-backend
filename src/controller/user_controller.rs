use actix_web::web::{Data, Json};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::DBPool;
use crate::orm::user_orm::{get_users, get_user, User, create_user};

#[derive(Debug, Deserialize, Serialize)]
pub struct JSONUser {
    pub id: Option<i32>,
    pub login: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
}

/// list users
#[get("/users")]
pub async fn list(pool: Data<DBPool>) -> HttpResponse {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let users = web::block(move || get_users(50, &mut conn)).await.unwrap();

    let json_users: Vec<JSONUser> = users.unwrap().results.iter().map(|user| JSONUser {
        id:user.id,
        login: Some(user.login.clone()),
        name: Some(user.name.clone()),
        password: Some(user.password.clone())
    }).collect();

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(json_users)
}

/// get one specific user
#[get("/user/{user_login}")]
pub async fn get(pool: Data<DBPool>, path: web::Path<String>) -> HttpResponse {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let user_login = path.into_inner();
    let user = web::block(move || get_user(user_login, &mut conn)).await.unwrap();

    let json_user = user.map(|user| JSONUser {
        id: user.id,
        login: Some(user.login.clone()),
        name: Some(user.name.clone()),
        password: Some(user.password.clone())
    }).unwrap();

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(json_user)
}

/// create a tweet `/tweets`
#[post("/user")]
pub async fn create(user: Json<JSONUser>, pool: Data<DBPool>) -> HttpResponse {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let new_user = User {
        id: None,
        login: user.login.as_ref().unwrap().clone(),
        name: user.name.as_ref().unwrap().clone(),
        password: user.password.as_ref().unwrap().clone(),
    };

    let user = web::block(move || create_user(new_user, &mut conn)).await;

    match user {
        Ok(Ok(user)) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(user),
        _ => HttpResponse::NoContent().await.unwrap(),
    }
}
