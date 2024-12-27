CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE IF NOT EXISTS user_permissions (
  user_id    INTEGER NOT NULL REFERENCES users(id),
  token      TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS csrf_tokens (
  csrf_token TEXT NOT NULL PRIMARY KEY UNIQUE
);

CREATE TABLE IF NOT EXISTS google_tokens (
  id              BIGSERIAL PRIMARY KEY,
  user_id         INTEGER NOT NULL UNIQUE REFERENCES users(id),
  access_secret   TEXT NOT NULL,
  refresh_secret  TEXT NOT NULL,
  created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
