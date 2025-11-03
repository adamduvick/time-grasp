-- 0001_init.sql (SQLite)

-- ===== Category Group =====
CREATE TABLE IF NOT EXISTS category_group (
  id            INTEGER PRIMARY KEY,
  global_id     BLOB NOT NULL UNIQUE,
  name          TEXT NOT NULL UNIQUE,
  note          TEXT,
  created_at    INTEGER NOT NULL CHECK (created_at > 0),
  updated_at    INTEGER NOT NULL CHECK (updated_at > 0),
  deleted_at    INTEGER,
  version       INTEGER NOT NULL DEFAULT 1 CHECK (version >= 1),
  is_system     INTEGER NOT NULL DEFAULT 0 CHECK (is_system IN (0,1)),
  CHECK (updated_at >= created_at),
  CHECK (deleted_at IS NULL OR deleted_at >= created_at)
) STRICT;

-- Protect system groups from rename or soft-delete
CREATE TRIGGER IF NOT EXISTS category_group_bu_protect_system
BEFORE UPDATE ON category_group
FOR EACH ROW
WHEN OLD.is_system = 1 AND (
       NEW.name       <> OLD.name OR
       NEW.deleted_at IS NOT OLD.deleted_at
     )
BEGIN
  SELECT RAISE(ABORT, 'system category_group cannot be renamed or soft-deleted');
END;

CREATE TRIGGER IF NOT EXISTS category_group_bd_protect_system
BEFORE DELETE ON category_group
FOR EACH ROW
WHEN OLD.is_system = 1
BEGIN
  SELECT RAISE(ABORT, 'system category_group cannot be deleted');
END;

-- Seed system rows with 16-byte BLOB UUIDs
-- 00000000-0000-0000-0000-000000000001 -> X'00000000000000000000000000000001'
INSERT OR IGNORE INTO category_group (id, global_id, name, note, is_system, created_at, updated_at)
VALUES (
  1,
  X'00000000000000000000000000000001',
  'Ungrouped',
  'Default catch-all group',
  1,
  CAST(strftime('%s','now') AS INTEGER) * 1000,
  CAST(strftime('%s','now') AS INTEGER) * 1000
);

-- ===== Category =====
CREATE TABLE IF NOT EXISTS category (
  id            INTEGER PRIMARY KEY,
  global_id     BLOB NOT NULL UNIQUE,
  name          TEXT NOT NULL UNIQUE,
  note          TEXT,
  -- NOT NULL with DEFAULT to the "Ungrouped" group (seeded below as id=1)
  group_id      INTEGER NOT NULL DEFAULT 1,
  created_at    INTEGER NOT NULL CHECK (created_at > 0),
  updated_at    INTEGER NOT NULL CHECK (updated_at > 0),
  deleted_at    INTEGER,
  version       INTEGER NOT NULL DEFAULT 1 CHECK (version >= 1),
  is_system     INTEGER NOT NULL DEFAULT 0 CHECK (is_system IN (0,1)),
  FOREIGN KEY (group_id) REFERENCES category_group(id)
    ON UPDATE RESTRICT
    ON DELETE RESTRICT,
  CHECK (updated_at >= created_at),
  CHECK (deleted_at IS NULL OR deleted_at >= created_at)
) STRICT;

-- Protect system category from rename, regroup, or soft-delete
CREATE TRIGGER IF NOT EXISTS category_bu_protect_system
BEFORE UPDATE ON category
FOR EACH ROW
WHEN OLD.is_system = 1 AND (
       NEW.name       <> OLD.name OR
       NEW.group_id   <> OLD.group_id OR
       NEW.deleted_at IS NOT OLD.deleted_at
     )
BEGIN
  SELECT RAISE(ABORT, 'system category cannot be renamed, moved, or soft-deleted');
END;

CREATE TRIGGER IF NOT EXISTS category_bd_protect_system
BEFORE DELETE ON category
FOR EACH ROW
WHEN OLD.is_system = 1
BEGIN
  SELECT RAISE(ABORT, 'system category cannot be deleted');
END;

-- Seed system rows with 16-byte BLOB UUIDs
INSERT OR IGNORE INTO category (id, global_id, name, note, group_id, is_system, created_at, updated_at)
VALUES (
  1,
  X'00000000000000000000000000000001',
  'Uncategorized',
  'Default catch-all category',
  1,
  1,
  CAST(strftime('%s','now') AS INTEGER) * 1000,
  CAST(strftime('%s','now') AS INTEGER) * 1000
);

-- ===== Entry =====
CREATE TABLE IF NOT EXISTS entry (
  id            INTEGER PRIMARY KEY,
  global_id     BLOB NOT NULL UNIQUE,
  name          TEXT NOT NULL,
  start_time    INTEGER NOT NULL CHECK (start_time > 0),
  end_time      INTEGER,
  note          TEXT,
  -- NOT NULL with DEFAULT to the "Uncategorized" category (seeded above as id=1)
  category_id   INTEGER NOT NULL DEFAULT 1,
  created_at    INTEGER NOT NULL CHECK (created_at > 0),
  updated_at    INTEGER NOT NULL CHECK (updated_at > 0),
  deleted_at    INTEGER,
  version       INTEGER NOT NULL DEFAULT 1 CHECK (version >= 1),
  FOREIGN KEY (category_id) REFERENCES category(id)
    ON UPDATE RESTRICT
    ON DELETE SET DEFAULT,
  CHECK (end_time IS NULL OR end_time >= start_time),
  CHECK (updated_at >= created_at),
  CHECK (deleted_at IS NULL OR deleted_at >= created_at)
) STRICT;

-- ===== Indexes (soft-delete aware) =====
CREATE INDEX IF NOT EXISTS idx_entry_active_start ON entry(start_time) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_entry_cat_start ON entry(category_id, start_time) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_entry_updated     ON entry(updated_at) WHERE deleted_at IS NULL;
