<template>
  <ion-page>
    <ion-header>
      <ion-toolbar>
        <ion-title>资产</ion-title>
      </ion-toolbar>
    </ion-header>
    <ion-content>
      <div class="example-content">
        <div>资产</div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script lang="ts">
import {IonContent, IonHeader, IonPage, IonTitle, IonToolbar} from '@ionic/vue';

export default {
  components: {IonHeader, IonToolbar, IonTitle, IonContent, IonPage},
};
</script>

<style scoped>
.example-content {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
}
</style>
