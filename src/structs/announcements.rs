//! School notices (announcements) data types.

use serde::Deserialize;

use crate::serde_helpers::string_or_int;

/// Response containing school notices (announcements).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseSchoolNotices {
    /// List of school notices.
    #[serde(rename = "SchoolNotices")]
    pub school_notices: Vec<SchoolNotice>,
    /// Related API resources.
    pub resources: Option<SchoolNoticesResources>,
    /// API URL for this response.
    pub url: String,
}

/// A school notice (announcement).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SchoolNotice {
    /// Notice ID (can be string or integer in API response).
    #[serde(deserialize_with = "string_or_int")]
    pub id: String,
    /// Start date of the notice.
    pub start_date: String,
    /// End date of the notice.
    pub end_date: String,
    /// Notice subject/title.
    pub subject: String,
    /// Notice content/body.
    pub content: String,
    /// Author reference.
    pub added_by: SchoolNoticeAddedBy,
    /// Creation date.
    pub creation_date: String,
    /// Whether the notice was read by the user.
    pub was_read: bool,
}

/// Reference to the author of the notice.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SchoolNoticeAddedBy {
    /// Author ID (can be string or integer in API response).
    #[serde(deserialize_with = "string_or_int")]
    pub id: String,
    /// API URL for the author.
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct SchoolNoticesResources {
    #[serde(rename = "..")]
    pub empty: SchoolNoticesUrl,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SchoolNoticesUrl {
    pub url: String,
}
