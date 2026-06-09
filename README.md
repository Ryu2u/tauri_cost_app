# 简易记账本

一个基于 **Tauri 2 + Vue 3 + Rust + SQLite** 构建的跨平台桌面记账应用，支持账本管理、分类标签、收支记录和资产统计等功能。

## 功能特性

- **多账本管理** — 支持创建多个账本，分别管理不同场景（个人/家庭/旅行等）
- **收支分类** — 内置常用收支分类，支持自定义分类和图标
- **标签系统** — 为每笔交易打标签，灵活筛选和统计
- **资产管理** — 记录和管理各类资产（现金/银行卡/电子钱包等）
- **数据持久化** — 基于 SQLite 本地存储，数据安全可靠
- **原生桌面体验** — Tauri 构建，体积小巧，运行流畅

## 技术栈

| 层级 | 技术 |
|------|------|
| 前端框架 | Vue 3 + TypeScript |
| UI 组件 | Ionic Vue |
| 构建工具 | Vite |
| 桌面框架 | Tauri 2 (Rust) |
| 数据库 | SQLite (sqlx) |
| 路由 | Vue Router |

## 快速开始

### 环境要求

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://rustup.rs/) 稳定版
- 推荐 IDE: VS Code + Vue Volar + rust-analyzer

### 安装依赖

```bash
# 安装前端依赖
npm install

# Rust 依赖会在首次构建时自动拉取
```

### 开发调试

```bash
# 启动开发服务器（含热更新）
npm run tauri dev
```

### 构建发布

```bash
# 构建生产版本
npm run tauri build
```

构建产物位于 `src-tauri/target/release/bundle/`。

## 项目结构

```
.
├── src/                          # 前端源码
│   ├── api/                      # API 封装（调用 Tauri 命令）
│   ├── compontes/                # 公共组件
│   ├── router/                   # 路由配置
│   ├── types/                    # TypeScript 类型定义
│   ├── views/                    # 页面视图
│   │   ├── LedgerPage.vue        # 记账首页
│   │   ├── AssetsPage.vue        # 资产管理
│   │   └── StatisticsPage.vue    # 统计报表
│   └── main.ts                   # 入口文件
│
├── src-tauri/                    # Tauri / Rust 后端
│   ├── db/migrations/            # 数据库迁移脚本
│   └── src/
│       ├── command.rs            # Tauri IPC 命令
│       ├── structs.rs            # 数据模型（Ledger/Category/Tags/Transaction）
│       ├── lib.rs                # 应用初始化
│       └── main.rs               # 入口
│
├── package.json
├── vite.config.ts
└── tsconfig.json
```

## 数据模型

| 实体 | 说明 |
|------|------|
| `Ledger` | 账本（名称、类型、图标） |
| `Category` | 收支分类（名称、图标、颜色、父分类） |
| `Tags` | 标签（名称、颜色） |
| `Transaction` | 交易记录（金额、类型、时间、备注、关联账本和分类） |
| `Asset` | 资产账户（名称、类型、余额） |

## 开发计划

- [x] 账本/分类/标签 CRUD
- [x] 交易记录增删改查
- [x] 资产页面
- [x] 数据统计
- [ ] 数据导入导出（CSV / Excel）
- [ ] 图表可视化（收支趋势、分类占比）
- [ ] 多语言支持

## 开源协议

[MIT](LICENSE)
