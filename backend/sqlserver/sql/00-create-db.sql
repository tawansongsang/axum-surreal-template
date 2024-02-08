USE master;
GO
IF DB_ID (N'DbTemplate') IS NULL
CREATE DATABASE DbTemplate 
ON
(NAME = DbTemplate_data,
    FILENAME = '/var/opt/mssql/data/DbTemplate_data.mdf',
    SIZE = 8 MB,
    MAXSIZE = UNLIMITED,
    FILEGROWTH = 80%)
LOG ON
(NAME = DbTemplate_log,
    FILENAME = '/var/opt/mssql/data/DbTemplate_log.ldf',
    SIZE = 8 MB,
    MAXSIZE = UNLIMITED,
    FILEGROWTH = 80%)
COLLATE Thai_CI_AS;
GO
