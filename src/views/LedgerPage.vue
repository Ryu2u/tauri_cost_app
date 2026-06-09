<template>
  <ion-page>
    <ion-header class="icost-header">
      <div class="header-top">
        <div class="month-selector">
          <ion-icon :icon="chevronBack" @click="prevMonth" class="month-arrow"></ion-icon>
          <span class="current-month">{{ currentYear }}年{{ currentMonth }}月</span>
          <ion-icon :icon="chevronForward" @click="nextMonth" class="month-arrow"></ion-icon>
        </div>
        <ion-icon :icon="optionsOutline" class="header-icon"></ion-icon>
      </div>

      <div class="month-summary">
        <div class="summary-row">
          <span class="summary-label">支出</span>
          <span class="summary-amount expense">¥{{ totalExpense.toFixed(2) }}</span>
        </div>
        <div class="summary-row">
          <span class="summary-label">收入</span>
          <span class="summary-amount income">¥{{ totalIncome.toFixed(2) }}</span>
        </div>
      </div>
    </ion-header>

    <ion-content class="icost-content">
      <div v-if="loading" class="loading-state">
        <ion-spinner name="crescent"></ion-spinner>
      </div>

      <template v-else v-for="group in filteredGroupedBillList">
        <div class="date-header">
          <div class="date-info">
            <span class="date-text">{{ group.date }}</span>
            <span class="date-summary">
              支出 <span class="expense">¥{{ getDayExpense(group.items) }}</span>
              收入 <span class="income">¥{{ getDayIncome(group.items) }}</span>
            </span>
          </div>
        </div>
        <CustomList :list="group.items"/>
      </template>

      <div v-if="!loading && filteredGroupedBillList.length === 0" class="empty-state">
        <ion-icon :icon="walletOutline" class="empty-icon"></ion-icon>
        <p>暂无记账记录</p>
        <p class="empty-hint">点击右下角按钮添加第一笔账</p>
      </div>

      <ion-fab slot="fixed" vertical="bottom" horizontal="end" class="add-fab">
        <ion-fab-button color="success" @click="showAddModal">
          <ion-icon :icon="addOutline"></ion-icon>
        </ion-fab-button>
      </ion-fab>
    </ion-content>

    <AddTransactionModal
        :is-open="isAddModalOpen"
        :categories="categories"
        @close="handleCloseModal"
        @save="handleSaveTransaction"
    />
  </ion-page>
</template>

<script setup lang="ts">
import {IonContent, IonFab, IonFabButton, IonHeader, IonIcon, IonPage, IonSpinner, toastController} from '@ionic/vue';
import CustomList from "@/compontes/CustomList.vue";
import AddTransactionModal from "@/compontes/AddTransactionModal.vue";
import {computed, onMounted, ref, watch} from 'vue';
import {addOutline, chevronBack, chevronForward, optionsOutline, walletOutline} from 'ionicons/icons';
import { listCategoriesByType, listTransactionsByMonth, addTransaction } from '@/api';
import type { Category, BillItem, GroupedBill, AddTransactionRequest } from '@/types/types';

// 模态框状态
const isAddModalOpen = ref(false);

// 加载状态
const loading = ref(false);

// 日期相关
const currentYear = ref(new Date().getFullYear());
const currentMonth = ref(new Date().getMonth() + 1);

// 分类列表
const categories = ref<Category[]>([]);

// 交易列表
const billList = ref<BillItem[]>([]);

// 加载数据
async function loadCategories() {
  try {
    categories.value = await listCategoriesByType(0); // 支出分类
    const incomeCategories = await listCategoriesByType(1); // 收入分类
    categories.value = [...categories.value, ...incomeCategories];
  } catch (error) {
    console.error('加载分类失败:', error);
  }
}

async function loadTransactions() {
  loading.value = true;
  try {
    const transactions = await listTransactionsByMonth(currentYear.value, currentMonth.value);
    billList.value = transactions.map(t => ({
      icon: t.image || 'cash-outline.svg',
      title: t.category_name || '未知',
      subTitle: t.comment || '',
      cost: t.amount,
      costType: t.cost_type as 0 | 1,
      date: t.transaction_time.split(' ')[0],
      time: t.transaction_time.split(' ')[1]?.substring(0, 5) || '',
      comment: t.comment || undefined,
      color: t.color || undefined,
    }));
  } catch (error) {
    console.error('加载交易记录失败:', error);
    const toast = await toastController.create({
      message: '加载数据失败',
      duration: 2000,
      position: 'bottom',
    });
    await toast.present();
  } finally {
    loading.value = false;
  }
}

// 监听月份变化，重新加载数据
watch([currentYear, currentMonth], () => {
  loadTransactions();
});

// 计算本月收入和支出
const totalIncome = computed(() => {
  return billList.value
      .filter(item => item.costType === 1)
      .reduce((sum, item) => sum + item.cost, 0);
});

