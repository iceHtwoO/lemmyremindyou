#[macro_use]
extern crate diesel_migrations;

use chrono::{ DateTime, Datelike, Duration, NaiveDateTime, TimeZone, Timelike, Utc };
use crate::diesel_migrations::MigrationHarness;
use env_logger::{ Builder, WriteStyle };
use lemmyremindyou::schema::reminder;
use lemmyremindyou::models::*;
use diesel::prelude::*;
use lemmyremindyou::*;
use log::LevelFilter;
use reqwest::Error;
use regex::Regex;
use std::thread;
use log::error;
use log::info;
mod api;
mod dto;

pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations = embed_migrations!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = Builder::new();

    builder.filter(None, LevelFilter::Info).write_style(WriteStyle::Always).init();

    let auth = get_auth_token_retry().await?;

    let pmconnection = &mut establish_connection();
    pmconnection.run_pending_migrations(MIGRATIONS).unwrap();

    let connection = &mut establish_connection();

    loop {
        let response = api::get_mentions(&auth).await?;
        let mention_obj: dto::GetMention = serde_json::from_value(response)?;

        let pm = process_mention(pmconnection, &auth, mention_obj.mentions);
        remind_user(connection, &auth).await?;

        pm.await?;
        thread::sleep(std::time::Duration::from_secs(get_update_interval().await as u64));
    }
}

async fn get_auth_token_retry() -> Result<String, Box<dyn std::error::Error>> {
    for _try in 1..4 {
        match api::get_auth_token().await {
            Ok(t) => {
                return Ok(t);
            }
            Err(e) => {
                error!("{}, waiting 5 seconds", e.to_string());
            }
        }
        thread::sleep(std::time::Duration::from_secs(5));
    }
    Err("Faild to Authenticate 4 Times".into())
}

async fn remind_user(dbcon: &mut PgConnection, auth: &str) -> Result<(), Error> {
    for reminder in get_reminders(dbcon).await {
        let naive = NaiveDateTime::from_timestamp_opt(reminder.reminder_timestamp, 0).expect("Err");
        let dt: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        let mut meassage = format!(
            "Here is your reminder for the {}-{}-{} at {}:{}:{} UTC ",
            dt.year(),
            dt.month(),
            dt.day(),
            dt.hour(),
            dt.minute(),
            dt.second()
        );
        if reminder.user_message.is_some() {
            meassage.push_str(
                format!("with the message:\"{}\"", reminder.user_message.unwrap()).as_str()
            );
        }
        info!("Reminded User!");
        api::comment(auth, meassage.as_str(), reminder.parent_id, reminder.post_id).await?;
        update_reminded_status(dbcon, reminder.id).await;
    }
    Ok(())
}

async fn update_reminded_status(dbcon: &mut PgConnection, update_id: i32) {
    use self::schema::reminder::dsl::*;
    diesel
        ::update(reminder.find(update_id))
        .set(reminded.eq(true))
        .returning(Reminder::as_returning())
        .get_result(dbcon)
        .unwrap();
}

async fn get_update_interval() -> i64 {
    std::env::var("UPDATE_INTERVAL").unwrap_or("30".to_string()).parse::<i64>().unwrap()
}

async fn get_reminders(dbcon: &mut PgConnection) -> Vec<Reminder> {
    use self::schema::reminder::dsl::*;
    let ts: i64 = (
        chrono::offset::Utc::now() + Duration::seconds(get_update_interval().await)
    ).timestamp();

    reminder
        .filter(reminder_timestamp.le(ts))
        .filter(reminded.eq(false))
        .select(Reminder::as_select())
        .load(dbcon)
        .expect("Error loading posts")
}

