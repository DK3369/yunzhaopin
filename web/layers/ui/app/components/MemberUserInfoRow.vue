<template>
  <div class="m_user_infolist">
    <div class="m_user_infobox">
      <div class="m_user_infonamebox">
        <div class="m_user_infoname">
          <NuxtLink v-if="to" :to="to">{{ title }}</NuxtLink>
          <span v-else>{{ title }}</span>
        </div>
        <span v-if="pay" class="m_user_info_xz">{{ pay }}</span>
      </div>
      <div v-if="meta" class="m_user_info_p">{{ meta }}</div>
      <div v-if="sub" class="m_user_info_com">{{ sub }}</div>
    </div>
    <div class="m_user_info_cz">
      <span v-if="time" class="m_user_info_time">{{ time }}</span>
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  title: string
  pay?: string
  meta?: string
  sub?: string
  time?: string
  to?: string
}>()
</script>
