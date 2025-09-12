<template>
  <div class="custom-list-item">
    <div class="list-item-icon">
      <ion-avatar>
        <ion-icon :src="'/src/assets/icons/' + props.icon"/>
      </ion-avatar>
    </div>
    <div class="list-item-content">
      <div class="list-item-title">
        <div>
          {{ props.title }}-{{ props.subTitle }}
        </div>
        <div>
          -¥{{ props.cost.toFixed(2) }}
        </div>
      </div>
      <div class="list-item-subtitle">
        {{ props.time }} - {{ !!props.comment ? props.comment : '' }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {IonIcon} from "@ionic/vue";

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

</script>

<style scoped>
.custom-list {
  --ion-background-color: #ffff;
}


.custom-list-item {
  width: 100%;
  display: flex;
  flex-direction: row;

  .list-item-icon {
    padding: 5px;
    width: 20%;
  }

  .list-item-content {
    width: 80%;

    .list-item-title {
      font-weight: bold;
      display: flex;
      justify-content: space-between;
    }

    .list-item-subtitle {
      color: #6f6f6f;
      font-size: 14px;
    }
  }

}

ion-icon {
  padding: 7px;
  color: #ffff;
  font-size: 27px;
  border-radius: 13px;
  background-color: #ee6454;
}
</style>