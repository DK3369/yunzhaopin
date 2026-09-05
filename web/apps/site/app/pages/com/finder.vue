<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-finder-list', () =>
  api.post('/v1/mcenter/finder/list', { page: 1, page_size: 20 }),
)
const form = reactive({
  name: '',
  keyword: '',
  cityid: 0,
  minsalary: '',
  maxsalary: '',
})
const msg = ref('')
async function create() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/finder', { ...form })
    msg.value = t('common.success')
    form.name = ''
    form.keyword = ''
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function remove(id: number) {
  if (!window.confirm(t('member_com_00083'))) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/finder/delete', { id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('member_com_00086') })
</script>

<template>
  <section>
    <h1>{{ $t('member_com_00086') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <form class="form" @submit.prevent="create">
      <input v-model="form.name" required :placeholder="$t('wap_00529')" />
      <input v-model="form.keyword" :placeholder="$t('common.resume')" />
      <input v-model.number="form.cityid" type="number" :placeholder="$t('common_02110')" />
      <input v-model="form.minsalary" type="number" :placeholder="$t('ui.min_salary')" />
      <input v-model="form.maxsalary" type="number" :placeholder="$t('ui.max_salary')" />
      <button type="submit">{{ $t('member_com_00556') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
    <p v-if="!(data?.list || []).length" class="muted">{{ $t('member_user_00492') }}</p>
    <div class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>{{ row.name }}</h3>
        <p class="muted">{{ row.para_n || row.para }}</p>
        <NuxtLink v-if="row.search_to" :to="row.search_to">{{ $t('common.search') }}</NuxtLink>
        <button type="button" @click="remove(row.id)">{{ $t('common.delete') }}</button>
      </article>
    </div>
  </section>
</template>
