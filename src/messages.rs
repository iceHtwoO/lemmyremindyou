use chrono::{ DateTime, Datelike, Timelike, Utc };
use std::collections::HashMap;
use crate::models::Reminder;

pub async fn create_reminder_message(dt: DateTime<Utc>, reminder: &Reminder) -> String {
    let dt_string = get_date_time_string(dt, &reminder.content);

    let mut message = format!(
        "Here is your reminder for {} the {} UTC ",
        dt.weekday(),
        dt_string.await
    );

    if reminder.user_message.is_some() {
        message.push_str(
            format!("with the message:\"{}\"", reminder.user_message.as_ref().unwrap()).as_str()
        );
    }
    message
}

pub async fn create_conformation_message(dt: DateTime<Utc>, content: &String) -> String {
    let dt_string = get_date_time_string(dt, content).await;

    format!(
        "Okay I'll remind you on {} the {} UTC",
        dt.weekday(),
        dt_string
    )
}

pub async fn invalid_request_string() -> String{
    "### Invalid Request\n
    **year**: x (year|yr|y)\n
    **days**: x (day|d)\n
    **hour**: x (hour|h)\n
    **minute**: x (minute|min|m)\n
    **second**: x (second|sec|s)\n
    Example: 4years 2d 3 mins and 2 seconds \"Your Reminder\"".to_string()
}

async fn get_date_time_string(dt: DateTime<Utc>, content: &String) -> String{
    match (){
        () if content.to_lowercase().contains("iso") => date_time_to_iso_string(dt).await,
        _ => date_time_to_ordinal_string(dt).await
    }
}

async fn date_time_to_iso_string(dt: DateTime<Utc>) -> String {
    format!(
        "{}-{}-{} at {}:{}:{}",
        dt.year(),
        dt.month(),
        dt.day(),
        dt.hour(),
        dt.minute(),
        dt.second()
    )
}

async fn date_time_to_ordinal_string(dt: DateTime<Utc>) -> String {
    format!("{} of {}, {} at {}", day_to_ordinal(&dt.day()).await, month_to_ordinal(&dt.month()).await, dt.year(), time_to_ordinal(dt).await)
}

async fn time_to_ordinal(dt: DateTime<Utc>) -> String{
    format!("{}:{} {}", dt.hour12().1, dt.minute(), match dt.hour12().0{false => "AM", true => "PM"})
}

async fn month_to_ordinal(month: &u32) -> String{
    let months = HashMap::from([
        (1, "January"),
        (2, "Feburary"),
        (3, "March"),
        (4, "April"),
        (5, "May"),
        (6, "June"),
        (7, "July"),
        (8, "August"),
        (9, "September"),
        (10, "October"),
        (11, "November"),
        (12, "December")
    ]);
    months.get(month).unwrap().to_string()
}

async fn day_to_ordinal(day: &u32) -> String {
    match day {
        1 => "1st".to_string(),
        2 => "2nd".to_string(),
        3 => "3rd".to_string(),
        _ => format!("{}th", day),
    }
}