-- 0001_init.sql (SQLite)

-- ===== Category Groups =====
CREATE TABLE IF NOT EXISTS category_groups (
  id            INTEGER PRIMARY KEY,
  global_id     TEXT NOT NULL UNIQUE,
  name          TEXT NOT NULL,
  note          TEXT,
  created_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  deleted_at    TEXT,
  version       INTEGER NOT NULL DEFAULT 1
);

-- Enforce unique name among active rows (deleted_at IS NULL)
CREATE UNIQUE INDEX IF NOT EXISTS uq_category_groups_name_active
  ON category_groups(name)
  WHERE deleted_at IS NULL;

CREATE TRIGGER IF NOT EXISTS category_groups_aiu_set_updated
AFTER UPDATE ON category_groups
FOR EACH ROW
WHEN NEW.version = OLD.version
BEGIN
  UPDATE category_groups
     SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ','now'),
         version    = OLD.version + 1
   WHERE id = NEW.id;
END;

CREATE INDEX IF NOT EXISTS idx_category_groups_active
  ON category_groups (deleted_at) WHERE deleted_at IS NULL;

-- ===== Categories =====
CREATE TABLE IF NOT EXISTS categories (
  id            INTEGER PRIMARY KEY,
  global_id     TEXT NOT NULL UNIQUE,
  name          TEXT NOT NULL,
  note          TEXT,
  group_id      INTEGER NOT NULL,
  created_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  deleted_at    TEXT,
  version       INTEGER NOT NULL DEFAULT 1,
  FOREIGN KEY (group_id) REFERENCES category_groups(id) ON UPDATE RESTRICT ON DELETE RESTRICT
);

-- Enforce unique (group_id, name) among active rows
CREATE UNIQUE INDEX IF NOT EXISTS uq_categories_group_name_active
  ON categories(group_id, name)
  WHERE deleted_at IS NULL;

CREATE TRIGGER IF NOT EXISTS categories_aiu_set_updated
AFTER UPDATE ON categories
FOR EACH ROW
WHEN NEW.version = OLD.version
BEGIN
  UPDATE categories
     SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ','now'),
         version    = OLD.version + 1
   WHERE id = NEW.id;
END;

CREATE INDEX IF NOT EXISTS idx_categories_group ON categories (group_id);
CREATE INDEX IF NOT EXISTS idx_categories_active
  ON categories (deleted_at) WHERE deleted_at IS NULL;

-- ===== Entries =====
CREATE TABLE IF NOT EXISTS entries (
  id            INTEGER PRIMARY KEY,
  global_id     TEXT NOT NULL UNIQUE,
  payee         TEXT NOT NULL,
  start_time    TEXT NOT NULL,
  end_time      TEXT,
  duration_ms   INTEGER GENERATED ALWAYS AS (
                   CASE
                     WHEN end_time IS NULL THEN NULL
                     ELSE CAST((julianday(end_time) - julianday(start_time)) * 86400000 AS INTEGER)
                   END
                 ) STORED,
  memo          TEXT,
  category_id   INTEGER NOT NULL,
  created_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  deleted_at    TEXT,
  version       INTEGER NOT NULL DEFAULT 1,
  FOREIGN KEY (category_id) REFERENCES categories(id) ON UPDATE RESTRICT ON DELETE RESTRICT,
  CHECK (end_time IS NULL OR julianday(end_time) >= julianday(start_time))
);

CREATE TRIGGER IF NOT EXISTS entries_aiu_set_updated
AFTER UPDATE ON entries
FOR EACH ROW
WHEN NEW.version = OLD.version
BEGIN
  UPDATE entries
     SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ','now'),
         version    = OLD.version + 1
   WHERE id = NEW.id;
END;

CREATE INDEX IF NOT EXISTS idx_entries_category ON entries (category_id);
CREATE INDEX IF NOT EXISTS idx_entries_start ON entries (start_time);
CREATE INDEX IF NOT EXISTS idx_entries_active
  ON entries (deleted_at) WHERE deleted_at IS NULL;