CREATE TABLE users (
	username VARCHAR(255) PRIMARY KEY,
	password VARCHAR(255) NOT NULL,

	failed_time BIGINT,
	failed_msg VARCHAR(255)
);

CREATE TABLE sessions (
	id BIGINT PRIMARY KEY,
	user VARCHAR(255) NOT NULL,
	time BIGINT NOT NULL,
	FOREIGN KEY(user) REFERENCES users(username)
);

CREATE TABLE groups (
	id BIGINT PRIMARY KEY,
	name VARCHAR(255) NOT NULL,
	founder VARCHAR(255) NOT NULL,
	FOREIGN KEY(founder) REFERENCES users(username)
);

CREATE TABLE group_membership (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	user VARCHAR(255) NOT NULL,
	groupid BIGINT NOT NULL,
	FOREIGN KEY(user) REFERENCES users(username),
	FOREIGN KEY(groupid) REFERENCES groups(id)
);