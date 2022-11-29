-- Your SQL goes here
ALTER TABLE feeds ADD description TEXT;
ALTER TABLE feeds ADD language TEXT;
DROP TABLE tags;

CREATE TABLE links(
	id SERIAL PRIMARY KEY,
	href TEXT,
	title TEXT,
	feed TEXT,
	post TEXT,
	CONSTRAINT fk_feed FOREIGN KEY (feed) REFERENCES feeds (id),
	CONSTRAINT fk_post FOREIGN KEY (post) REFERENCES posts (id)
);

CREATE TABLE categories (
	id SERIAL PRIMARY KEY,
	term TEXT NOT NULL,
	label TEXT,
	feed TEXT,
	post TEXT,
	CONSTRAINT fk_feed FOREIGN KEY (feed) REFERENCES feeds (id),
	CONSTRAINT fk_post FOREIGN KEY (post) REFERENCES posts (id)
);


CREATE TABLE medias (
	id SERIAL PRIMARY KEY,
	uri TEXT,
	title TEXT,
	mime TEXT,
	description TEXT,
	post TEXT,
	CONSTRAINT fk_post FOREIGN KEY (post) REFERENCES posts (id)
);

CREATE TABLE captions (
	id SERIAL PRIMARY KEY,
	content TEXT NOT NULL,
	start_time INTERVAL,
	end_time INTERVAL,
	media SERIAL,
	CONSTRAINT fk_media FOREIGN KEY (media) REFERENCES medias (id)
);

CREATE TABLE images (
	id SERIAL PRIMARY KEY,
	uri TEXT NOT NULL,
	title TEXT,
	description TEXT,
	media INTEGER,
	feed TEXT,
	CONSTRAINT fk_feed FOREIGN KEY (feed) REFERENCES feeds (id),
	CONSTRAINT fk_media FOREIGN KEY (media) REFERENCES medias (id)
);

ALTER TABLE posts ADD content TEXT;
ALTER TABLE posts ADD summary TEXT;
