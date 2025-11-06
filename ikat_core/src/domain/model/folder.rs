use crate::domain::common::Id;
use crate::domain::model::user::User;

pub struct Folder {
    id: Id<Folder>,
    name: String,
    user_id: Id<User>,
    parent_id: Option<Id<Folder>>,
}
