use crate::domain::{
    common::{Id, Url},
    model::{article::Article, feed::Feed, user::User},
};

pub struct ArticleStates {
    user_id: Id<User>,
    article_id: Id<Article>,
    is_read: bool,
    is_favorite: bool,
}
