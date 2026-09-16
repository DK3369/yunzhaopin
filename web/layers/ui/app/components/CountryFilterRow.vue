<template>
  <div :class="cls.root">
    <div :class="cls.name">{{ label }}：</div>
    <div :class="cls.sub">
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
        :class="[cls.a, { [cls.cur]: !current }]"
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
        :class="[cls.a, { [cls.cur]: current === item.code }]"
      >
        {{ item.name }}
      </NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { CountryOpt } from '../composables/useRegionCascade'

const props = withDefaults(
  defineProps<{
    label: string
    items: CountryOpt[]
    current?: string
    path: string
    allLabel: string
    skin?: 'jobs' | 'firm'
  }>(),
  { skin: 'jobs' },
)
const route = useRoute()
const cls = computed(() =>
  props.skin === 'firm'
    ? {
        root: 'fiem_seach_chlose_list',
        name: 'fiem_seach_chlosename',
        sub: 'fiem_seach_chlose_list_r',
        a: 'fiem_seach_chlose_list_a',
        cur: 'fiem_seach_chlose_list_cur',
      }
    : {
        root: 'Search_jobs_form_list',
        name: 'Search_jobs_name',
        sub: 'Search_jobs_sub',
        a: 'Search_jobs_sub_a',
        cur: 'Search_jobs_sub_cur',
      },
)
</script>

