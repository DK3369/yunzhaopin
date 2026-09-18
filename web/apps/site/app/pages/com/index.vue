<script setup lang="ts">
import { formatUnixDate, isMemberModuleOn, mediaUrl, safeHref } from '~/utils/site'
import { ApiError } from '~/utils/envelope'

type Profile = {
  name?: string
  logo?: string
  uid?: number
  r_status?: number
  yyzz_status?: number
  hits?: number
}
type JobCounts = {
  total: number
  online: number
  breakjob_num?: number
  top_num?: number
  rec_num?: number
  urgent_num?: number
}
type YearReport = {
  login_days?: number
  job_count?: number
  view_count?: number
  received_resumes?: number
  viewed_resumes?: number
  invited_count?: number
  night_work_count?: number
  last_night_work_at?: number
  company_name?: string
  linkman?: string
}
type Dash = {
  applies_received: number
  applies_unread: number
  interviews_sent?: number
  resume_downloads?: number
  job_msg_unanswered?: number
  unread_messages?: number
  integral_balance?: number
  today?: {
    look_job?: { num?: number }
    apply?: { num?: number }
    invite?: { num?: number }
  }
  job_counts?: JobCounts
  year_report?: YearReport
}
type Vip = {
  active: boolean
  started_at?: number | null
  expires_at?: number | null
  rating_name?: string
  rating_type?: number
  job_num?: number
  breakjob_num?: number
  down_resume?: number
  invite_resume?: number
  integral?: number
}
type RecRow = { uid: number; display_name: string; sex?: number; education?: number; lastupdate?: number }
type PublishGap = { key: string; href: string }
type Notice = { cls: string; text: string; to?: string; action?: string }

