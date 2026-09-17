<script setup lang="ts">
import { seoJoin } from '~/utils/seo'
import { ensureLogin, ensurePublicFound, type CompanyLike, type JobLike } from '~/utils/site'

type SpecialCompany = CompanyLike & { com_name?: string }

const id = Number(useRoute().params.id)
const route = useRoute()
const { t } = useI18n()
const api = useApi()
const { me } = useSiteChrome()
const hy = computed(() => Number(route.query.hy || 0) || 0)
const { data, error } = await useAsyncData(`special-${id}`, () => api.get('/v1/wap/specials/detail', { id }))
const isGl = computed(() => {
  const tpl = String(data.value?.tpl || '').toLowerCase()
  return ['gl', 'famous', 'senior'].some((k) => tpl.includes(k))
})
const { data: companies } = await useAsyncData(
  () => `special-com-${id}-${hy.value}`,
  () =>
    api
      .get<{ list?: SpecialCompany[] }>('/v1/wap/specials/companies', {
        id,
        hy: hy.value || undefined,
        page: 1,
        page_size: 20,
      })
      .catch(() => ({ list: [] })),
)
const { data: hotcom } = await useAsyncData(`special-hot-${id}`, () =>
  isGl.value
    ? api.get<{ list?: SpecialCompany[] }>('/v1/wap/specials/companies', { id, page: 1, page_size: 12 }).catch(() => ({ list: [] }))
    : Promise.resolve({ list: [] as SpecialCompany[] }),
)
const { data: industries } = await useAsyncData(`special-hy-${id}`, () =>
  isGl.value
    ? api.get<Array<{ hy: number; hy_n: string }>>('/v1/wap/specials/industries', { id }).catch(() => [])
    : Promise.resolve([] as Array<{ hy: number; hy_n: string }>),
)
const { data: jobs } = await useAsyncData(`special-job-${id}`, () =>
  api.get<{ list?: JobLike[] }>('/v1/wap/specials/jobs', { id, page: 1, page_size: 20 }).catch(() => ({ list: [] })),
)
const applyMsg = ref('')
async function apply() {
  applyMsg.value = ''
  if (!(await ensureLogin(me.value, route.fullPath))) return
  try {
    await api.post('/v1/wap/specials/apply', { id })
    applyMsg.value = t('common.success')
  } catch (e: unknown) {
    applyMsg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
ensurePublicFound(Boolean(data.value?.title || data.value?.id), error.value)
useSeoMeta({
  title: () => String(data.value?.title || t('ui.specials')),
  description: () => seoJoin([data.value?.intro, data.value?.body, data.value?.title]),
})
useHead({ link: [{ rel: 'canonical', href: `/specials/${id}` }] })
</script>

<template>
  <article class="site-pc">
    <h1>{{ data?.title || $t('common_02409') }}</h1>
    <p v-if="data?.intro" class="muted">{{ data.intro }}</p>
    <div v-if="data?.body" v-html="data.body" />
    <p v-else-if="!data?.title" class="muted">{{ $t('common_02409') }}</p>
    <p v-if="Number(data?.com_bm) === 1">
      <button type="button" @click="apply">{{ $t('common.submit') }}</button>
    </p>
    <p v-if="applyMsg">{{ applyMsg }}</p>
    <template v-if="isGl">
      <h2>{{ $t('common.hot') }}</h2>
      <p v-if="!(hotcom?.list || []).length" class="muted">{{ $t('wap_00590') }}</p>
      <CompanyCard v-for="c in hotcom?.list || []" :key="'hot-' + c.uid" :company="{ ...c, name: c.com_name || c.name }" />
      <p>
        <NuxtLink :to="{ query: { hy: undefined } }">{{ $t('common.all') }}</NuxtLink>
        <NuxtLink v-for="row in industries || []" :key="row.hy" :to="{ query: { hy: row.hy } }">{{ row.hy_n }}</NuxtLink>
      </p>
    </template>
    <h2>{{ $t('default_00114') }}</h2>
    <p v-if="!(companies?.list || []).length" class="muted">{{ $t('wap_00590') }}</p>
    <CompanyCard v-for="c in companies?.list || []" :key="c.uid" :company="{ ...c, name: c.com_name || c.name }" />
    <h2>{{ $t('default_00246') }}</h2>
    <p v-if="!(jobs?.list || []).length" class="muted">{{ $t('default_00033') }}</p>
    <JobCard v-for="j in jobs?.list || []" :key="j.id" :job="j" />
  </article>
  <div class="site-h5">
    <div class="special_banner">
      <h1>{{ data?.title || $t('common_02409') }}</h1>
      <p v-if="data?.intro" class="muted">{{ data.intro }}</p>
    </div>
    <div class="special_rules_box">
      <div v-if="data?.body" v-html="data.body" />
      <p v-if="Number(data?.com_bm) === 1">
        <button type="button" class="s_bth" @click="apply">{{ $t('common.submit') }}</button>
      </p>
      <p v-if="applyMsg">{{ applyMsg }}</p>
    </div>
    <div class="zt_qy_box">
      <template v-if="isGl">
        <h2>{{ $t('common.hot') }}</h2>
        <p v-if="!(hotcom?.list || []).length" class="muted">{{ $t('wap_00590') }}</p>
        <CompanyCard v-for="c in hotcom?.list || []" :key="'h5hot-' + c.uid" :company="{ ...c, name: c.com_name || c.name }" />
        <p>
          <NuxtLink :to="{ query: { hy: undefined } }">{{ $t('common.all') }}</NuxtLink>
          <NuxtLink v-for="row in industries || []" :key="'h5hy-' + row.hy" :to="{ query: { hy: row.hy } }">{{ row.hy_n }}</NuxtLink>
        </p>
      </template>
      <h2>{{ $t('default_00114') }}</h2>
      <p v-if="!(companies?.list || []).length" class="muted">{{ $t('wap_00590') }}</p>
      <CompanyCard v-for="c in companies?.list || []" :key="'h5-' + c.uid" :company="{ ...c, name: c.com_name || c.name }" />
      <h2>{{ $t('default_00246') }}</h2>
      <p v-if="!(jobs?.list || []).length" class="muted">{{ $t('default_00033') }}</p>
      <JobCard v-for="j in jobs?.list || []" :key="'h5j-' + j.id" :job="j" />
    </div>
  </div>
</template>
