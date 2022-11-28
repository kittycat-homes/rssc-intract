-- This file should undo anything in `up.sql`
ALTER TABLE posts DROP COLUMN summary;
ALTER TABLE posts DROP COLUMN content;

DROP TABLE images;
DROP TABLE captions;
DROP TABLE medias;
DROP TABLE categories;
DROP TABLE links;

CREATE TABLE tags (
	id SERIAL PRIMARY KEY,
	value TEXT NOT NULL,
	username TEXT NOT NULL,
	post_id TEXT NOT NULL,
	CONSTRAINT fk_user FOREIGN KEY (username) REFERENCES users(username),
	CONSTRAINT fk_post FOREIGN KEY (post_id) REFERENCES posts(id)
);

ALTER TABLE feeds DROP COLUMN language;
ALTER TABLE feeds DROP COLUMN description;
