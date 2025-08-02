-- Add columns without unique constraint first
ALTER TABLE pages ADD COLUMN slug VARCHAR;
ALTER TABLE pages ADD COLUMN status VARCHAR NOT NULL DEFAULT 'draft';

-- Update existing pages to have proper slugs based on their titles
UPDATE pages SET slug = 
    CASE 
        WHEN LOWER(title) = 'contact us' THEN 'contact-us'
        WHEN LOWER(title) = 'our services' THEN 'our-services'
        WHEN LOWER(title) = 'rocket' THEN 'rocket'
        WHEN LOWER(title) = 'extraordinary' THEN 'extraordinary'
        ELSE LOWER(REPLACE(REPLACE(title, ' ', '-'), '''', ''))
    END;

-- Set all existing pages to published status
UPDATE pages SET status = 'published';

-- Now make slug required and unique after data is populated
ALTER TABLE pages ALTER COLUMN slug SET NOT NULL;
CREATE UNIQUE INDEX pages_slug_idx ON pages(slug);