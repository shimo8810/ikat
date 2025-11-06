use crate::domain::{
    common::{Id, Url},
    model::{feed::Feed, user::User},
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PublishedAt(chrono::DateTime<Utc>);

pub struct Article {
    id: Id<Article>,
    title: String,
    url: Url,
    feed_id: Id<Feed>,
    published_at: PublishedAt,
}
