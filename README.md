# LemmyRemindYou
This is a Reminder Bot for your Lemmy server. Users can mention it in a comment with a time frame and the bot will remind the user after the timer has run out.

## Usage
### Syntax
``` @YourBot@example.com TIME "MESSAGE" ```

**TIME**, **MESSAGE** and **MENTION** can come in any order.
- **TIME** is required 
- **MESSAGE** is optional
### Time
|          |                 |
| -------- | --------------- |
| years    | year, yr, y     |
| days     | day, d          |
| hours    | hour, h         |
| minutes  | minute, min, m  |
| seconds  | second, sec, s. |

Example: ```@remindme@lemmy.icyserver.eu 4years 2d 3 mins and 2 seconds "Your Reminder"```

## Setup
Run the docker-compose file with username, password and server URL of a bot account.
