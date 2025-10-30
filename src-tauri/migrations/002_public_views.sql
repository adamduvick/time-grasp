-- 0002_public_views.sql

DROP VIEW IF EXISTS v_public_entries;
DROP VIEW IF EXISTS v_public_categories;
DROP VIEW IF EXISTS v_public_category_groups;

-- Entries: expose category name
CREATE VIEW v_public_entries AS
SELECT
  e.global_id            AS global_id,
  e.payee,
  e.start_time,
  e.end_time,
  e.memo,
  c.name                 AS category
FROM entries e
JOIN categories c
  ON c.id = e.category_id
 AND c.deleted_at IS NULL
JOIN category_groups g
  ON g.id = c.group_id
 AND g.deleted_at IS NULL
WHERE e.deleted_at IS NULL;

-- Categories: expose group name
CREATE VIEW v_public_categories AS
SELECT
  c.global_id            AS global_id,
  c.name,
  c.note,
  g.name                 AS "group"
FROM categories c
JOIN category_groups g
  ON g.id = c.group_id
 AND g.deleted_at IS NULL
WHERE c.deleted_at IS NULL;

-- Category groups
CREATE VIEW v_public_category_groups AS
SELECT
  cg.global_id           AS global_id,
  cg.name,
  cg.note
FROM category_groups cg
WHERE cg.deleted_at IS NULL;