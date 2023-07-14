use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct Login{
    pub jwt: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GetMention{
    pub mentions: Vec<Mention>
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Mention{
    pub person_mention: PersonMention,
    pub comment: Comment
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PersonMention{
    pub id: i32,
    pub recipient_id: i32,
    pub comment_id: i32,
    pub read: bool,
    pub published: String

}

#[derive(Serialize, Deserialize, Clone)]
pub struct Comment{
    pub id: i32,
    pub creator_id: i32,
    pub post_id: i32,
    pub content: String,
    pub removed: bool,
    pub published: String,
    pub deleted: bool,
    pub ap_id: String,
    pub local: bool,
    pub path: String,
    pub distinguished: bool,
    pub language_id: u8
}