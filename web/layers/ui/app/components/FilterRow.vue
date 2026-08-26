<template>
  <div class="Search_jobs_form_list">
    <div class="Search_jobs_name">{{ label }}：</div>
    <div class="Search_jobs_sub">
      <div class="Search_jobs_sub_Box">
        <NuxtLink
          :to="{ path, query: mergeQuery(route.query, { [param]: undefined }) }"
          class="Search_jobs_sub_a"
          :class="{ Search_jobs_sub_cur: !current }"
        >
          {{ allLabel }}
        </NuxtLink>
        <NuxtLink
          v-for="item in items"
          :key="item.id"
          :to="{ path, query: mergeQuery(route.query, { [param]: item.id }) }"
          class="Search_jobs_sub_a"
          :class="{ Search_jobs_sub_cur: current === item.id }"
        >
          {{ item.name }}
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { DictItem } from '../utils/query'

defineProps<{
  label: string
  param: string
  items: DictItem[]
  current?: number
  path: string
  allLabel: string
}>()
const route = useRoute()
</script>
