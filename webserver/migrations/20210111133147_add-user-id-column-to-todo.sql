-- Add migration script here
ALTER TABLE todos
ADD COLUMN user_id INTEGER NOT NULL,
  ADD CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id);
