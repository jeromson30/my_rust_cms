-- Add missing columns to settings table
ALTER TABLE settings 
ADD COLUMN setting_type VARCHAR NOT NULL DEFAULT 'system',
ADD COLUMN description TEXT,
ADD COLUMN updated_at TIMESTAMP DEFAULT NOW();

-- Update existing records to have proper types
UPDATE settings SET setting_type = 'site' WHERE setting_key LIKE 'site_%';
UPDATE settings SET setting_type = 'system' WHERE setting_key LIKE 'system_%';
UPDATE settings SET setting_type = 'backup' WHERE setting_key LIKE 'backup_%';

-- Create index for faster lookups
CREATE INDEX idx_settings_type ON settings(setting_type);
CREATE INDEX idx_settings_key ON settings(setting_key);