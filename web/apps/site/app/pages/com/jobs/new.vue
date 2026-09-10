<script setup lang="ts">
import { catTree, type CatNode } from '~/utils/site'
import type { DictItem } from '~/utils/query'

const api = useApi()
const { t, locale } = useI18n()
const { settings } = useSiteChrome()
const editId = computed(() => Number(useRoute().query.id || 0))
const showNegotiable = computed(() => String(settings.value.com_job_myswitch || '') === '1')
const nameLocked = computed(() => String(settings.value.joblock || '') === '1' && !!editId.value)
const form = reactive({
  name: '',
  job1: 0,
  job1_son: 0,
  job_post: 0,
  provinceid: 0,
  cityid: 0,
  three_cityid: 0,
  salary: 0,
  minsalary: 0,
  maxsalary: 0,
  salary_type: 0,
  type: 57,
  number: 1,
  exp: 0,
  edu: 0,
  content: '',
  wel: '',
  sdate: 0,
  edate: 0,
  hy: 0,
  report: 0,
  age: 0,
  sex: 0,
  marriage: 0,
  lang: '',
  is_graduate: 0,
  zp_minage: 0,
  zp_maxage: 0,
  link_id: 0,
  is_link: 1,
  is_message: 1,
  is_email: 1,
  exp_req: '',
  edu_req: '',
  sex_req: 0,
  minage_req: 0,
  maxage_req: 0,
  is_tblink: 0,
})
const sdateN = ref('')
const welIds = ref<number[]>([])
const langIds = ref<number[]>([])
const msg = ref('')
const { data: cats } = await useAsyncData(
  () => `job-cats-${locale.value}`,
  () => api.get<CatNode[]>('/v1/wap/categories', { kind: 'job' }).catch(() => [] as CatNode[]),
)
const jobRoots = computed(() => catTree(cats.value || [], 80))
const jobLevel2 = computed(() => jobRoots.value.find((c) => c.id === form.job1)?.children || [])
const jobLevel3 = computed(() => jobLevel2.value.find((c) => c.id === form.job1_son)?.children || [])
const { data: edus } = await useAsyncData(
  () => `dict-edu-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/educations').catch(() => [] as DictItem[]),
)
const { data: exps } = await useAsyncData(
  () => `dict-exp-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/experiences').catch(() => [] as DictItem[]),
)
const { data: welfares } = await useAsyncData(
  () => `dict-welfare-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/welfares').catch(() => [] as DictItem[]),
)
const { data: jobTypes } = await useAsyncData(
  () => `dict-job-type-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/job-types').catch(() => [] as DictItem[]),
)
const { data: industries } = await useAsyncData(
  () => `dict-hy-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/industries').catch(() => [] as DictItem[]),
)
const { data: reports } = await useAsyncData(
  () => `dict-report-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/reports').catch(() => [] as DictItem[]),
)
const { data: marriages } = await useAsyncData(
  () => `dict-marriage-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/marriages').catch(() => [] as DictItem[]),
)
const { data: langs } = await useAsyncData(
  () => `dict-lang-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/langs').catch(() => [] as DictItem[]),
)
type AddrRow = { id: number; link_man: string; link_moblie: string; link_address?: string | null }
const { data: addrs } = await useAsyncData('job-publish-addrs', () =>
  api
    .post<{ list: AddrRow[] }>('/v1/mcenter/company-addresses', { page: 1, page_size: 50 })
    .catch(() => ({ list: [] as AddrRow[] })),
)
const addrList = computed(() => addrs.value?.list || [])
watch(
  () => form.job1,
  (n, o) => {
    if (o && n !== o) {
      form.job1_son = 0
      form.job_post = 0
    }
  },
)
watch(
  () => form.job1_son,
  (n, o) => {
    if (o && n !== o) form.job_post = 0
  },
)
if (editId.value) {
  const row = await api.post<Record<string, unknown>>('/v1/mcenter/jobs/detail', { id: editId.value }).catch(() => null)
  if (row) {
    form.name = String(row.name || '')
    form.job1 = Number(row.job1 || 0)
    form.job1_son = Number(row.job1_son || 0)
    form.job_post = Number(row.job_post || 0)
    form.provinceid = Number(row.provinceid || 0)
    form.cityid = Number(row.cityid || 0)
    form.three_cityid = Number(row.three_cityid || 0)
    form.minsalary = Number(row.minsalary || 0)
    form.maxsalary = Number(row.maxsalary || 0)
    form.type = Number(row.type || row.job_type || 57)
    form.number = Number(row.number || 1)
    form.exp = Number(row.exp || 0)
    form.edu = Number(row.edu || 0)
    form.content = String(row.description || row.content || '')
    form.wel = String(row.welfare || row.wel || '')
    form.sdate = Number(row.sdate || 0)
    form.hy = Number(row.hy || 0)
    form.report = Number(row.report || 0)
    form.age = Number(row.age || 0)
    form.sex = Number(row.sex || 0)
    form.marriage = Number(row.marriage || 0)
    form.lang = String(row.lang || '')
    form.is_graduate = Number(row.is_graduate || 0) ? 1 : 0
    form.zp_minage = Number(row.zp_minage || 0)
    form.zp_maxage = Number(row.zp_maxage || 0)
    form.link_id = Number(row.link_id || 0)
    form.is_message = Number(row.is_message || 1) === 2 ? 2 : 1
    form.is_email = Number(row.is_email || 1) === 3 ? 3 : 1
    form.exp_req = String(row.exp_req || '')
    form.edu_req = String(row.edu_req || '')
    form.sex_req = Number(row.sex_req || 0)
    form.minage_req = Number(row.minage_req || 0)
    form.maxage_req = Number(row.maxage_req || 0)
    form.is_link = Number(row.is_link || 1) === 3 ? 3 : 1
    form.salary_type = form.minsalary === 0 && form.maxsalary === 0 ? 1 : 0
    if (form.sdate > 0) {
      const d = new Date(form.sdate * 1000)
      sdateN.value = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`
    }
  }
}
watch(welfares, (list) => {
  if (!form.wel || !list?.length) return
  const names = new Set(form.wel.split(',').map((s) => s.trim()).filter(Boolean))
  welIds.value = list.filter((w) => names.has(w.name)).map((w) => w.id)
}, { immediate: true })
watch(langs, (list) => {
  if (!form.lang || !list?.length) return
  const ids = new Set(form.lang.split(',').map((s) => Number(s.trim())).filter((n) => n > 0))
  langIds.value = list.filter((w) => ids.has(w.id)).map((w) => w.id)
}, { immediate: true })
async function submit() {
  msg.value = ''
  try {
    const wel = (welfares.value || [])
      .filter((w) => welIds.value.includes(w.id))
      .map((w) => w.name)
      .join(',')
    const lang = langIds.value.filter((id) => id > 0).join(',')
    if (sdateN.value) {
      form.sdate = Math.floor(new Date(`${sdateN.value}T00:00:00`).getTime() / 1000)
    }
    const body = { ...form, wel, lang, is_graduate: form.is_graduate ? 1 : 0 }
    if (editId.value) {
      await api.post('/v1/mcenter/jobs/update', { id: editId.value, ...body })
    } else {
      await api.post('/v1/mcenter/jobs', body)
    }
    msg.value = t('common.success')
    await navigateTo('/com/jobs')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('wap_00322') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_00322') }}</h1>
    <form class="form" @submit.prevent="submit">
      <input v-model="form.name" :placeholder="$t('wap_com_00288')" required :disabled="nameLocked" />
      <select v-model.number="form.job1" required>
        <option :value="0">{{ $t('common.job') }}</option>
        <option v-for="c in jobRoots" :key="c.id" :value="c.id">{{ c.name }}</option>
      </select>
      <select v-if="jobLevel2.length" v-model.number="form.job1_son">
        <option :value="0">{{ $t('common.all') }}</option>
        <option v-for="c in jobLevel2" :key="c.id" :value="c.id">{{ c.name }}</option>
      </select>
      <select v-if="jobLevel3.length" v-model.number="form.job_post">
        <option :value="0">{{ $t('common.all') }}</option>
        <option v-for="c in jobLevel3" :key="c.id" :value="c.id">{{ c.name }}</option>
      </select>
      <LocationFields
        v-model:province-id="form.provinceid"
        v-model:city-id="form.cityid"
        v-model:district-id="form.three_cityid"
      />
      <label v-if="showNegotiable">
        <input v-model="form.salary_type" type="checkbox" :true-value="1" :false-value="0" />
        {{ $t('wap_com_00291') }}
      </label>
      <template v-if="form.salary_type !== 1">
        <input v-model.number="form.minsalary" type="number" :placeholder="$t('ui.min_salary')" />
        <input v-model.number="form.maxsalary" type="number" :placeholder="$t('ui.max_salary')" />
      </template>
      <select v-model.number="form.type">
        <option v-for="tp in jobTypes || []" :key="tp.id" :value="tp.id">{{ tp.name }}</option>
      </select>
      <input v-model.number="form.number" type="number" min="1" :placeholder="$t('ui.headcount')" />
      <select v-model.number="form.exp">
        <option :value="0">{{ $t('common.not_limited') }}</option>
        <option v-for="x in exps || []" :key="x.id" :value="x.id">{{ x.name }}</option>
      </select>
      <select v-model.number="form.edu">
        <option :value="0">{{ $t('common.not_limited') }}</option>
        <option v-for="e in edus || []" :key="e.id" :value="e.id">{{ e.name }}</option>
      </select>
      <select v-model.number="form.hy">
        <option :value="0">{{ $t('wap_user_00100') }}</option>
        <option v-for="h in industries || []" :key="h.id" :value="h.id">{{ h.name }}</option>
      </select>
      <select v-model.number="form.report">
        <option :value="0">{{ $t('wap_com_00279') }}</option>
        <option v-for="r in reports || []" :key="r.id" :value="r.id">{{ r.name }}</option>
      </select>
      <select v-model.number="form.sex">
        <option :value="0">{{ $t('wap_com_00303') }}</option>
        <option :value="1">{{ $t('common_02092') }}</option>
        <option :value="2">{{ $t('common_02069') }}</option>
      </select>
      <select v-model.number="form.marriage">
        <option :value="0">{{ $t('default_00241') }}</option>
        <option v-for="m in marriages || []" :key="m.id" :value="m.id">{{ m.name }}</option>
      </select>
      <input v-model.number="form.zp_minage" type="number" min="0" max="99" :placeholder="$t('wap_com_00285')" />
      <input v-model.number="form.zp_maxage" type="number" min="0" max="99" :placeholder="$t('wap_com_00308')" />
      <label>
        <input v-model="form.is_graduate" type="checkbox" :true-value="1" :false-value="0" />
        {{ $t('member_com_00241') }}
      </label>
      <div v-if="(langs || []).length">
        <p class="muted">{{ $t('wap_com_00292') }}</p>
        <label v-for="lg in langs || []" :key="lg.id">
          <input v-model="langIds" type="checkbox" :value="lg.id" /> {{ lg.name }}
        </label>
      </div>
      <input v-model="sdateN" type="date" />
      <div>
        <label v-for="w in welfares || []" :key="w.id">
          <input v-model="welIds" type="checkbox" :value="w.id" /> {{ w.name }}
        </label>
      </div>
      <textarea v-model="form.content" :placeholder="$t('ui.job_desc')" rows="8" />
      <p class="muted">{{ $t('member_com_00242') }}</p>
      <input v-model="form.exp_req" :placeholder="$t('wap_com_00305')" />
      <input v-model="form.edu_req" :placeholder="$t('wap_com_00301')" />
      <select v-model.number="form.sex_req">
        <option :value="0">{{ $t('common_01936') }}</option>
        <option :value="1">{{ $t('common_02092') }}</option>
        <option :value="2">{{ $t('common_02069') }}</option>
      </select>
      <input v-model.number="form.minage_req" type="number" min="0" max="99" :placeholder="$t('wap_com_00285')" />
      <input v-model.number="form.maxage_req" type="number" min="0" max="99" :placeholder="$t('wap_com_00308')" />
      <p class="muted">{{ $t('member_user_00198') }}</p>
      <label>
        <input v-model.number="form.link_id" type="radio" :value="0" />
        {{ $t('member_com_00528') }}
      </label>
      <label v-for="a in addrList" :key="a.id">
        <input v-model.number="form.link_id" type="radio" :value="a.id" />
        {{ a.link_man }} {{ a.link_moblie }} {{ a.link_address || '' }}
      </label>
      <p>
        <NuxtLink to="/com/addresses">{{ $t('wap_com_00304') }}</NuxtLink>
      </p>
      <label>
        <input v-model="form.is_link" type="checkbox" :true-value="3" :false-value="1" />
        {{ $t('wap_com_00276') }}
      </label>
      <label>
        <input v-model="form.is_tblink" type="checkbox" :true-value="1" :false-value="0" />
        {{ $t('wap_00892') }}
      </label>
      <label>
        <input v-model="form.is_message" type="checkbox" :true-value="1" :false-value="2" />
        {{ $t('wap_00893') }} · {{ $t('wap_com_00261') }}
      </label>
      <label>
        <input v-model="form.is_email" type="checkbox" :true-value="1" :false-value="3" />
        {{ $t('wap_com_00293') }} · {{ $t('wap_com_00262') }}
      </label>
      <button type="submit">{{ $t('ui.submit_audit') }}</button>
      <p v-if="msg">{{ msg }}</p>
    </form>
  </section>
</template>
