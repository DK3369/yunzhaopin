<template>
  <div class="Posted_body_card">
    <NuxtLink v-if="to" :to="to" class="Posted_card_top">
      <div class="Posted_card_name">{{ title }}</div>
      <div v-if="pay" class="Posted_card_pay">{{ pay }}</div>
    </NuxtLink>
    <div v-else class="Posted_card_top">
      <div class="Posted_card_name">{{ title }}</div>
      <div v-if="pay" class="Posted_card_pay">{{ pay }}</div>
    </div>
    <div v-if="tags.length" class="Posted_card_cen">
      <ul>
        <li v-for="(item, i) in tags" :key="i">{{ item }}</li>
      </ul>
    </div>
    <div v-if="sub || time" class="Posted_card_bom">
      <div class="Posted_bom_box">
        <div v-if="logo" class="Posted_box_logo">
          <img :src="logo" alt="" width="100%" height="100%" />
        </div>
        <div class="Posted_box_name">{{ sub }}</div>
      </div>
      <div class="Posted_bom_time">{{ time }}</div>
    </div>
    <slot />
  </div>
</template>

<script setup lang="ts">
withDefaults(
  defineProps<{
    title: string
    pay?: string
    sub?: string
    time?: string
    logo?: string
    to?: string
    tags?: string[]
  }>(),
  { tags: () => [] },
)
</script>
