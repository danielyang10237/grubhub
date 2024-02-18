CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

CREATE TABLE groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT,
    url TEXT,
    description TEXT,
    profile_photo_url TEXT
);

CREATE TABLE events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL
);

CREATE TABLE announcements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    group_id REFERENCES groups(id),
    title TEXT NOT NULL,
    body TEXT NOT NULL
);

CREATE TABLE interests (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

CREATE TABLE group_members (
    group_id REFERENCES groups(id),
    user_id REFERENCES users(id),
    commitment INTEGER
);

CREATE TABLE group_events (
    group_id REFERENCES groups(id),
    event_id REFERENCES events(id)
);

CREATE TABLE user_interests (
    user_id REFERENCES users(id),
    interest_id REFERENCES interests(id),
    score INTEGER
);

CREATE TABLE user_announcements (
    user_id REFERENCES users(id),
    announcement_id REFERENCES announcements(id),
    viewed INTEGER
);

CREATE TABLE user_events (
    user_id REFERENCES users(id),
    event_id REFERENCES events(id),
    attended INTEGER,
    rsvp INTEGER
);

CREATE TABLE group_tags (
    group_id REFERENCES groups(id),
    tag TEXT NOT NULL
);