const api = useApi()
const { t } = useI18n()
const { comItems } = useMemberNav()
const { data } = await useAuthMe()
const { data: dicts } = await usePublicDicts()
const { data: profile } = await useAsyncData(
  'com-home-profile',
  () => api.post<Profile>('/v1/mcenter/company/list', {}).catch(() => null),
  reuseAsyncCache(),
)
const { data: dash } = await useAsyncData(
  'com-dash',
  () => api.post<Dash>('/v1/mcenter/com-dashboard/full', {}).catch(() => null),
  reuseAsyncCache(),
)
const { data: vip } = await useAsyncData(
  'com-vip-current',
  () => api.post<Vip>('/v1/mcenter/vip/current', {}).catch(() => null),
  reuseAsyncCache(),
)
const { data: recs } = await useAsyncData(
  'com-home-rec',
  () => api.post<RecRow[]>('/v1/mcenter/recommend/resumes', { limit: 8 }).catch(() => [] as RecRow[]),
)
const { data: adsPack } = await useAdsBundle('com-home-ads', [
  { slot: '530', limit: 1 },
  { slot: '511', limit: 1 },
])
const counts = computed(() => dash.value?.job_counts || null)
const today = computed(() => dash.value?.today || null)
const recList = computed(() => (Array.isArray(recs.value) ? recs.value : []) as RecRow[])
const ads530 = computed(() => (adsPack.value?.['530'] || []).filter((a) => a.image_n || a.image))
const ads511 = computed(() => (adsPack.value?.['511'] || []).filter((a) => a.image_n || a.image))
const { data: fans } = await useAsyncData('com-fans-n', () =>
  api.post<{ total: number }>('/v1/mcenter/fans', { page: 1, page_size: 1 }).catch(() => ({ total: 0 })),
)
const { data: looks } = await useAsyncData('com-looks-n', () =>
  api.post<{ total: number }>('/v1/mcenter/look-jobs/list', { page: 1, page_size: 1 }).catch(() => ({ total: 0 })),
)
const { data: gzh } = await useAsyncData('com-gzh', () =>
  api.post<{ subscribe?: number }>('/v1/mcenter/wechat/subscribe', {}).catch(() => ({ subscribe: 1 })),
)
const { data: signSt, refresh: refreshSign } = await useAsyncData('com-home-sign', () =>
  api.post<{ signed_today?: boolean }>('/v1/mcenter/sign/status', {}).catch(() => null),
)
const gzhNeed = computed(() => Number(gzh.value?.subscribe || 0) !== 1)
const yearOpen = ref(false)
const kefuQrOpen = ref(false)
const { wxQr, settings } = useSiteChrome()
const webtel = computed(() => String(settings.value.sy_comwebtel || settings.value.sy_freewebtel || ''))
const webmoblie = computed(() => String(settings.value.sy_webmoblie || ''))
const kfqq = computed(() => String(settings.value.sy_qq || ''))
const guwenPic = computed(() => String(settings.value.sy_guwen || ''))
const priceName = computed(() => String(settings.value.integral_pricename || t('wap_user_00008')))
const posterOn = computed(() => String(settings.value.sy_haibao_isopen || '') === '1')
const buyAddedOn = computed(() => false)
const msg = ref('')
const nowTs = Math.floor(Date.now() / 1000)
const vipExpire = computed(() => Number(vip.value?.expires_at || 0))
const vipStart = computed(() => Number(vip.value?.started_at || 0))
const vipOpened = computed(() => vipExpire.value > 0)
const vipLive = computed(() => vipOpened.value && vipExpire.value >= nowTs)
const vipDown = computed(() => vipOpened.value && vipExpire.value < nowTs)
const vipRemind = computed(() => vipLive.value && vipExpire.value - nowTs < 7 * 86400)
const vipSjtip0 = computed(() => vipLive.value && Boolean(String(vip.value?.rating_name || '').trim()))
const vipRange = computed(() => {
  const a = formatUnixDate(vipStart.value).replace(/-/g, '.')
  const b = formatUnixDate(vipExpire.value).replace(/-/g, '.')
  if (a && b) return `${a} - ${b}`
  return b || a
})
const integral = computed(() => Number(vip.value?.integral ?? dash.value?.integral_balance ?? 0))
const year = computed(() => dash.value?.year_report || null)
const yearOn = computed(() => {
  const y = year.value
  if (!y) return false
  if (String(settings.value.sy_yearreport_isopen || '') === '0') return false
  return (
    Number(y.login_days || 0) +
      Number(y.job_count || 0) +
      Number(y.view_count || 0) +
      Number(y.received_resumes || 0) +
      Number(y.viewed_resumes || 0) +
      Number(y.invited_count || 0) +
      Number(y.night_work_count || 0) >
    0
  )
})
const yearRows = computed(() => {
  const y = year.value
  if (!y) return [] as Array<{ k: string; v: string }>
  const rows: Array<{ k: string; v: string }> = [
    { k: t('admin_01104'), v: String(y.login_days ?? 0) },
    { k: t('admin_yunying_00165'), v: String(y.job_count ?? 0) },
    { k: t('admin_yunying_00164'), v: String(y.view_count ?? 0) },
    { k: t('admin_yunying_00166'), v: String(y.received_resumes ?? 0) },
    { k: t('member_com_00371'), v: String(y.viewed_resumes ?? 0) },
    { k: t('wap_com_00046'), v: String(y.invited_count ?? 0) },
    { k: t('admin_yunying_00163'), v: String(y.night_work_count ?? 0) },
  ]
  if (y.last_night_work_at) {
    rows.push({ k: t('admin_yunying_00161'), v: formatUnixDate(y.last_night_work_at) })
  }
  return rows
})
const notices = computed(() => {
  const list: Notice[] = []
  const st = Number(profile.value?.r_status)
  if (st !== 1) {
    if (st === 0) {
      list.push({ cls: 'one', text: t('member_com_00563') })
    } else if (st === 2 || st === 4) {
      list.push({ cls: 'one', text: webtel.value ? `${t('wap_01862')}（${webtel.value}）` : t('wap_01862') })
    } else if (st === 3) {
      list.push({
        cls: 'one',
        text: webtel.value ? `${t('wap_user_00167')}（${webtel.value}）` : t('wap_user_00167'),
      })
    } else {
      list.push({ cls: 'one', text: t('wap_com_00080') })
    }
  }
  if (!String(profile.value?.name || '').trim()) {
    list.push({ cls: 'three', text: t('member_com_00158'), to: '/com/profile', action: t('wap_user_00189') })
  }
  if (Number(profile.value?.yyzz_status) !== 1) {
    list.push({ cls: 'two', text: t('member_com_00169'), to: '/com/cert', action: t('wap_user_00120') })
  }
  if (vipDown.value) {
    list.push({ cls: 'four', text: t('wap_01287'), to: '/com/member-right', action: t('wap_com_00066') })
  } else if (vipRemind.value) {
    list.push({ cls: 'four', text: t('member_com_00153'), to: '/com/member-right', action: t('member_com_00145') })
  }
  return list
})
const quotaTiles = computed(() => [
  { title: t('member_com_00134'), num: vip.value?.job_num ?? 0, unit: t('common_02052') },
  { title: t('member_com_00136'), num: counts.value?.breakjob_num ?? 0, unit: t('common_02088') },
  { title: t('member_com_00137'), num: vip.value?.invite_resume ?? 0, unit: t('common_02088') },
  { title: t('member_com_00135'), num: vip.value?.down_resume ?? 0, unit: t('common_02088') },
  { title: t('wap_user_00209'), num: counts.value?.top_num ?? 0, unit: t('common_02067') },
  { title: t('wap_com_00043'), num: counts.value?.urgent_num ?? 0, unit: t('common_02067') },
  { title: t('wap_com_00041'), num: counts.value?.rec_num ?? 0, unit: t('common_02067') },
])

