INSERT INTO users
VALUES
    (0, "Bob"),
    (1, "Alice")
;

INSERT INTO groups
VALUES
    (0, "Stanford Space Initiative", "ssi@stanford.edu", "stanfordssi.org", NULL, NULL, 3, "Monday"),
    (1, "Stanford Women in Business", NULL, NULL, NULL, NULL, 2, "Monday"),
    (2, "Stanford Polo Club", NULL, NULL, NULL, NULL, 1, "Monday"),
    (3, "Stanford FashionX", NULL, NULL, NULL, NULL, 1, "Monday"),
    (4, "Stanford Women in Design", NULL, NULL, NULL, NULL, 1, "Monday")
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
    (0, "Engineering"),
    (0, "Tech"),
    (1, "Entrepreneurship"),
    (1, "Tech"),
    (1, "Design"),
    (2, "Outdoors"),
    (2, "Sports"),
    (3, "Design"),
    (3, "Engineering"),
    (3, "Tech"), 
    (4, "Design")
;

INSERT INTO group_members
VALUES
    (0, 0, 3),
    (0, 1, 2),
    (1, 1, 1)
;