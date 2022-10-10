-- Your SQL goes here
CREATE TABLE users (
	username TEXT PRIMARY KEY,
	display_name TEXT,
	hash TEXT,
	salt TEXT
);

CREATE TABLE sessionid (
	id TEXT PRIMARY KEY,
	username TEXT NOT NULL,
	last_active TIMESTAMP,
	name TEXT,
	CONSTRAINT fk_user FOREIGN KEY (username) REFERENCES users(username)
);

CREATE TABLE follows (
	follower TEXT NOT NULL,
	followed TEXT NOT NULL,
	CONSTRAINT pk_following PRIMARY KEY (follower, followed),
	CONSTRAINT fk_follower FOREIGN KEY (follower) REFERENCES users(username),
	CONSTRAINT fk_followed FOREIGN KEY (followed) REFERENCES users(username)
);

CREATE TABLE feeds (
	id TEXT PRIMARY KEY,
	url TEXT NOT NULL,
	title TEXT,
	last_updated TIMESTAMP
);

CREATE TABLE posts (
	id TEXT PRIMARY KEY,
	url TEXT NOT NULL,
	title TEXT,
	description TEXT,
	feed_id TEXT,
	time TIMESTAMP NOT NULL,
	CONSTRAINT fk_feed FOREIGN KEY (feed_id) REFERENCES feeds(id)
);

CREATE TABLE shares (
	post_id TEXT,
	username TEXT,
	user_comment TEXT,
	time TIMESTAMP NOT NULL,
	CONSTRAINT pk_shares PRIMARY KEY (post_id, username),
	CONSTRAINT fk_user FOREIGN KEY (username) REFERENCES users(username),
	CONSTRAINT fk_post FOREIGN KEY (post_id) REFERENCES posts(id)
);

CREATE TABLE subscriptions (
	feed_id TEXT NOT NULL,
	username TEXT NOT NULL,
	CONSTRAINT pk_subscriptions PRIMARY KEY (feed_id, username),
	CONSTRAINT fk_user FOREIGN KEY (username) REFERENCES users(username),
	CONSTRAINT fk_feed FOREIGN KEY (feed_id) REFERENCES feeds(id)
);

CREATE TABLE tags (
	id SERIAL PRIMARY KEY,
	tag TEXT NOT NULL,
	username TEXT NOT NULL,
	post_id TEXT NOT NULL,
	CONSTRAINT fk_user FOREIGN KEY (username) REFERENCES users(username),
	CONSTRAINT fk_post FOREIGN KEY (post_id) REFERENCES posts(id)
);


