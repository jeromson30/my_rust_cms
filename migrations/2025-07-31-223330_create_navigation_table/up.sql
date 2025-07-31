CREATE TABLE navigation (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    url VARCHAR NOT NULL,
    order_position INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create index for ordering
CREATE INDEX idx_navigation_order ON navigation(order_position);