USE NS ns_template;
USE DB db_template;

-- Create schemafull task table
DEFINE TABLE task SCHEMAFULL;

-- Define some fields.
DEFINE FIELD title ON TABLE task TYPE string;
DEFINE FIELD done ON TABLE task TYPE bool DEFAULT false;
DEFINE FIELD create_by ON TABLE task TYPE record<user_info>;
DEFINE FIELD create_on ON TABLE task TYPE datetime DEFAULT time::now();
DEFINE FIELD update_by ON TABLE task TYPE record<user_info>;
DEFINE FIELD update_on ON TABLE task TYPE datetime DEFAULT time::now();
