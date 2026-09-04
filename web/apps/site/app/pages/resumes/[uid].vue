<script setup lang="ts">
import { mediaUrl, PLACEHOLDER_LOGO } from '~/utils/site'

const route = useRoute()
const { t, te, locale } = useI18n()
const uid = Number(route.params.uid)
const api = useApi()
const { me } = useSiteChrome()
const clientVisitorBlocked = ref(false)
const { data, error, refresh } = await useAsyncData(
  () => `resume-${locale.value}-${uid}`,
  () => api.get('/v1/wap/resumes/detail', { uid }),
)
const row = computed(() => (data.value || {}) as Record<string, unknown>)
const visitorBlocked = computed(() => Boolean(row.value.visitor_blocked) || clientVisitorBlocked.value)
const name = computed(() => String(row.value.display_name || row.value.name || row.value.uname || ''))
const works = computed(() => (Array.isArray(row.value.works) ? row.value.works : []) as Record<string, unknown>[])
const edus = computed(() => (Array.isArray(row.value.edus) ? row.value.edus : []) as Record<string, unknown>[])
const skills = computed(() => (Array.isArray(row.value.skills) ? row.value.skills : []) as Record<string, unknown>[])
const projects = computed(() => (Array.isArray(row.value.projects) ? row.value.projects : []) as Record<string, unknown>[])
const trainings = computed(() => (Array.isArray(row.value.trainings) ? row.value.trainings : []) as Record<string, unknown>[])
const certs = computed(() => (Array.isArray(row.value.certs) ? row.value.certs : []) as Record<string, unknown>[])
const expects = computed(() => (Array.isArray(row.value.expects) ? row.value.expects : []) as Record<string, unknown>[])
const others = computed(() => (Array.isArray(row.value.others) ? row.value.others : []) as Record<string, unknown>[])
const shows = computed(() => (Array.isArray(row.value.shows) ? row.value.shows : []) as Record<string, unknown>[])
const docs = computed(() => (Array.isArray(row.value.docs) ? row.value.docs : []) as Record<string, unknown>[])
const tel = computed(() => String(row.value.telphone || ''))
const reportOpen = ref(false)
function errKey(e: unknown): string {
  if (e && typeof e === 'object' && 'key' in e) return String((e as { key: unknown }).key || '')
  return ''
}
function docIsFile(raw: unknown) {
  const s = String(raw || '')
  return Boolean(s) && !/<[a-z]/i.test(s)
}
const expect0 = computed(() => expects.value[0] || {})
const expectJobs = computed(() =>
  expects.value
    .map((e) => String(e.name || e.job_class_n || e.job_classid_n || ''))
    .filter(Boolean)
    .slice(0, 5),
)
const expectCities = computed(() =>
  expects.value
    .map((e) => String(e.city_class_n || ''))
    .filter(Boolean)
    .slice(0, 5),
)
const expectTitle = computed(() => expectJobs.value[0] || '')
const expectCity = computed(() => expectCities.value[0] || '')
const expectSalary = computed(() => String(expect0.value.salary_n || ''))
const unlocked = computed(() => Number(row.value.m_status) === 1)
const bodyOpen = computed(() => Number(row.value.resume_check ?? 1) === 1)
const tj = computed(() => (row.value.tj || {}) as Record<string, unknown>)
const photo = computed(() => mediaUrl(String(row.value.photo_n || row.value.photo || ''), PLACEHOLDER_LOGO))
const alreadyInvited = computed(() => Boolean(row.value.invited))
const sexLabel = computed(() => {
  const n = Number(row.value.sex)
  if (n === 1) return t('common_02092')
  if (n === 2) return t('common_02069')
  return ''
})
const tags = computed(() => {
  const raw = row.value.tag
  if (Array.isArray(raw)) return raw.map(String).filter(Boolean).slice(0, 8)
  if (typeof raw === 'string' && raw) return raw.split(/[,，]/).map((s) => s.trim()).filter(Boolean).slice(0, 8)
  return [] as string[]
})
const fav = ref(false)
const actionMsg = ref('')
const yqmsOpen = ref(false)
const yqmsJobs = ref<Array<Record<string, unknown>>>([])
const yqmsTpls = ref<Array<Record<string, unknown>>>([])
const yqms = reactive({
  job_id: 0,
  content: '',
  address: '',
  intertime: '',
  linkman: '',
  linktel: '',
  save_yqmb: false,
  ymid: 0,
  mappic: '',
})
function readVisitorCookie() {
  if (!import.meta.client) return 0
  const hit = document.cookie
    .split(';')
    .map((x) => x.trim())
    .find((x) => x.startsWith('resumevisitors='))
  if (!hit) return 0
  return Number(hit.split('=')[1]) || 0
}
function bumpVisitorCookie() {
  if (!import.meta.client) return
  const n = readVisitorCookie() + 1
  const tomorrow = new Date()
  tomorrow.setHours(24, 0, 0, 0)
  document.cookie = `resumevisitors=${n}; path=/; expires=${tomorrow.toUTCString()}; SameSite=Lax`
}
type DownloadResult = {
  status: number
  private_phone?: string
  msg_key?: string
  jifen?: number
  price?: number
}
function payConfirmText(res: DownloadResult) {
  if (res.jifen) {
    return `${t('common_00697')}${res.jifen}${t('common_01935')}?`
  }
  if (res.price) {
    return `${t('common_00696')}${res.price}${t('common_00757')}?`
  }
  return te((res.msg_key || 'common_00696') as never)
    ? t((res.msg_key || 'common_00696') as never)
    : t('common_00696')
}
useSeoMeta({
  title: () => name.value || t('common.resume'),
  description: () => String(expectTitle.value || name.value || t('common.resume')),
  keywords: () => [name.value, expectTitle.value, expectCity.value].filter(Boolean).join(','),
})
onMounted(async () => {
  if (Boolean(row.value.visitor_blocked)) {
    clientVisitorBlocked.value = true
    return
  }
  const max = Number(row.value.visitor_max || 0)
  if (!me.value && max > 0 && import.meta.client) {
    if (readVisitorCookie() >= max) {
      clientVisitorBlocked.value = true
      return
    }
    bumpVisitorCookie()
  }
  fav.value = Boolean(row.value.in_talentpool)
  const eid = Number(expect0.value.id || row.value.def_job || 0)
  if (eid > 0) {
    api.post('/v1/wap/resumes/expects/hits', { eid }).catch(() => {})
  }
  try {
    const r = await api.post<{ exists?: boolean; favorited?: boolean }>('/v1/mcenter/favorites/exists', {
      kind: 3,
      target_id: uid,
    })
    fav.value = Boolean(row.value.in_talentpool) || Boolean(r.exists || r.favorited)
  } catch {
    /* guest */
  }
})
async function download(confirm = false) {
  if (!me.value) {
    await navigateTo('/login')
    return
  }
  if (me.value.usertype !== 2) {
    actionMsg.value = t('resume_00038')
    return
  }
  if (!unlocked.value && !confirm) {
    const pack = Number(row.value.downresumes || 0)
    const free = Number(row.value.free_look || 0)
    const ok = window.confirm(`${t('resume_00029')} (${pack}/${free})`)
    if (!ok) return
  }
  try {
    const res = await api.post<DownloadResult>('/v1/mcenter/resume-downloads', {
      uid,
      eid: Number(row.value.def_job || expect0.value.id || 0) || undefined,
      confirm,
    })
    if (res.status === 2) {
      if (window.confirm(payConfirmText(res))) {
        await download(true)
      }
      return
    }
    actionMsg.value = res.private_phone || t('common.confirm')
    await refresh()
  } catch (e: unknown) {
    if (errKey(e) === 'need_buy_down_resume' || errKey(e) === 'vip_day_limit' || errKey(e) === 'integral_insufficient') {
      actionMsg.value = e instanceof Error ? e.message : t('common_00888')
      await navigateTo('/com/pay')
      return
    }
    actionMsg.value = e instanceof Error ? e.message : t('common_00888')
    if (!me.value) await navigateTo('/login')
  }
}
async function lookAll() {
  const mode = Number(row.value.resume_open_check || 2)
  if (!me.value) {
    await navigateTo('/login')
    return
  }
  if (mode === 3) {
    actionMsg.value = t('wap_00322')
    if (me.value.usertype === 2) await navigateTo('/com/jobs')
    return
  }
  if (mode === 4) {
    await download()
    return
  }
  actionMsg.value = t('wap_00485')
}
async function invite() {
  if (!me.value) {
    await navigateTo('/login')
    return
  }
  if (me.value.usertype !== 2) {
    actionMsg.value = t('resume_00038')
    return
  }
  if (alreadyInvited.value) {
    actionMsg.value = t('wap_00291')
    return
  }
  yqmsOpen.value = true
  try {
    const jobs = await api.post<{ list?: Array<Record<string, unknown>> }>('/v1/mcenter/jobs/list', {
      state: 1,
      page: 1,
      page_size: 50,
    })
    yqmsJobs.value = jobs.list || []
    const tpls = await api.post<Array<Record<string, unknown>>>('/v1/mcenter/interview-templates/list', {})
    yqmsTpls.value = Array.isArray(tpls) ? tpls : []
    if (!yqms.job_id && yqmsJobs.value[0]) yqms.job_id = Number(yqmsJobs.value[0].id || 0)
    const tpl = yqmsTpls.value[0]
    if (tpl) applyTpl(tpl)
  } catch (e: unknown) {
    actionMsg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
function applyTpl(tpl: Record<string, unknown>) {
  yqms.ymid = Number(tpl.id || 0)
  yqms.content = String(tpl.content || '')
  yqms.address = String(tpl.address || yqms.address)
  yqms.linkman = String(tpl.linkman || yqms.linkman)
  yqms.linktel = String(tpl.linktel || yqms.linktel)
  if (tpl.intertime_n) yqms.intertime = String(tpl.intertime_n)
}
type YqmsResult = {
  status: number
  id?: number
  msg_key?: string
  jifen?: number
  price?: number
}
function payConfirmTextYqms(res: YqmsResult) {
  if (res.jifen) {
    return `${t('common_00697')}${res.jifen}${t('common_01935')}?`
  }
  if (res.price) {
    return `${t('common_00696')}${res.price}${t('common_00757')}?`
  }
  return te((res.msg_key || 'common_00696') as never)
    ? t((res.msg_key || 'common_00696') as never)
    : t('common_00696')
}
async function submitYqms(confirm = false) {
  if (!yqms.intertime.trim()) {
    actionMsg.value = t('member_com_00681')
    return
  }
  try {
    const res = await api.post<YqmsResult>('/v1/mcenter/company/yqms/create', {
      seeker_uid: uid,
      job_id: yqms.job_id,
      content: yqms.content,
      address: yqms.address,
      intertime: yqms.intertime,
      linkman: yqms.linkman,
      linktel: yqms.linktel,
      save_yqmb: yqms.save_yqmb,
      ymid: yqms.ymid,
      mappic: yqms.mappic,
      confirm,
    })
    if (res.status === 2) {
      if (window.confirm(payConfirmTextYqms(res))) {
        await submitYqms(true)
      }
      return
    }
    yqmsOpen.value = false
    actionMsg.value = t('common.confirm')
    await refresh()
  } catch (e: unknown) {
    if (errKey(e) === 'need_buy_invite' || errKey(e) === 'vip_day_limit' || errKey(e) === 'integral_insufficient') {
      actionMsg.value = e instanceof Error ? e.message : t('common_00888')
      await navigateTo('/com/pay')
      return
    }
    actionMsg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
async function toggleFav() {
  const eid = Number(row.value.def_job || expect0.value.id || 0)
  try {
    if (eid) {
      await api.post('/v1/mcenter/talent-pool', { eid, seeker_uid: uid })
      fav.value = true
    } else {
      const r = await api.post<{ favorited: boolean }>('/v1/mcenter/favorites', { kind: 3, target_id: uid })
      fav.value = Boolean(r.favorited)
    }
  } catch {
    await navigateTo('/login')
  }
}
async function report() {
  reportOpen.value = true
}
</script>

<template>
  <article v-if="error" class="site-inner">
    <h1>{{ $t('common.resume') }}</h1>
    <p class="muted">{{ $t('ui.load_failed') }}</p>
  </article>
  <article v-else-if="visitorBlocked" class="site-inner">
    <h1>{{ $t('common.resume') }}</h1>
    <p class="muted">{{ $t('wap_00424') }}{{ row.visitor_max }}{{ $t('wap_00422') }}</p>
    <NuxtLink to="/login">{{ $t('common.login') }}</NuxtLink>
  </article>
  <article v-else>
    <div class="site-pc">
      <div class="yun_newedition_resume_top">
        <div class="w1200">
          <div class="yun_newedition_resume_current">
            {{ $t('common.home') }} > {{ $t('common.resume') }} > {{ $t('resume_00062') }}
          </div>
          <div class="yun_newedition_resume_info">
            <div class="yun_newedition_resume_info_c">
              <div class="yun_newedition_resume_info_pic">
                <img :src="photo" width="140" height="140" alt="" />
                <i class="resume_list_xb" :class="{ resume_list_xb_nv: Number(row.sex) === 2 }" />
              </div>
              <div>
                <span class="yun_newedition_resume_username">{{ name || $t('common.resume') }}</span>
                <span v-if="Number(row.idcard_status) === 1" class="yun_newedition_resume_rz yun_newedition_resume_rz_sm">{{
                  $t('member_com_00026')
                }}</span>
                <span v-if="Number(row.moblie_status) === 1" class="yun_newedition_resume_rz yun_newedition_resume_rz_sj">{{
                  $t('member_com_00071')
                }}</span>
                <span v-if="Number(row.email_status) === 1" class="yun_newedition_resume_rz yun_newedition_resume_rz_yx">{{
                  $t('wap_com_00186')
                }}</span>
              </div>
              <div class="yun_newedition_resume_basic">
                <template v-if="sexLabel">{{ sexLabel }}<span class="yun_newedition_resume_line">|</span></template>
                <template v-if="row.age">{{ row.age }}{{ $t('home.age_suffix') }}<span class="yun_newedition_resume_line">|</span></template>
                <template v-if="row.height">{{ row.height }}cm </template>
                <template v-if="row.weight">{{ row.weight }}kg <span class="yun_newedition_resume_line">|</span></template>
                <template v-if="row.exp_n">{{ row.exp_n }}{{ $t('home.experience_suffix') }}<span class="yun_newedition_resume_line">|</span></template>
                <template v-if="row.edu_n || row.education_n">{{ row.edu_n || row.education_n }}{{ $t('home.education_suffix') }}</template>
                <template v-if="row.living"><span class="yun_newedition_resume_line">|</span>{{ $t('common_02013') }}{{ row.living }}</template>
              </div>
              <div v-if="tags.length" class="yun_newedition_resume_fl">
                <span v-for="tag in tags" :key="tag">{{ tag }}</span>
              </div>
              <div class="yun_newedition_resume_data">
                <span v-if="row.lastupdate_n" class="yun_newedition_resume_data_time">{{ $t('wap_00225') }}：{{ row.lastupdate_n }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="w1200">
        <div class="yun_newedition_resume_cont">
          <div class="yun_newedition_resume_left">
            <div class="yun_newedition_tit">
              <span class="yun_newedition_tit_s">{{ $t('wap_00460') }}</span>
              <i class="yun_newedition_tit_line" />
            </div>
            <ul class="yun_newedition_yx_list">
              <li style="width: 100%">
                <span class="yun_newedition_yx_name">{{ $t('wap_user_00055') }}：</span>
                <span v-for="j in expectJobs" :key="j" class="yun_newedition_yx_job">{{ j }}</span>
              </li>
              <li v-if="expectSalary">
                <span class="yun_newedition_yx_name">{{ $t('wap_user_00016') }}：</span>{{ expectSalary }}
              </li>
              <li v-if="expect0.hy_n">
                <span class="yun_newedition_yx_name">{{ $t('wap_user_00010') }}：</span>{{ expect0.hy_n }}
              </li>
              <li v-if="expect0.report_n">
                <span class="yun_newedition_yx_name">{{ $t('wap_com_00279') }}：</span>{{ expect0.report_n }}
              </li>
              <li v-if="expect0.jobstatus_n">
                <span class="yun_newedition_yx_name">{{ $t('wap_user_00017') }}：</span>{{ expect0.jobstatus_n }}
              </li>
              <li v-if="expect0.type_n">
                <span class="yun_newedition_yx_name">{{ $t('wap_user_00012') }}：</span>{{ expect0.type_n }}
              </li>
              <li v-if="expectCities.length" style="width: 100%">
                <span class="yun_newedition_yx_name">{{ $t('member_user_00198') }}：</span>
                <template v-for="c in expectCities" :key="c">{{ c }}&nbsp;</template>
              </li>
            </ul>
            <div v-if="unlocked">
              <div class="yun_newedition_tit">
                <span class="yun_newedition_tit_s">{{ $t('wap_00462') }}</span>
                <i class="yun_newedition_tit_line" />
              </div>
              <div class="tcktouch_box_tip">{{ $t('member_com_00024') }}</div>
              <div v-if="row.telphone" class="tcktouch_box_p">
                {{ $t('member_user_00163') }}：<span class="tcktouch_box_p_sj">{{ row.telphone }}</span>
              </div>
              <div v-if="row.email" class="tcktouch_box_p">{{ $t('member_user_00282') }}：{{ row.email }}</div>
              <div v-if="row.qq" class="tcktouch_box_p">Q Q：{{ row.qq }}</div>
            </div>
            <div v-else class="firm_login" style="margin: 12px 0">
              <p class="muted">{{ $t('wap_00376') }}</p>
              <NuxtLink v-if="!me" to="/login" class="firm_login_dl">{{ $t('common.login') }}</NuxtLink>
              <button v-else-if="me.usertype === 2" type="button" class="user_yqms" @click="download">{{ $t('resume_00029') }}</button>
              <p v-else class="muted">{{ $t('resume_00038') }}</p>
            </div>
            <div v-if="bodyOpen && row.description" class="yun_newedition_tit" style="margin-top: 16px">
              <span class="yun_newedition_tit_s">{{ $t('wap_00463') }}</span>
              <i class="yun_newedition_tit_line" />
            </div>
            <div v-if="bodyOpen && row.description" v-html="String(row.description)" />
            <template v-if="bodyOpen">
            <div v-if="works.length" class="yun_newedition_tit">
              <span class="yun_newedition_tit_s">{{ $t('wap_00457') }}</span>
              <i class="yun_newedition_tit_line" />
            </div>
            <div v-for="w in works" :key="String(w.id)" class="muted">
              <strong>{{ w.name }}</strong> {{ w.title || w.department }} {{ w.sdate_n }} - {{ w.edate_n }}
              <div v-html="String(w.content || '')" />
            </div>
            <div v-if="edus.length" class="yun_newedition_tit">
              <span class="yun_newedition_tit_s">{{ $t('wap_00459') }}</span>
              <i class="yun_newedition_tit_line" />
            </div>
            <div v-for="e in edus" :key="String(e.id)" class="muted">
              {{ e.name }} {{ e.specialty }} {{ e.education_n }} {{ e.sdate_n }} - {{ e.edate_n }}
            </div>
            <div v-if="projects.length" class="yun_newedition_tit">
              <span class="yun_newedition_tit_s">{{ $t('wap_00465') }}</span>
              <i class="yun_newedition_tit_line" />
            </div>
            <div v-for="p in projects" :key="'p' + String(p.id)" class="muted">
              <strong>{{ p.name }}</strong> {{ p.role }} {{ p.sdate_n }} - {{ p.edate_n }}
              <div v-html="String(p.content || '')" />
            </div>
            <div v-if="trainings.length" class="yun_newedition_tit">
              <span class="yun_newedition_tit_s">{{ $t('wap_00455') }}</span>
              <i class="yun_newedition_tit_line" />
            </div>
            <div v-for="tr in trainings" :key="'t' + String(tr.id)" class="muted">
              {{ tr.name }} {{ tr.sdate_n }} - {{ tr.edate_n }}
              <div v-html="String(tr.content || '')" />
            </div>
            <div v-if="certs.length" class="yun_newedition_tit">
              <span class="yun_newedition_tit_s">{{ $t('wap_00454') }}</span>
              <i class="yun_newedition_tit_line" />
            </div>
            <div v-for="c in certs" :key="'c' + String(c.id)" class="muted">{{ c.name }} {{ c.sdate_n }}</div>
            <div v-if="skills.length" class="yun_newedition_tit">
              <span class="yun_newedition_tit_s">{{ $t('member_com_00027') }}</span>
              <i class="yun_newedition_tit_line" />
            </div>
            <div v-if="skills.length" class="yun_newedition_skill">
              <div v-for="s in skills" :key="'sk' + String(s.id)" class="yun_newedition_skilllist">
                <div class="yun_newedition_skill_name">{{ s.name }}</div>
                <div v-if="s.level_n" class="yun_newedition_skill_zt">{{ s.level_n }}</div>
                <div v-if="s.years" class="yun_newedition_skill_time">{{ s.years }}{{ $t('common_02077') }}</div>
              </div>
            </div>
            <div v-if="others.length" class="yun_newedition_tit">
              <span class="yun_newedition_tit_s">{{ $t('wap_00493') }}</span>
              <i class="yun_newedition_tit_line" />
            </div>
            <div v-for="o in others" :key="'o' + String(o.id)" class="muted">
              <strong>{{ o.name }}</strong>
              <div v-if="o.content" v-html="String(o.content)" />
            </div>
            <div v-if="shows.length" class="yun_newedition_tit">
              <span class="yun_newedition_tit_s">{{ $t('wap_01601') }}</span>
              <i class="yun_newedition_tit_line" />
            </div>
            <div v-if="shows.length" class="com_show_image">
              <div v-for="s in shows" :key="'show' + String(s.id)" class="com_show_image_list">
                <img :src="mediaUrl(String(s.picurl || ''), PLACEHOLDER_LOGO)" width="210" height="153" :alt="String(s.title || '')" />
              </div>
            </div>
            <div v-if="unlocked && docs.length" class="yun_newedition_tit">
              <span class="yun_newedition_tit_s">{{ $t('wap_00495') }}</span>
              <i class="yun_newedition_tit_line" />
            </div>
            <div v-for="d in docs" :key="'doc' + String(d.id)" class="yun_newedition_js">
              <a
                v-if="docIsFile(d.doc)"
                :href="mediaUrl(String(d.doc))"
                target="_blank"
                rel="noopener"
              >{{ d.doc }}</a>
              <div v-else v-html="String(d.doc || '')" />
            </div>
            </template>
            <div v-else class="resume_bg" style="margin-top: 16px">
              <p v-if="Number(tj.edu_num)" class="muted">{{ $t('wap_00459') }} {{ tj.edu_num }}</p>
              <p v-if="Number(tj.work_num)" class="muted">{{ $t('wap_00457') }} {{ $t('wap_00467') }}{{ tj.work_num }}{{ $t('common_01887') }}</p>
              <p v-if="Number(tj.project_num)" class="muted">{{ $t('wap_00465') }} {{ $t('wap_00443') }}{{ tj.project_num }}{{ $t('wap_00466') }}</p>
              <p v-if="Number(tj.training_num)" class="muted">{{ $t('wap_00458') }} {{ $t('wap_00467') }}{{ tj.training_num }}</p>
              <p v-if="Number(tj.skill_num)" class="muted">{{ $t('wap_00450') }}{{ tj.skill_num }}</p>
              <p v-if="Number(tj.cert_num)" class="muted">{{ $t('wap_00454') }} {{ tj.cert_num }}</p>
              <a href="javascript:;" class="resume_lookall_a" @click.prevent="lookAll">{{ $t('wap_01602') }}</a>
            </div>
            <p style="margin-top: 16px">
              <button type="button" class="user_yqms" @click="download">{{ $t('resume_00029') }}</button>
              <button v-if="alreadyInvited" type="button" class="user_yqms" disabled>{{ $t('wap_00291') }}</button>
              <button v-else type="button" class="user_yqms" @click="invite">{{ $t('wap_user_00216') }}</button>
              <a href="javascript:;" class="job_ceil_jobsc" @click.prevent="toggleFav">{{
                fav ? $t('wap_00378') : $t('wap_00379')
              }}</a>
              <a href="javascript:;" class="job_ceil_jobsc" @click.prevent="report">{{ $t('wap_com_00350') }}</a>
            </p>
            <p v-if="actionMsg" class="muted">{{ actionMsg }}</p>
          </div>
        </div>
      </div>
    </div>

    <div class="site-h5">
      <div class="Edit_your_resume_header" style="display: flex; align-items: center; justify-content: space-between; padding: 0.2rem 0.32rem">
        <NuxtLink to="/resumes" class="Edit_your_resume_header_left">
          <img src="/legacy/h5/images/return.png" alt="" width="24" height="24" />
        </NuxtLink>
        <div>
          <a href="javascript:;" class="new_user_headericon" @click.prevent="toggleFav">
            <img :src="fav ? '/legacy/h5/images/ysc_n.png' : '/legacy/h5/images/r_sc.png'" alt="" width="24" height="24" />
          </a>
          <a href="javascript:;" class="new_user_headerjb" @click.prevent="report">
            <img src="/legacy/h5/images/details_report.png" alt="" width="24" height="24" />
          </a>
        </div>
      </div>
      <div class="resume_body">
        <div class="resume_body_card">
          <div class="new_user_box">
            <span class="new_user_name">{{ name || $t('common.resume') }}</span>
            <div class="Edit_your_resume_card_name_logo">
              <img :src="photo" alt="" style="width: 100%; height: 100%; border-radius: 50%" />
              <i v-if="Number(row.sex) === 2" class="yun_newedition_resume_userxb" />
              <i v-else-if="Number(row.sex) === 1" class="yun_newedition_resume_userxb_n" />
            </div>
          </div>
          <div class="new_user_info">
            {{ row.exp_n }}{{ $t('wap_com_00305') }} · {{ row.edu_n || row.education_n }}{{ $t('wap_com_00301') }}
            <template v-if="row.age"> · {{ row.age }}{{ $t('common_02074') }}</template>
            <template v-if="row.living"> · {{ row.living }}</template>
          </div>
        </div>
        <div class="Preview_your_resume_intention">
          <div class="Preview_your_resume_header">
            <div class="Preview_your_resume_word">{{ $t('wap_00460') }}</div>
          </div>
          <div class="user_qwinfobox">
            <div class="user_qwinfo">
              <span>{{ expectTitle }}</span>
              <span v-if="expectCity">· {{ expectCity }}</span>
            </div>
            <div class="user_qwxz" v-if="expectSalary || expect0.hy_n || expect0.report_n || expect0.jobstatus_n || expect0.type_n">
              <span v-if="expectSalary">{{ expectSalary }}</span>
              <span v-if="expect0.hy_n"> · {{ expect0.hy_n }}</span>
              <span v-if="expect0.report_n"> · {{ expect0.report_n }}</span>
              <span v-if="expect0.jobstatus_n"> · {{ expect0.jobstatus_n }}</span>
              <span v-if="expect0.type_n"> · {{ expect0.type_n }}</span>
            </div>
          </div>
          <div v-if="row.lastupdate_n" class="Preview_your_resume_category">
            {{ row.lastupdate_n }} {{ $t('wap_00225') }}
          </div>
        </div>
        <div v-if="bodyOpen && row.description" class="Preview_your_resume_experience">
          <div class="Preview_your_resume_header">
            <div class="Preview_your_resume_word">{{ $t('wap_00463') }}</div>
          </div>
          <div v-html="String(row.description)" />
        </div>
        <div v-if="bodyOpen && works.length" class="Preview_your_resume_experience">
          <div class="Preview_your_resume_header">
            <div class="Preview_your_resume_word">{{ $t('wap_00457') }}</div>
          </div>
          <div v-for="w in works" :key="String(w.id)">
            <strong>{{ w.name }}</strong> {{ w.sdate_n }} - {{ w.edate_n }}
            <div v-html="String(w.content || '')" />
          </div>
        </div>
        <div v-if="bodyOpen && edus.length" class="Preview_your_resume_education">
          <div class="Preview_your_resume_header">
            <div class="Preview_your_resume_word">{{ $t('wap_00459') }}</div>
          </div>
          <div v-for="e in edus" :key="String(e.id)">
            {{ e.name }} {{ e.specialty }} {{ e.education_n }} {{ e.sdate_n }} - {{ e.edate_n }}
          </div>
        </div>
        <div v-if="bodyOpen && projects.length" class="Preview_your_resume_experience">
          <div class="Preview_your_resume_header">
            <div class="Preview_your_resume_word">{{ $t('wap_00465') }}</div>
          </div>
          <div v-for="p in projects" :key="'hp' + String(p.id)">
            <strong>{{ p.name }}</strong> {{ p.role }} {{ p.sdate_n }} - {{ p.edate_n }}
          </div>
        </div>
        <div v-if="bodyOpen && trainings.length" class="Preview_your_resume_experience">
          <div class="Preview_your_resume_header">
            <div class="Preview_your_resume_word">{{ $t('wap_00455') }}</div>
          </div>
          <div v-for="tr in trainings" :key="'ht' + String(tr.id)">{{ tr.name }} {{ tr.sdate_n }}</div>
        </div>
        <div v-if="bodyOpen && certs.length" class="Preview_your_resume_experience">
          <div class="Preview_your_resume_header">
            <div class="Preview_your_resume_word">{{ $t('wap_00454') }}</div>
          </div>
          <div v-for="c in certs" :key="'hc' + String(c.id)">{{ c.name }}</div>
        </div>
        <div v-if="bodyOpen && skills.length" class="Preview_your_resume_experience">
          <div class="Preview_your_resume_header">
            <div class="Preview_your_resume_word">{{ $t('member_com_00027') }}</div>
          </div>
          <div v-for="s in skills" :key="'hsk' + String(s.id)">
            {{ s.name }}
            <template v-if="s.level_n"> · {{ s.level_n }}</template>
            <template v-if="s.years"> · {{ s.years }}{{ $t('common_02077') }}</template>
          </div>
        </div>
        <div v-if="bodyOpen && others.length" class="Preview_your_resume_experience">
          <div class="Preview_your_resume_header">
            <div class="Preview_your_resume_word">{{ $t('wap_00493') }}</div>
          </div>
          <div v-for="o in others" :key="'ho' + String(o.id)">
            <div>{{ o.name }}</div>
            <div v-if="o.content" v-html="String(o.content)" />
          </div>
        </div>
        <div v-if="bodyOpen && shows.length" class="Preview_your_resume_experience">
          <div class="Preview_your_resume_header">
            <div class="Preview_your_resume_word">{{ $t('wap_01601') }}</div>
          </div>
          <div class="business_album">
            <img
              v-for="s in shows"
              :key="'hshow' + String(s.id)"
              :src="mediaUrl(String(s.picurl || ''), PLACEHOLDER_LOGO)"
              :alt="String(s.title || '')"
            />
          </div>
        </div>
        <div v-if="unlocked && docs.length" class="Preview_your_resume_advantage">
          <div class="Preview_your_resume_header">
            <div class="Preview_your_resume_word">{{ $t('wap_00495') }}</div>
          </div>
          <div v-for="d in docs" :key="'hdoc' + String(d.id)">
            <a
              v-if="docIsFile(d.doc)"
              :href="mediaUrl(String(d.doc))"
              target="_blank"
              rel="noopener"
            >{{ d.doc }}</a>
            <div v-else v-html="String(d.doc || '')" />
          </div>
        </div>
        <div v-if="!bodyOpen" class="Preview_your_resume_advantage">
          <p v-if="Number(tj.edu_num)" class="muted">{{ $t('wap_00459') }} {{ tj.edu_num }}</p>
          <p v-if="Number(tj.work_num)" class="muted">{{ $t('wap_00457') }} {{ $t('wap_00467') }}{{ tj.work_num }}{{ $t('common_01887') }}</p>
          <p v-if="Number(tj.project_num)" class="muted">{{ $t('wap_00465') }} {{ $t('wap_00443') }}{{ tj.project_num }}{{ $t('wap_00466') }}</p>
          <p v-if="Number(tj.training_num)" class="muted">{{ $t('wap_00458') }} {{ $t('wap_00467') }}{{ tj.training_num }}</p>
          <p v-if="Number(tj.skill_num)" class="muted">{{ $t('wap_00450') }}{{ tj.skill_num }}</p>
          <a href="javascript:;" class="resume_lookall_a" @click.prevent="lookAll">{{ $t('wap_01602') }}</a>
        </div>
        <div v-if="unlocked" class="new_user_touchbox">
          <div v-if="row.telphone" class="new_user_touch">{{ row.telphone }}</div>
          <div v-if="row.email" class="new_user_touch">{{ row.email }}</div>
          <div v-if="row.qq" class="new_user_touch">{{ row.qq }}</div>
        </div>
        <div v-else class="resume_body_card">
          <p class="muted">{{ me?.usertype === 2 ? $t('resume_00029') : $t('wap_00376') }}</p>
          <button v-if="me?.usertype === 2" type="button" class="job_ceil_jobtd" @click="download">{{ $t('resume_00029') }}</button>
          <NuxtLink v-else to="/login" class="job_ceil_jobtd">{{ $t('common.login') }}</NuxtLink>
        </div>
        <p v-if="actionMsg" class="muted">{{ actionMsg }}</p>
      </div>
      <div class="yun_czfoot">
        <div class="yun_czfootfixed">
          <div class="yun_czfoot_r">
            <div class="yun_czfoot_lt yun_czfoot_lt_td">
              <a v-if="unlocked && tel" :href="`tel:${tel}`">{{ $t('wap_00279') }}</a>
              <a v-else-if="me?.usertype === 2" href="javascript:;" @click.prevent="download">{{ $t('wap_00279') }}</a>
              <NuxtLink v-else to="/login">{{ $t('wap_00279') }}</NuxtLink>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div
      v-if="yqmsOpen"
      class="firm_login"
      style="position: fixed; inset: 0; z-index: 80; background: rgba(0, 0, 0, 0.45); display: flex; align-items: center; justify-content: center"
      @click.self="yqmsOpen = false"
    >
      <div class="resume_body_card" style="width: min(520px, 92vw); max-height: 90vh; overflow: auto; background: #fff; padding: 16px">
        <h3>{{ $t('wap_user_00216') }}</h3>
        <p>
          <label>{{ $t('wap_00190') }}</label>
          <select v-model.number="yqms.job_id">
            <option v-for="j in yqmsJobs" :key="String(j.id)" :value="Number(j.id)">{{ j.name }}</option>
          </select>
        </p>
        <p v-if="yqmsTpls.length">
          <label>{{ $t('member_com_00268') }}</label>
          <select @change="applyTpl(yqmsTpls[Number(($event.target as HTMLSelectElement).value)] || {})">
            <option v-for="(tpl, idx) in yqmsTpls" :key="String(tpl.id)" :value="idx">{{ tpl.name }}</option>
          </select>
        </p>
        <p>
          <input v-model="yqms.intertime" type="datetime-local" />
        </p>
        <p>
          <input v-model="yqms.linkman" :placeholder="$t('common_02051')" />
        </p>
        <p>
          <input v-model="yqms.linktel" :placeholder="$t('common.phone')" />
        </p>
        <p>
          <input v-model="yqms.address" :placeholder="$t('wap_00040')" />
        </p>
        <p>
          <textarea v-model="yqms.content" rows="4" />
        </p>
        <p>
          <label><input v-model="yqms.save_yqmb" type="checkbox" /> {{ $t('member_com_00512') }}</label>
        </p>
        <p>
          <button type="button" class="user_yqms" @click="submitYqms">{{ $t('common.confirm') }}</button>
          <button type="button" class="user_yqms" @click="yqmsOpen = false">{{ $t('common.cancel') }}</button>
        </p>
      </div>
    </div>
    <ReportSheet
      v-if="reportOpen"
      :target-kind="3"
      :target-id="Number(row.def_job || expect0.id || 0)"
      @close="reportOpen = false"
      @done="actionMsg = $t('common.confirm')"
    />
  </article>
</template>
