<script setup lang="ts">
import { catTree } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { settings } = useSiteChrome()
const editId = computed(() => Number(useRoute().query.id || 0))
const showNegotiable = computed(() => String(settings.value.com_job_myswitch || '') === '1')
const nameLocked = computed(() => String(settings.value.joblock || '') === '1' && !!editId.value)
const hideApplyReq = computed(() => String(settings.value.sqjob_req || '') === '2')
const showSex = computed(() => String(settings.value.com_job_sexswitch || '') === '1')
const form = reactive({
  name: '',
  job1: 0,
  job1_son: 0,
  job_post: 0,
  provinceid: 0,
  cityid: 0,
  three_cityid: 0,
  x: '',
  y: '',
  salary: 0,
  minsalary: 0,
  maxsalary: 0,
  salary_type: 0,
  type: 0,
  number: 0,
  zp_num: 0,
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
  link_id: -1,
  is_link: 1,
  custom_link_man: '',
  custom_link_moblie: '',
  custom_link_phone: '',
  custom_link_email: '',
  custom_link_address: '',
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
const gateMsg = ref('')
type PublishGap = { key: string; href: string }
type PublishCheck = {
  addjobnum: number
  job_num: number
  gaps?: PublishGap[]
}
const { data: chk } = await useAsyncData('job-publish-check', () =>
  api.post<PublishCheck>('/v1/mcenter/jobs/check', {}).catch(() => null),
)
if (chk.value && !editId.value) {
  if (Number(chk.value.addjobnum) === 0) {
    await navigateTo('/com/member-right')
  } else {
    const hard = (chk.value.gaps || []).filter((g) => g.key !== 'member_com_00695')
    if (hard[0]) {
      await navigateTo(hard[0].href)
    }
  }
}
const gz = (chk.value?.gaps || []).find((g) => g.key === 'member_com_00695')
if (gz) gateMsg.value = t(gz.key)
const { data: cats } = await useJobCats()
const jobRoots = computed(() => catTree(cats.value || [], 80))
const jobLevel2 = computed(() => jobRoots.value.find((c) => c.id === form.job1)?.children || [])
const jobLevel3 = computed(() => jobLevel2.value.find((c) => c.id === form.job1_son)?.children || [])
const { data: dicts } = await usePublicDicts()
const edus = computed(() => dicts.value?.educations ?? [])
const exps = computed(() => dicts.value?.experiences ?? [])
const welfares = computed(() => dicts.value?.welfares ?? [])
const jobTypes = computed(() => dicts.value?.job_types ?? [])
const industries = computed(() => dicts.value?.industries ?? [])
const reports = computed(() => dicts.value?.reports ?? [])
const marriages = computed(() => dicts.value?.marriages ?? [])
const langs = computed(() => dicts.value?.langs ?? [])
type AddrRow = {
  id: number
  link_man: string
  link_moblie: string
  link_address?: string | null
  provinceid?: number
  cityid?: number
  three_cityid?: number
  x?: string | null
  y?: string | null
}
type CompanyRow = {
  name?: string
  linkman?: string
  linkphone?: string
  provinceid?: number
  cityid?: number
  three_cityid?: number
  x?: string
  y?: string
}
const { data: addrs } = await useAsyncData('job-publish-addrs', () =>
  api
    .post<{ list: AddrRow[] }>('/v1/mcenter/company-addresses', { page: 1, page_size: 50 })
    .catch(() => ({ list: [] as AddrRow[] })),
)
const { data: company } = await useAsyncData('job-publish-company', () =>
  api.post<CompanyRow>('/v1/mcenter/company/list', {}).catch(() => null),
)
const addrList = computed(() => addrs.value?.list || [])
watch(jobTypes, (list) => {
  if (list?.length && !form.type) form.type = list[0].id
}, { immediate: true })
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
    form.x = String(row.x || '')
    form.y = String(row.y || '')
    form.minsalary = Number(row.minsalary || 0)
    form.maxsalary = Number(row.maxsalary || 0)
    form.type = Number(row.type || row.job_type || 0)
    form.zp_num = Number(row.zp_num || row.number || 0)
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
    const lid = Number(row.link_id || 0)
    const isLink = Number(row.is_link || 1)
    form.link_id = isLink === 2 && lid <= 0 ? 0 : lid > 0 ? lid : -1
    form.custom_link_man = String(row.custom_link_man || '')
    form.custom_link_moblie = String(row.custom_link_moblie || '')
    form.custom_link_phone = String(row.custom_link_phone || '')
    form.custom_link_email = String(row.custom_link_email || '')
    form.custom_link_address = String(row.custom_link_address || row.address || '')
    form.is_message = Number(row.is_message || 1) === 2 ? 2 : 1
    form.is_email = Number(row.is_email || 1) === 3 ? 3 : 1
    form.exp_req = String(row.exp_req || '')
    form.edu_req = String(row.edu_req || '')
    form.sex_req = Number(row.sex_req || 0)
    form.minage_req = Number(row.minage_req || 0)
    form.maxage_req = Number(row.maxage_req || 0)
    form.is_link = isLink === 3 ? 3 : isLink === 2 ? 2 : 1
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

function applyLinkGeo() {
  if (form.link_id === 0) return
  if (form.link_id > 0) {
    const a = addrList.value.find((row) => row.id === form.link_id)
    if (!a) return
    form.provinceid = Number(a.provinceid || 0)
    form.cityid = Number(a.cityid || 0)
    form.three_cityid = Number(a.three_cityid || 0)
    form.x = String(a.x || '')
    form.y = String(a.y || '')
    return
  }
  form.provinceid = Number(company.value?.provinceid || 0)
  form.cityid = Number(company.value?.cityid || 0)
  form.three_cityid = Number(company.value?.three_cityid || 0)
  form.x = String(company.value?.x || '')
  form.y = String(company.value?.y || '')
}
watch(() => form.link_id, applyLinkGeo)
watch([addrList, company], applyLinkGeo, { immediate: true })

function clientCheck(): string {
  if (form.name.trim().length < 2) return t('member_com_00585')
  if (jobLevel3.value.length && !form.job_post) return t('member_com_00586')
  if (!jobLevel3.value.length && jobLevel2.value.length && !form.job1_son) return t('member_com_00586')
  if (!form.job1) return t('member_com_00586')
  if (form.salary_type === 1) {
    if (!showNegotiable.value) return t('member_com_00238')
  } else {
    if (!form.minsalary) return t('member_com_00238')
    if (form.maxsalary) {
      if (form.maxsalary < form.minsalary) return t('wap_com_00264')
      if (form.maxsalary === form.minsalary) return t('wap_com_00255')
    }
  }
  if (!form.zp_num) return t('wap_00888')
  if (form.zp_minage && form.zp_minage < 16) return t('wap_com_00257')
  if (form.zp_maxage && (form.zp_maxage < 16 || form.zp_maxage > 99)) return t('wap_com_00269')
  if (form.minage_req && form.minage_req < 16) return t('wap_com_00257')
  if (form.maxage_req && (form.maxage_req < 16 || form.maxage_req > 99)) return t('wap_com_00269')
  if (!form.content.replace(/<[^>]+>/g, '').trim()) return t('member_com_00587')
  if (form.link_id === null || Number.isNaN(Number(form.link_id))) return t('member_com_00588')
  if (form.link_id === 0) {
    if (!form.custom_link_man.trim()) return t('member_com_00588')
    if (!form.custom_link_moblie.trim() && !form.custom_link_phone.trim()) return t('common_00670')
  }
  return ''
}

function afterSaveW(state: number, status: number) {
  if (state === 0) return 0
  if (status === 1) return 4
  return 1
}

async function submit() {
  msg.value = ''
  const blocked = clientCheck()
  if (blocked) {
    msg.value = blocked
    return
  }
  try {
    applyLinkGeo()
    const wel = (welfares.value || [])
      .filter((w) => welIds.value.includes(w.id))
      .map((w) => w.name)
      .join(',')
    const lang = langIds.value.filter((id) => id > 0).join(',')
    if (sdateN.value) {
      form.sdate = Math.floor(new Date(`${sdateN.value}T00:00:00`).getTime() / 1000)
    }
    const jobclassid = form.job_post || form.job1_son || form.job1
    const isLink = form.is_link === 3 ? 3 : form.link_id === -1 ? 1 : 2
    const body = {
      ...form,
      wel,
      lang,
      jobclassid,
      zp_num: form.zp_num,
      number: form.zp_num,
      is_graduate: form.is_graduate ? 1 : 0,
      is_link: isLink,
    }
    const r = editId.value
      ? await api.post<{ id: number; state: number; status: number }>('/v1/mcenter/jobs/update', { id: editId.value, ...body })
      : await api.post<{ id: number; state: number; status: number }>('/v1/mcenter/jobs', body)
    const w = afterSaveW(Number(r.state), Number(r.status))
    if (Number(r.state) === 0) {
      msg.value = t('common_01369')
    } else {
      msg.value = t('admin_tool_00502')
    }
    await navigateTo(`/com/jobs?w=${w}`)
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('wap_00322') })
</script>

<template>
  <MemberPanel :title="$t('wap_00322')">
    <p v-if="gateMsg" class="muted">{{ gateMsg }}</p>
    <form class="com_release_box site-pc" @submit.prevent="submit">
      <ul>
      <MemberReleaseRow :label="$t('wap_com_00288')" required><input v-model="form.name" required :disabled="nameLocked" class="com_release_textnew_text" /></MemberReleaseRow>
      <MemberReleaseRow :label="$t('common.job')" required>
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
      </MemberReleaseRow>
      <MemberReleaseRow v-if="showNegotiable" :label="$t('wap_com_00291')">
        <label>
          <input v-model="form.salary_type" type="checkbox" :true-value="1" :false-value="0" />
          {{ $t('wap_com_00291') }}
        </label>
      </MemberReleaseRow>
      <template v-if="form.salary_type !== 1">
        <MemberReleaseRow :label="$t('ui.min_salary')" required><input v-model.number="form.minsalary" type="number" min="1" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('ui.max_salary')"><input v-model.number="form.maxsalary" type="number" min="0" /></MemberReleaseRow>
      </template>
      <MemberReleaseRow :label="$t('ui.headcount')" required><input v-model.number="form.zp_num" type="number" min="1" /></MemberReleaseRow>
      <MemberReleaseRow :label="$t('wap_user_00240')">
        <select v-model.number="form.exp">
          <option :value="0">{{ $t('common.not_limited') }}</option>
          <option v-for="x in exps || []" :key="x.id" :value="x.id">{{ x.name }}</option>
        </select>
      </MemberReleaseRow>
      <MemberReleaseRow :label="$t('wap_com_00301')">
        <select v-model.number="form.edu">
          <option :value="0">{{ $t('common.not_limited') }}</option>
          <option v-for="e in edus || []" :key="e.id" :value="e.id">{{ e.name }}</option>
        </select>
      </MemberReleaseRow>
      <MemberReleaseRow :label="$t('wap_user_00100')">
        <select v-model.number="form.hy">
          <option :value="0">{{ $t('wap_user_00100') }}</option>
          <option v-for="h in industries || []" :key="h.id" :value="h.id">{{ h.name }}</option>
        </select>
      </MemberReleaseRow>
      <MemberReleaseRow :label="$t('wap_com_00279')">
        <select v-model.number="form.report">
          <option :value="0">{{ $t('wap_com_00279') }}</option>
          <option v-for="r in reports || []" :key="r.id" :value="r.id">{{ r.name }}</option>
        </select>
      </MemberReleaseRow>
      <MemberReleaseRow v-if="showSex" :label="$t('wap_com_00303')">
        <select v-model.number="form.sex">
          <option :value="0">{{ $t('wap_com_00303') }}</option>
          <option :value="1">{{ $t('common_02092') }}</option>
          <option :value="2">{{ $t('common_02069') }}</option>
        </select>
      </MemberReleaseRow>
      <MemberReleaseRow :label="$t('default_00241')">
        <select v-model.number="form.marriage">
          <option :value="0">{{ $t('default_00241') }}</option>
          <option v-for="m in marriages || []" :key="m.id" :value="m.id">{{ m.name }}</option>
        </select>
      </MemberReleaseRow>
      <MemberReleaseRow :label="$t('wap_com_00285')"><input v-model.number="form.zp_minage" type="number" min="0" max="99" /></MemberReleaseRow>
      <MemberReleaseRow :label="$t('wap_com_00308')"><input v-model.number="form.zp_maxage" type="number" min="0" max="99" /></MemberReleaseRow>
      <MemberReleaseRow :label="$t('member_com_00241')">
        <label>
          <input v-model="form.is_graduate" type="checkbox" :true-value="1" :false-value="0" />
          {{ $t('member_com_00241') }}
        </label>
      </MemberReleaseRow>
      <MemberReleaseRow v-if="(langs || []).length" :label="$t('wap_com_00292')">
        <label v-for="lg in langs || []" :key="lg.id">
          <input v-model="langIds" type="checkbox" :value="lg.id" /> {{ lg.name }}
        </label>
      </MemberReleaseRow>
      <MemberReleaseRow :label="$t('member_user_00106')"><input v-model="sdateN" type="date" /></MemberReleaseRow>
      <MemberReleaseRow :label="$t('common_02017')">
        <label v-for="w in welfares || []" :key="w.id">
          <input v-model="welIds" type="checkbox" :value="w.id" /> {{ w.name }}
        </label>
      </MemberReleaseRow>
      <MemberReleaseRow :label="$t('ui.job_desc')" area required><textarea v-model="form.content" rows="8" /></MemberReleaseRow>
      <p class="muted">{{ $t('member_com_00242') }}</p>
      <template v-if="!hideApplyReq">
        <MemberReleaseRow :label="$t('wap_com_00305')"><input v-model="form.exp_req" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00301')"><input v-model="form.edu_req" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00303')">
          <select v-model.number="form.sex_req">
            <option :value="0">{{ $t('common_01936') }}</option>
            <option :value="1">{{ $t('common_02092') }}</option>
            <option :value="2">{{ $t('common_02069') }}</option>
          </select>
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00285')"><input v-model.number="form.minage_req" type="number" min="0" max="99" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00308')"><input v-model.number="form.maxage_req" type="number" min="0" max="99" /></MemberReleaseRow>
        <p class="muted">{{ $t('member_user_00198') }}</p>
      </template>
      <MemberReleaseRow :label="$t('member_com_00528')" required>
        <select v-model.number="form.link_id">
          <option :value="-1">
            {{ company?.linkman || $t('member_com_00528') }}
            {{ company?.linkphone || '' }}
            （{{ $t('member_com_00528') }}）
          </option>
          <option :value="0">{{ $t('wap_01431') }}</option>
          <option v-for="a in addrList" :key="a.id" :value="a.id">
            {{ a.link_man }} {{ a.link_moblie }} {{ a.link_address || '' }}
          </option>
        </select>
      </MemberReleaseRow>
      <template v-if="form.link_id === 0">
        <MemberReleaseRow :label="$t('wap_01431')" required>
          <input v-model="form.custom_link_man" required class="com_release_textnew_text" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('common.phone')">
          <input v-model="form.custom_link_moblie" class="com_release_textnew_text" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00014')">
          <input v-model="form.custom_link_phone" class="com_release_textnew_text" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('member_user_00282')">
          <input v-model="form.custom_link_email" class="com_release_textnew_text" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_01362')">
          <input v-model="form.custom_link_address" class="com_release_textnew_text" />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_user_00243')">
          <MapPick v-model:x="form.x" v-model:y="form.y" />
        </MemberReleaseRow>
      </template>
      <p>
        <NuxtLink to="/com/addresses">{{ $t('wap_com_00304') }}</NuxtLink>
      </p>
      <MemberReleaseRow :label="$t('wap_com_00276')">
        <label>
          <input v-model="form.is_link" type="checkbox" :true-value="3" :false-value="1" />
          {{ $t('wap_com_00276') }}
        </label>
      </MemberReleaseRow>
      <MemberReleaseRow :label="$t('wap_00892')">
        <label>
          <input v-model="form.is_tblink" type="checkbox" :true-value="1" :false-value="0" />
          {{ $t('wap_00892') }}
        </label>
      </MemberReleaseRow>
      <MemberReleaseRow :label="$t('wap_00893')">
        <label>
          <input v-model="form.is_message" type="checkbox" :true-value="1" :false-value="2" />
          {{ $t('wap_00893') }} · {{ $t('wap_com_00261') }}
        </label>
      </MemberReleaseRow>
      <MemberReleaseRow :label="$t('wap_com_00293')">
        <label>
          <input v-model="form.is_email" type="checkbox" :true-value="1" :false-value="3" />
          {{ $t('wap_com_00293') }} · {{ $t('wap_com_00262') }}
        </label>
      </MemberReleaseRow>
      </ul>
      <button type="submit" class="btn_01">{{ $t('member_com_00248') }}</button>
      <p v-if="msg">{{ msg }}</p>
    </form>
    <div class="site-h5 issue_post_body">
      <form class="yun_createbox" @submit.prevent="submit">
        <MemberField wap :label="$t('wap_com_00288')">
          <input v-model="form.name" required :disabled="nameLocked" />
        </MemberField>
        <MemberField wap :label="$t('common.job')">
          <select v-model.number="form.job1" required>
            <option :value="0">{{ $t('common.job') }}</option>
            <option v-for="c in jobRoots" :key="'h5j1-' + c.id" :value="c.id">{{ c.name }}</option>
          </select>
          <select v-if="jobLevel2.length" v-model.number="form.job1_son">
            <option :value="0">{{ $t('common.all') }}</option>
            <option v-for="c in jobLevel2" :key="'h5j2-' + c.id" :value="c.id">{{ c.name }}</option>
          </select>
          <select v-if="jobLevel3.length" v-model.number="form.job_post">
            <option :value="0">{{ $t('common.all') }}</option>
            <option v-for="c in jobLevel3" :key="'h5j3-' + c.id" :value="c.id">{{ c.name }}</option>
          </select>
        </MemberField>
        <MemberField v-if="showNegotiable" wap :label="$t('wap_com_00291')">
          <label>
            <input v-model="form.salary_type" type="checkbox" :true-value="1" :false-value="0" />
            {{ $t('wap_com_00291') }}
          </label>
        </MemberField>
        <template v-if="form.salary_type !== 1">
          <MemberField wap :label="$t('ui.min_salary')"><input v-model.number="form.minsalary" type="number" min="1" /></MemberField>
          <MemberField wap :label="$t('ui.max_salary')"><input v-model.number="form.maxsalary" type="number" min="0" /></MemberField>
        </template>
        <MemberField wap :label="$t('ui.headcount')"><input v-model.number="form.zp_num" type="number" min="1" /></MemberField>
        <MemberField wap :label="$t('wap_user_00240')">
          <select v-model.number="form.exp">
            <option :value="0">{{ $t('common.not_limited') }}</option>
            <option v-for="x in exps || []" :key="'h5exp-' + x.id" :value="x.id">{{ x.name }}</option>
          </select>
        </MemberField>
        <MemberField wap :label="$t('wap_com_00301')">
          <select v-model.number="form.edu">
            <option :value="0">{{ $t('common.not_limited') }}</option>
            <option v-for="e in edus || []" :key="'h5edu-' + e.id" :value="e.id">{{ e.name }}</option>
          </select>
        </MemberField>
        <MemberField wap :label="$t('wap_user_00100')">
          <select v-model.number="form.hy">
            <option :value="0">{{ $t('wap_user_00100') }}</option>
            <option v-for="h in industries || []" :key="'h5hy-' + h.id" :value="h.id">{{ h.name }}</option>
          </select>
        </MemberField>
        <MemberField wap :label="$t('wap_com_00279')">
          <select v-model.number="form.report">
            <option :value="0">{{ $t('wap_com_00279') }}</option>
            <option v-for="r in reports || []" :key="'h5rp-' + r.id" :value="r.id">{{ r.name }}</option>
          </select>
        </MemberField>
        <MemberField v-if="showSex" wap :label="$t('wap_com_00303')">
          <select v-model.number="form.sex">
            <option :value="0">{{ $t('wap_com_00303') }}</option>
            <option :value="1">{{ $t('common_02092') }}</option>
            <option :value="2">{{ $t('common_02069') }}</option>
          </select>
        </MemberField>
        <MemberField wap :label="$t('default_00241')">
          <select v-model.number="form.marriage">
            <option :value="0">{{ $t('default_00241') }}</option>
            <option v-for="m in marriages || []" :key="'h5mg-' + m.id" :value="m.id">{{ m.name }}</option>
          </select>
        </MemberField>
        <MemberField wap :label="$t('wap_com_00285')"><input v-model.number="form.zp_minage" type="number" min="0" max="99" /></MemberField>
        <MemberField wap :label="$t('wap_com_00308')"><input v-model.number="form.zp_maxage" type="number" min="0" max="99" /></MemberField>
        <MemberField wap :label="$t('member_com_00241')">
          <label>
            <input v-model="form.is_graduate" type="checkbox" :true-value="1" :false-value="0" />
            {{ $t('member_com_00241') }}
          </label>
        </MemberField>
        <MemberField v-if="(langs || []).length" wap :label="$t('wap_com_00292')">
          <label v-for="lg in langs || []" :key="'h5lg-' + lg.id">
            <input v-model="langIds" type="checkbox" :value="lg.id" /> {{ lg.name }}
          </label>
        </MemberField>
        <MemberField wap :label="$t('member_user_00106')"><input v-model="sdateN" type="date" /></MemberField>
        <MemberField wap :label="$t('common_02017')">
          <label v-for="w in welfares || []" :key="'h5wel-' + w.id">
            <input v-model="welIds" type="checkbox" :value="w.id" /> {{ w.name }}
          </label>
        </MemberField>
        <MemberField wap area :label="$t('ui.job_desc')"><textarea v-model="form.content" rows="8" /></MemberField>
        <p class="muted">{{ $t('member_com_00242') }}</p>
        <template v-if="!hideApplyReq">
          <MemberField wap :label="$t('wap_com_00305')"><input v-model="form.exp_req" /></MemberField>
          <MemberField wap :label="$t('wap_com_00301')"><input v-model="form.edu_req" /></MemberField>
          <MemberField wap :label="$t('wap_com_00303')">
            <select v-model.number="form.sex_req">
              <option :value="0">{{ $t('common_01936') }}</option>
              <option :value="1">{{ $t('common_02092') }}</option>
              <option :value="2">{{ $t('common_02069') }}</option>
            </select>
          </MemberField>
          <MemberField wap :label="$t('wap_com_00285')"><input v-model.number="form.minage_req" type="number" min="0" max="99" /></MemberField>
          <MemberField wap :label="$t('wap_com_00308')"><input v-model.number="form.maxage_req" type="number" min="0" max="99" /></MemberField>
          <p class="muted">{{ $t('member_user_00198') }}</p>
        </template>
        <MemberField wap :label="$t('member_com_00528')">
          <label>
            <input v-model.number="form.link_id" type="radio" :value="-1" />
            {{ company?.linkman || $t('member_com_00528') }} {{ company?.linkphone || '' }}
          </label>
          <label v-for="a in addrList" :key="'h5addr-' + a.id">
            <input v-model.number="form.link_id" type="radio" :value="a.id" />
            {{ a.link_man }} {{ a.link_moblie }} {{ a.link_address || '' }}
          </label>
          <label>
            <input v-model.number="form.link_id" type="radio" :value="0" />
            {{ $t('wap_01431') }}
          </label>
        </MemberField>
        <template v-if="form.link_id === 0">
          <MemberField wap :label="$t('wap_01431')"><input v-model="form.custom_link_man" required /></MemberField>
          <MemberField wap :label="$t('common.phone')"><input v-model="form.custom_link_moblie" /></MemberField>
          <MemberField wap :label="$t('wap_com_00014')"><input v-model="form.custom_link_phone" /></MemberField>
          <MemberField wap :label="$t('member_user_00282')"><input v-model="form.custom_link_email" /></MemberField>
          <MemberField wap :label="$t('wap_01362')"><input v-model="form.custom_link_address" /></MemberField>
          <MemberField wap :label="$t('wap_user_00243')">
            <MapPick v-model:x="form.x" v-model:y="form.y" />
          </MemberField>
        </template>
        <p>
          <NuxtLink to="/com/addresses">{{ $t('wap_com_00304') }}</NuxtLink>
        </p>
        <MemberField wap :label="$t('wap_com_00276')">
          <label>
            <input v-model="form.is_link" type="checkbox" :true-value="3" :false-value="1" />
            {{ $t('wap_com_00276') }}
          </label>
        </MemberField>
        <MemberField wap :label="$t('wap_00892')">
          <label>
            <input v-model="form.is_tblink" type="checkbox" :true-value="1" :false-value="0" />
            {{ $t('wap_00892') }}
          </label>
        </MemberField>
        <MemberField wap :label="$t('wap_00893')">
          <label>
            <input v-model="form.is_message" type="checkbox" :true-value="1" :false-value="2" />
            {{ $t('wap_00893') }} · {{ $t('wap_com_00261') }}
          </label>
        </MemberField>
        <MemberField wap :label="$t('wap_com_00293')">
          <label>
            <input v-model="form.is_email" type="checkbox" :true-value="1" :false-value="3" />
            {{ $t('wap_com_00293') }} · {{ $t('wap_com_00262') }}
          </label>
        </MemberField>
        <button type="submit" class="issue_post_body_btn">{{ $t('member_com_00248') }}</button>
        <p v-if="msg">{{ msg }}</p>
      </form>
    </div>
  </MemberPanel>
</template>
