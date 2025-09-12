<template>
  <ion-page>
    <ion-header>
      <ion-toolbar>
        <ion-buttons slot="start">
          <ion-menu-button :auto-hide="false"></ion-menu-button>
        </ion-buttons>
        <ion-title>账本</ion-title>
      </ion-toolbar>
    </ion-header>
    <ion-content>
      <div class="ledger-list-content">
        <ion-searchbar :animated="true" placeholder="搜索" :clear-icon="false"></ion-searchbar>
        <template v-for="item in billList">
          <CustomList :list="item"/>
        </template>
      </div>

      <ion-fab slot="fixed" vertical="bottom" horizontal="end">
        <ion-fab-button color="primary">
          <ion-icon src="src/assets/icons/add-outline.svg"></ion-icon>
        </ion-fab-button>
        <!--        <ion-fab-list side="top">-->
        <!--          <ion-fab-button>-->
        <!--            <ion-icon :icon="document"></ion-icon>-->
        <!--          </ion-fab-button>-->
        <!--          <ion-fab-button>-->
        <!--            <ion-icon :icon="colorPalette"></ion-icon>-->
        <!--          </ion-fab-button>-->
        <!--          <ion-fab-button>-->
        <!--            <ion-icon :icon="globe"></ion-icon>-->
        <!--          </ion-fab-button>-->
        <!--        </ion-fab-list>-->
      </ion-fab>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {IonContent, IonHeader, IonIcon, IonPage, IonToolbar} from '@ionic/vue';
import CustomList from "@/compontes/CustomList.vue";
import {onMounted, ref} from 'vue';

type BillList = BillItem[][];

const billList = ref<BillList>([]);

onMounted(() => {
  for (let i = 0; i < 3; i++) {
    let tmp = [];
    for (let j = 0; j < 5; j++) {
      tmp.push({
        title: '餐饮',
        subTitle: '烟酒',
        icon: 'fast-food-outline.svg',
        cost: 25.00,
        costType: 0,
        time: '16:34',
        comment: '芙蓉王'
      })
    }
    billList.value.push(tmp);
  }

})


</script>

<style scoped>
.ledger-list-content {
  padding-bottom: 100px;
  height: 100%;
}
</style>
