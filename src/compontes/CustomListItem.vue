<template>
  <div class="custom-list-item">
    <div class="list-item-icon" :style="{ backgroundColor: iconBackgroundColor }">
      <ion-icon :icon="categoryIcon"/>
    </div>
    <div class="list-item-content">
      <div class="list-item-title">
        <div class="category-info">
          <span class="category-name">{{ props.title }}</span>
          <span class="category-detail" v-if="props.subTitle">{{ props.subTitle }}</span>
        </div>
        <div class="amount" :class="{ 'income': props.costType === 1, 'expense': props.costType === 0 }">
          {{ props.costType === 1 ? '+' : '-' }}¥{{ props.cost.toFixed(2) }}
        </div>
      </div>
      <div class="list-item-subtitle">
        <span class="time">{{ props.time }}</span>
        <span class="comment" v-if="props.comment">{{ props.comment }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed} from 'vue';
import {IonIcon} from "@ionic/vue";
import {
  airplaneOutline,
  bagOutline,
  bookOutline,
  cafeOutline,
  cashOutline,
  fastFoodOutline,
  filmOutline,
  giftOutline,
  heartOutline,
  homeOutline,
  phonePortraitOutline,
  trainOutline
} from 'ionicons/icons';

interface Props {
  icon: String,
  title: String,
  subTitle: String,
  cost: Number,
  costType: Number,
  time: String,
  comment?: String
}

const props = defineProps<Props>();

// 分类图标映射
const iconMap: { [key: string]: any } = {
  'fast-food-outline.svg': fastFoodOutline,
  'cash-outline.svg': cashOutline,
  'train-outline.svg': trainOutline,
  'bag-outline.svg': bagOutline,
  'film-outline.svg': filmOutline,
  'home-outline.svg': homeOutline,
  'heart-outline.svg': heartOutline,
  'book-outline.svg': bookOutline,
  'gift-outline.svg': giftOutline,
  'airplane-outline.svg': airplaneOutline,
  'cafe-outline.svg': cafeOutline,
  'phone-portrait-outline.svg': phonePortraitOutline
};

// 分类颜色映射
const colorMap: { [key: string]: string } = {
  '餐饮': '#FF6B6B',
  '工资': '#4ECDC4',
  '交通': '#45B7D1',
  '购物': '#96CEB4',
  '娱乐': '#FFEAA7',
  '住房': '#DDA0DD',
  '医疗': '#FF7675',
  '教育': '#74B9FF',
  '礼物': '#FD79A8',
  '旅行': '#FDCB6E',
  '咖啡': '#E17055',
  '通讯': '#00CEC9',
  '健身': '#00B894',
  '游戏': '#A29BFE',
  '宠物': '#FDCB6E'
};

const categoryIcon = computed(() => {
  return iconMap[props.icon as string] || fastFoodOutline;
});

const iconBackgroundColor = computed(() => {
  return colorMap[props.title as string] || '#667eea';
});
</script>

<style scoped>
.custom-list {
  --ion-background-color: #ffff;
}

.custom-list-item {
  width: 100%;
  display: flex;
  flex-direction: row;
  padding: 12px 0;
  border-bottom: 1px solid #f0f0f0;
}

.list-item-icon {
  width: 48px;
  height: 48px;
  border-radius: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.list-item-icon ion-icon {
  font-size: 24px;
  color: white;
}

.list-item-content {
  flex: 1;
  padding-left: 12px;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.list-item-title {
  font-weight: 500;
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 4px;
}

.category-info {
  display: flex;
  align-items: center;
  gap: 6px;
}

.category-name {
  font-size: 16px;
  color: #333;
}

.category-detail {
  font-size: 13px;
  color: #999;
}

.amount {
  font-size: 16px;
  font-weight: 600;
}

.amount.income {
  color: #4ECDC4;
}

.amount.expense {
  color: #FF6B6B;
}

.list-item-subtitle {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
}

.time {
  color: #999;
}

.comment {
  color: #bbb;
  max-width: 150px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
