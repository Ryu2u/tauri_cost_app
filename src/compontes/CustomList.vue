<template>
  <ion-list class="custom-list" :inset="true">
    <template v-for="item in props.list">
      <ion-item>
        <ion-label class="width: 100%">
          <CustomListItem :title="item.title"
                          :sub-title="item.subTitle"
                          :icon="item.icon"
                          :cost="item.cost"
                          :time="item.time"
                          :comment="!!item.comment? item.comment : ''"
                          :cost-type="item.costType"/>
        </ion-label>
      </ion-item>
    </template>
  </ion-list>
</template>

<script setup lang="ts">
import CustomListItem from "@/compontes/CustomListItem.vue";

interface Props {
  list: BillItem[]
}

const props = defineProps<Props>();

</script>

<style scoped>
.custom-list {
  --ion-background-color: #ffff;
}
</style>