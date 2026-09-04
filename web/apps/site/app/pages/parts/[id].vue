<script setup lang="ts">
const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { me } = useSiteChrome()
const { data, error } = await useAsyncData(`part-${id}`, () => api.get('/v1/wap/parts/detail', { id }))
const row = computed(() => (data.value || {}) as Record<string, unknown>)
const msg = ref('')
const acting = ref(false)

async function requireSeeker(): Promise<boolean> {
  if (!me.value) {
    await navigateTo({ path: '/login', query: { next: `/parts/${id}` } })
    return false
  }
  if (me.value.usertype !== 1) {
    msg.value = t('wap_01396')
    return false
  }
  return true
}

async function apply() {
  msg.value = ''
  if (!(await requireSeeker())) return
  acting.value = true
  try {
    await api.post('/v1/wap/parts/apply', { id })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  } finally {
    acting.value = false
  }
}

async function collect() {
  msg.value = ''
  if (!(await requireSeeker())) return
  acting.value = true
  try {
    await api.post('/v1/wap/parts/collect', { id, com_id: Number(row.value.uid || 0) || undefined })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  } finally {
    acting.value = false
  }
}

const salaryLine = computed(() => {
  const n = Number(row.value.salary || 0)
  const unit = String(row.value.salary_type_n || '')
  const cycle = String(row.value.billing_cycle_n || '')
  if (!n && !unit) return ''
  return [n || '', unit, cycle].filter(Boolean).join(' ')
})
const cityLine = computed(() =>
  [row.value.province_name, row.value.city_name, row.value.three_city_name].map((v) => String(v || '')).filter(Boolean).join('-'),
)
const tel = computed(() => String(row.value.linktel || row.value.linktel_n || ''))
useSeoMeta({ title: () => String(row.value.name || t('wap_user_00220')) })
useHead({ link: [{ rel: 'canonical', href: `/parts/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ row.name || $t('member_com_00477') }}</h1>
    <p v-if="error" class="muted">{{ $t('ui.load_failed') }}</p>
    <template v-else>
      <p v-if="row.com_name || cityLine" class="muted">{{ row.com_name }} {{ cityLine }}</p>
      <ul class="stack">
        <li v-if="row.part_type_n">{{ $t('member_com_00313') }}：{{ row.part_type_n }}</li>
        <li v-if="salaryLine">{{ salaryLine }}</li>
        <li v-if="row.number">{{ $t('wap_com_00333') }}：{{ row.number }}{{ $t('common_02051') }}</li>
        <li v-if="row.sex_n">{{ $t('wap_com_00332') }}：{{ row.sex_n }}</li>
        <li v-if="row.worktime">{{ $t('wap_00456') }}：{{ row.worktime }}</li>
        <li v-if="row.address">{{ $t('wap_user_00243') }}：{{ row.address }}</li>
      </ul>
      <div v-if="row.content" v-html="String(row.content)" />
      <p v-else-if="!row.name" class="muted">{{ $t('member_com_00477') }}</p>
      <h2>{{ $t('wap_00462') }}</h2>
      <p v-if="row.linkman">{{ $t('wap_01431') }}：{{ row.linkman }}</p>
      <p v-if="Number(row.link_tip) > 0" class="muted">{{ $t('wap_01395') }}</p>
      <p v-else-if="tel">{{ $t('wap_user_00265') }}：{{ tel }}</p>
      <p>
        <button type="button" :disabled="acting" @click="apply">{{ $t('wap_com_00235') }}</button>
        <button type="button" :disabled="acting" @click="collect">{{ $t('wap_00379') }}</button>
      </p>
      <p v-if="msg">{{ msg }}</p>
    </template>
  </article>
</template>
