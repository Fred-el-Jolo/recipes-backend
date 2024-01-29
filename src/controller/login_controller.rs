use actix_web::web::{Data, Json};
use actix_web::{web, HttpResponse};
use actix_session::Session;
use serde::{Deserialize, Serialize};

use crate::constants::{APPLICATION_HTML, CONNECTION_POOL_ERROR};
use crate::orm::user_orm::get_user;
use crate::DBPool;

#[derive(Debug, Deserialize, Serialize)]
pub struct JSONLogin {
    pub login: String,
}

/// create a tweet `/tweets`
#[post("/login")]
pub async fn login(session: Session, input: Json<JSONLogin>, pool: Data<DBPool>) -> HttpResponse {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let user = web::block(move || get_user(input.login.clone(), &mut conn)).await.unwrap();

    match user {
        Ok(user) => {
            let _ = session.insert("user_id", user.id.unwrap().to_string());
            HttpResponse::Created()
            .content_type(APPLICATION_HTML)
            .body(format!("<p>Logged in !!! id= {}</p>", user.id.unwrap()))
        },
        _ => HttpResponse::Unauthorized().await.unwrap(),
    }
}
