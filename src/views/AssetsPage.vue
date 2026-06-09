<template>
  <ion-page>
    <ion-header class="assets-header">
      <div class="hero-card">
        <div class="hero-top">
          <div>
            <p class="eyebrow">资产总览</p>
            <h1>我的账户</h1>
          </div>
          <div class="month-pill">{{ currentYear }}年{{ currentMonth }}月</div>
        </div>

        <div class="hero-amount">¥{{ formatCurrency(netAssets) }}</div>
        <p class="hero-caption">净资产 = 可用资产 - 负债</p>

        <div class="hero-metrics">
          <div class="hero-metric">
            <span>可用资产</span>
            <strong>¥{{ formatCurrency(positiveAssets) }}</strong>
          </div>
          <div class="hero-metric">
            <span>负债</span>
            <strong>¥{{ formatCurrency(liabilities) }}</strong>
          </div>
          <div class="hero-metric">
            <span>账户数</span>
            <strong>{{ assets.length }}</strong>
          </div>
        </div>
      </div>
    </ion-header>

    <ion-content class="assets-content">
      <div v-if="loading" class="loading-state">
        <ion-spinner name="crescent" />
      </div>

      <template v-else>
        <section class="monthly-section">
          <div class="section-title-row">
            <div>
              <p class="section-kicker">本月现金流</p>
              <h2>收支变化</h2>
            </div>
          </div>

          <div class="cashflow-grid">
            <article class="mini-card income-card">
              <span>收入</span>
              <strong>¥{{ formatCurrency(monthIncome) }}</strong>
            </article>
            <article class="mini-card expense-card">
              <span>支出</span>
              <strong>¥{{ formatCurrency(monthExpense) }}</strong>
            </article>
            <article class="mini-card balance-card">
              <span>结余</span>
              <strong>¥{{ formatCurrency(monthBalance) }}</strong>
            </article>
          </div>
        </section>

        <section class="section-block">
          <div class="section-title-row">
            <div>
              <p class="section-kicker">账户分布</p>
              <h2>资产分类</h2>
            </div>
          </div>

          <div class="type-grid">
            <article
              v-for="group in assetTypeCards"
              :key="group.type"
              class="type-card"
            >
              <div class="type-card-icon" :style="{ background: group.tint }">
                <ion-icon :icon="group.icon" />
              </div>
              <div class="type-card-copy">
                <strong>{{ group.label }}</strong>
                <span>{{ group.count }} 个账户</span>
              </div>
              <div class="type-card-value">¥{{ formatCurrency(group.total) }}</div>
            </article>
          </div>
        </section>

        <section class="section-block">
          <div class="section-title-row">
            <div>
              <p class="section-kicker">账户明细</p>
              <h2>全部资产</h2>
            </div>
          </div>

          <div v-if="sortedAssets.length === 0" class="empty-state">
            <ion-icon :icon="walletOutline" class="empty-icon" />
            <p>还没有资产账户</p>
            <p class="empty-hint">先保留默认账户，后续我们可以继续加新增/编辑资产功能。</p>
          </div>

          <div v-else class="asset-list">
            <article
              v-for="asset in sortedAssets"
              :key="asset.id"
              class="asset-item"
            >
              <div class="asset-main">
                <div class="asset-icon" :style="{ background: asset.color }">
                  <ion-icon :icon="getAssetIcon(asset.icon, asset.asset_type)" />
                </div>
                <div class="asset-copy">
                  <div class="asset-name-row">
                    <strong>{{ asset.name }}</strong>
                    <span class="asset-tag">{{ getAssetTypeLabel(asset.asset_type) }}</span>
                  </div>
                  <span class="asset-subtitle">{{ getAssetStatusText(asset.balance) }}</span>
                </div>
              </div>

              <div class="asset-side">
                <strong :class="['asset-balance', asset.balance < 0 ? 'negative' : 'positive']">
                  {{ asset.balance < 0 ? '-' : '' }}¥{{ formatCurrency(Math.abs(asset.balance)) }}
                </strong>
                <span class="asset-share">{{ getAssetShare(asset.balance) }}</span>
              </div>
            </article>
          </div>
        </section>
      </template>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonContent,
  IonHeader,
  IonIcon,
  IonPage,
  IonSpinner,
  onIonViewWillEnter,
} from '@ionic/vue';
import { computed, ref } from 'vue';
import {
  briefcaseOutline,
  cardOutline,
  cashOutline,
  pieChartOutline,
  walletOutline,
} from 'ionicons/icons';
import { listAssets, listTransactionsByMonth } from '@/api';
import type { Asset, AssetType, Transaction } from '@/types/types';

