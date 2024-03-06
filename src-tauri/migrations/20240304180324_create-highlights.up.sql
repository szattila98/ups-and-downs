CREATE TABLE highlight (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	content TEXT NOT NULL CHECK (length(content) > 1 AND length(content) < 2000),
	kind TEXT NOT NULL CHECK (kind = 'BEST' OR kind = 'WORST'),
	created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at DATETIME
);
