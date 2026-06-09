<template>
  <ion-page>
    <ion-header class="statistics-header">
      <div class="stats-top">
        <div>
          <p class="stats-kicker">月度统计</p>
          <h1>收支分析</h1>
        </div>

        <div class="month-switcher">
          <button class="month-arrow" type="button" @click="prevMonth">
            <ion-icon :icon="chevronBack" />
          </button>
          <span>{{ currentYear }}年{{ currentMonth }}月</span>
          <button class="month-arrow" type="button" @click="nextMonth">
            <ion-icon :icon="chevronForward" />
          </button>
        </div>
      </div>
    </ion-header>

    <ion-content class="statistics-content">
      <div v-if="loading" class="loading-state">
        <ion-spinner name="crescent" />
      </div>

      <template v-else>
        <section class="overview-grid">
          <article class="overview-card expense-surface">
            <span>总支出</span>
            <strong>¥{{ formatCurrency(totalExpense) }}</strong>
            <small>本月 {{ expenseTransactions.length }} 笔</small>
          </article>
          <article class="overview-card income-surface">
            <span>总收入</span>
            <strong>¥{{ formatCurrency(totalIncome) }}</strong>
            <small>本月 {{ incomeTransactions.length }} 笔</small>
          </article>
          <article class="overview-card balance-surface">
            <span>本月结余</span>
            <strong>¥{{ formatCurrency(monthBalance) }}</strong>
            <small>{{ compareLabel }}</small>
          </article>
        </section>

        <section class="insight-row">
          <article class="insight-card">
            <span>日均支出</span>
            <strong>¥{{ formatCurrency(averageDailyExpense) }}</strong>
          </article>
          <article class="insight-card">
            <span>活跃天数</span>
            <strong>{{ activeDays }}</strong>
          </article>
          <article class="insight-card">
            <span>最大单笔</span>
            <strong>¥{{ formatCurrency(largestTransaction) }}</strong>
          </article>
        </section>

        <section class="section-block">
          <div class="section-head">
            <div>
              <p class="section-kicker">支出结构</p>
              <h2>分类排行</h2>
            </div>

            <div class="mode-switch">
              <button
                type="button"
                :class="['mode-pill', rankingMode === 'expense' ? 'active' : '']"
                @click="rankingMode = 'expense'"
              >
                支出
              </button>
              <button
                type="button"
                :class="['mode-pill', rankingMode === 'income' ? 'active' : '']"
                @click="rankingMode = 'income'"
              >
                收入
              </button>
            </div>
          </div>

          <div v-if="categoryRanking.length === 0" class="empty-panel">
            <ion-icon :icon="pieChartOutline" class="empty-panel-icon" />
            <p>这个月份还没有可统计的数据</p>
          </div>

          <div v-else class="ranking-list">
            <article
              v-for="item in categoryRanking"
              :key="`${rankingMode}-${item.name}`"
              class="ranking-item"
            >
              <div class="ranking-main">
                <div class="ranking-dot" :style="{ background: item.color }"></div>
                <div class="ranking-copy">
                  <strong>{{ item.name }}</strong>
                  <span>{{ item.count }} 笔</span>
                </div>
              </div>
              <div class="ranking-side">
                <strong>¥{{ formatCurrency(item.amount) }}</strong>
                <span>{{ item.percentage.toFixed(1) }}%</span>
              </div>
              <div class="ranking-bar">
                <div class="ranking-bar-fill" :style="{ width: `${item.percentage}%`, background: item.color }"></div>
              </div>
            </article>
          </div>
        </section>

        <section class="section-block">
          <div class="section-head">
            <div>
              <p class="section-kicker">每日趋势</p>
              <h2>本月节奏</h2>
            </div>
          </div>

          <div v-if="dailySeries.length === 0" class="empty-panel">
            <ion-icon :icon="pulseOutline" class="empty-panel-icon" />
            <p>这个月份暂无交易趋势</p>
          </div>

          <div v-else class="daily-chart">
            <article
              v-for="item in dailySeries"
              :key="item.day"
              class="day-card"
            >
              <div class="day-label">{{ item.dayLabel }}</div>
              <div class="bar-stack">
                <div class="bar-track expense-track">
                  <div class="bar-fill expense-fill" :style="{ width: `${item.expenseWidth}%` }"></div>
                </div>
                <div class="bar-track income-track">
                  <div class="bar-fill income-fill" :style="{ width: `${item.incomeWidth}%` }"></div>
                </div>
              </div>
              <div class="day-values">
                <span>支出 ¥{{ formatCurrency(item.expense) }}</span>
                <span>收入 ¥{{ formatCurrency(item.income) }}</span>
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
  chevronBack,
  chevronForward,
  pieChartOutline,
  pulseOutline,
} from 'ionicons/icons';
import { listTransactionsByMonth } from '@/api';
import type { CostType, Transaction } from '@/types/types';