const loading = ref(false);
const assets = ref<Asset[]>([]);
const monthTransactions = ref<Transaction[]>([]);
const currentDate = new Date();
const currentYear = currentDate.getFullYear();
const currentMonth = currentDate.getMonth() + 1;

const assetTypeMeta: Record<AssetType, { label: string; icon: string; tint: string }> = {
  0: { label: '现金', icon: cashOutline, tint: 'linear-gradient(135deg, #48c78e, #2e9b62)' },
  1: { label: '银行卡', icon: cardOutline, tint: 'linear-gradient(135deg, #5ca9ff, #2b6ce0)' },
  2: { label: '信用卡', icon: cardOutline, tint: 'linear-gradient(135deg, #ff8f7a, #ef4e4e)' },
  3: { label: '投资', icon: pieChartOutline, tint: 'linear-gradient(135deg, #8f7aff, #5e44db)' },
  4: { label: '其他', icon: briefcaseOutline, tint: 'linear-gradient(135deg, #9ca3af, #6b7280)' },
};

async function loadData() {
  loading.value = true;
  try {
    const [assetResult, transactionResult] = await Promise.all([
      listAssets(),
      listTransactionsByMonth(currentYear, currentMonth),
    ]);
    assets.value = assetResult;
    monthTransactions.value = transactionResult;
  } finally {
    loading.value = false;
  }
}

const positiveAssets = computed(() => {
  return assets.value
    .filter(asset => asset.balance > 0)
    .reduce((sum, asset) => sum + asset.balance, 0);
});

const liabilities = computed(() => {
  return Math.abs(
    assets.value
      .filter(asset => asset.balance < 0)
      .reduce((sum, asset) => sum + asset.balance, 0),
  );
});

const netAssets = computed(() => {
  return assets.value.reduce((sum, asset) => sum + asset.balance, 0);
});

const monthIncome = computed(() => {
  return monthTransactions.value
    .filter(item => item.cost_type === 1)
    .reduce((sum, item) => sum + item.amount, 0);
});

const monthExpense = computed(() => {
  return monthTransactions.value
    .filter(item => item.cost_type === 0)
    .reduce((sum, item) => sum + item.amount, 0);
});

const monthBalance = computed(() => monthIncome.value - monthExpense.value);

const sortedAssets = computed(() => {
  return [...assets.value].sort((a, b) => Math.abs(b.balance) - Math.abs(a.balance));
});

const assetTypeCards = computed(() => {
  return (Object.keys(assetTypeMeta) as unknown as AssetType[]).map(type => {
    const groupAssets = assets.value.filter(asset => asset.asset_type === type);
    const total = groupAssets.reduce((sum, asset) => sum + asset.balance, 0);
    const meta = assetTypeMeta[type];

    return {
      type,
      label: meta.label,
      icon: meta.icon,
      tint: meta.tint,
      count: groupAssets.length,
      total,
    };
  });
});

function formatCurrency(value: number) {
  return value.toFixed(2);
}

function getAssetIcon(iconName: string, type: AssetType) {
  if (iconName.includes('card')) return cardOutline;
  if (iconName.includes('cash')) return cashOutline;
  return assetTypeMeta[type].icon;
}

function getAssetTypeLabel(type: AssetType) {
  return assetTypeMeta[type].label;
}

function getAssetStatusText(balance: number) {
  if (balance < 0) return '当前为负债账户';
  if (balance === 0) return '当前余额为 0';
  return '状态良好，可持续记账';
}

function getAssetShare(balance: number) {
  if (balance <= 0 || positiveAssets.value <= 0) return '不计入可用资产占比';
  return `占可用资产 ${((balance / positiveAssets.value) * 100).toFixed(1)}%`;
}

onIonViewWillEnter(() => {
  loadData();
});
</script>

<style scoped>
.assets-header {
  background:
    radial-gradient(circle at top right, rgba(255, 255, 255, 0.28), transparent 32%),
    linear-gradient(160deg, #10314f 0%, #174b68 42%, #215d7b 100%);
  padding: 18px 16px 20px;
}

.hero-card {
  color: #fff;
}

.hero-top {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 12px;
}

.eyebrow {
  margin: 0 0 4px;
  font-size: 12px;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: rgba(255, 255, 255, 0.72);
}

.hero-top h1 {
  margin: 0;
  font-size: 28px;
  font-weight: 700;
}

.month-pill {
  padding: 8px 12px;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.14);
  font-size: 13px;
  white-space: nowrap;
}

