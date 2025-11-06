use crate::domain::{
    common::Id,
    model::{feed::Feed, user::User},
};

pub struct Subscription {
    user_id: Id<User>,
    feed_id: Id<Feed>,
}
