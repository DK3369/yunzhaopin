<script setup lang="ts">
import { seoJoin } from '~/utils/seo'
import type { CompanyLike, JobLike } from '~/utils/site'

const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { me } = useSiteChrome()
const tab = computed(() => String(useRoute().query.tab || 'intro'))
const { data } = await useAsyncData(`fair-${id}`, () => api.get('/v1/wap/zph/detail', { id }))
const { data: companies } = await useAsyncData(`fair-com-${id}`, () =>
  api.get<{ list?: CompanyLike[] }>('/v1/wap/zph/companies', { id, page: 1, page_size: 20 }).catch(() => ({ list: [] })),
)
const { data: jobs } = await useAsyncData(`fair-job-${id}`, async () => {
  const raw = await api.get<JobLike[] | { list?: JobLike[] }>('/v1/wap/zph/jobs', { id, page: 1, page_size: 20 }).catch(() => [])
  if (Array.isArray(raw)) return raw
  return raw?.list || []
})
const { data: spaces } = await useAsyncData(`fair-space-${id}`, () =>
  api.get<Array<{ id: number; name: string; price: number; taken: boolean }>>('/v1/wap/zph/spaces', { id }).catch(() => []),
)
const reserveMsg = ref('')
const reserveForm = reactive({
  bid: 0,
  name: '',
  moblie: '',
  job_ids: [] as number[],
})
const comStatus = ref<{ state?: string; status?: number; jobs?: Array<{ id: number; name: string }> } | null>(null)
async function loadComStatus() {
  if (!me.value || me.value.usertype !== 2) return
  try {
    comStatus.value = await api.post('/v1/mcenter/zph/com-status', { id })
    if ((comStatus.value?.jobs || []).length && !reserveForm.job_ids.length) {
      reserveForm.job_ids = (comStatus.value?.jobs || []).map((j) => j.id)
    }
  } catch {
    comStatus.value = null
  }
}
onMounted(loadComStatus)
watch(() => me.value?.uid, loadComStatus)
async function goReserve() {
  if (!me.value) {
    await navigateTo({ path: '/login', query: { next: `/fairs/${id}?tab=reserve` } })
    return
  }
  if (me.value.usertype !== 2) {
    reserveMsg.value = t('wap_01342')
    await navigateTo({ query: { tab: 'reserve' } })
    return
  }
  await navigateTo({ query: { tab: 'reserve' } })
}
async function submitReserve() {
  reserveMsg.value = ''
  if (!me.value) {
    await navigateTo({ path: '/login', query: { next: `/fairs/${id}?tab=reserve` } })
    return
  }
  try {
    await api.post('/v1/mcenter/zph/reserve', {
      id,
      bid: reserveForm.bid,
      name: reserveForm.name,
      moblie: reserveForm.moblie,
      job_ids: reserveForm.job_ids.join(','),
    })
    reserveMsg.value = t('common.success')
    await loadComStatus()
  } catch (e: unknown) {
    reserveMsg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
useSeoMeta({
  title: () => String(data.value?.title || t('ui.fairs')),
  description: () => seoJoin([data.value?.address, data.value?.body, data.value?.title]),
})
useHead({ link: [{ rel: 'canonical', href: `/fairs/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.title || $t('zph_00001') }}</h1>
    <p v-if="data?.address" class="muted">{{ data.address }} · {{ data.start_at_n }}</p>
    <p>
      <NuxtLink :to="{ query: { tab: 'intro' } }">{{ $t('common.home') }}</NuxtLink>
      <NuxtLink :to="{ query: { tab: 'companies' } }">{{ $t('default_00114') }}</NuxtLink>
      <NuxtLink :to="{ query: { tab: 'jobs' } }">{{ $t('default_00246') }}</NuxtLink>
      <a href="javascript:;" @click.prevent="goReserve">{{ $t('wap_01344') }}</a>
    </p>
    <div v-if="tab === 'companies'">
      <p v-if="!(companies?.list || []).length" class="muted">{{ $t('wap_00590') }}</p>
      <CompanyCard v-for="c in companies?.list || []" :key="c.uid" :company="c" />
    </div>
    <div v-else-if="tab === 'jobs'">
      <p v-if="!(jobs || []).length" class="muted">{{ $t('default_00033') }}</p>
      <JobCard v-for="j in jobs || []" :key="j.id" :job="j" />
    </div>
    <div v-else-if="tab === 'reserve'">
      <p v-if="comStatus?.state === 'applied'" class="muted">{{ $t('common.success') }} · {{ comStatus.status }}</p>
      <p v-else-if="comStatus?.state === 'no_jobs'" class="muted">{{ $t('default_00033') }}</p>
      <form v-else class="form" @submit.prevent="submitReserve">
        <select v-model.number="reserveForm.bid" required>
          <option :value="0">{{ $t('wap_01344') }}</option>
          <option v-for="s in spaces || []" :key="s.id" :value="s.id" :disabled="s.taken">
            {{ s.name }} · {{ s.price }}{{ $t('common_02056') }}{{ s.taken ? ` · ${$t('wap_01347')}` : '' }}
          </option>
        </select>
        <label v-for="j in comStatus?.jobs || []" :key="j.id">
          <input v-model="reserveForm.job_ids" type="checkbox" :value="j.id" />
          {{ j.name }}
        </label>
        <input v-model="reserveForm.name" :placeholder="$t('wap_01431')" required />
        <input v-model="reserveForm.moblie" :placeholder="$t('wap_01619')" required />
        <button type="submit">{{ $t('wap_01344') }}</button>
      </form>
      <p v-if="reserveMsg">{{ reserveMsg }}</p>
    </div>
    <template v-else>
      <div v-if="data?.body" v-html="data.body" />
      <p v-else-if="!data?.title" class="muted">{{ $t('wap_00603') }}</p>
    </template>
  </article>
</template>
