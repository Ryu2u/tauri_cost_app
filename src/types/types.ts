// 账本类型 0=支出账本, 1=收入账本, 2=资产账本
export type LedgerType = 0 | 1 | 2

// 账本
export interface Ledger {
  id: number
  ledger_name: string
  ledger_type: LedgerType
  ledger_image: string
  create_time: string
  update_time: string
}

// 分类
export interface Category {
  id: number
  name: string
  parent_id: number | null
  image: string
  color: string
  ledger_type: LedgerType
  sort_order: number
  created_at: string
}

// 标签
export interface Tag {
  id: number
  name: string
  color: string
  created_at: string
}

// 交易类型 0=支出, 1=收入
export type CostType = 0 | 1

// 交易记录
export interface Transaction {
  id: number
  ledger_id: number
  category_id: number
  amount: number
  cost_type: CostType
  comment: string | null
  transaction_time: string
  created_at: string
  updated_at: string
  category_name: string | null
  image: string | null
  color: string | null
}

// 交易标签关联
export interface TransactionTag {
  transaction_id: number
  tag_id: number
}

// 资产类型 0=现金, 1=银行, 2=信用卡, 3=投资, 4=其他
export type AssetType = 0 | 1 | 2 | 3 | 4

// 资产
export interface Asset {
  id: number
  name: string
  asset_type: AssetType
  balance: number
  color: string
  icon: string
  ledger_id: number | null
  created_at: string
  updated_at: string
}

// ==================== API 请求/响应类型 ====================

// 添加交易请求
export interface AddTransactionRequest {
  ledger_id: number
  category_id: number
  amount: number
  cost_type: CostType
  comment?: string
  transaction_time: string
}

// 按月份查询请求
export interface QueryByMonthRequest {
  year: number
  month: number
}

// 按日期查询请求
export interface QueryByDayRequest {
  day: string  // 格式: YYYY-MM-DD
}

// ==================== 前端业务类型 ====================

// 分类选项（用于 UI 选择）
export interface CategoryOption {
  id: number
  name: string
  icon: string
  color: string
  type: CostType
}

// 账单项（用于列表展示）
export interface BillItem {
  icon: string
  title: string
  subTitle: string
  cost: number
  costType: CostType
  date: string
  time: string
  comment?: string
  color?: string
}

// 分组账单
export interface GroupedBill {
  date: string
  dateFull: string
  items: BillItem[]
}
