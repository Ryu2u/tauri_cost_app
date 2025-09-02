<template>
  <ion-page>
    <ion-header>
      <ion-toolbar>
        <ion-buttons slot="start">
          <ion-menu-button :auto-hide="false"></ion-menu-button>
        </ion-buttons>
        <ion-title>Menu Button</ion-title>
      </ion-toolbar>
    </ion-header>
    <ion-content>
      <div class="example-content">
        <ion-searchbar :animated="true" placeholder="搜索" :clear-icon="false"></ion-searchbar>
        <CustomList/>
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
</script>

<style scoped>
.example-content {
  height: 100%;
}
</style>
