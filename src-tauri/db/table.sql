CREATE TABLE if not exists "tb_ledger"
(
    "id"           INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "ledger_name"  TEXT    NOT NULL,
    "ledger_type"  integer NOT NULL,
    "ledger_image" TEXT    NOT NULL,
    "create_time"  text    NOT NULL
);