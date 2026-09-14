<template>
  <div>
    <div class="site-pc yun_resume_h1">
      <span class="yun_resume_h1_s">
        <i v-if="icon" class="yun_resume_h1_icon" :class="icon" />
        {{ title }}
      </span>
    </div>
    <div class="site-h5 cord_work_experience_one">
      <div class="cord_intention_top_word">{{ title }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  title: string
  icon?: string
}>()
</script>
