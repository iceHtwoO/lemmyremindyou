-- Your SQL goes here
CREATE TABLE reminder(
    id SERIAL,
    post_id int NOT NULL,
    parent_id int NOT NULL,
    content varchar(255) NOT NULL,
    user_message varchar(255),
    post_timestamp bigint NOT NULL,
    reminder_timestamp bigint NOT NULL,
    reminded BOOLEAN NOT NULL,
    PRIMARY KEY (ID)
)