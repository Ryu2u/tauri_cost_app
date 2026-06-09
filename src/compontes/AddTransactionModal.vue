<template>
  <ion-modal
    :is-open="isOpen"
    @didDismiss="handleClose"
    :breakpoints="[0, 1]"
    :initial-breakpoint="1"
    class="icost-add-modal"
  >
    <div class="modal-content">
      <!-- 顶部日期和备注 -->
      <div class="top-section">
        <div class="date-display">
          <ion-icon :icon="calendarOutline"></ion-icon>
          <span>{{ formattedDate }}</span>
        </div>
        <div class="note-input-wrapper">
          <input
            type="text"
            v-model="comment"
            placeholder="添加备注"
            class="note-input"
          />
        </div>
      </div>

      <!-- 金额输入 -->
      <div class="amount-section">
        <div class="type-toggle">
          <div
            class="toggle-btn"
            :class="{ active: transactionType === 0 }"
            @click="transactionType = 0"
          >
            支出
          </div>
          <div class="toggle-btn"
            :class="{ active: transactionType === 1 }"
            @click="transactionType = 1"
          >
            收入
          </div>
        </div>
        <div class="amount-display-area">
          <span class="currency">¥</span>
          <div class="amount-text">{{ amountDisplay || '0' }}</div>
        </div>
      </div>

      <!-- 分类选择 -->
      <div class="category-section">
        <div class="category-scroll">
          <div
            v-for="cat in filteredCategories"
            :key="cat.id"
            class="category-item"
            :class="{ active: selectedCategory?.id === cat.id }"
            @click="selectCategory(cat)"
          >
            <div class="category-icon" :style="{ backgroundColor: cat.color }">
              <ion-icon :icon="getIcon(cat.image)" />
            </div>
            <span class="category-name">{{ cat.name }}</span>
          </div>
        </div>
      </div>

      <!-- 自定义键盘 -->
      <div class="keyboard-section">
        <div class="quick-amounts">
          <div
            class="quick-btn"
            v-for="quick in quickAmounts"
            :key="quick"
            @click="addQuickAmount(quick)"
          >
            {{ quick }}
          </div>
        </div>
        <div class="keyboard-grid">
          <div class="keyboard-row">
            <div class="key" @click="appendNumber('1')">1</div>
            <div class="key" @click="appendNumber('2')">2</div>
            <div class="key" @click="appendNumber('3')">3</div>
          </div>
          <div class="keyboard-row">
            <div class="key" @click="appendNumber('4')">4</div>
            <div class="key" @click="appendNumber('5')">5</div>
            <div class="key" @click="appendNumber('6')">6</div>
          </div>
          <div class="keyboard-row">
            <div class="key" @click="appendNumber('7')">7</div>
            <div class="key" @click="appendNumber('8')">8</div>
            <div class="key" @click="appendNumber('9')">9</div>
          </div>
          <div class="keyboard-row">
            <div class="key" @click="appendDot()">.</div>
            <div class="key" @click="appendNumber('0')">0</div>
            <div class="key delete-key" @click="deleteLast">
              <ion-icon :icon="backspaceOutline"></ion-icon>
            </div>
          </div>
        </div>
      </div>

      <!-- 底部按钮 -->
      <div class="bottom-section">
        <ion-button
          expand="block"
          class="save-btn"
          :disabled="!canSave"
          @click="handleSave"
        >
          保存
        </ion-button>
      </div>
    </div>
  </ion-modal>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import {
  IonModal,
  IonIcon,
  IonButton
} from '@ionic/vue';
import {
  calendarOutline,
  fastFoodOutline,
  cashOutline,
  trainOutline,
  bagOutline,
  filmOutline,
  homeOutline,
  giftOutline,
  airplaneOutline,
  phonePortraitOutline,
  pricetagOutline,
  cartOutline,
  medicalOutline,
  schoolOutline,
  ribbonOutline,
  todayOutline,
  backspaceOutline,
  bookOutline,
  heartOutline,
  cardOutline,
} from 'ionicons/icons';
import type { Category } from '@/types/types';

