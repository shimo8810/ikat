use crate::domain::common::{Id, Url};
use crate::domain::model::folder::Folder;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct LastFetchedAt(chrono::DateTime<Utc>);

pub struct Feed {
    id: Id<Feed>,
    name: String,
    rss_url: Url,
    website_url: Url,
    folder_id: Id<Folder>,
    last_fetched_at: LastFetchedAt,
}
