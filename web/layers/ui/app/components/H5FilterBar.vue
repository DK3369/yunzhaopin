<template>
  <div class="h5-filter">
    <div class="job_header_nav_right">
      <ul>
        <li v-for="tab in tabs" :key="tab.key" @click="open = open === tab.key ? '' : tab.key">
          {{ tab.current || tab.label }}
          <i class="nav_right_open" />
        </li>
      </ul>
    </div>
    <div v-if="open" class="h5-filter-panel">
      <NuxtLink
        class="h5-filter-item"
        :class="{ on: !activeId }"
        :to="{ path: route.path, query: mergeQuery(route.query, clearPatch) }"
        @click="open = ''"
      >
        {{ allLabel }}
      </NuxtLink>
      <NuxtLink
        v-for="item in activeItems"
        :key="item.id"
        class="h5-filter-item"
        :class="{ on: activeId === item.id }"
        :to="{ path: route.path, query: mergeQuery(route.query, { [open]: item.id }) }"
        @click="open = ''"
      >
        {{ item.name }}
      </NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { DictItem } from '../utils/query'

const props = defineProps<{
  tabs: Array<{ key: string; label: string; current?: string; items: DictItem[] }>
  allLabel: string
}>()
const route = useRoute()
const open = ref('')
const activeItems = computed(() => props.tabs.find((t) => t.key === open.value)?.items || [])
const activeId = computed(() => numQuery(route.query[open.value]))
const clearPatch = computed(() => ({ [open.value]: undefined }))
</script>
