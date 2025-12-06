CREATE TABLE IF NOT EXISTS category_group (
    id BLOB PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    note TEXT,
    -- sync columns
    version INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (CAST(unixepoch('subsec') * 1000 AS INTEGER)),
    updated_at INTEGER NOT NULL DEFAULT (CAST(unixepoch('subsec') * 1000 AS INTEGER)),
    deleted_at INTEGER,
    deleted_by_user BLOB,
    deleted_by_device BLOB,
    tombstone_reason TEXT,
    CHECK (created_at <= updated_at)
) STRICT;
CREATE TABLE IF NOT EXISTS category (
    id BLOB PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    note TEXT,
    group_id BLOB NOT NULL REFERENCES category_group(id),
    -- sync columns
    version INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (CAST(unixepoch('subsec') * 1000 AS INTEGER)),
    updated_at INTEGER NOT NULL DEFAULT (CAST(unixepoch('subsec') * 1000 AS INTEGER)),
    deleted_at INTEGER,
    deleted_by_user BLOB,
    deleted_by_device BLOB,
    tombstone_reason TEXT,
    CHECK (created_at <= updated_at)
) STRICT;
CREATE TABLE IF NOT EXISTS entry (
    id BLOB PRIMARY KEY,
    name TEXT NOT NULL,
    note TEXT,
    category_id BLOB NOT NULL REFERENCES category(id),
    start_time INTEGER NOT NULL,
    end_time INTEGER,
    duration INTEGER GENERATED ALWAYS AS (
        CASE
            WHEN end_time IS NOT NULL THEN end_time - start_time
        END
    ) VIRTUAL,
    -- sync columns
    version INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (CAST(unixepoch('subsec') * 1000 AS INTEGER)),
    updated_at INTEGER NOT NULL DEFAULT (CAST(unixepoch('subsec') * 1000 AS INTEGER)),
    deleted_at INTEGER,
    deleted_by_user BLOB,
    deleted_by_device BLOB,
    tombstone_reason TEXT,
    CHECK (created_at <= updated_at),
    CHECK (
        end_time IS NULL
        OR start_time <= end_time
    )
) STRICT;
-- ensure version increments and updated_at is set on update
CREATE TRIGGER IF NOT EXISTS category_group_after_update
AFTER
UPDATE ON category_group FOR EACH ROW BEGIN
UPDATE category_group
SET version = CASE
        WHEN NEW.version > OLD.version THEN NEW.version
        ELSE OLD.version + 1
    END,
    updated_at = CAST(unixepoch('subsec') * 1000 AS INTEGER)
WHERE id = OLD.id;
END;
CREATE TRIGGER IF NOT EXISTS category_after_update
AFTER
UPDATE ON category FOR EACH ROW BEGIN
UPDATE category
SET version = CASE
        WHEN NEW.version > OLD.version THEN NEW.version
        ELSE OLD.version + 1
    END,
    updated_at = CAST(unixepoch('subsec') * 1000 AS INTEGER)
WHERE id = OLD.id;
END;
CREATE TRIGGER IF NOT EXISTS entry_after_update
AFTER
UPDATE ON entry FOR EACH ROW BEGIN
UPDATE entry
SET version = CASE
        WHEN NEW.version > OLD.version THEN NEW.version
        ELSE OLD.version + 1
    END,
    updated_at = CAST(unixepoch('subsec') * 1000 AS INTEGER)
WHERE id = OLD.id;
END;
-- ensure deleted_at is set on update of deletion fields
CREATE TRIGGER IF NOT EXISTS category_group_after_delete
AFTER
UPDATE OF deleted_by_user,
    deleted_by_device,
    tombstone_reason ON category_group FOR EACH ROW BEGIN
UPDATE category_group
SET deleted_at = CAST(unixepoch('subsec') * 1000 AS INTEGER)
WHERE id = OLD.id;
END;
CREATE TRIGGER IF NOT EXISTS category_after_delete
AFTER
UPDATE OF deleted_by_user,
    deleted_by_device,
    tombstone_reason ON category FOR EACH ROW BEGIN
UPDATE category
SET deleted_at = CAST(unixepoch('subsec') * 1000 AS INTEGER)
WHERE id = OLD.id;
END;
CREATE TRIGGER IF NOT EXISTS entry_after_delete
AFTER
UPDATE OF deleted_by_user,
    deleted_by_device,
    tombstone_reason ON entry FOR EACH ROW BEGIN
UPDATE entry
SET deleted_at = CAST(unixepoch('subsec') * 1000 AS INTEGER)
WHERE id = OLD.id;
END;