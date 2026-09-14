<template>
  <div class="job_td_list_box">
    <div v-for="g in groups" :key="g.label" class="job_td_list">
      {{ g.label }}
      <a
        v-for="opt in g.options"
        :key="String(opt.value)"
        href="javascript:;"
        :class="opt.on ? 'job_td_list_cur' : 'job_td_list_a'"
        @click.prevent="opt.select()"
      >{{ opt.label }}</a>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  groups: Array<{
    label: string
    options: Array<{ value: unknown; label: string; on: boolean; select: () => void }>
  }>
}>()
</script>