useSeoMeta({ title: t('member_com_00290') })

function eduName(id?: number) {
  const n = Number(id || 0)
  if (!n) return ''
  return dicts.value?.educations?.find((x) => x.id === n)?.name || ''
}
function sexName(id?: number) {
  if (Number(id) === 1) return t('common_02092')
  if (Number(id) === 2) return t('common_02069')
  return ''
}
function recMeta(row: RecRow) {
  return [sexName(row.sex), eduName(row.education), formatUnixDate(row.lastupdate)].filter(Boolean).join(' . ')
}
function isQuotaErr(e: unknown) {
  if (!(e instanceof ApiError)) return false
  return (
    e.key === 'job_refresh_quota' ||
    e.key === 'model_00056' ||
    e.key === 'member_com_00696' ||
    e.key === 'wap_01287' ||
    e.key === 'api_wxapp_00002' ||
    e.key === 'common_00207' ||
    e.key === 'common_00206' ||
    e.key === 'common_00180'
  )
}
async function sign() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/sign', {})
    msg.value = t('common.success')
    await refreshSign()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function publishJob() {
  msg.value = ''
  try {
    const chk = await api.post<{ addjobnum?: number; gaps?: PublishGap[] }>('/v1/mcenter/jobs/check', {})
    if (Number(chk.addjobnum) === 0) {
      msg.value = t('wap_01287')
      if (import.meta.client && window.confirm(`${t('wap_01287')} ${t('wap_com_00048')}?`)) {
        await navigateTo('/com/member-right')
      }
      return
    }
    const hard = (chk.gaps || []).filter((g) => g.key !== 'member_com_00695')
    if (hard[0]?.href) {
      msg.value = t(hard[0].key)
      await navigateTo(hard[0].href)
      return
    }
    await navigateTo('/com/jobs/new')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function refreshJobs() {
  msg.value = ''
  try {
    const pack = await api.post<{ jobs?: { list?: Array<{ id: number }> } }>('/v1/mcenter/jobs/overview', {
      page: 1,
      page_size: 50,
      w: 1,
    })
    const ids = (pack.jobs?.list || []).map((j) => j.id).filter((id) => Number(id) > 0)
    if (!ids.length) {
      msg.value = t('wap_com_00211')
      return
    }
    await api.post('/v1/mcenter/jobs/batch/refresh', { ids })
    msg.value = t('common.success')
  } catch (e: unknown) {
    if (isQuotaErr(e)) {
      msg.value = e instanceof ApiError ? e.message : t('wap_com_00048')
      if (import.meta.client && window.confirm(`${msg.value} ${t('wap_com_00048')}?`)) {
        await navigateTo('/com/member-right')
      }
      return
    }
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

const h5Task = computed(() =>
  [
    { to: '/com/profile', icon: '/legacy/h5/images/enterprise_data.png', key: 'wap_com_00096', hint: '', last: false },
    { to: '/com/pay', icon: '/legacy/h5/images/caiwuegl.png', key: 'wap_user_00213', hint: '', last: false },
    {
      to: '/com/otherservice',
      icon: '/legacy/h5/images/resume_index.png',
      key: 'wap_user_00196',
      hint: t('wap_com_00089'),
      last: false,
    },
    { to: '/com/set', icon: '/legacy/h5/images/sz.png', key: 'wap_user_00214', hint: '', last: true },
  ].filter((item) => isMemberModuleOn(settings.value, item.to)),
)
function labelOf(to: string, key?: string) {
  return comItems.value.find((i) => i.to === to)?.label || (key ? t(key) : t('common.more'))
}
</script>

<template>
  <section v-if="!data?.uid" class="site-inner">
    <h1>{{ $t('member_com_00290') }}</h1>
    <p class="muted">{{ $t('common_01153') }}</p>
    <NuxtLink to="/login">{{ $t('ui.go_login') }}</NuxtLink>
  </section>
  <template v-else>
    <div class="memberSubRight site-pc">
      <div class="membRighTops">
        <ul>
          <li class="membRighTops_mr">
            <NuxtLink to="/com/applications" class="membRiTopText">
              <div class="membRiTopInfo">
                <span>{{ $t('member_com_00152') }}</span>
              </div>
              <div class="membRiTopNum">
                <span>{{ dash?.applies_received ?? 0 }}</span>
              </div>
              <div v-if="dash?.applies_unread" class="membRiTopInx">
                <span>{{ $t('member_com_00130') }}</span>
                <span>{{ dash.applies_unread }}</span>
                <span>{{ $t('common_02153') }}></span>
              </div>
            </NuxtLink>
            <div class="membRiTopImg">
              <img src="/legacy/member/com/memimg1.png" alt="" />
            </div>
          </li>
          <li class="membRighTops_mr">
            <NuxtLink to="/com/fans" class="membRiTopText">
              <div class="membRiTopInfo">
                <span>{{ $t('wap_com_00407') }}</span>
              </div>
              <div class="membRiTopNum">
                <span>{{ fans?.total ?? 0 }}</span>
              </div>
              <div class="membRiTopInx">
                <span>{{ $t('member_com_00124') }}</span>
                <span>{{ $t('member_com_00380') }}></span>
              </div>
            </NuxtLink>
            <div class="membRiTopImg">
              <img src="/legacy/member/com/memimg3.png" alt="" />
            </div>
          </li>
          <li>
            <NuxtLink to="/com/looks" class="membRiTopText">
              <div class="membRiTopInfo">
                <span>{{ $t('wap_user_00276') }}</span>
              </div>
              <div class="membRiTopNum">
                <span>{{ looks?.total ?? 0 }}</span>
              </div>
              <div class="membRiTopInx">
                <span>{{ $t('member_com_00131') }}</span>
                <span>{{ $t('member_com_00380') }}></span>
              </div>
            </NuxtLink>
            <div class="membRiTopImg">
              <img src="/legacy/member/com/memimg4.png" alt="" />
            </div>
          </li>
        </ul>
      </div>
      <div class="memberSubGuanl">
        <div class="memberSubzaopi">
          <div class="membSubGuanTite">
            <span>{{ $t('member_com_00150') }}</span>
            <NuxtLink v-if="buyAddedOn" to="/com/added">{{ $t('wap_com_00393') }} ></NuxtLink>
          </div>
          <div class="membSubGuaTwo">
            <ul>
              <li v-for="tile in quotaTiles" :key="tile.title">
                <div class="twoDivimg">
                  <img src="/legacy/member/com/tuayuan.png" alt="" />
                </div>
                <div class="twoDivTite"><span>{{ tile.title }}</span></div>
                <div class="twoDivNum">
                  <span>{{ tile.num }}</span>
                  <b>{{ tile.unit }}</b>
                </div>
              </li>
            </ul>
          </div>
        </div>
      </div>
      <div v-if="ads530.length" class="memberSubBanner">
        <a
          v-for="(ad, i) in ads530"
          :key="'530-' + i"
          :href="safeHref(ad.link)"
          :target="safeHref(ad.link) ? '_blank' : undefined"
        >
          <img :src="ad.image_n || ad.image" :alt="ad.title || ''" />
        </a>
      </div>
      <div class="membSubPoster">
        <div v-if="notices.length" class="yun_Announcement">
          <ul class="tiplist">
            <li v-for="(n, i) in notices" :key="'n-' + i" :class="n.cls">
              {{ $t('wap_user_00205') }}：<span>{{ n.text }}</span>
              <span v-if="n.to" class="tiplist_bth">
                <NuxtLink :to="n.to">{{ n.action }}></NuxtLink>
              </span>
            </li>
          </ul>
        </div>
        <div class="membSuosTite">
          <span>{{ $t('default_00196') }}</span>
          <NuxtLink to="/resumes">{{ $t('common.more') }}></NuxtLink>
        </div>
        <div v-if="recList.length" class="membSubposCont">
          <ul>
            <li v-for="row in recList" :key="row.uid">
              <div class="postdivTite">
                <NuxtLink :to="`/resumes/${row.uid}`">{{ row.display_name || row.uid }}</NuxtLink>
                <b>{{ recMeta(row) }}</b>
              </div>
              <div class="postdivInfo">
                <div class="postdivChatrs">
                  <NuxtLink :to="`/resumes/${row.uid}`" class="spanDown">{{ $t('wap_com_00316') }}</NuxtLink>
                </div>
              </div>
            </li>
          </ul>
        </div>
        <div v-else class="com_msg_no">
          <p class="com_msg_no_name">{{ $t('member_com_00121') }}</p>
          <NuxtLink to="/resumes" class="com_msg_no_bth com_submit">{{ $t('member_com_00138') }}</NuxtLink>
        </div>
      </div>
    </div>
    <MemberComHomeAside
      :profile="profile"
      :vip="vip"
      :vip-live="vipLive"
      :vip-opened="vipOpened"
      :vip-range="vipRange"
      :integral="integral"
      :price-name="priceName"
      :poster-on="posterOn"
      :gzh-need="gzhNeed"
      :wx-qr="wxQr"
      :webtel="webtel"
      :webmoblie="webmoblie"
      :guwen-pic="guwenPic"
      :kfqq="kfqq"
      :ads="ads511"
      :year-on="yearOn"
      :msg="msg"
      @publish="publishJob"
      @refresh="refreshJobs"
      @year="yearOpen = true"
      @kefu-qr="kefuQrOpen = true"
    />
      <div class="site-h5 member-page member-page-com">
      <div v-if="notices.length" class="comzhtip" style="height: auto">
        <div class="comzhtip_tit">{{ $t('wap_user_00205') }}</div>
        <div v-for="(n, i) in notices" :key="'h5n-' + i">
          <div class="comzhtip_p1">{{ n.text }}</div>
          <NuxtLink v-if="n.to" :to="n.to" class="comzhtip_p2">{{ n.action }}</NuxtLink>
          <div v-else-if="webtel" class="comzhtip_p2">{{ webtel }}</div>
        </div>
      </div>
      <div class="commemberheaderbg">
        <div class="commemberheader commemberTops">
          <div class="compauNamImgs">
            <img
              :src="signSt?.signed_today ? '/legacy/h5/images/comtop2.png' : '/legacy/h5/images/comtop22.png'"
              alt=""
              @click="signSt?.signed_today ? undefined : sign()"
            />
            <NuxtLink to="/com/set">
              <img src="/legacy/h5/images/comtop4.png" alt="" />
            </NuxtLink>
          </div>
          <div class="company">
            <div class="company_left">
              <img v-if="profile?.logo" :src="mediaUrl(profile.logo)" alt="" width="100%" height="100%" />
            </div>
            <div class="company_center">
              <div class="compauNamers">
                <div class="company_center_top">{{ profile?.name || data?.username || data?.uid }}</div>
              </div>
              <NuxtLink :to="`/companies/${profile?.uid || data?.uid}`" class="company_center_bto">
                <i class="company_center_bto_name">{{ $t('wap_com_00095') }}</i>
                <img src="/legacy/h5/images/comtop1.png" class="company_center_bto_nav" alt="" />
              </NuxtLink>
            </div>
          </div>
          <div class="particulars_new">
            <ul>
              <li>
                <NuxtLink to="/com/applications">
                  <i class="particulars_number">{{ dash?.applies_received ?? 0 }}</i>
                  <i class="particulars_word">{{ $t('wap_00794') }}</i>
                </NuxtLink>
              </li>
              <li>
                <NuxtLink to="/com/interviews">
                  <i class="particulars_number">{{ dash?.interviews_sent ?? today?.invite?.num ?? 0 }}</i>
                  <i class="particulars_word">{{ $t('wap_com_00046') }}</i>
                </NuxtLink>
              </li>
              <li>
                <NuxtLink to="/com/looks">
                  <i class="particulars_number">{{ looks?.total ?? today?.look_job?.num ?? 0 }}</i>
                  <i class="particulars_word">{{ $t('wap_com_00112') }}</i>
                </NuxtLink>
              </li>
              <li>
                <NuxtLink to="/com/jobs">
                  <i class="particulars_number">{{ counts?.online ?? 0 }}</i>
                  <i class="particulars_word">{{ $t('wap_com_00243') }}</i>
                </NuxtLink>
              </li>
            </ul>
          </div>
        </div>
        <div class="comvipDaoBnagc">
          <div v-if="!vipSjtip0" class="comvip_nav comvipDaohang">
            <div class="comvip_nav_left">
              <div class="comvipDaoText">
                <img src="/legacy/h5/images/comtop5.png" alt="" />
                <span class="spancomva1">{{ $t('wap_com_00087') }}</span>
              </div>
            </div>
            <NuxtLink to="/com/member-right" class="comvip_nav_right">{{ $t('wap_com_00108') }}</NuxtLink>
          </div>
          <div v-else class="comvip_nav comvipDaohang">
            <div class="comvip_nav_left">
              <div class="comvipDaoText">
                <img src="/legacy/h5/images/comtop5.png" alt="" />
                <span class="spancomva1">{{ vip?.rating_name }}</span>
                <span class="spandates">{{ formatUnixDate(vipExpire) }}</span>
              </div>
              <div class="comvipDaoTips">
                <span>{{ $t('wap_com_00083') }}</span>
              </div>
            </div>
            <NuxtLink to="/com/member-right" class="comvip_nav_right">{{ $t('wap_com_00098') }}</NuxtLink>
          </div>
          <div class="comvipDaoBorder">
            <div class="comvipDaoOline">
              <div class="comvipDaineTite">
                <b>{{ counts?.online ?? 0 }}</b>
                <span>{{ $t('wap_com_00094') }}</span>
              </div>
              <div class="comvipDaineLink">
                <NuxtLink v-if="counts?.online" to="/com/jobs">
                  <span>{{ $t('wap_com_00109') }}</span>
                </NuxtLink>
                <a v-else href="javascript:;" @click.prevent="publishJob">
                  <span>{{ $t('wap_com_00047') }}</span>
                </a>
              </div>
            </div>
            <div class="Member_Center" style="padding-top: 0.42rem">
              <ul>
                <li>
                  <NuxtLink to="/com/jobs">
                    <div class="Member_Center_img">
                      <img src="/legacy/h5/images/yun_cy_icon6.png" alt="" width="100%" height="100%" />
                    </div>
                    <i class="Member_Center_word">{{ $t('wap_com_00106') }}</i>
                  </NuxtLink>
                </li>
                <li>
                  <NuxtLink to="/com/applications">
                    <div class="Member_Center_img">
                      <img src="/legacy/h5/images/yun_cy_icon5.png" alt="" width="100%" height="100%" />
                    </div>
                    <i class="Member_Center_word">{{ $t('wap_com_00105') }}</i>
                  </NuxtLink>
                </li>
                <li>
                  <NuxtLink to="/com/stats">
                    <div class="Member_Center_img">
                      <img src="/legacy/h5/images/yun_cy_icon3.png" alt="" width="100%" height="100%" />
                    </div>
                    <i class="Member_Center_word">{{ $t('wap_com_00103') }}</i>
                  </NuxtLink>
                </li>
                <li>
                  <NuxtLink to="/com/member-right">
                    <div class="Member_Center_img">
                      <img src="/legacy/h5/images/yun_cy_icon4.png" alt="" width="100%" height="100%" />
                    </div>
                    <i class="Member_Center_word">{{ $t('wap_com_00097') }}</i>
                  </NuxtLink>
                </li>
              </ul>
            </div>
          </div>
        </div>
      </div>
      <div v-if="yearOn" class="djck" @click="yearOpen = true">
        <i class="djck_icon" />
        {{ $t('wap_com_00085') }}
        <span class="djck_bth">{{ $t('wap_00802') }}</span>
      </div>
      <p v-if="msg" class="muted">{{ msg }}</p>
      <div class="taskbar_box">
        <NuxtLink
          v-for="item in h5Task"
          :key="item.to"
          :to="item.to"
          :class="item.last ? 'taskbar_enterprise_last' : 'taskbar_enterprise'"
        >
          <div class="taskbar_datum">
            <div class="taskbar_datum_img">
              <img :src="item.icon" alt="" width="100%" height="100%" />
            </div>
            <div class="taskbar_datum_word">{{ labelOf(item.to, item.key) }}</div>
          </div>
          <div class="taskbar_nav">
            <div v-if="item.hint" class="taskbar_nav_word">{{ item.hint }}</div>
            <div class="taskbar_nav_img">
              <img src="/legacy/h5/images/my_more.png" alt="" width="100%" height="100%" />
            </div>
          </div>
        </NuxtLink>
      </div>
    </div>
    <Teleport to="body">
      <div v-if="yearOpen && year" class="member-year-mask" @click.self="yearOpen = false">
        <div class="member-year-box">
          <h2>{{ $t('admin_yunying_00167') }}</h2>
          <p v-if="year.company_name" class="muted">{{ year.company_name }}</p>
          <ul>
            <li v-for="row in yearRows" :key="row.k">
              <span>{{ row.k }}</span>
              <b>{{ row.v }}</b>
            </li>
          </ul>
          <button type="button" class="btn_01 member-year-close" @click="yearOpen = false">{{ $t('common.close') }}</button>
        </div>
      </div>
      <div v-if="kefuQrOpen && wxQr" class="member-year-mask" @click.self="kefuQrOpen = false">
        <div class="yun_wxbd_box" @click.stop>
          <div class="yun_wxbd_tit">{{ $t('member_com_00125') }}</div>
          <div class="yun_wxbd_img_c">
            <div class="yun_wxbd_img">
              <img :src="wxQr" width="180" height="180" alt="" />
            </div>
          </div>
          <div class="yun_wxbd_p">{{ $t('wap_00214') }}</div>
          <a href="javascript:;" @click.prevent="kefuQrOpen = false">{{ $t('common.close') }}</a>
        </div>
      </div>
    </Teleport>
  </template>
</template>
