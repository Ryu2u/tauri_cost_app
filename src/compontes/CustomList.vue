<template>
  <div class="icost-list">
    <div
        v-for="(item, index) in props.list"
        :key="index"
        class="icost-item"
        :class="{ 'last-item': index === props.list.length - 1 }"
    >
      <div class="item-left">
        <div class="item-icon" :style="{ backgroundColor: iconBackground(item.title) }">
          <ion-icon :icon="getIcon(item.icon)"/>
        </div>
        <div class="item-info">
          <div class="item-title">
            <span class="category">{{ item.title }}</span>
            <span class="detail" v-if="item.subTitle">{{ item.subTitle }}</span>
          </div>
          <div class="item-time">{{ item.time }}</div>
        </div>
      </div>
      <div class="item-right">
        <div class="item-amount" :class="{ 'income': item.costType === 1, 'expense': item.costType === 0 }">
          {{ item.costType === 1 ? '+' : '-' }}¥{{ item.cost.toFixed(2) }}
        </div>
        <div class="item-comment" v-if="item.comment">{{ item.comment }}</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
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
  list: BillItem[]
}

const props = defineProps<Props>();

// 分类颜色映射
const colorMap: { [key: string]: string } = {
  '餐饮': '#FF9500',
  '工资': '#34C759',
  '交通': '#007AFF',
  '购物': '#FF2D55',
  '娱乐': '#AF52DE',
  '住房': '#5856D6',
  '医疗': '#FF3B30',
  '教育': '#5AC8FA',
  '礼物': '#FF2D55',
  '旅行': '#FFCC00',
  '咖啡': '#A2845E',
  '通讯': '#007AFF',
  '奖金': '#34C759',
  '兼职': '#34C759'
};

// 图标映射
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

const iconBackground = (title: string) => {
  return colorMap[title as string] || '#8E8E93';
};

const getIcon = (icon: string) => {
  return iconMap[icon] || fastFoodOutline;
};
</script>

<style scoped>
.icost-list {
  background: white;
  margin: 0 12px;
  border-radius: 10px;
  overflow: hidden;
}

.icost-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 14px;
  border-bottom: 0.5px solid #E5E5EA;
}

.icost-item.last-item {
  border-bottom: none;
}

.item-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.item-icon {
  width: 36px;
  height: 36px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.item-icon ion-icon {
  font-size: 18px;
  color: white;
}

.item-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.item-title {
  display: flex;
  align-items: center;
  gap: 4px;
}

.category {
  font-size: 15px;
  color: #000;
  font-weight: 400;
}

.detail {
  font-size: 13px;
  color: #8E8E93;
}

.item-time {
  font-size: 12px;
  color: #8E8E93;
}

.item-right {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 2px;
}

.item-amount {
  font-size: 15px;
  font-weight: 500;
}

.item-amount.income {
  color: #34C759;
}

.item-amount.expense {
  color: #000;
}

.item-comment {
  font-size: 12px;
  color: #8E8E93;
  max-width: 100px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
