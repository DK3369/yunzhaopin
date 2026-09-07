<template>
  <div class="Search_jobs_form_list">
    <div class="Search_jobs_name">{{ label }}：</div>
    <div class="Search_jobs_sub">
      <div class="Search_jobs_sub_Box">
        <NuxtLink
          :to="{
            path,
            query: mergeQuery(route.query, {
              country: undefined,
              province_id: undefined,
              city_id: undefined,
              three_city_id: undefined,
            }),
          }"
          class="Search_jobs_sub_a"
          :class="{ Search_jobs_sub_cur: !current }"
        >
          {{ allLabel }}
        </NuxtLink>
        <NuxtLink
          v-for="item in items"
          :key="item.code"
          :to="{
            path,
            query: mergeQuery(route.query, {
              country: item.code,
              province_id: undefined,
              city_id: undefined,
              three_city_id: undefined,
            }),
          }"
          class="Search_jobs_sub_a"
          :class="{ Search_jobs_sub_cur: current === item.code }"
        >
          {{ item.name }}
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { CountryOpt } from '../composables/useRegionCascade'

defineProps<{
  label: string
  items: CountryOpt[]
  current?: string
  path: string
  allLabel: string
}>()
const route = useRoute()
</script>
