CREATE TABLE users (
	username VARCHAR(255) PRIMARY KEY,
	password VARCHAR(255) NOT NULL,

	failedTime TIMESTAMP,
	failedMsg VARCHAR(255)
);

CREATE TABLE sessions (
	id INTEGER PRIMARY KEY,
	user VARCHAR(255) NOT NULL,
	time TIMESTAMP NOT NULL,
	FOREIGN KEY(user) REFERENCES users(username)
);

CREATE TABLE groups (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name VARCHAR(255) NOT NULL
);

CREATE TABLE groupMembership (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	user VARCHAR(255) NOT NULL,
	groupid INTEGER NOT NULL,
	FOREIGN KEY(user) REFERENCES users(username),
	FOREIGN KEY(groupid) REFERENCES groups(id)
);