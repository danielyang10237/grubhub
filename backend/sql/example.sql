INSERT INTO users
VALUES
    (0, "Bob"),
    (1, "Alice")
;

INSERT INTO groups
VALUES
    (0, "Stanford Space Initiative", "ssi@stanford.edu", "stanfordssi.org", NULL, NULL),
    (1, "Stanford Women in Business", NULL, NULL, NULL, NULL),
    (2, "Stanford Polo Club", NULL, NULL, NULL, NULL),
    (3, "Stanford Fashion X", NULL, NULL, NULL, NULL),
    (4, "Stanford Women in Design", NULL, NULL, NULL, NULL)
;

INSERT INTO announcements
VALUES
    (0, 0, "Welcome!", "Welcome to SSI y'all!")
;

INSERT INTO user_announcements
VALUES
    (0, 0, 1),
    (1, 0, 0)
;

INSERT INTO group_tags
VALUES
    (0, "Science"),
    (0, "Math"),
    (0, "Engineering")
;

INSERT INTO group_members
VALUES
    (0, 0, 3),
    (0, 1, 2),
    (1, 1, 1)
;