.hero-amount {
  margin-top: 26px;
  font-size: 40px;
  font-weight: 700;
  letter-spacing: -0.04em;
}

.hero-caption {
  margin: 6px 0 0;
  color: rgba(255, 255, 255, 0.72);
  font-size: 14px;
}

.hero-metrics {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 10px;
  margin-top: 20px;
}

.hero-metric {
  padding: 12px;
  border-radius: 18px;
  background: rgba(255, 255, 255, 0.12);
  backdrop-filter: blur(10px);
}

.hero-metric span {
  display: block;
  font-size: 12px;
  color: rgba(255, 255, 255, 0.72);
}

.hero-metric strong {
  display: block;
  margin-top: 8px;
  font-size: 17px;
}

.assets-content {
  --background: linear-gradient(180deg, #eff5f8 0%, #f8fafb 100%);
}

.loading-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 88px 24px;
}

.section-block,
.monthly-section {
  padding: 18px 16px 0;
}

.section-title-row {
  display: flex;
  justify-content: space-between;
  align-items: end;
  margin-bottom: 14px;
}

.section-kicker {
  margin: 0 0 4px;
  font-size: 12px;
  color: #6b7280;
}

.section-title-row h2 {
  margin: 0;
  font-size: 22px;
  color: #122033;
}

.cashflow-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 10px;
}

.mini-card {
  padding: 14px 12px;
  border-radius: 18px;
  color: #fff;
  box-shadow: 0 12px 24px rgba(15, 23, 42, 0.08);
}

.mini-card span {
  display: block;
  font-size: 12px;
  opacity: 0.82;
}

.mini-card strong {
  display: block;
  margin-top: 8px;
  font-size: 18px;
}

.income-card {
  background: linear-gradient(135deg, #2db86b, #4dd99a);
}

.expense-card {
  background: linear-gradient(135deg, #ff9346, #ffb66e);
}

.balance-card {
  background: linear-gradient(135deg, #4c7fff, #6fa9ff);
}

.type-grid {
  display: grid;
  gap: 12px;
}

.type-card {
  display: grid;
  grid-template-columns: auto 1fr auto;
  align-items: center;
  gap: 12px;
  padding: 14px;
  border-radius: 20px;
  background: rgba(255, 255, 255, 0.88);
  box-shadow: 0 14px 28px rgba(15, 23, 42, 0.06);
}

.type-card-icon,
.asset-icon {
  width: 44px;
  height: 44px;
  border-radius: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  box-shadow: inset 0 -8px 18px rgba(0, 0, 0, 0.12);
}

.type-card-icon ion-icon,
.asset-icon ion-icon {
  font-size: 22px;
}

.type-card-copy strong,
.asset-copy strong {
  display: block;
  color: #152238;
  font-size: 16px;
}

.type-card-copy span,
.asset-subtitle {
  font-size: 12px;
  color: #7c8698;
}

.type-card-value {
  font-size: 16px;
  font-weight: 700;
  color: #0f2746;
}

.asset-list {
  display: grid;
  gap: 12px;
  padding-bottom: 28px;
}

.asset-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  padding: 16px;
  border-radius: 22px;
  background: rgba(255, 255, 255, 0.92);
  box-shadow: 0 16px 30px rgba(15, 23, 42, 0.07);
}

.asset-main {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
}

.asset-copy {
  min-width: 0;
}

.asset-name-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.asset-tag {
  padding: 4px 8px;
  border-radius: 999px;
  background: #edf2f7;
  color: #667085;
  font-size: 11px;
}

.asset-side {
  text-align: right;
}

.asset-balance {
  display: block;
  font-size: 20px;
  color: #133255;
}

.asset-balance.negative {
  color: #d14343;
}

.asset-share {
  display: block;
  margin-top: 4px;
  font-size: 12px;
  color: #7c8698;
}

.empty-icon {
  font-size: 56px;
  color: #b6c0cd;
}

.empty-state p {
  margin: 8px 0 0;
  color: #5c6677;
}

.empty-hint {
  max-width: 260px;
  text-align: center;
  font-size: 13px;
  color: #8d95a3;
}

@media (max-width: 380px) {
  .hero-metrics,
  .cashflow-grid {
    grid-template-columns: 1fr;
  }

  .type-card,
  .asset-item {
    grid-template-columns: auto 1fr;
  }

  .type-card-value,
  .asset-side {
    grid-column: 1 / -1;
    text-align: left;
    padding-left: 56px;
  }
}
</style>
