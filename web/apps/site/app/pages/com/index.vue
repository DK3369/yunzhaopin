<script setup lang="ts">
import { mediaUrl, isMemberModuleOn } from '~/utils/site'
const api = useApi()
const { t } = useI18n()
const { comItems } = useMemberNav()
const { data } = await useAuthMe()
const { data: profile } = await useAsyncData(
  'com-home-profile',
  () =>
    api
      .post<{ name?: string; logo?: string; uid?: number; r_status?: number }>('/v1/mcenter/company/list', {})
      .catch(() => null),
  reuseAsyncCache(),
)
const { data: dash } = await useAsyncData(
  'com-dash',
  () =>
    api
      .post<{
        applies_received: number
        applies_unread: number
        interviews_sent?: number
        resume_downloads?: number
        job_msg_unanswered?: number
        unread_messages?: number
      }>('/v1/mcenter/com-dashboard', {})
      .catch(() => null),
  reuseAsyncCache(),
)
const { data: counts } = await useAsyncData('com-home-job-counts', () =>
  api
    .post<{
      total: number
      online: number
      breakjob_num?: number
      top_num?: number
      rec_num?: number
      urgent_num?: number
    }>('/v1/mcenter/jobs/counts', {})
    .catch(() => null),
)
const { data: today } = await useAsyncData('com-home-today', () =>
  api
    .post<{
      look_job?: { num?: number }
      apply?: { num?: number }
      invite?: { num?: number }
    }>('/v1/mcenter/com-stats/today', {})
    .catch(() => null),
)
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
const gzhOpen = ref(true)
const { wxQr, settings } = useSiteChrome()
const webtel = computed(() => String(settings.value.sy_comwebtel || settings.value.sy_freewebtel || ''))
const msg = ref('')
useSeoMeta({ title: t('member_com_00290') })
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
  <div v-else class="member-page member-page-com">
    <div class="site-pc">
      <p v-if="gzhNeed" class="muted" style="padding: 8px 0">
        {{ $t('common_00655') }}
        <img v-if="wxQr" :src="wxQr" alt="" width="80" height="80" />
      </p>
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
                <span>{{ dash.applies_unread }}</span>
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
            <NuxtLink to="/com/added">{{ $t('wap_com_00393') }} ></NuxtLink>
          </div>
          <div class="membSubGuaTwo">
            <ul>
              <li>
                <div class="twoDivimg">
                  <img src="/legacy/member/com/tuayuan.png" alt="" />
                </div>
                <div class="twoDivTite"><span>{{ $t('member_com_00134') }}</span></div>
                <div class="twoDivNum">
                  <span>{{ counts?.online ?? 0 }}</span>
                  <b>{{ $t('common_02052') }}</b>
                </div>
              </li>
              <li>
                <div class="twoDivimg">
                  <img src="/legacy/member/com/tuayuan.png" alt="" />
                </div>
                <div class="twoDivTite"><span>{{ $t('member_com_00136') }}</span></div>
                <div class="twoDivNum">
                  <span>{{ counts?.breakjob_num ?? 0 }}</span>
                  <b>{{ $t('common_02088') }}</b>
                </div>
              </li>
              <li>
                <div class="twoDivimg">
                  <img src="/legacy/member/com/tuayuan.png" alt="" />
                </div>
                <div class="twoDivTite"><span>{{ $t('member_com_00137') }}</span></div>
                <div class="twoDivNum">
                  <span>{{ dash?.interviews_sent ?? 0 }}</span>
                  <b>{{ $t('common_02088') }}</b>
                </div>
              </li>
              <li>
                <div class="twoDivimg">
                  <img src="/legacy/member/com/tuayuan.png" alt="" />
                </div>
                <div class="twoDivTite"><span>{{ $t('member_com_00135') }}</span></div>
                <div class="twoDivNum">
                  <span>{{ dash?.resume_downloads ?? 0 }}</span>
                  <b>{{ $t('common_02088') }}</b>
                </div>
              </li>
              <li>
                <div class="twoDivimg">
                  <img src="/legacy/member/com/tuayuan.png" alt="" />
                </div>
                <div class="twoDivTite"><span>{{ $t('wap_user_00209') }}</span></div>
                <div class="twoDivNum">
                  <span>{{ counts?.top_num ?? 0 }}</span>
                  <b>{{ $t('common_02067') }}</b>
                </div>
              </li>
              <li>
                <div class="twoDivimg">
                  <img src="/legacy/member/com/tuayuan.png" alt="" />
                </div>
                <div class="twoDivTite"><span>{{ $t('wap_com_00041') }}</span></div>
                <div class="twoDivNum">
                  <span>{{ counts?.rec_num ?? 0 }}</span>
                  <b>{{ $t('common_02067') }}</b>
                </div>
              </li>
              <li>
                <div class="twoDivimg">
                  <img src="/legacy/member/com/tuayuan.png" alt="" />
                </div>
                <div class="twoDivTite"><span>{{ $t('wap_com_00043') }}</span></div>
                <div class="twoDivNum">
                  <span>{{ counts?.urgent_num ?? 0 }}</span>
                  <b>{{ $t('common_02067') }}</b>
                </div>
              </li>
            </ul>
          </div>
        </div>
      </div>
    </div>
    <div class="site-h5">
      <div v-if="Number(profile?.r_status) !== 1" class="comzhtip">
        <div class="comzhtip_tit">{{ $t('wap_user_00205') }}</div>
        <div v-if="Number(profile?.r_status) === 0" class="comzhtip_p1">{{ $t('wap_com_00080') }}</div>
        <div v-else-if="Number(profile?.r_status) === 3" class="comzhtip_p1">{{ $t('wap_user_00167') }}</div>
        <div v-else class="comzhtip_p1">{{ $t('wap_com_00080') }}</div>
        <div v-if="webtel" class="comzhtip_p2">{{ webtel }}</div>
      </div>
      <div v-if="gzhNeed && gzhOpen" class="member-gzh-mask" @click="gzhOpen = false">
        <div class="gzh_gzbox" @click.stop>
          <div class="gzh_gzbox_n">{{ $t('wap_user_00191') }}</div>
          <img v-if="wxQr" :src="wxQr" alt="" />
          <div class="gzh_gzbox_p">{{ $t('wap_user_00188') }}</div>
          <div class="gzh_gzbox_p">{{ $t('wap_user_00185') }}</div>
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
                <i class="particulars_number">{{ counts?.online ?? 0 }}</i>
                <i class="particulars_word">{{ $t('wap_com_00111') }}</i>
              </li>
            </ul>
          </div>
        </div>
        <div class="comvipDaoBnagc">
          <div class="comvip_nav comvipDaohang">
            <div class="comvip_nav_left">
              <div class="comvipDaoText">
                <img src="/legacy/h5/images/comtop5.png" alt="" />
                <span class="spancomva1">{{ $t('wap_com_00087') }}</span>
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
                <NuxtLink v-else to="/com/jobs/new">
                  <span>{{ $t('wap_com_00047') }}</span>
                </NuxtLink>
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
      <div class="companyDatapage">
        <div v-if="webtel" class="companyDataTell">{{ webtel }}</div>
      </div>
    </div>
  </div>
</template>
