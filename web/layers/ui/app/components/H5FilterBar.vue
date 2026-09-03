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
    <div v-if="openTab" class="h5-filter-panel">
      <template v-if="openTab.kind === 'more'">
        <div v-for="g in openTab.groups || []" :key="g.param" class="h5-filter-group">
          <div class="h5-filter-group-label">{{ g.label }}</div>
          <NuxtLink
            class="h5-filter-item"
            :class="{ on: !numQuery(route.query[g.param]) }"
            :to="{ path: route.path, query: mergeQuery(route.query, { [g.param]: undefined, ...(g.extraClear || {}) }) }"
            @click="open = ''"
          >
            {{ allLabel }}
          </NuxtLink>
          <NuxtLink
            v-for="item in g.items"
            :key="g.param + item.id"
            class="h5-filter-item"
            :class="{ on: numQuery(route.query[g.param]) === item.id }"
            :to="{ path: route.path, query: mergeQuery(route.query, { [g.param]: item.id }) }"
            @click="open = ''"
          >
            {{ item.name }}
          </NuxtLink>
        </div>
      </template>
      <template v-else>
        <NuxtLink
          class="h5-filter-item"
          :class="{ on: !activeId && !childId && !grandId }"
          :to="{ path: route.path, query: mergeQuery(route.query, clearPatch) }"
          @click="open = ''"
        >
          {{ allLabel }}
        </NuxtLink>
        <NuxtLink
          v-for="item in openTab.items"
          :key="item.id"
          class="h5-filter-item"
          :class="{ on: activeId === item.id }"
          :to="{
            path: route.path,
            query: mergeQuery(route.query, {
              [openTab.key]: item.id,
              ...(openTab.childKey ? { [openTab.childKey]: undefined } : {}),
              ...(openTab.grandKey ? { [openTab.grandKey]: undefined } : {}),
            }),
          }"
          @click="openTab.childKey ? undefined : (open = '')"
        >
          {{ item.name }}
        </NuxtLink>
        <template v-if="openTab.childKey && (openTab.childItems || []).length">
          <div class="h5-filter-group-label">{{ $t('common_01972') }}</div>
          <NuxtLink
            v-for="item in openTab.childItems"
            :key="'c' + item.id"
            class="h5-filter-item"
            :class="{ on: childId === item.id }"
            :to="{
              path: route.path,
              query: mergeQuery(route.query, {
                [openTab.childKey]: item.id,
                ...(openTab.grandKey ? { [openTab.grandKey]: undefined } : {}),
              }),
            }"
            @click="openTab.grandKey ? undefined : (open = '')"
          >
            {{ item.name }}
          </NuxtLink>
        </template>
        <template v-if="openTab.grandKey && (openTab.grandItems || []).length">
          <div class="h5-filter-group-label">{{ $t('admin_00223') }}</div>
          <NuxtLink
            v-for="item in openTab.grandItems"
            :key="'g' + item.id"
            class="h5-filter-item"
            :class="{ on: grandId === item.id }"
            :to="{ path: route.path, query: mergeQuery(route.query, { [openTab.grandKey]: item.id }) }"
            @click="open = ''"
          >
            {{ item.name }}
          </NuxtLink>
        </template>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { DictItem } from '../utils/query'

export type H5FilterTab = {
  key: string
  label: string
  current?: string
  items: DictItem[]
  childKey?: string
  childItems?: DictItem[]
  grandKey?: string
  grandItems?: DictItem[]
  kind?: 'list' | 'more'
  groups?: Array<{
    label: string
    param: string
    items: DictItem[]
    extraClear?: Record<string, undefined>
  }>
}

const props = defineProps<{
  tabs: H5FilterTab[]
  allLabel: string
}>()
const route = useRoute()
const open = ref('')
const openTab = computed(() => props.tabs.find((t) => t.key === open.value))
const activeId = computed(() => (openTab.value ? numQuery(route.query[openTab.value.key]) : undefined))
const childId = computed(() =>
  openTab.value?.childKey ? numQuery(route.query[openTab.value.childKey]) : undefined,
)
const grandId = computed(() =>
  openTab.value?.grandKey ? numQuery(route.query[openTab.value.grandKey]) : undefined,
)
const clearPatch = computed(() => {
  const tab = openTab.value
  if (!tab) return {}
  const patch: Record<string, undefined> = { [tab.key]: undefined }
  if (tab.childKey) patch[tab.childKey] = undefined
  if (tab.grandKey) patch[tab.grandKey] = undefined
  return patch
})
</script>
