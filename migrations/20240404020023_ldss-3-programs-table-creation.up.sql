CREATE TABLE IF NOT EXISTS programs (
    program_code VARCHAR(20) PRIMARY KEY,
    display_name VARCHAR(255) NOT NULL,
    program_type VARCHAR(50) NOT NULL,
    duration_type VARCHAR(50) NOT NULL,
    duration INT NOT NULL
);