-- This file should undo anything in `up.sql`
CREATE TABLE follows (
	follower TEXT NOT NULL,
	followed TEXT NOT NULL,
	CONSTRAINT pk_following PRIMARY KEY (follower, followed),
	CONSTRAINT fk_follower FOREIGN KEY (follower) REFERENCES users(username),
	CONSTRAINT fk_followed FOREIGN KEY (followed) REFERENCES users(username)
);

CREATE TABLE shares (
	post_id TEXT NOT NULL,
	username TEXT NOT NULL,
	user_comment TEXT,
	time TIMESTAMP NOT NULL,
	CONSTRAINT pk_shares PRIMARY KEY (post_id, username),
	CONSTRAINT fk_user FOREIGN KEY (username) REFERENCES users(username),
	CONSTRAINT fk_post FOREIGN KEY (post_id) REFERENCES posts(id)
);
