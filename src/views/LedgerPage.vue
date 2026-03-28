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
      <template v-for="group in filteredGroupedBillList">
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

      <div v-if="filteredGroupedBillList.length === 0" class="empty-state">
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
        @close="handleCloseModal"
        @save="handleSaveTransaction"
    />
  </ion-page>
</template>

<script setup lang="ts">
import {IonContent, IonFab, IonFabButton, IonHeader, IonIcon, IonPage} from '@ionic/vue';
import CustomList from "@/compontes/CustomList.vue";
import AddTransactionModal from "@/compontes/AddTransactionModal.vue";
import {computed, onMounted, ref} from 'vue';
import {addOutline, chevronBack, chevronForward, optionsOutline, walletOutline} from 'ionicons/icons';

// 模态框状态
const isAddModalOpen = ref(false);

// 日期相关
const currentYear = ref(new Date().getFullYear());
const currentMonth = ref(new Date().getMonth() + 1);

// 分类映射
const categoryMap: { [key: number]: { title: string; icon: string; subTitle: string } } = {
  1: {title: '餐饮', icon: 'fast-food-outline.svg', subTitle: '餐费'},
  2: {title: '交通', icon: 'train-outline.svg', subTitle: '出行'},
  3: {title: '购物', icon: 'bag-outline.svg', subTitle: '购物'},
  4: {title: '娱乐', icon: 'film-outline.svg', subTitle: '娱乐'},
  5: {title: '住房', icon: 'home-outline.svg', subTitle: '住房'},
  6: {title: '医疗', icon: 'heart-outline.svg', subTitle: '医疗'},
  7: {title: '教育', icon: 'book-outline.svg', subTitle: '教育'},
  8: {title: '礼物', icon: 'gift-outline.svg', subTitle: '礼物'},
  9: {title: '旅行', icon: 'airplane-outline.svg', subTitle: '旅行'},
  10: {title: '通讯', icon: 'phone-portrait-outline.svg', subTitle: '通讯'},
  11: {title: '工资', icon: 'cash-outline.svg', subTitle: '薪资'},
  12: {title: '奖金', icon: 'cash-outline.svg', subTitle: '奖金'},
  13: {title: '兼职', icon: 'cash-outline.svg', subTitle: '兼职'},
};

// 模拟账单数据
interface GroupedBill {
  date: string;
  dateFull: string;
  items: BillItem[];
}

const billList = ref<BillItem[]>([]);

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

const handleSaveTransaction = (data: {
  amount: number;
  categoryId: number;
  comment: string;
  type: number;
  date: string;
}) => {
  const category = categoryMap[data.categoryId];
  const now = new Date();
  const time = `${String(now.getHours()).padStart(2, '0')}:${String(now.getMinutes()).padStart(2, '0')}`;

  const newBill: BillItem = {
    title: category?.title || '其他',
    subTitle: category?.subTitle || '',
    icon: category?.icon || 'cash-outline.svg',
    cost: data.amount,
    costType: data.type,
    date: data.date,
    time: time,
    comment: data.comment
  };

  billList.value.unshift(newBill);
  handleCloseModal();
};

onMounted(() => {
  // 模拟数据
  const today = new Date().toISOString().split('T')[0];
  const yesterday = new Date(Date.now() - 86400000).toISOString().split('T')[0];
  const lastWeek = new Date(Date.now() - 7 * 86400000).toISOString().split('T')[0];

  billList.value = [
    {
      title: '餐饮',
      subTitle: '三餐',
      icon: 'fast-food-outline.svg',
      cost: 25.00,
      costType: 0,
      date: today,
      time: '16:34',
      comment: '午餐'
    },
    {
      title: '工资',
      subTitle: '薪资',
      icon: 'cash-outline.svg',
      cost: 8000.00,
      costType: 1,
      date: today,
      time: '09:00',
      comment: '月工资'
    },
    {
      title: '交通',
      subTitle: '地铁',
      icon: 'train-outline.svg',
      cost: 4.00,
      costType: 0,
      date: yesterday,
      time: '08:30',
      comment: '上班通勤'
    },
    {
      title: '购物',
      subTitle: '日用品',
      icon: 'bag-outline.svg',
      cost: 156.50,
      costType: 0,
      date: yesterday,
      time: '19:20',
      comment: '超市采购'
    },
    {
      title: '娱乐',
      subTitle: '电影',
      icon: 'film-outline.svg',
      cost: 45.00,
      costType: 0,
      date: lastWeek,
      time: '21:00',
      comment: '周末电影'
    }
  ];
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
