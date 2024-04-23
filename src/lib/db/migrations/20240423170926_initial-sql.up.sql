CREATE TABLE clip(
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    short_code VARCHAR(255) UNIQUE NOT NULL,
    title VARCHAR(255),
    content TEXT NOT NULL,
    password TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP,
    no_of_hits BIGINT NOT NULL
)