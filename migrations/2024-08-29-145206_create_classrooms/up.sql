-- Your SQL goes here

CREATE TABLE classrooms (
    classroom_id INTEGER NOT NULL PRIMARY KEY AUTO_INCREMENT,
    classroom_name VARCHAR(255) NOT NULL,
    classroom_status BOOLEAN DEFAULT TRUE
)