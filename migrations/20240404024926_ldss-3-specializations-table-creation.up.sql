CREATE TABLE IF NOT EXISTS specializations (
    specialization_code VARCHAR(20) PRIMARY KEY,
    display_name VARCHAR(255) NOT NULL,
    program_code VARCHAR(20) NOT NULL,
    FOREIGN KEY (program_code) REFERENCES programs(program_code)
);