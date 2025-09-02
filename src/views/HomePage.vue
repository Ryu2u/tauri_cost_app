<template>
  <ion-page>
    <ion-tabs>
      <ion-router-outlet></ion-router-outlet>
      <ion-tab-bar slot="bottom">

        <ion-tab-button tab="ledger" href="/ledgerPage">
          <ion-icon :icon="radio"/>
          <ion-label>账本</ion-label>
        </ion-tab-button>

        <ion-tab-button tab="library" href="/assetsPage">
          <ion-icon :icon="library"/>
          <ion-label>资产</ion-label>
        </ion-tab-button>

        <ion-tab-button tab="statistics" href="/statisticsPage">
          <ion-icon :icon="library"/>
          <ion-label>统计</ion-label>
        </ion-tab-button>

      </ion-tab-bar>
    </ion-tabs>
  </ion-page>
</template>

<script setup lang="ts">
import {IonIcon, IonLabel, IonPage, IonRouterOutlet, IonTabBar, IonTabButton, IonTabs} from '@ionic/vue';
import {library, radio} from 'ionicons/icons';
</script>

<style scoped>
#container {
  text-align: center;

  position: absolute;
  left: 0;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
}

#container strong {
  font-size: 20px;
  line-height: 26px;
}

#container p {
  font-size: 16px;
  line-height: 22px;

  color: #8c8c8c;

  margin: 0;
}

#container a {
  text-decoration: none;
}
</style>