const totalExpense = computed(() => {
  return billList.value
      .filter(item => item.costType === 0)
      .reduce((sum, item) => sum + item.cost, 0);
});

// 获取当天的支出
const getDayExpense = (items: BillItem[]) => {
  return items.filter(i => i.costType === 0).reduce((sum, i) => sum + i.cost, 0).toFixed(2);
};

// 获取当天的收入
const getDayIncome = (items: BillItem[]) => {
  return items.filter(i => i.costType === 1).reduce((sum, i) => sum + i.cost, 0).toFixed(2);
};

// 按日期分组
const filteredGroupedBillList = computed((): GroupedBill[] => {
  const groups: { [key: string]: BillItem[] } = {};

  billList.value.forEach(item => {
    const dateKey = item.date;
    if (!groups[dateKey]) {
      groups[dateKey] = [];
    }
    groups[dateKey].push(item);
  });

  const result: GroupedBill[] = [];
  const today = new Date().toISOString().split('T')[0];
  const yesterday = new Date(Date.now() - 86400000).toISOString().split('T')[0];

  const weekDays = ['星期日', '星期一', '星期二', '星期三', '星期四', '星期五', '星期六'];

  Object.keys(groups).sort().reverse().forEach(date => {
    let displayDate = date;
    if (date === today) displayDate = '今天';
    else if (date === yesterday) displayDate = '昨天';
    else {
      const d = new Date(date);
      displayDate = `${d.getMonth() + 1}月${d.getDate()}日 ${weekDays[d.getDay()]}`;
    }

    result.push({
      date: displayDate,
      dateFull: date,
      items: groups[date]
    });
  });

  return result;
});

const prevMonth = () => {
  if (currentMonth.value === 1) {
    currentMonth.value = 12;
    currentYear.value--;
  } else {
    currentMonth.value--;
  }
};

const nextMonth = () => {
  if (currentMonth.value === 12) {
    currentMonth.value = 1;
    currentYear.value++;
  } else {
    currentMonth.value++;
  }
};

const showAddModal = () => {
  isAddModalOpen.value = true;
};

const handleCloseModal = () => {
  isAddModalOpen.value = false;
};

const handleSaveTransaction = async (data: {
  amount: number;
  categoryId: number;
  comment: string;
  type: number;
  date: string;
}) => {
  try {
    const request: AddTransactionRequest = {
      ledger_id: data.type === 0 ? 1 : 2, // 1=支出账本, 2=收入账本
      category_id: data.categoryId,
      amount: data.amount,
      cost_type: data.type as 0 | 1,
      comment: data.comment || undefined,
      transaction_time: data.date,
    };
    await addTransaction(request);

    const toast = await toastController.create({
      message: '保存成功',
      duration: 2000,
      position: 'bottom',
    });
    await toast.present();

    // 刷新数据
    await loadTransactions();
    handleCloseModal();
  } catch (error) {
    console.error('保存失败:', error);
    const toast = await toastController.create({
      message: '保存失败',
      duration: 2000,
      position: 'bottom',
    });
    await toast.present();
  }
};

onMounted(async () => {
  await loadCategories();
  await loadTransactions();
});
</script>

<style scoped>
.icost-header {
  background: white;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
}

.header-top {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px 12px;
}

.month-selector {
  display: flex;
  align-items: center;
  gap: 12px;
}

.current-month {
  font-size: 18px;
  font-weight: 600;
  color: #333;
}

.month-arrow {
  font-size: 20px;
  color: #666;
  cursor: pointer;
}

.header-icon {
  font-size: 22px;
  color: #666;
}

.month-summary {
  padding: 0 20px 16px;
}

.summary-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 0;
}

.summary-label {
  font-size: 14px;
  color: #666;
}

.summary-amount {
  font-size: 18px;
  font-weight: 600;
}

.summary-amount.expense {
  color: #FF9500;
}

.summary-amount.income {
  color: #007AFF;
}

.icost-content {
  background: #f5f5f5;
}

.loading-state {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 80px 20px;
}

.date-header {
  background: #f5f5f5;
  padding: 12px 16px 8px;
}

.date-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.date-text {
  font-size: 14px;
  font-weight: 500;
  color: #333;
}

.date-summary {
  font-size: 13px;
  color: #666;
}

.date-summary .expense {
  color: #FF9500;
  margin-right: 8px;
}

.date-summary .income {
  color: #007AFF;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 20px;
  color: #999;
}

.empty-icon {
  font-size: 64px;
  margin-bottom: 16px;
  opacity: 0.3;
  color: #ccc;
}

.empty-state p {
  margin: 8px 0;
}

.empty-hint {
  font-size: 13px;
  color: #bbb;
}

.add-fab {
  margin-bottom: 24px;
  margin-right: 16px;
}

.add-fab ion-fab-button {
  --background: #34C759;
  --color: white;
  --box-shadow: 0 4px 12px rgba(52, 199, 89, 0.4);
}

.add-fab ion-fab-button:hover {
  --background: #2DB840;
}
</style>
