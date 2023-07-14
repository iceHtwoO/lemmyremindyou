use diesel::prelude::*;
use diesel::Queryable;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::reminder)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Reminder {
    pub id: i32,
    pub post_id: i32,
    pub parent_id: i32,
    pub content: String,
    pub user_message: Option<String>,
    pub post_timestamp: i64,
    pub reminder_timestamp:i64,
    pub reminded: bool,
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::reminder)]
pub struct NewReminder {
    pub post_id: i32,
    pub parent_id: i32,
    pub content: String,
    pub user_message: Option<String>,
    pub post_timestamp: i64,
    pub reminder_timestamp:i64,
    pub reminded: bool,
}