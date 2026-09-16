<template>
  <div :class="cls.root">
    <div :class="cls.name">{{ label }}：</div>
    <div :class="cls.sub">
      <NuxtLink
        :to="{ path, query: mergeQuery(route.query, { [param]: undefined, ...(extraClear || {}) }) }"
        :class="[cls.a, { [cls.cur]: !current }]"
      >
        {{ allLabel }}
      </NuxtLink>
      <NuxtLink
        v-for="(item, idx) in items"
        v-show="expanded || idx < limit"
        :key="item.id"
        :to="{ path, query: mergeQuery(route.query, { [param]: item.id, ...(extraClear || {}) }) }"
        :class="[cls.a, { [cls.cur]: current === item.id }]"
      >
        {{ item.name }}
      </NuxtLink>
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
    extraClear?: Record<string, undefined>
    /** 找企业列表用 PHP `fiem_seach_*`，职位/简历仍用 `Search_jobs_*` */
    skin?: 'jobs' | 'firm'
  }>(),
  { limit: 7, extraClass: '', skin: 'jobs' },
)
const cls = computed(() =>
  props.skin === 'firm'
    ? {
        root: ['fiem_seach_chlose_list', props.extraClass],
        name: 'fiem_seach_chlosename',
        sub: 'fiem_seach_chlose_list_r',
        a: 'fiem_seach_chlose_list_a',
        cur: 'fiem_seach_chlose_list_cur',
      }
    : {
        root: ['Search_jobs_form_list', props.extraClass],
        name: 'Search_jobs_name',
        sub: 'Search_jobs_sub',
        a: 'Search_jobs_sub_a',
        cur: 'Search_jobs_sub_cur',
      },
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
