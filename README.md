# LemmyRemindYou
![Static Badge](https://img.shields.io/badge/Docker-latest-0db7ed?style=flat&logo=docker&logoColor=0db7ed&link=https%3A%2F%2Fhub.docker.com%2Fr%2Ficeh2%2Flemmyremindyou)

This is a Reminder Bot for your Lemmy server. By simply mentioning the bot in a comment along with a desired time frame, users can set reminders and receive notifications once the specified timer expires.

## Usage
### Syntax
``` @YourBot@example.com TIME OPTION "MESSAGE" ```

**TIME**, **MESSAGE** **OPTION** and **MENTION** can come in any order.
- **TIME** is required 
- **MESSAGE** is optional
### Time
|          |                 |
| -------- | --------------- |
| years    | year, yr, y     |
| days     | day, d          |
| hours    | hour, h         |
| minutes  | minute, min, m  |
| seconds  | second, sec, s  |

Example: ```@remindme@lemmy.icyserver.eu 4years 2d 3 mins and 2 seconds "Your Reminder"```

### Options
|          |                 |
| -------- | --------------- |
| iso      | this will retrun the date and time in iso formation (YYYY-MM-DD at HH:MM:SS) |


## Setup
Run the docker-compose file with following Environment variables
| ENV             | DESCRIPTION                  |
| --------------- | ---------------------------- | 
| USERNAME        | username of bot account      |
| PASSWORD        | password of bot account      |
| HOST            | Lemmy instance domain        |
| DATABASE_URL    | postgres database url        |
| UPDATE_INTERVAL | By default, the bot scans for new mentions with a waiting time of 30 seconds. Reducing this interval can enhance response time, but it will also increase the load on the system |
