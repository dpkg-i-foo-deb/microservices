BEGIN;

    -- Create the user table
    CREATE TABLE users (
    id VARCHAR(255) PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    username VARCHAR(255) UNIQUE NOT NULL
    );

    -- Create the sessions table
    CREATE TABLE sessions (
    id VARCHAR(255) PRIMARY KEY,
    start_date TIMESTAMP NOT NULL,
    end_date TIMESTAMP,
    user_id VARCHAR(255) NOT NULL REFERENCES users(id)
    );

COMMIT;