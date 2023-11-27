use actix_web::web::{Data, Json};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::orm::tweet_orm::{NewTweet, list_tweets, create_tweet};
use crate::DBPool;

#[derive(Debug, Deserialize, Serialize)]
pub struct JSONTweetMessage {
    pub id: Option<i32>,
    pub created_at: Option<String>,
    pub message: Option<String>,
}

/// list 50 last tweets `/tweets`
#[get("/tweets")]
pub async fn list(pool: Data<DBPool>) -> HttpResponse {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let tweets = web::block(move || list_tweets(50, &mut conn)).await.unwrap();

    let json_tweets: Vec<JSONTweetMessage> = tweets.unwrap().results.iter().map(|tweet| JSONTweetMessage {
        id: Some(tweet.id),
        created_at: Some(tweet.created_at.clone()),
        message: Some(tweet.message.clone()),
    }).collect();

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(json_tweets)
}

/// create a tweet `/tweets`
#[post("/tweets")]
pub async fn create(tweet: Json<JSONTweetMessage>, pool: Data<DBPool>) -> HttpResponse {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let new_tweet = NewTweet{
        created_at: Utc::now().to_string(),
        message: tweet.message.as_ref().unwrap().clone(),
    };

    let tweet = web::block(move || create_tweet(new_tweet, &mut conn)).await;

    match tweet {
        Ok(Ok(tweet)) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(tweet),
        _ => HttpResponse::NoContent().await.unwrap(),
    }
}
