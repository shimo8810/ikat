use crate::domain::common::Id;

struct UserName(String);

pub struct User {
    id: Id<User>,
    name: UserName,
}
