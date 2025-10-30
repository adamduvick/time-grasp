-- 0001_init.sql
-- Assumptions:
-- - SQLite (sqlx) target.
-- - Timestamps stored as UTC ISO-8601 text.
-- - UUIDv4 stored as TEXT with a generator default.
-- - BIGINT maps to INTEGER (SQLite 64-bit).
-- - Soft delete via setting deleted_at; no hard delete cascade.
-- - Application sets deleted_at on soft delete; triggers handle updated_at/version.

/* =========================
   CATEGORY GROUPS
   ========================= */
CREATE TABLE IF NOT EXISTS category_groups (
  id            INTEGER PRIMARY KEY,                        -- bigint
  global_id     BLOB NOT NULL UNIQUE,                       -- supplied by app (uuid crate)
  name          TEXT NOT NULL,
  note          TEXT,
  created_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  deleted_at    TEXT,                                       -- null = active; non-null = soft-deleted
  version       INTEGER NOT NULL DEFAULT 1,
  -- Keep names unique among active groups; allow duplicates if one is soft-deleted
  UNIQUE (name) FILTER (WHERE deleted_at IS NULL)
);

CREATE TRIGGER IF NOT EXISTS category_groups_aiu_set_updated
AFTER UPDATE ON category_groups
FOR EACH ROW
WHEN NEW.version = OLD.version   -- avoid infinite recursion
BEGIN
  UPDATE category_groups
     SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ','now'),
         version    = OLD.version + 1
   WHERE id = NEW.id;
END;

CREATE INDEX IF NOT EXISTS idx_category_groups_active ON category_groups (deleted_at)
  WHERE deleted_at IS NULL;

/* =========================
   CATEGORIES
   ========================= */
CREATE TABLE IF NOT EXISTS categories (
  id            INTEGER PRIMARY KEY,                        -- bigint
  global_id     BLOB NOT NULL UNIQUE,                       -- supplied by app
  name          TEXT NOT NULL,
  note          TEXT,
  group_id      INTEGER NOT NULL,
  created_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  deleted_at    TEXT,
  version       INTEGER NOT NULL DEFAULT 1,
  FOREIGN KEY (group_id) REFERENCES category_groups(id) ON UPDATE RESTRICT ON DELETE RESTRICT,
  -- Unique category names per group among active rows
  UNIQUE (group_id, name) FILTER (WHERE deleted_at IS NULL)
);

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
CREATE INDEX IF NOT EXISTS idx_categories_active ON categories (deleted_at)
  WHERE deleted_at IS NULL;

/* =========================
   ENTRIES
   ========================= */
CREATE TABLE IF NOT EXISTS entries (
  id            INTEGER PRIMARY KEY,                        -- bigint
  global_id     BLOB NOT NULL UNIQUE,                       -- supplied by app
  payee         TEXT NOT NULL,
  start_time    TEXT NOT NULL,                              -- UTC ISO8601
  end_time      TEXT,                                       -- nullable while in-progress
  memo          TEXT,
  category_id   INTEGER NOT NULL,
  created_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  deleted_at    TEXT,
  version       INTEGER NOT NULL DEFAULT 1,
  FOREIGN KEY (category_id) REFERENCES categories(id) ON UPDATE RESTRICT ON DELETE RESTRICT,
  -- Sanity checks
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

-- Down migration (if your tool needs it):
-- DROP TABLE IF EXISTS entries;
-- DROP TABLE IF EXISTS categories;
-- DROP TABLE IF EXISTS category_groups;