interface Emits {
  (e: 'close'): void;
  (e: 'save', data: {
    amount: number;
    categoryId: number;
    comment: string;
    type: number;
    date: string;
  }): void;
}

const props = defineProps<{
  isOpen: boolean;
  categories: Category[];
}>();
const emit = defineEmits<Emits>();

// 图标映射
const iconMap: Record<string, any> = {
  'fast-food-outline.svg': fastFoodOutline,
  'train-outline.svg': trainOutline,
  'bag-outline.svg': bagOutline,
  'film-outline.svg': filmOutline,
  'home-outline.svg': homeOutline,
  'gift-outline.svg': giftOutline,
  'airplane-outline.svg': airplaneOutline,
  'phone-portrait-outline.svg': phonePortraitOutline,
  'pricetag-outline.svg': pricetagOutline,
  'cart-outline.svg': cartOutline,
  'medical-outline.svg': medicalOutline,
  'school-outline.svg': schoolOutline,
  'ribbon-outline.svg': ribbonOutline,
  'today-outline.svg': todayOutline,
  'book-outline.svg': bookOutline,
  'heart-outline.svg': heartOutline,
  'cash-outline.svg': cashOutline,
  'card-outline.svg': cardOutline,
};

function getIcon(iconName: string) {
  return iconMap[iconName] || cashOutline;
}

// 交易类型 0:支出 1:收入
const transactionType = ref(0);
const amount = ref<number>(0);
const amountDisplay = ref('');
const selectedCategory = ref<Category | null>(null);
const comment = ref('');

// 快捷金额
const quickAmounts = ['10', '20', '50', '100', '200', '500'];

// 当前过滤分类
const filteredCategories = computed(() => {
  return props.categories.filter(cat => cat.ledger_type === transactionType.value || cat.ledger_type === 2);
});

// 追加数字
const appendNumber = (num: string) => {
  if (num === '.' && amountDisplay.value.includes('.')) return;
  if (amountDisplay.value.includes('.')) {
    const decimal = amountDisplay.value.split('.')[1];
    if (decimal && decimal.length >= 2) return;
  }
  if (amountDisplay.value.length >= 10) return;

  amountDisplay.value += num;
  amount.value = parseFloat(amountDisplay.value) || 0;
};

// 添加快捷金额
const addQuickAmount = (quick: string) => {
  if (amountDisplay.value === '0' || amountDisplay.value === '') {
    amountDisplay.value = quick;
  } else {
    const current = parseFloat(amountDisplay.value) || 0;
    amountDisplay.value = (current + parseFloat(quick)).toString();
  }
  amount.value = parseFloat(amountDisplay.value) || 0;
};

// 添加小数点
const appendDot = () => {
  if (!amountDisplay.value) {
    amountDisplay.value = '0.';
  } else if (!amountDisplay.value.includes('.')) {
    amountDisplay.value += '.';
  }
};

// 删除最后一个字符
const deleteLast = () => {
  if (amountDisplay.value.length > 0) {
    amountDisplay.value = amountDisplay.value.slice(0, -1);
    amount.value = parseFloat(amountDisplay.value) || 0;
  }
};

// 格式化日期
const formattedDate = computed(() => {
  const now = new Date();
  const weekDays = ['周日', '周一', '周二', '周三', '周四', '周五', '周六'];
  return `${now.getMonth() + 1}月${now.getDate()}日 ${weekDays[now.getDay()]}`;
});

// 当前日期
const selectedDate = computed(() => {
  const now = new Date();
  return `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}-${String(now.getDate()).padStart(2, '0')}`;
});

// 是否可以保存
const canSave = computed(() => {
  return amount.value > 0 && selectedCategory.value;
});

// 选择分类
const selectCategory = (cat: Category) => {
  selectedCategory.value = cat;
};

// 关闭
const handleClose = () => {
  resetForm();
  emit('close');
};

// 保存
const handleSave = () => {
  if (!canSave.value || !selectedCategory.value) return;

  emit('save', {
    amount: amount.value,
    categoryId: selectedCategory.value.id,
    comment: comment.value,
    type: transactionType.value,
    date: selectedDate.value
  });

  resetForm();
};