type RankingMode = 'expense' | 'income';

const now = new Date();
const currentYear = ref(now.getFullYear());
const currentMonth = ref(now.getMonth() + 1);
const loading = ref(false);
const rankingMode = ref<RankingMode>('expense');
const transactions = ref<Transaction[]>([]);
const previousMonthTransactions = ref<Transaction[]>([]);

async function loadData() {
  loading.value = true;

  const previous = getPreviousMonth(currentYear.value, currentMonth.value);

  try {
    const [currentResult, previousResult] = await Promise.all([
      listTransactionsByMonth(currentYear.value, currentMonth.value),
      listTransactionsByMonth(previous.year, previous.month),
    ]);
    transactions.value = currentResult;
    previousMonthTransactions.value = previousResult;
  } finally {
    loading.value = false;
  }
}

const expenseTransactions = computed(() => {
  return transactions.value.filter(item => item.cost_type === 0);
});

const incomeTransactions = computed(() => {
  return transactions.value.filter(item => item.cost_type === 1);
});

const totalExpense = computed(() => {
  return expenseTransactions.value.reduce((sum, item) => sum + item.amount, 0);
});

const totalIncome = computed(() => {
  return incomeTransactions.value.reduce((sum, item) => sum + item.amount, 0);
});

const monthBalance = computed(() => totalIncome.value - totalExpense.value);

const averageDailyExpense = computed(() => {
  const daysInMonth = new Date(currentYear.value, currentMonth.value, 0).getDate();
  return daysInMonth === 0 ? 0 : totalExpense.value / daysInMonth;
});

const activeDays = computed(() => {
  return new Set(transactions.value.map(item => item.transaction_time.split(' ')[0])).size;
});

const largestTransaction = computed(() => {
  if (transactions.value.length === 0) return 0;
  return Math.max(...transactions.value.map(item => item.amount));
});

const previousBalance = computed(() => {
  const previousExpense = previousMonthTransactions.value
    .filter(item => item.cost_type === 0)
    .reduce((sum, item) => sum + item.amount, 0);
  const previousIncome = previousMonthTransactions.value
    .filter(item => item.cost_type === 1)
    .reduce((sum, item) => sum + item.amount, 0);
  return previousIncome - previousExpense;
});

const compareLabel = computed(() => {
  const delta = monthBalance.value - previousBalance.value;
  if (delta === 0) return '与上月持平';
  return `${delta > 0 ? '较上月增加' : '较上月减少'} ¥${formatCurrency(Math.abs(delta))}`;
});

const categoryRanking = computed(() => {
  const targetType: CostType = rankingMode.value === 'expense' ? 0 : 1;
  const source = transactions.value.filter(item => item.cost_type === targetType);
  const total = source.reduce((sum, item) => sum + item.amount, 0);
  const grouped = new Map<string, { amount: number; count: number; color: string }>();

  source.forEach(item => {
    const name = item.category_name || '未分类';
    const current = grouped.get(name) ?? {
      amount: 0,
      count: 0,
      color: item.color || (targetType === 0 ? '#ff9f43' : '#34c759'),
    };
    current.amount += item.amount;
    current.count += 1;
    if (item.color) current.color = item.color;
    grouped.set(name, current);
  });

  return [...grouped.entries()]
    .map(([name, value]) => ({
      name,
      amount: value.amount,
      count: value.count,
      color: value.color,
      percentage: total > 0 ? (value.amount / total) * 100 : 0,
    }))
    .sort((a, b) => b.amount - a.amount)
    .slice(0, 6);
});

