USE DbTemplate;
IF OBJECT_ID(N'dbo.UserInfo', N'U') IS NULL
CREATE TABLE dbo.UserInfo
(
    UserInfoID UNIQUEIDENTIFIER NOT NULL PRIMARY KEY CLUSTERED DEFAULT NEWID(),
    Username VARCHAR(50) NOT NULL UNIQUE NONCLUSTERED,
    Name NVARCHAR(250),
    Email VARCHAR(50) NOT NULL UNIQUE NONCLUSTERED,
    EmailVerified DATETIME2,
    Password VARCHAR(200),
    PasswordSalt VARCHAR(200),
    TokenSalt VARCHAR(200),
    Role VARCHAR(5),
    CreateBy UNIQUEIDENTIFIER FOREIGN KEY REFERENCES dbo.UserInfo(UserInfoID),
    CreateOn DATETIME2,
    UpdateBy UNIQUEIDENTIFIER FOREIGN KEY REFERENCES dbo.UserInfo(UserInfoID),
    UpdateOn DATETIME2,
    Active CHAR(1) NOT NULL CHECK (Active = 'Y' OR Active = 'N'),
    Deleted CHAR(1) NOT NULL CHECK (Deleted = 'Y' OR Deleted = 'N'),
    DeleteOn DATETIME2,
    DeleteBy UNIQUEIDENTIFIER FOREIGN KEY REFERENCES dbo.UserInfo(UserInfoID)
);