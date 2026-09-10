<script setup lang="ts">
type Group = { id: number; name: string; parent_id?: number }

const LS_NEW = 'phpyun_article_newc'
const LS_OLD = 'phpyun_article_oldc'

const { t } = useI18n()
const api = useApi()
const { data: groups } = await useAsyncData('article-groups-channels', () =>
  api.get<Group[]>('/v1/wap/articles/groups').catch(() => [] as Group[]),
)

const all = computed(() => (groups.value || []).filter((g) => !g.parent_id))
const newc = ref<number[]>([])
const oldc = ref<number[]>([])
const editing = ref(true)

function readIds(key: string): number[] {
  if (!import.meta.client) return []
  try {
    const raw = localStorage.getItem(key)
    if (!raw) return []
    const parsed = JSON.parse(raw)
    return Array.isArray(parsed) ? parsed.map((x) => Number(x)).filter((n) => n > 0) : []
  } catch {
    return []
  }
}

function persist() {
  if (!import.meta.client) return
  localStorage.setItem(LS_NEW, JSON.stringify(newc.value))
  localStorage.setItem(LS_OLD, JSON.stringify(oldc.value))
}

onMounted(() => {
  const savedNew = readIds(LS_NEW)
  const savedOld = readIds(LS_OLD)
  const ids = all.value.map((g) => g.id)
  if (savedNew.length || savedOld.length) {
    newc.value = savedNew.filter((id) => ids.includes(id))
    const rest = ids.filter((id) => !newc.value.includes(id))
    oldc.value = (savedOld.length ? savedOld.filter((id) => ids.includes(id)) : rest).filter(
      (id) => !newc.value.includes(id),
    )
    for (const id of rest) {
      if (!oldc.value.includes(id) && !newc.value.includes(id)) oldc.value.push(id)
    }
  } else {
    newc.value = ids.slice(0, 6)
    oldc.value = ids.slice(6)
  }
})

function nameOf(id: number) {
  return all.value.find((g) => g.id === id)?.name || String(id)
}

function dropNew(id: number) {
  newc.value = newc.value.filter((x) => x !== id)
  if (!oldc.value.includes(id)) oldc.value = [...oldc.value, id]
  persist()
}

function addNew(id: number) {
  oldc.value = oldc.value.filter((x) => x !== id)
  if (!newc.value.includes(id)) newc.value = [...newc.value, id]
  persist()
}

useSeoMeta({ title: t('wap_00145') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_00145') }}</h1>
    <p>
      <NuxtLink to="/articles">{{ $t('wap_user_00298') }}</NuxtLink>
    </p>
    <h2>{{ $t('wap_01463') }}</h2>
    <p v-if="!newc.length" class="muted">{{ $t('common_02409') }}</p>
    <ul class="stack">
      <li v-for="id in newc" :key="'n' + id">
        {{ nameOf(id) }}
        <button v-if="editing" type="button" @click="dropNew(id)">{{ $t('common.delete') }}</button>
      </li>
    </ul>
    <h2>{{ $t('wap_01464') }}</h2>
    <p v-if="!oldc.length" class="muted">{{ $t('common_02409') }}</p>
    <ul class="stack">
      <li v-for="id in oldc" :key="'o' + id">
        {{ nameOf(id) }}
        <button v-if="editing" type="button" @click="addNew(id)">{{ $t('wap_js_00091') }}</button>
      </li>
    </ul>
  </section>
</template>
