<template>
  <div class="Search_jobs_form_list" :class="extraClass">
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
          v-for="(item, idx) in items"
          v-show="expanded || idx < limit"
          :key="item.id"
          :to="{ path, query: mergeQuery(route.query, { [param]: item.id }) }"
          class="Search_jobs_sub_a"
          :class="{ Search_jobs_sub_cur: current === item.id }"
        >
          {{ item.name }}
        </NuxtLink>
      </div>
    </div>
    <div v-if="items.length > limit" class="zh_more">
      <a href="javascript:;" :class="{ showcheck: expanded }" @click.prevent="expanded = !expanded">{{
        $t('common.more')
      }}</a>
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
    limit?: number
    extraClass?: string
  }>(),
  { limit: 7, extraClass: '' },
)
const route = useRoute()
const expanded = ref(false)

watch(
  () => [props.current, props.items, props.limit] as const,
  () => {
    const idx = props.items.findIndex((i) => i.id === props.current)
    if (idx >= props.limit) expanded.value = true
  },
  { immediate: true },
)
</script>
