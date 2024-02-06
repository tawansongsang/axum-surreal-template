USE NS ns_template;
USE DB db_template;

-- Create schemafull user_info table
DEFINE TABLE account SCHEMAFULL;

-- Define some fields.
DEFINE FIELD user_info_id ON TABLE account TYPE record<user_info>;
DEFINE FIELD type ON TABLE account TYPE string;
DEFINE FIELD provider ON TABLE account TYPE string;
DEFINE FIELD provider_account_id ON TABLE account TYPE string;
DEFINE FIELD refresh_token ON TABLE account TYPE option<string>;
DEFINE FIELD access_token ON TABLE account TYPE option<string>;
DEFINE FIELD expires_at ON TABLE account TYPE option<number>;
DEFINE FIELD token_type ON TABLE account TYPE option<string>;
DEFINE FIELD scope ON TABLE account TYPE option<string>;
DEFINE FIELD id_token ON TABLE account TYPE option<string>;
DEFINE FIELD session_state ON TABLE account TYPE option<string>;
DEFINE FIELD create_by ON TABLE account TYPE option<record<user_info>>;
DEFINE FIELD create_on ON TABLE account TYPE datetime DEFAULT time::now();
DEFINE FIELD update_by ON TABLE account TYPE option<record<user_info>>;
DEFINE FIELD update_on ON TABLE account TYPE datetime DEFAULT time::now();

DEFINE INDEX providerIndex ON TABLE account COLUMNS provider,provider_account_id UNIQUE;
