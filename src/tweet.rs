use actix_web::web::{Data, Json};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use diesel::result::Error;
use diesel::{ExpressionMethods, Insertable, Queryable, Selectable, QueryDsl, RunQueryDsl, QueryResult};
use serde::{Deserialize, Serialize};

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::response::Response;
use crate::{DBPool, DBPooledConnection};


pub type Tweets = Response<Tweet>;

#[derive(Debug, Deserialize, Serialize)]
pub struct JSONTweet {
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::tweets)]
pub struct Tweet {
    pub id: i32,
    pub created_at: String,
    pub message: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::tweets)]
pub struct NewTweet {
    pub created_at: String,
    pub message: String,
}

fn list_tweets(total_tweets: i64, conn: &mut DBPooledConnection) -> Result<Tweets, Error> {
    use crate::schema::tweets::dsl::*;

    let _tweets = match tweets
        .order(created_at.desc())
        .limit(total_tweets)
        .load::<Tweet>(conn)
    {
        Ok(tws) => tws,
        Err(_) => vec![],
    };

    Ok(Tweets {
        results: _tweets,
    })
}

// fn find_tweet(_id: i32, conn: &mut DBPooledConnection) -> Result<Tweet, Error> {
//     use crate::schema::tweets::dsl::*;

//     let res = tweets.filter(id.eq(_id)).load::<TweetDB>(conn);
//     match res {
//         Ok(tweets_db) => match tweets_db.first() {
//             Some(tweet_db) => Ok(tweet_db.to_tweet()),
//             _ => Err(Error::NotFound),
//         },
//         Err(err) => Err(err),
//     }
// }

fn create_tweet(tweet: NewTweet, conn: &mut DBPooledConnection) -> Result<Tweet, Error> {
    use crate::schema::tweets::dsl::*;

    let result: QueryResult<Tweet> = diesel::insert_into(tweets).values(&tweet).get_result::<Tweet>(conn);

    result
}

// fn delete_tweet(_id: i32, conn: &mut DBPooledConnection) -> Result<(), Error> {
//     use crate::schema::tweets::dsl::*;

//     let res = diesel::delete(tweets.filter(id.eq(_id))).execute(conn);
//     match res {
//         Ok(_) => Ok(()),
//         Err(err) => Err(err),
//     }
// }

/// list 50 last tweets `/tweets`
#[get("/tweets")]
pub async fn list(pool: Data<DBPool>) -> HttpResponse {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let tweets = web::block(move || list_tweets(50, &mut conn)).await.unwrap();

    let tweets_with_likes = Tweets {
        results: tweets
            .unwrap()
            .results,
    };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tweets_with_likes)
}

/// create a tweet `/tweets`
#[post("/tweets")]
pub async fn create(tweet: Json<JSONTweet>, pool: Data<DBPool>) -> HttpResponse {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let new_tweet = NewTweet{
        created_at: Utc::now().to_string(),
        message: tweet.message.clone(),
    };

    let tweet = web::block(move || create_tweet(new_tweet, &mut conn)).await;

    match tweet {
        Ok(Ok(tweet)) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(tweet),
        _ => HttpResponse::NoContent().await.unwrap(),
    }
}

// /// find a tweet by its id `/tweets/{id}`
// #[get("/tweets/{id}")]
// pub async fn get(path: Path<(String,)>, pool: Data<DBPool>) -> HttpResponse {
//     let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
//     let tweet =
//         web::block(move || find_tweet(path.0.parse::<i32>().unwrap(), &mut conn)).await;

//     match tweet {
//         Ok(Ok(tweet)) => {
//             HttpResponse::Ok()
//                 .content_type(APPLICATION_JSON)
//                 .json(tweet)
//         }
//         _ => HttpResponse::NoContent()
//             .content_type(APPLICATION_JSON)
//             .await
//             .unwrap(),
//     }
// }

// /// delete a tweet by its id `/tweets/{id}`
// #[delete("/tweets/{id}")]
// pub async fn delete(path: Path<(String,)>, pool: Data<DBPool>) -> HttpResponse {
//     // in any case return status 204
//     let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);

//     let _ = web::block(move || delete_tweet(path.0.parse::<i32>().unwrap(), &mut conn)).await;

//     HttpResponse::NoContent()
//         .content_type(APPLICATION_JSON)
//         .await
//         .unwrap()
// }