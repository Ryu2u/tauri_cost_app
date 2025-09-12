<template>
  <ion-page>
    <ion-header>
      <ion-toolbar>
        <ion-title>统计</ion-title>
      </ion-toolbar>
    </ion-header>
    <ion-content>
      <div class="ledger-list-content">Radio content</div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {IonContent, IonHeader, IonPage, IonTitle, IonToolbar} from '@ionic/vue';
</script>

<style scoped>
.ledger-list-content {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
}
</style>