// 重置表单
const resetForm = () => {
  transactionType.value = 0;
  amount.value = 0;
  amountDisplay.value = '';
  selectedCategory.value = null;
  comment.value = '';
};

// 监听弹窗打开，重置表单
watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    resetForm();
  }
});
</script>

<style scoped>
.icost-add-modal {
  --border-radius: 16px 16px 0 0;
}

.modal-content {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: #F2F2F7;
}

.top-section {
  padding: 16px 20px;
  background: white;
}

.date-display {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  color: #000;
  font-size: 17px;
  font-weight: 500;
  margin-bottom: 12px;
}

.date-display ion-icon {
  font-size: 18px;
  color: #007AFF;
}

.note-input-wrapper {
  background: #F2F2F7;
  border-radius: 10px;
  padding: 10px 14px;
}

.note-input {
  width: 100%;
  border: none;
  outline: none;
  font-size: 16px;
  background: transparent;
  color: #000;
}

.note-input::placeholder {
  color: #8E8E93;
}

.amount-section {
  padding: 20px 20px 16px;
  background: white;
}

.type-toggle {
  display: flex;
  background: #F2F2F7;
  border-radius: 8px;
  padding: 3px;
  margin-bottom: 16px;
}

.toggle-btn {
  flex: 1;
  padding: 8px;
  text-align: center;
  border-radius: 6px;
  font-size: 15px;
  color: #8E8E93;
  cursor: pointer;
  transition: all 0.2s;
}

.toggle-btn.active {
  background: white;
  color: #000;
  font-weight: 500;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.amount-display-area {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
  min-height: 50px;
}

.currency {
  font-size: 32px;
  font-weight: 600;
  color: #000;
}

.amount-text {
  font-size: 40px;
  font-weight: 600;
  color: #000;
  min-width: 100px;
  text-align: center;
}

.keyboard-section {
  background: white;
  padding: 12px 0;
  border-top: 0.5px solid #E5E5EA;
}

.quick-amounts {
  display: flex;
  justify-content: space-around;
  padding: 0 16px 12px;
  border-bottom: 0.5px solid #E5E5EA;
}

.quick-btn {
  padding: 8px 16px;
  background: #F2F2F7;
  border-radius: 6px;
  font-size: 15px;
  color: #000;
  cursor: pointer;
}

.keyboard-grid {
  padding: 8px 12px 0;
}

.keyboard-row {
  display: flex;
  justify-content: center;
  gap: 8px;
  margin-bottom: 8px;
}

.key {
  flex: 1;
  max-width: 100px;
  height: 50px;
  background: #F2F2F7;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
  color: #000;
  cursor: pointer;
  user-select: none;
}

.key:active {
  background: #D1D1D6;
}

.delete-key {
  background: #A8A8AD;
}

.delete-key:active {
  background: #8E8E93;
}

.delete-key ion-icon {
  font-size: 22px;
  color: white;
}

.category-section {
  flex: 1;
  background: white;
  padding: 16px 0;
  overflow-y: auto;
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.category-section::-webkit-scrollbar {
  display: none;
}

.category-scroll {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  padding: 0 16px;
}

.category-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  cursor: pointer;
  padding: 8px;
  border-radius: 10px;
  transition: all 0.2s;
}

.category-item:active {
  background: #F2F2F7;
}

.category-item.active {
  background: #F2F2F7;
}

.category-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.category-icon ion-icon {
  font-size: 24px;
  color: white;
}

.category-name {
  font-size: 13px;
  color: #000;
}

.category-item.active .category-name {
  font-weight: 500;
}

.bottom-section {
  padding: 16px 20px;
  background: white;
  padding-bottom: calc(16px + env(safe-area-inset-bottom));
}

.save-btn {
  --background: #007AFF;
  --border-radius: 12px;
  height: 50px;
  font-size: 17px;
  font-weight: 500;
  --background-activated: #0066CC;
}

.save-btn:disabled {
  --background: #C7C7CC;
  opacity: 1;
}
</style>
