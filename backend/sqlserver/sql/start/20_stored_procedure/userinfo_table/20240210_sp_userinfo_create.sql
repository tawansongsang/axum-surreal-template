USE DbTemplate;

DROP PROCEDURE IF EXISTS dbo.sp_userinfo_create;

BEGIN
	EXEC ('
	CREATE PROCEDURE dbo.sp_userinfo_create 
		@Username VARCHAR(51) = NULL
		, @Name NVARCHAR(51) = NULL
		, @Email VARCHAR(51) = NULL
		, @EmailVerified DATETIME2 = NULL
		, @Password VARCHAR(201) = NULL
		, @Role VARCHAR(5) = ''USER''
		, @CreateOn DATETIME2 = NULL
		, @UpdateBy UNIQUEIDENTIFIER = NULL
		, @UpdateOn DATETIME2 = NULL
		, @Active CHAR(1) = ''Y''
		, @Deleted CHAR(1) = ''N''
		, @DeleteOn DATETIME2 = NULL
		, @DeleteBy UNIQUEIDENTIFIER = NULL
	AS
	SET NOCOUNT ON;
	INSERT INTO dbo.UserInfo (Username, Name
                            , Email, EmailVerified
							, Password, Role
                            , CreateOn, UpdateBy, UpdateOn
							, Active, Deleted, DeleteOn, DeleteBy)
	VALUES (@Username, @Name, @Email
			, @EmailVerified, @Password, @Role
			, @CreateOn, @UpdateBy,  @UpdateOn
			, @Active, @Deleted, @DeleteOn, @DeleteBy);
')
END
