INSERT INTO users
VALUES
    (0, "Bob"),
    (1, "Alice")
;

INSERT INTO groups
VALUES
    (0, "Stanford Space Initiative", "ssi@stanford.edu", "http://stanfordssi.org", NULL, "The Stanford Student Space initiative makes pointy ends go up and flamey ends go down", 3, "Monday"),
    (1, "Stanford Women in Business", NULL, "Stanford Women in business is for all those who want to dominate silicon valley", NULL, NULL, 3, "Monday"),
    (2, "Stanford Polo Club", NULL, "Horses, mallets, fields, what more could you want?", NULL, NULL, 2, "Monday"),
    (3, "Stanford FashionX", NULL, "Because your outfits are bad, so we show you how its done", NULL, NULL, 3, "Monday"),
    (4, "Stanford Women in Design", NULL, "", NULL, NULL, 3, "Monday")
;

INSERT INTO announcements
VALUES
    (0, 0, "jsiskind@stanford.edu", "2022-09-27 12:12:00Z", "Welcome!", "Welcome", "Welcome to SSI y'all!"),
    (1, 0, "jsiskind@stanford.edu", "2022-09-27 12:13:00Z", "Get to work", "Get cracking", "Now that y'all have had some time to settle in to your new roles at SSI, we really ought to have finished this mars mission last week. Get crackin'"),
    (2, 0, "njain@stanford.edu", "2022-09-26 01:00:00Z", "Starship!", "Starship watch party!", "Hello all, SSI will be hosting a watch party for the third starship test flight. We'd love if you'd join!"),
    (3, 0, "emilyredmond@stanford.edu", "2023-09-26 01:00:00Z", "[swibboard23] W6 SWIB This Week ", "[swibboard23] W6 SWIB This Week ", "Hi SWIB,

Wow, what a week of SWIB! We welcomed SWIB and Stanford alums Trinity and Avery from McKinsey, learning you can enter consulting from a CS, Mechanical Engineering, or History background – anything is possible! To close out our week, we had our Sequoia trek with SWID – thank you to Chi and Ines for leading! Read on to see what to look forward to this week in SWIB. Wishing you all luck for mid-quarter exams! 🤓❣️

This Week:
💼 Board Meeting: DaVita Sponsor Session  – Tuesday, February 13th, 7-8pm  @ WCC
DaVita is one of our leading and long-term sponsors. They are a Fortune 200 company at the intersection of Health and Business.
We will have a special Galentine’s Day surprise for those who come to meeting! 😍

🥘 Alpine Investors Dinner – Thursday, February 15th, 6:30pm @ Bird Dog, Palo Alto
Alpine Investors is a leading private equity firm based out of San Francisco with over $16B AUM. They are one of our most generous sponsors and there are many SWIB members (past and present!) who have worked with the firm. ")
;

INSERT INTO user_announcements
VALUES
    (0, 0, 1),
    (1, 0, 0),
    (0, 1, 0),
    (1, 1, 0),
    (0, 2, 0),
    (1, 2, 0),

    (1, 3, 1)
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