const dailySeries = computed(() => {
  const grouped = new Map<string, { expense: number; income: number }>();

  transactions.value.forEach(item => {
    const day = item.transaction_time.split(' ')[0];
    const current = grouped.get(day) ?? { expense: 0, income: 0 };
    if (item.cost_type === 0) current.expense += item.amount;
    if (item.cost_type === 1) current.income += item.amount;
    grouped.set(day, current);
  });

  const rows = [...grouped.entries()]
    .map(([day, value]) => ({
      day,
      dayLabel: formatDayLabel(day),
      expense: value.expense,
      income: value.income,
    }))
    .sort((a, b) => b.day.localeCompare(a.day))
    .slice(0, 8);

  const maxAmount = rows.reduce((max, item) => Math.max(max, item.expense, item.income), 0);

  return rows.map(item => ({
    ...item,
    expenseWidth: maxAmount > 0 ? (item.expense / maxAmount) * 100 : 0,
    incomeWidth: maxAmount > 0 ? (item.income / maxAmount) * 100 : 0,
  }));
});

function prevMonth() {
  if (currentMonth.value === 1) {
    currentMonth.value = 12;
    currentYear.value -= 1;
  } else {
    currentMonth.value -= 1;
  }
  loadData();
}

function nextMonth() {
  if (currentMonth.value === 12) {
    currentMonth.value = 1;
    currentYear.value += 1;
  } else {
    currentMonth.value += 1;
  }
  loadData();
}

function getPreviousMonth(year: number, month: number) {
  if (month === 1) {
    return { year: year - 1, month: 12 };
  }
  return { year, month: month - 1 };
}

function formatCurrency(value: number) {
  return value.toFixed(2);
}

function formatDayLabel(dateText: string) {
  const date = new Date(dateText);
  return `${date.getMonth() + 1}/${date.getDate()}`;
}

onIonViewWillEnter(() => {
  loadData();
});
</script>

<style scoped>
.statistics-header {
  background:
    radial-gradient(circle at top right, rgba(255, 255, 255, 0.26), transparent 35%),
    linear-gradient(155deg, #33184f 0%, #4e2877 48%, #69409a 100%);
  padding: 18px 16px 20px;
  color: #fff;
}

.stats-top {
  display: flex;
  flex-direction: column;
  gap: 18px;
}

.stats-kicker {
  margin: 0 0 4px;
  font-size: 12px;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: rgba(255, 255, 255, 0.72);
}

.stats-top h1 {
  margin: 0;
  font-size: 28px;
}

.month-switcher {
  display: inline-flex;
  align-items: center;
  align-self: flex-start;
  gap: 10px;
  padding: 8px 12px;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.14);
  font-size: 14px;
}

.month-arrow,
.mode-pill {
  border: none;
  outline: none;
  cursor: pointer;
}

.month-arrow {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.16);
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
}

