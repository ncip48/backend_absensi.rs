-- Your SQL goes here
CREATE TABLE users (
    user_id INTEGER NOT NULL PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR(100) NOT NULL,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL,
    password VARCHAR(122) NOT NULL,
    role TINYINT NOT NULL
);

insert into
    users (
        username,
        name,
        email,
        password,
        role
    )
values (
        'admin',
        'Admin',
        'admin@gmail.com',
        '12345678',
        0
    ),
    (
        'kepsek',
        'Kepsek',
        'kepsek@gmail.com',
        '12345678',
        1
    ),
    (
        'guru',
        'Guru',
        'guru@gmail.com',
        '12345678',
        2
    );