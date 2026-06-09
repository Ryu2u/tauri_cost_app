-- ================================================
-- 记账软件数据库架构 v1.0
-- 使用 SQLx + SQLite
-- ================================================

-- 1. 账本表 (Ledger)
-- 用于管理多个账本，如"日常账本"、"旅行账本"等
CREATE TABLE IF NOT EXISTS ledger (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    ledger_name     TEXT    NOT NULL,
    ledger_type     INTEGER NOT NULL DEFAULT 2,  -- 1=支出账本, 2=通用账本, 3=收入账本
    ledger_image    TEXT    NOT NULL DEFAULT '',
    create_time     TEXT    NOT NULL DEFAULT (datetime('now', 'localtime')),
    update_time     TEXT    NOT NULL DEFAULT (datetime('now', 'localtime'))
);

-- 2. 分类表 (Category)
-- 用于收入/支出的分类，如"餐饮"、"交通"、"工资"等
-- 支持二级分类（parent_id）
CREATE TABLE IF NOT EXISTS category (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    name            TEXT    NOT NULL,
    parent_id       INTEGER,                          -- 上级分类ID，NULL为一级分类
    image           TEXT    NOT NULL DEFAULT '',
    color           TEXT    NOT NULL DEFAULT '#000000',
    ledger_type     INTEGER NOT NULL DEFAULT 2,       -- 适用的账本类型：1=仅支出, 2=通用, 3=仅收入
    sort_order      INTEGER NOT NULL DEFAULT 0,        -- 排序权重
    created_at      TEXT    NOT NULL DEFAULT (datetime('now', 'localtime')),
    FOREIGN KEY (parent_id) REFERENCES category(id) ON DELETE SET NULL
);

-- 3. 标签表 (Tags)
-- 用于给交易打标签，如"重要"、"出差"、"报销"等
CREATE TABLE IF NOT EXISTS tags (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    name            TEXT    NOT NULL UNIQUE,
    color           TEXT    NOT NULL DEFAULT '#000000',
    created_at      TEXT    NOT NULL DEFAULT (datetime('now', 'localtime'))
);

-- 4. 资产表 (Asset)
-- 用于管理资产账户，如"现金"、"银行卡"、"支付宝"等
CREATE TABLE IF NOT EXISTS assets (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    name            TEXT    NOT NULL,
    asset_type      INTEGER NOT NULL DEFAULT 1,       -- 1=现金, 2=银行卡, 3=电子钱包, 4=信用卡, 5=其他
    balance         REAL    NOT NULL DEFAULT 0.0,     -- 当前余额
    color           TEXT    NOT NULL DEFAULT '#000000',
    icon            TEXT    NOT NULL DEFAULT '',
    ledger_id       INTEGER,                          -- 关联的账本ID，NULL表示通用资产
    created_at      TEXT    NOT NULL DEFAULT (datetime('now', 'localtime')),
    updated_at      TEXT    NOT NULL DEFAULT (datetime('now', 'localtime')),
    FOREIGN KEY (ledger_id) REFERENCES ledger(id) ON DELETE SET NULL
);

-- 5. 交易表 (Transaction)
-- 核心交易记录表
CREATE TABLE IF NOT EXISTS transactions (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    ledger_id           INTEGER NOT NULL,            -- 所属账本
    category_id         INTEGER NOT NULL,            -- 交易分类
    amount              REAL    NOT NULL,             -- 交易金额（正数）
    cost_type           INTEGER NOT NULL,             -- 1=支出, 2=收入, 3=转账
    comment             TEXT,                         -- 备注
    transaction_time     TEXT    NOT NULL,            -- 交易时间（格式：YYYY-MM-DD HH:MM:SS）
    created_at          TEXT    NOT NULL DEFAULT (datetime('now', 'localtime')),
    updated_at          TEXT    NOT NULL DEFAULT (datetime('now', 'localtime')),
    FOREIGN KEY (ledger_id)     REFERENCES ledger(id)     ON DELETE CASCADE,
    FOREIGN KEY (category_id)    REFERENCES category(id)  ON DELETE RESTRICT
);

-- 6. 交易-标签关联表 (TransactionTags)
-- 多对多关系
CREATE TABLE IF NOT EXISTS transaction_tags (
    transaction_id  INTEGER NOT NULL,
    tag_id          INTEGER NOT NULL,
    PRIMARY KEY (transaction_id, tag_id),
    FOREIGN KEY (transaction_id) REFERENCES transactions(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id)         REFERENCES tags(id)        ON DELETE CASCADE
);

-- ================================================
-- 索引优化
-- ================================================

-- 交易表按时间查询的常用索引
CREATE INDEX IF NOT EXISTS idx_transactions_time
    ON transactions(transaction_time);

-- 交易表按账本查询的索引
CREATE INDEX IF NOT EXISTS idx_transactions_ledger
    ON transactions(ledger_id);

-- 交易表按分类查询的索引
CREATE INDEX IF NOT EXISTS idx_transactions_category
    ON transactions(category_id);

-- 分类表按类型查询的索引
CREATE INDEX IF NOT EXISTS idx_category_type
    ON category(ledger_type);

-- 资产表按账本查询的索引
CREATE INDEX IF NOT EXISTS idx_assets_ledger
    ON assets(ledger_id);

-- ================================================
-- 初始数据
-- ================================================

-- 插入默认账本
INSERT INTO ledger (ledger_name, ledger_type, ledger_image) VALUES
    ('日常账本', 2, 'wallet');

-- 插入默认支出分类（ledger_type=1表示仅支出，2表示通用）
INSERT INTO category (name, parent_id, image, color, ledger_type, sort_order) VALUES
    ('餐饮', NULL, 'restaurant', '#FF6B6B', 1, 1),
    ('交通', NULL, 'car', '#4ECDC4', 1, 2),
    ('购物', NULL, 'shopping', '#45B7D1', 1, 3),
    ('居住', NULL, 'home', '#96CEB4', 1, 4),
    ('娱乐', NULL, 'game', '#DDA0DD', 1, 5),
    ('医疗', NULL, 'medical', '#FF69B4', 1, 6),
    ('教育', NULL, 'book', '#9B59B6', 1, 7),
    ('通讯', NULL, 'phone', '#3498DB', 1, 8),
    ('其他支出', NULL, 'more', '#95A5A6', 1, 99);

-- 插入默认收入分类
INSERT INTO category (name, parent_id, image, color, ledger_type, sort_order) VALUES
    ('工资', NULL, 'salary', '#27AE60', 3, 1),
    ('奖金', NULL, 'bonus', '#F39C12', 3, 2),
    ('投资收益', NULL, 'invest', '#8E44AD', 3, 3),
    ('其他收入', NULL, 'more', '#95A5A6', 3, 99);

-- 插入默认资产账户
INSERT INTO assets (name, asset_type, balance, color, icon, ledger_id) VALUES
    ('现金', 1, 0.0, '#27AE60', 'cash', NULL),
    ('银行卡', 2, 0.0, '#3498DB', 'card', NULL),
    ('支付宝', 3, 0.0, '#1890FF', 'alipay', NULL),
    ('微信钱包', 3, 0.0, '#07C160', 'wechat', NULL);