.statistics-content {
  --background: linear-gradient(180deg, #f6f0fb 0%, #f9fafc 100%);
}

.loading-state,
.empty-panel {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.loading-state {
  padding: 88px 24px;
}

.overview-grid,
.insight-row,
.section-block {
  padding: 18px 16px 0;
}

.overview-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 10px;
}

.overview-card,
.insight-card {
  padding: 16px;
  border-radius: 22px;
  color: #fff;
  box-shadow: 0 16px 28px rgba(38, 24, 68, 0.08);
}

.overview-card span,
.insight-card span {
  display: block;
  font-size: 12px;
  opacity: 0.82;
}

.overview-card strong,
.insight-card strong {
  display: block;
  margin-top: 8px;
  font-size: 22px;
}

.overview-card small {
  display: block;
  margin-top: 8px;
  font-size: 12px;
  opacity: 0.82;
}

.expense-surface {
  background: linear-gradient(135deg, #ff9452, #ffb36a);
}

.income-surface {
  background: linear-gradient(135deg, #31c67a, #56dc9d);
}

.balance-surface {
  background: linear-gradient(135deg, #6660ff, #8b80ff);
}

.insight-row {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 10px;
}

.insight-card {
  color: #18243a;
  background: rgba(255, 255, 255, 0.92);
}

.insight-card span {
  color: #778196;
  opacity: 1;
}

.section-head {
  display: flex;
  justify-content: space-between;
  align-items: end;
  gap: 12px;
  margin-bottom: 14px;
}

.section-kicker {
  margin: 0 0 4px;
  font-size: 12px;
  color: #7d6b96;
}

.section-head h2 {
  margin: 0;
  font-size: 22px;
  color: #221a38;
}

.mode-switch {
  display: inline-flex;
  gap: 8px;
  padding: 4px;
  border-radius: 999px;
  background: rgba(123, 94, 173, 0.12);
}

.mode-pill {
  padding: 8px 14px;
  border-radius: 999px;
  background: transparent;
  color: #6f5f8e;
  font-size: 13px;
}

.mode-pill.active {
  background: #fff;
  color: #2e2248;
  box-shadow: 0 8px 16px rgba(56, 32, 86, 0.08);
}

.ranking-list,
.daily-chart {
  display: grid;
  gap: 12px;
  padding-bottom: 24px;
}

.ranking-item,
.day-card,
.empty-panel {
  padding: 16px;
  border-radius: 22px;
  background: rgba(255, 255, 255, 0.92);
  box-shadow: 0 16px 28px rgba(38, 24, 68, 0.06);
}

.ranking-main {
  display: flex;
  align-items: center;
  gap: 10px;
}

.ranking-dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;
}

.ranking-copy strong,
.ranking-side strong {
  display: block;
  color: #211734;
}

.ranking-copy span,
.ranking-side span,
.day-values span {
  font-size: 12px;
  color: #7c748f;
}

.ranking-item {
  display: grid;
  grid-template-columns: 1fr auto;
  gap: 10px 12px;
}

.ranking-side {
  text-align: right;
}

.ranking-bar {
  grid-column: 1 / -1;
  height: 8px;
  border-radius: 999px;
  background: #eee8f7;
  overflow: hidden;
}

.ranking-bar-fill {
  height: 100%;
  border-radius: inherit;
}

.day-card {
  display: grid;
  gap: 10px;
}

.day-label {
  font-size: 13px;
  font-weight: 600;
  color: #2a2140;
}

.bar-stack {
  display: grid;
  gap: 8px;
}

.bar-track {
  height: 10px;
  border-radius: 999px;
  overflow: hidden;
}

.expense-track {
  background: #ffe4d0;
}

.income-track {
  background: #dff6ea;
}

.bar-fill {
  height: 100%;
  border-radius: inherit;
}

.expense-fill {
  background: linear-gradient(90deg, #ff9f43, #ff7b45);
}

.income-fill {
  background: linear-gradient(90deg, #2ccf78, #5ae2a1);
}

.day-values {
  display: flex;
  justify-content: space-between;
  gap: 12px;
}

.empty-panel {
  min-height: 180px;
  color: #6c6482;
}

.empty-panel-icon {
  font-size: 48px;
  margin-bottom: 10px;
  color: #9d92b8;
}

@media (max-width: 420px) {
  .overview-grid,
  .insight-row {
    grid-template-columns: 1fr;
  }

  .section-head {
    flex-direction: column;
    align-items: flex-start;
  }

  .day-values {
    flex-direction: column;
    gap: 4px;
  }
}
</style>
