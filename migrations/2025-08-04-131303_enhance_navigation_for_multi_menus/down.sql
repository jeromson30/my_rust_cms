-- Drop indexes
DROP INDEX IF EXISTS idx_component_templates_default;
DROP INDEX IF EXISTS idx_component_templates_type;
DROP INDEX IF EXISTS idx_menu_templates_type;
DROP INDEX IF EXISTS idx_navigation_area_order;
DROP INDEX IF EXISTS idx_navigation_parent_id;
DROP INDEX IF EXISTS idx_navigation_menu_area;

-- Drop new tables
DROP TABLE IF EXISTS component_templates;
DROP TABLE IF EXISTS menu_areas;
DROP TABLE IF EXISTS menu_templates;

-- Remove new columns from navigation table
ALTER TABLE navigation DROP COLUMN IF EXISTS description;
ALTER TABLE navigation DROP COLUMN IF EXISTS mobile_visible;
ALTER TABLE navigation DROP COLUMN IF EXISTS target;
ALTER TABLE navigation DROP COLUMN IF EXISTS css_class;
ALTER TABLE navigation DROP COLUMN IF EXISTS icon;
ALTER TABLE navigation DROP COLUMN IF EXISTS parent_id;
ALTER TABLE navigation DROP COLUMN IF EXISTS menu_area;