<template>
  <div class="Search_jobs_more_chlose" @mouseenter="open = true" @mouseleave="open = false">
    <span class="Search_jobs_more_chlose_s">{{ currentName || label }}</span>
    <i />
    <div
      :class="[
        wide ? 'Search_jobs_more_chlose_hylist' : 'Search_jobs_more_chlose_list',
        { none: !open },
      ]"
    >
      <ul>
        <li>
          <NuxtLink :to="{ path, query: mergeQuery(route.query, { [param]: undefined }) }">{{ allLabel }}</NuxtLink>
        </li>
        <li v-for="item in items" :key="item.id">
          <NuxtLink :to="{ path, query: mergeQuery(route.query, { [param]: item.id }) }">{{ item.name }}</NuxtLink>
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { DictItem } from '../utils/query'

const props = withDefaults(
  defineProps<{
    label: string
    param: string
    items: DictItem[]
    current?: number
    path: string
    allLabel: string
    wide?: boolean
  }>(),
  { wide: false },
)

const route = useRoute()
const open = ref(false)
const currentName = computed(() => props.items.find((x) => x.id === props.current)?.name || '')
</script>
