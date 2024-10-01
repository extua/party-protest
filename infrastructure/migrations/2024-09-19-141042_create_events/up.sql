CREATE TABLE events (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "name" VARCHAR NOT NULL,
  "description" TEXT NOT NULL,
  start_time INTEGER NOT NULL,
  end_time INTEGER,
  "location" TEXT
)
