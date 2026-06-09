import { invoke } from '@tauri-apps/api/core'
import type {
  Ledger,
  Category,
  Tag,
  Transaction,
  Asset,
  AddTransactionRequest,
} from '@/types/types'

// ==================== 账本相关 ====================

export async function listLedgers(): Promise<Ledger[]> {
  return invoke<Ledger[]>('list_ledgers')
}

// ==================== 分类相关 ====================

export async function listCategories(): Promise<Category[]> {
  return invoke<Category[]>('list_categories')
}

export async function listCategoriesByType(ledgerType: number): Promise<Category[]> {
  return invoke<Category[]>('list_categories_by_type', { ledgerType })
}

// ==================== 标签相关 ====================

export async function listTags(): Promise<Tag[]> {
  return invoke<Tag[]>('list_tags')
}

// ==================== 交易相关 ====================

export async function addTransaction(request: AddTransactionRequest): Promise<number> {
  return invoke<number>('add_transaction', {
    ledgerId: request.ledger_id,
    categoryId: request.category_id,
    amount: request.amount,
    costType: request.cost_type,
    comment: request.comment ?? null,
    transactionTime: request.transaction_time,
  })
}

export async function listTransactionsByDay(day: string): Promise<Transaction[]> {
  return invoke<Transaction[]>('list_transactions_by_day', { day })
}

export async function listTransactionsByMonth(year: number, month: number): Promise<Transaction[]> {
  return invoke<Transaction[]>('list_transactions_by_month', { year, month })
}

// ==================== 资产相关 ====================

export async function listAssets(): Promise<Asset[]> {
  return invoke<Asset[]>('list_assets')
}
