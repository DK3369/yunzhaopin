<template>
  <div v-if="kind === 'user'" class="site-pc user_header">
    <div class="w1200">
      <div class="user_headerlogo">
        <NuxtLink to="/" :title="$t('member_user_00116')">
          <img v-if="userLogo" :src="userLogo" :alt="siteName" />
        </NuxtLink>
      </div>
      <div class="user_headerright">
        <NuxtLink to="/" class="user_m_fanh" :title="$t('member_user_00116')">{{ $t('member_user_00119') }}</NuxtLink>
        <div class="yun_m_headermsg" @mouseenter="userMsgOpen = true" @mouseleave="userMsgOpen = false">
          <i class="yun_m_headermsg_icon" />{{ $t('member_user_00498') }}
          <span v-if="userMsgTotal" class="yun_m_headermsg_n">{{ userMsgTotal }}</span>
          <div class="yun_m_headermsg_box" :style="{ display: userMsgOpen ? 'block' : 'none' }">
            <div class="yun_m_headermsg_list">
              <NuxtLink to="/user/interviews">
                {{ $t('wap_user_00216') }}
                <em v-if="userDash?.wkyqnum" class="yun_m_headermsg_list_n">{{ userDash.wkyqnum }}</em>
              </NuxtLink>
            </div>
            <div class="yun_m_headermsg_list">
              <NuxtLink to="/user/messages">
                {{ $t('wap_user_00363') }}
                <em v-if="userDash?.sysnum" class="yun_m_headermsg_list_n">{{ userDash.sysnum }}</em>
              </NuxtLink>
            </div>
            <div class="yun_m_headermsg_list">
              <NuxtLink to="/user/consults">
                {{ $t('member_user_00115') }}
                <em v-if="userDash?.commsgnum" class="yun_m_headermsg_list_n">{{ userDash.commsgnum }}</em>
              </NuxtLink>
            </div>
          </div>
        </div>
        <div class="header_m_navright">
          <div class="yun_m_indexinfo_user_qd" @click="sign">
            <i class="yun_m_indexinfo_user_qd_icon" />{{ signSt?.signed_today ? $t('wap_01022') : $t('wap_01023') }}
          </div>
          <div class="yun_m_headertx" @mouseenter="userInfoOpen = true" @mouseleave="userInfoOpen = false">
            <NuxtLink to="/user/resume" class="yun_m_headertxa">
              <img v-if="userPhoto" :src="userPhoto" width="30" height="30" alt="" />
            </NuxtLink>
            <div class="yun_m_headertx_hi">{{ userName }}</div>
            <div class="yun_m_header_info" :style="{ display: userInfoOpen ? 'block' : 'none' }">
              <div class="user_tc_tit">{{ $t('common.mine') }}{{ $t('wap_user_00008') }}</div>
              <div class="user_t_jf">
                <span class="yun_m_index_pay_n">{{ bal?.balance ?? 0 }}</span>
                <NuxtLink to="/user/pay" class="yun_m_index_pay_a">{{ $t('common_01946') }}</NuxtLink>
                <NuxtLink to="/user/integral" class="yun_m_index_pay_a">{{ $t('wap_00712') }}{{ $t('wap_user_00008') }}</NuxtLink>
              </div>
              <div class="user_tc_tit">{{ $t('member_user_00118') }}</div>
              <div class="user_tc_tset">
                <NuxtLink to="/user/password" class="user_set_bth">
                  <i class="user_set_icon user_set_icon1" />{{ $t('wap_user_00214') }}
                </NuxtLink>
                <NuxtLink to="/user/account" class="user_set_bth">
                  <i class="user_set_icon user_set_icon4" />{{ $t('member_user_00505') }}
                </NuxtLink>
              </div>
              <div class="user_tcdl">
                <a href="javascript:;" class="user_tcdlbth" style="border: none" @click.prevent="logout">{{ $t('wap_user_00342') }}</a>
              </div>
            </div>
          </div>
          <span v-if="phone" class="user_headertel">{{ $t('common_02162') }}：<span class="user_headertel_n">{{ phone }}</span></span>
        </div>
      </div>
    </div>
  </div>
  <div v-else class="site-pc header">
    <div class="header_fixed">
      <div class="header-logo fltL">
        <NuxtLink to="/">
          <img v-if="comLogo" :src="comLogo" class="png" alt="" />
        </NuxtLink>
      </div>
      <div class="user_headerright">
        <NuxtLink to="/" class="user_m_fanh" :title="$t('member_user_00116')">{{ $t('member_user_00119') }}</NuxtLink>
        <div class="yun_m_headermsg" @mouseenter="comMsgOpen = true" @mouseleave="comMsgOpen = false">
          <i class="yun_m_headermsg_icon" />{{ $t('member_user_00498') }}
          <span v-if="comMsgTotal" class="yun_m_headermsg_n">{{ comMsgTotal }}</span>
          <div class="yun_m_headermsg_box" :style="{ display: comMsgOpen ? 'block' : 'none' }">
            <div class="yun_m_headermsg_list">
              <NuxtLink to="/com/applications">
                {{ $t('default_00009') }}
                <em v-if="comDash?.applies_unread" class="yun_m_headermsg_list_n">{{ comDash.applies_unread }}</em>
              </NuxtLink>
            </div>
            <div class="yun_m_headermsg_list">
              <NuxtLink to="/com/messages">
                {{ $t('wap_user_00363') }}
                <em v-if="comDash?.unread_messages" class="yun_m_headermsg_list_n">{{ comDash.unread_messages }}</em>
              </NuxtLink>
            </div>
            <div class="yun_m_headermsg_list">
              <NuxtLink to="/com/job-messages">
                {{ $t('wap_com_00408') }}
                <em v-if="comDash?.job_msg_unanswered" class="yun_m_headermsg_list_n">{{ comDash.job_msg_unanswered }}</em>
              </NuxtLink>
            </div>
          </div>
        </div>
        <div class="header_m_navright">
          <div class="yun_m_headertx" @mouseenter="comInfoOpen = true" @mouseleave="comInfoOpen = false">
            <NuxtLink to="/com/profile" class="yun_m_headertxa">
              <img v-if="comPhoto" :src="comPhoto" width="30" height="30" alt="" />
            </NuxtLink>
            <div class="yun_m_headertx_hi">{{ comName }}</div>
            <div class="yun_m_header_info" :style="{ display: comInfoOpen ? 'block' : 'none' }">
              <div class="user_infobox">
                <div class="user_infobox_c">
                  <div class="user_infobox_zg">
                    <span class="user_infobox_zg_n">{{ comName }}</span>
                  </div>
                  <div class="user_infobox_comname">
                    {{ comName }}
                    <NuxtLink to="/com/profile" class="user_infobox_combj">{{ $t('common.edit') }}</NuxtLink>
                  </div>
                </div>
              </div>
              <div class="user_tc_tset">
                <NuxtLink to="/com/password" class="user_set_bth">
                  <i class="user_set_icon user_set_icon2" />{{ $t('member_user_00226') }}
                </NuxtLink>
                <NuxtLink to="/com/binding" class="user_set_bth">
                  <i class="user_set_icon user_set_icon3" />{{ $t('member_com_00093') }}
                </NuxtLink>
              </div>
              <div class="user_tcdl">
                <a href="javascript:;" class="user_tcdlbth" style="border: none" @click.prevent="logout">{{ $t('wap_user_00342') }}</a>
              </div>
            </div>
          </div>
          <span v-if="comTel" class="user_headertel">{{ $t('common_02149') }}：<span class="user_headertel_n">{{ comTel }}</span></span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { mediaUrl } from '../utils/site'

