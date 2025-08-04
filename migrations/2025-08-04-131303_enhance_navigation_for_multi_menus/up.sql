-- Add new columns to navigation table for enhanced multi-menu system
ALTER TABLE navigation ADD COLUMN menu_area VARCHAR NOT NULL DEFAULT 'header';
ALTER TABLE navigation ADD COLUMN parent_id INTEGER REFERENCES navigation(id) ON DELETE CASCADE;
ALTER TABLE navigation ADD COLUMN icon VARCHAR;
ALTER TABLE navigation ADD COLUMN css_class VARCHAR;
ALTER TABLE navigation ADD COLUMN target VARCHAR DEFAULT '_self';
ALTER TABLE navigation ADD COLUMN mobile_visible BOOLEAN NOT NULL DEFAULT true;
ALTER TABLE navigation ADD COLUMN description TEXT;

-- Create menu templates table for component templates
CREATE TABLE menu_templates (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    template_type VARCHAR NOT NULL, -- 'header', 'footer', 'floating', 'sidebar', 'modal', 'main_container'
    layout_style VARCHAR NOT NULL DEFAULT 'default', -- 'horizontal', 'vertical', 'grid', 'hamburger'
    settings JSONB NOT NULL DEFAULT '{}',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create menu areas table for area-specific settings
CREATE TABLE menu_areas (
    id SERIAL PRIMARY KEY,
    area_name VARCHAR NOT NULL UNIQUE, -- 'header', 'footer', 'floating'
    display_name VARCHAR NOT NULL,
    template_id INTEGER REFERENCES menu_templates(id),
    settings JSONB NOT NULL DEFAULT '{}',
    mobile_behavior VARCHAR DEFAULT 'default', -- 'hamburger', 'hidden', 'collapse', 'default'
    hamburger_icon VARCHAR DEFAULT 'menu',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create component templates table for major layout components
CREATE TABLE component_templates (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    component_type VARCHAR NOT NULL, -- 'header', 'footer', 'sidebar', 'modal', 'main_container'
    template_data JSONB NOT NULL DEFAULT '{}',
    breakpoints JSONB NOT NULL DEFAULT '{}', -- Media breakpoint settings
    width_setting VARCHAR DEFAULT 'full', -- 'fixed', 'full', 'container'
    max_width VARCHAR DEFAULT '1200px',
    is_default BOOLEAN NOT NULL DEFAULT false,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Insert default menu areas
INSERT INTO menu_areas (area_name, display_name, mobile_behavior, hamburger_icon) VALUES 
('header', 'Header Navigation', 'hamburger', 'menu'),
('footer', 'Footer Navigation', 'default', 'menu'),
('floating', 'Floating Navigation', 'hidden', 'menu');

-- Insert default menu templates
INSERT INTO menu_templates (name, template_type, layout_style, settings) VALUES 
('Default Header', 'header', 'horizontal', '{"alignment": "left", "sticky": true}'),
('Hamburger Header', 'header', 'hamburger', '{"breakpoint": "768px", "animation": "slide"}'),
('Horizontal Footer', 'footer', 'horizontal', '{"alignment": "center", "columns": 1}'),
('Vertical Footer', 'footer', 'vertical', '{"alignment": "left", "columns": 3}'),
('Floating Menu', 'floating', 'vertical', '{"position": "fixed", "side": "right"}');

-- Insert default component templates
INSERT INTO component_templates (name, component_type, template_data, breakpoints, width_setting, is_default) VALUES 
('Default Header', 'header', '{"background": "inherit", "padding": "1rem 0", "border": "none"}', '{"mobile": "0px", "tablet": "768px", "desktop": "1024px"}', 'full', true),
('Fixed Header', 'header', '{"background": "inherit", "padding": "1rem 0", "position": "sticky"}', '{"mobile": "0px", "tablet": "768px", "desktop": "1024px"}', 'container', false),
('Default Footer', 'footer', '{"background": "inherit", "padding": "2rem 0", "border": "none"}', '{"mobile": "0px", "tablet": "768px", "desktop": "1024px"}', 'full', true),
('Minimal Footer', 'footer', '{"background": "transparent", "padding": "1rem 0"}', '{"mobile": "0px", "tablet": "768px", "desktop": "1024px"}', 'container', false),
('Default Sidebar', 'sidebar', '{"width": "250px", "position": "fixed", "background": "inherit"}', '{"mobile": "0px", "tablet": "768px", "desktop": "1024px"}', 'fixed', false),
('Modal Overlay', 'modal', '{"backdrop": "blur", "animation": "fade", "closeOnOutside": true}', '{"mobile": "0px", "tablet": "768px", "desktop": "1024px"}', 'container', false),
('Full Width Container', 'main_container', '{"margin": "0 auto", "padding": "2rem 1rem"}', '{"mobile": "0px", "tablet": "768px", "desktop": "1024px"}', 'full', true),
('Fixed Width Container', 'main_container', '{"margin": "0 auto", "padding": "2rem 1rem"}', '{"mobile": "0px", "tablet": "768px", "desktop": "1024px"}', 'container', false);

-- Update existing navigation items to use header area
UPDATE navigation SET menu_area = 'header' WHERE menu_area IS NULL;

-- Create indexes for performance
CREATE INDEX idx_navigation_menu_area ON navigation(menu_area);
CREATE INDEX idx_navigation_parent_id ON navigation(parent_id);
CREATE INDEX idx_navigation_area_order ON navigation(menu_area, order_position);
CREATE INDEX idx_menu_templates_type ON menu_templates(template_type);
CREATE INDEX idx_component_templates_type ON component_templates(component_type);
CREATE INDEX idx_component_templates_default ON component_templates(is_default, component_type);