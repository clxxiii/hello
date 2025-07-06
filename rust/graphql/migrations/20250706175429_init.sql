-- Create Tables --
CREATE TABLE User (id INTEGER PRIMARY KEY, username TEXT NOT NULL);

CREATE TABLE Post (
    id INTEGER PRIMARY KEY,
    content TEXT NOT NULL DEFAULT '',
    author_id INTEGER NOT NULL,
    FOREIGN KEY (author_id) REFERENCES User (id)
);

CREATE TABLE Like (
    user_id TEXT NOT NULL,
    post_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES User (id),
    FOREIGN KEY (post_id) REFERENCES Post (id)
);

-- Add example data --
INSERT INTO
    User (id, username)
VALUES
    (0, 'DJ Khaled'),
    (1, 'clxxiii');

INSERT INTO
    Post (content, author_id)
VALUES
    ('The key is to make it!', 0),
    ('They call me exparagus', 0),
    ('Life is like roblox', 0),
    ('And what is this called?', 0),
    ('I like this DJ Khaled guy', 1);