async fn process_mention(
    dbcon: &mut PgConnection,
    auth: &str,
    mentions: Vec<dto::Mention>
) -> Result<(), Error> {
    for mention in mentions {
        let clone: dto::Mention = mention.clone();
        if !mention.person_mention.read {
            let plublished_dt = &mention.person_mention.published
                .split(".")
                .next()
                .expect("Error Converting publish Date");
            let start_time = Utc.from_utc_datetime(
                &NaiveDateTime::parse_from_str(plublished_dt, "%Y-%m-%dT%H:%M:%S").unwrap()
            );
            let dt = match match_time(mention.comment.content, start_time).await {
                Ok(t) => t,
                Err(_e) => {
                    return invalid_request(auth, clone).await;
                }
            };
            if dt == start_time {
                return invalid_request(auth, clone).await;
            } else {
                let meassage = format!(
                    "Okay I'll remind you on {}-{}-{} {}:{}:{} UTC",
                    dt.year(),
                    dt.month(),
                    dt.day(),
                    dt.hour(),
                    dt.minute(),
                    dt.second()
                );
                api::comment(&auth, &meassage, mention.comment.id, mention.comment.post_id).await?;
                api::mark_read(&auth, mention.person_mention.id).await?;
                save_request(dbcon, clone, dt.timestamp(), start_time.timestamp()).await;
                info!("Request Saved {}", dt);
            }
        }
    }
    Ok(())
}

async fn invalid_request(auth: &str, mention: dto::Mention) -> Result<(), Error> {
    let message =
        "### Invalid Request\n
                        **year**: x (year|yr|y)\n
                        **days**: x (day|d)\n
                        **hour**: x (hour|h)\n
                        **minute**: x (minute|min|m)\n
                        **second**: x (second|sec|s)\n
                        Example: 4years 2d 3 mins and 2 seconds \"Your Reminder\"";

    api::comment(&auth, message, mention.comment.id, mention.comment.post_id).await?;
    api::mark_read(&auth, mention.person_mention.id).await?;
    info!("Recived Invalid Reqiest");
    Ok(())
}

async fn save_request(
    dbcon: &mut PgConnection,
    mention: dto::Mention,
    reminder_timestamp: i64,
    post_timestamp: i64
) {
    let msg = extract_message(mention.clone().comment.content).await;

    let new_reminder = NewReminder {
        post_id: mention.comment.post_id,
        parent_id: mention.comment.id,
        content: mention.comment.content,
        user_message: msg,
        post_timestamp: post_timestamp,
        reminder_timestamp: reminder_timestamp,
        reminded: false,
    };

    diesel
        ::insert_into(reminder::table)
        .values(&new_reminder)
        .returning(Reminder::as_returning())
        .get_result(dbcon)
        .expect("Error Saving Reminder");
}

async fn extract_message(content: String) -> Option<String> {
    let message_regex: Regex = Regex::new(r#""(.*?)""#).unwrap();
    if message_regex.is_match(&content) {
        Some(message_regex.find(content.as_str()).unwrap().as_str().to_string().replace("\"", ""))
    } else {
        None
    }
}

async fn match_time(
    content: String,
    start_time: DateTime<chrono::Utc>
) -> Result<DateTime<Utc>, Box<dyn std::error::Error>> {
    let re = Regex::new(r"(\d+)\s?(year|yr|y|day|d|hour|h|minute|min|m|second|sec|s)").unwrap();
    let mut dt = start_time;
    for (_, [ammount, duration_str]) in re.captures_iter(&content).map(|c| c.extract()) {
        let time = ammount.parse::<i64>()?;
        dt += get_duration(time, duration_str).await;
    }
    Ok(dt)
}

async fn get_duration(ammount: i64, time: &str) -> Duration {
    let year: Regex = Regex::new(r"year|yr|Y").unwrap();
    let day = Regex::new(r"day|d").unwrap();
    let hour = Regex::new(r"hour|h").unwrap();
    let minute = Regex::new(r"minute|min|m").unwrap();
    let seconds = Regex::new(r"second|sec|s").unwrap();
    match () {
        () if year.is_match(&time) => Duration::days(ammount * 365),
        () if day.is_match(&time) => Duration::days(ammount),
        () if hour.is_match(&time) => Duration::hours(ammount),
        () if minute.is_match(&time) => Duration::minutes(ammount),
        () if seconds.is_match(&time) => Duration::seconds(ammount),
        () => Duration::zero(),
    }
}
