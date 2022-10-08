-- Your SQL goes here
CREATE TABLE users (
	username TEXT PRIMARY KEY,
	display_name TEXT,
	pfp TEXT,
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

CREATE TABLE following (
	username TEXT NOT NULL,
	following TEXT NOT NULL,
	CONSTRAINT pk_following PRIMARY KEY (username, following),
	CONSTRAINT fk_user FOREIGN KEY (username) REFERENCES users(username),
	CONSTRAINT fk_userfollowing FOREIGN KEY (following) REFERENCES users(username)
);

CREATE TABLE feeds (
	url TEXT PRIMARY KEY,
	title TEXT
);

CREATE TABLE posts (
	url TEXT PRIMARY KEY,
	title TEXT,
	description TEXT,
	feed_url TEXT,
	CONSTRAINT fk_feed FOREIGN KEY (feed_url) REFERENCES feeds(url)
);

CREATE TABLE shares (
	post_url TEXT,
	username TEXT,
	CONSTRAINT pk_shares PRIMARY KEY (post_url, username),
	CONSTRAINT fk_user FOREIGN KEY (username) REFERENCES users(username),
	CONSTRAINT fk_post FOREIGN KEY (post_url) REFERENCES posts(url)
);

CREATE TABLE subscriptions (
	feed_url TEXT NOT NULL,
	username TEXT NOT NULL,
	CONSTRAINT pk_subscriptions PRIMARY KEY (feed_url, username),
	CONSTRAINT fk_user FOREIGN KEY (username) REFERENCES users(username),
	CONSTRAINT fk_feed FOREIGN KEY (feed_url) REFERENCES feeds(url)
);


