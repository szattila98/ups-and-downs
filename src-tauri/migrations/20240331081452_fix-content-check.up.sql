CREATE TABLE new_table (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	content TEXT NOT NULL CHECK (length(content) > 0 AND length(content) <= 2000),
	kind TEXT NOT NULL CHECK (kind = 'BEST' OR kind = 'WORST'),
	created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at DATETIME
);

INSERT INTO new_table (id, content, kind, created_at, updated_at)
	SELECT id, content, kind, created_at, updated_at
	FROM highlight;

DROP TABLE highlight;

ALTER TABLE new_table RENAME TO highlight;
