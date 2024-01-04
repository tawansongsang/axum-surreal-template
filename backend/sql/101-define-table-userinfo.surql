USE NS ns_template;
USE DB db_template;

-- Create schemafull user_info table
DEFINE TABLE user_info SCHEMAFULL;

-- Define some fields.
DEFINE FIELD username ON TABLE user_info TYPE string;
DEFINE FIELD password ON TABLE user_info TYPE string VALUE crypto::argon2::generate($value);
DEFINE FIELD password_salt ON TABLE user_info TYPE string DEFAULT rand::uuid::v4();
DEFINE FIELD token_salt ON TABLE user_info TYPE string DEFAULT rand::uuid::v4();
DEFINE FIELD create_by ON TABLE user_info TYPE option<record>;
DEFINE FIELD create_on ON TABLE user_info TYPE datetime DEFAULT time::now();
DEFINE FIELD update_by ON TABLE user_info TYPE option<record>;
DEFINE FIELD update_on ON TABLE user_info TYPE datetime DEFAULT time::now();

DEFINE INDEX usernameIndex ON TABLE user_info COLUMNS username UNIQUE;