const api = useApi()
const route = useRoute()
const { siteName, logoPc, phone, settings, me, logout } = useSiteChrome()
const kind = computed(() => (route.path.startsWith('/com') ? 'com' : 'user'))
const userMsgOpen = ref(false)
const userInfoOpen = ref(false)
const comMsgOpen = ref(false)
const comInfoOpen = ref(false)

const { data: userDash } = useAsyncData(
  'member-hdr-user-dash',
  () =>
    kind.value === 'user'
      ? api
          .post<{ wkyqnum?: number; sysnum?: number; commsgnum?: number }>('/v1/mcenter/dashboard', {})
          .catch(() => null)
      : Promise.resolve(null),
)
const { data: userResume } = useAsyncData(
  'member-hdr-user-resume',
  () =>
    kind.value === 'user'
      ? api.post<{ name?: string; photo?: string }>('/v1/mcenter/resume/list', {}).catch(() => null)
      : Promise.resolve(null),
)
const { data: bal } = useAsyncData(
  'member-hdr-user-bal',
  () => (kind.value === 'user' ? api.post<{ balance?: number }>('/v1/mcenter/integral/balance', {}).catch(() => null) : Promise.resolve(null)),
)
const { data: signSt, refresh: refreshSign } = useAsyncData(
  'member-hdr-sign',
  () => api.post<{ signed_today?: boolean }>('/v1/mcenter/sign/status', {}).catch(() => null),
)
const { data: comDash } = useAsyncData(
  'member-hdr-com-dash',
  () =>
    kind.value === 'com'
      ? api
          .post<{
            applies_unread?: number
            unread_messages?: number
            job_msg_unanswered?: number
          }>('/v1/mcenter/com-dashboard', {})
          .catch(() => null)
      : Promise.resolve(null),
)
const { data: comProfile } = useAsyncData(
  'member-hdr-com-profile',
  () =>
    kind.value === 'com'
      ? api.post<{ name?: string; logo?: string }>('/v1/mcenter/company/list', {}).catch(() => null)
      : Promise.resolve(null),
)

const userLogo = computed(() => mediaUrl(settings.value.sy_member_logo) || logoPc.value)
const comLogo = computed(() => mediaUrl(settings.value.sy_unit_logo) || logoPc.value)
const userPhoto = computed(() => mediaUrl(userResume.value?.photo))
const userName = computed(() => userResume.value?.name || me.value?.username || '')
const comPhoto = computed(() => mediaUrl(comProfile.value?.logo))
const comName = computed(() => comProfile.value?.name || me.value?.username || '')
const comTel = computed(() => String(settings.value.sy_comwebtel || settings.value.sy_freewebtel || '').trim())
const userMsgTotal = computed(
  () => Number(userDash.value?.wkyqnum || 0) + Number(userDash.value?.sysnum || 0) + Number(userDash.value?.commsgnum || 0),
)
const comMsgTotal = computed(
  () =>
    Number(comDash.value?.applies_unread || 0) +
    Number(comDash.value?.unread_messages || 0) +
    Number(comDash.value?.job_msg_unanswered || 0),
)

async function sign() {
  if (signSt.value?.signed_today) return
  try {
    await api.post('/v1/mcenter/sign', {})
    await refreshSign()
  } catch {
    /* ignore */
  }
}
</script>
