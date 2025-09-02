<template>
  <ion-list class="custom-list" :inset="true">
    <ion-item>
      <ion-label>
        <div class="custom-list-item">
          <div class="list-item-icon">
            <ion-icon src="/src/assets/icons/fast-food.svg" size="large"/>
          </div>
          <div>
            Pokémon Yellow
          </div>
        </div>
      </ion-label>
    </ion-item>
    <ion-item>
      <ion-label>Mega Man X</ion-label>
    </ion-item>
    <ion-item>
      <ion-label>The Legend of Zelda</ion-label>
    </ion-item>
    <ion-item>
      <ion-label>Pac-Man</ion-label>
    </ion-item>
    <ion-item>
      <ion-label>Super Mario World</ion-label>
    </ion-item>
  </ion-list>
</template>

<script setup lang="ts">

import {IonIcon} from "@ionic/vue";
</script>

<style scoped>
.custom-list {
  --ion-background-color: #ffff;
}

.custom-list-item {
  display: flex;
  flex-direction: row;

}

.list-item-icon {
  padding: 10px;

}

</style>