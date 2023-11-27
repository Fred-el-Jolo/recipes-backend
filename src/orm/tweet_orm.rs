use diesel::result::Error;
use diesel::{ExpressionMethods, Insertable, Queryable, Selectable, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::response::Response;
use crate::DBPooledConnection;

pub type Tweets = Response<Tweet>;

#[derive(Debug, Deserialize, Serialize, Queryable, Selectable, Insertable)]
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

pub fn list_tweets(total_tweets: i64, conn: &mut DBPooledConnection) -> Result<Tweets, Error> {
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

pub fn create_tweet(tweet: NewTweet, conn: &mut DBPooledConnection) -> Result<Tweet, Error> {
    use crate::schema::tweets::dsl::*;
    
    diesel::insert_into(tweets).values(&tweet).get_result::<Tweet>(conn)
}
