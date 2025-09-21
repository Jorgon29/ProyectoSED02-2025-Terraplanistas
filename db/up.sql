CREATE TABLE SONGS(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username TEXT,
    password TEXT
);

