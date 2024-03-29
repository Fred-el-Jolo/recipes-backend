use actix_web::web::{Data, Json};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use actix_session::Session;
use serde::{Deserialize, Serialize};

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::orm::tweet_orm::{Tweet, list_tweets, create_tweet};
use crate::DBPool;

#[derive(Debug, Deserialize, Serialize)]
pub struct JSONTweetMessage {
    pub id: Option<i32>,
    pub created_at: Option<String>,
    pub message: Option<String>,
    pub author_id: Option<i32>,
}

/// list 50 last tweets `/tweets`
#[get("/tweets")]
pub async fn list(pool: Data<DBPool>) -> HttpResponse {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let tweets = web::block(move || list_tweets(50, &mut conn)).await.unwrap();

    let json_tweets: Vec<JSONTweetMessage> = tweets.unwrap().results.iter().map(|tweet| JSONTweetMessage {
        id:tweet.id,
        created_at: Some(tweet.created_at.clone()),
        message: Some(tweet.message.clone()),
        author_id: tweet.author_id,
    }).collect();

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(json_tweets)
}

/// create a tweet `/tweets`
#[post("/tweets")]
pub async fn create(session: Session, tweet: Json<JSONTweetMessage>, pool: Data<DBPool>) -> HttpResponse {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let id = match tweet.author_id {
        Some(v) => Some(v),
        None => match session.get::<String>("user_id") {
            Ok(Some(u)) => Some(u.parse::<i32>().unwrap()),
            _ => None,
        },
    };

    println!("Fred there! {:?}", tweet.id);
    println!("Fred there2! {:?}", id);

    let new_tweet = Tweet {
        id: None,
        created_at: Utc::now().to_string(),
        message: tweet.message.as_ref().unwrap().clone(),
        author_id: id,
    };

    let tweet = web::block(move || create_tweet(new_tweet, &mut conn)).await;

    match tweet {
        Ok(Ok(tweet)) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(tweet),
        _ => HttpResponse::NoContent().await.unwrap(),
    }
}
