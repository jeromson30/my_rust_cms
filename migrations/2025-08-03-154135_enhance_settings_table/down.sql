-- Remove indexes
DROP INDEX IF EXISTS idx_settings_type;
DROP INDEX IF EXISTS idx_settings_key;

-- Remove added columns
ALTER TABLE settings 
DROP COLUMN IF EXISTS setting_type,
DROP COLUMN IF EXISTS description,
DROP COLUMN IF EXISTS updated_at;