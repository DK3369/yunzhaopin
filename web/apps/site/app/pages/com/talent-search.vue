<script setup lang="ts">
import { isUnauthErr, mediaUrl } from '~/utils/site'

type ResumeRow = {
  uid: number
  eid?: number
  def_job?: number
  name?: string
  display_name?: string
  uname?: string
  photo?: string
  photo_n?: string
  sex_n?: string
  edu_n?: string
  education_n?: string
  exp_n?: string
  expect_salary_n?: string
  salary?: string
  lastupdate_n?: string
}

type HrRow = {
  key: string | number
  name: string
  time?: string
  to?: string
  photo?: string
  info?: string[]
  salary?: string
}

const api = useApi()
const { t } = useI18n()
const { settings } = useSiteChrome()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const keyword = ref('')
const education = ref(0)
const exp = ref(0)
const sex = ref(0)
const uptime = ref(0)
const photo = ref(false)
const provinceId = ref(0)
const cityId = ref(0)
const msg = ref('')
const country = computed(() => String(settings.value.sy_web_country || settings.value.sy_country || 'CN'))

const { data: dicts } = await usePublicDicts()
const eduItems = computed(() => dicts.value?.educations_user ?? [])
const expItems = computed(() => dicts.value?.experiences_user ?? [])
const { provinceItems, cityItems } = await useRegionCascade({
  country,
  provinceId,
  cityId,
})

function params() {
  return {
    page: page.value,
    page_size: pageSize,
    keyword: keyword.value || undefined,
    education: education.value || undefined,
    exp: exp.value || undefined,
    sex: sex.value || undefined,
    uptime: uptime.value || undefined,
    photo: photo.value || undefined,
    province_id: provinceId.value || undefined,
    city_id: cityId.value || undefined,
  }
}

const { data, error, refresh } = await useAsyncData(
  () => `talent-search-${JSON.stringify(params())}`,
  () => api.post<{ list: ResumeRow[]; total: number }>('/v1/wap/resumes', params()),
)

const list = computed(() => data.value?.list || [])
const total = computed(() => inferTotal(data.value))
const rows = computed<HrRow[]>(() =>
  list.value.map((row) => ({
    key: row.uid,
    name: String(row.display_name || row.name || row.uname || row.uid),
    time: row.lastupdate_n,
    to: `/resumes/${row.uid}?eid=${row.eid || row.def_job || ''}`,
    photo: row.photo_n || row.photo ? mediaUrl(String(row.photo_n || row.photo)) : undefined,
    info: [row.sex_n, row.edu_n || row.education_n, row.exp_n].map((x) => String(x || '')).filter(Boolean),
    salary: String(row.expect_salary_n || row.salary || ''),
  })),
)

function rowOf(key: string | number) {
  return list.value.find((x) => x.uid === Number(key))
}

async function download(row?: ResumeRow) {
  if (!row) return
  msg.value = ''
  try {
    const r = await api.post<{ status?: number; jifen?: number; price?: number }>('/v1/mcenter/resume-downloads', {
      uid: row.uid,
      eid: row.eid || row.def_job || 0,
    })
    if (Number(r.status) === 2) {
      const text = r.jifen
        ? `${t('common_00697')}${r.jifen}${t('common_01935')}?`
        : r.price
          ? `${t('common_00696')}${r.price}${t('common_00757')}?`
          : t('common_00696')
      if (window.confirm(text)) {
        await api.post('/v1/mcenter/resume-downloads', {
          uid: row.uid,
          eid: row.eid || row.def_job || 0,
          confirm: true,
        })
      }
    }
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

function search() {
  go(1)
}

useSeoMeta({ title: t('default_00312') })
</script>

<template>
  <MemberPanel :title="$t('default_00312')" :error="error && !isUnauthErr(error) ? error : undefined">
    <template #pcTabs><MemberHrTabs /></template>
    <template #h5Tabs><MemberHrTabs /></template>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <template v-else>
      <MemberComScreen
        v-model:keyword="keyword"
        searchable
        :search-placeholder="$t('member_com_00608')"
        @search="search"
      >
        <select v-model.number="education">
          <option :value="0">{{ $t('wap_00459') }}</option>
          <option v-for="d in eduItems" :key="d.id" :value="d.id">{{ d.name }}</option>
        </select>
        <select v-model.number="exp">
          <option :value="0">{{ $t('wap_00457') }}</option>
          <option v-for="d in expItems" :key="d.id" :value="d.id">{{ d.name }}</option>
        </select>
        <select v-model.number="sex">
          <option :value="0">{{ $t('common.all') }}</option>
          <option :value="1">{{ $t('common_02092') }}</option>
          <option :value="2">{{ $t('common_02069') }}</option>
        </select>
        <select v-model.number="uptime">
          <option :value="0">{{ $t('common.all') }}</option>
          <option :value="1">{{ $t('common_01940') }}</option>
          <option :value="3">{{ $t('wap_00432') }}</option>
          <option :value="7">{{ $t('wap_00433') }}</option>
        </select>
        <select v-model.number="provinceId" @change="cityId = 0">
          <option :value="0">{{ $t('member_com_00378') }}</option>
          <option v-for="p in provinceItems" :key="p.id" :value="p.id">{{ p.name }}</option>
        </select>
        <select v-model.number="cityId">
          <option :value="0">{{ $t('common.all') }}</option>
          <option v-for="c in cityItems" :key="c.id" :value="c.id">{{ c.name }}</option>
        </select>
        <label><input v-model="photo" type="checkbox" /> {{ $t('ui.image') }}</label>
      </MemberComScreen>
      <MemberHrResumeRows :rows="rows">
        <template #pc-acts="{ row }">
          <a href="javascript:;" class="cblue" @click="download(rowOf(row.key))">{{ $t('wap_00451') }}</a>
        </template>
        <template #h5-acts="{ row }">
          <a href="javascript:;" @click="download(rowOf(row.key))">{{ $t('wap_00451') }}</a>
        </template>
      </MemberHrResumeRows>
      <p v-if="!rows.length" class="muted">{{ $t('ui.no_items') }}</p>
      <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
      <p v-if="msg">{{ msg }}</p>
    </template>
  </MemberPanel>
</template>
