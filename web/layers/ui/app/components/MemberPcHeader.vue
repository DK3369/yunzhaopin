<template>
  <div v-if="kind === 'user'" class="site-pc user_header member-pc-header">
    <div class="w1200 member-pc-header__inner">
      <div class="user_headerlogo">
        <NuxtLink to="/" :title="$t('member_user_00116')">
          <img v-if="userLogo" :src="userLogo" :alt="siteName" />
        </NuxtLink>
      </div>
      <div class="user_headerright member-pc-header__right">
        <span v-if="phone" class="user_headertel">{{ $t('common_02162') }}：<span class="user_headertel_n">{{ phone }}</span></span>
        <div class="yun_m_indexinfo_user_qd" :class="{ 'is-signed': signSt?.signed_today }">
          <a
            href="javascript:;"
            class="yun_m_indexinfo_user_qd_a"
            :class="{ yqd: signSt?.signed_today }"
            @click.prevent="sign"
          >{{ signSt?.signed_today ? $t('wap_01022') : $t('wap_01023') }}</a>
        </div>
          <div class="yun_m_headertx" @mouseenter="userInfoOpen = true; ensureBal()" @mouseleave="userInfoOpen = false">
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
        <div class="yun_m_headermsg" @mouseenter="userMsgOpen = true" @mouseleave="userMsgOpen = false">
          <i class="yun_m_headermsg_icon" />
          <span>{{ $t('member_user_00498') }}</span>
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
        <LangSwitch />
        <NuxtLink to="/" class="user_m_fanh" :title="$t('member_user_00116')">{{ $t('member_user_00119') }}</NuxtLink>
      </div>
    </div>
  </div>
  <div v-else class="site-pc header member-pc-header member-pc-header--com">
    <div class="header_fixed">
      <div class="w1200 member-pc-header__inner">
        <div class="header-logo">
          <NuxtLink to="/">
            <img v-if="comLogo" :src="comLogo" class="png" alt="" />
          </NuxtLink>
        </div>
        <div class="user_headerright member-pc-header__right">
          <span v-if="comTel" class="user_headertel">{{ $t('common_02149') }}：<span class="user_headertel_n">{{ comTel }}</span></span>
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
          <div class="yun_m_headermsg" @mouseenter="comMsgOpen = true" @mouseleave="comMsgOpen = false">
            <i class="yun_m_headermsg_icon" />
            <span>{{ $t('member_user_00498') }}</span>
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
          <LangSwitch />
          <NuxtLink to="/" class="user_m_fanh" :title="$t('member_user_00116')">{{ $t('member_user_00119') }}</NuxtLink>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { mediaUrl } from '../utils/site'

const api = useApi()
const { siteName, logoPc, phone, settings, me, logout, memberKind } = useSiteChrome()
const kind = computed(() => memberKind.value)
const userMsgOpen = ref(false)
const userInfoOpen = ref(false)
const comMsgOpen = ref(false)
const comInfoOpen = ref(false)

const { data: userDash } = useAsyncData(
  () => (kind.value === 'user' ? 'user-dash' : 'hdr-skip-user-dash'),
  () =>
    kind.value === 'user'
      ? api
          .post<{ wkyqnum?: number; sysnum?: number; commsgnum?: number }>('/v1/mcenter/dashboard', {})
          .catch(() => null)
      : Promise.resolve(null),
  reuseAsyncCache(),
)
const { data: userResume } = useAsyncData(
  () => (kind.value === 'user' ? 'user-home-resume' : 'hdr-skip-user-resume'),
  () =>
    kind.value === 'user'
      ? api.post<{ name?: string; photo?: string }>('/v1/mcenter/resume/list', {}).catch(() => null)
      : Promise.resolve(null),
  reuseAsyncCache(),
)
const bal = ref<{ balance?: number } | null>(null)
let balLoading = false
async function ensureBal() {
  if (kind.value !== 'user' || bal.value != null || balLoading) return
  balLoading = true
  try {
    bal.value = await api.post<{ balance?: number }>('/v1/mcenter/integral/balance', {}).catch(() => null)
  } finally {
    balLoading = false
  }
}
const { data: signSt, refresh: refreshSign } = useAsyncData(
  () => (kind.value === 'user' ? 'user-home-sign' : 'hdr-skip-sign'),
  () =>
    kind.value === 'user'
      ? api.post<{ signed_today?: boolean }>('/v1/mcenter/sign/status', {}).catch(() => null)
      : Promise.resolve(null),
  reuseAsyncCache(),
)
const { data: comDash } = useAsyncData(
  () => (kind.value === 'com' ? 'com-dash' : 'hdr-skip-com-dash'),
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
  reuseAsyncCache(),
)
const { data: comProfile } = useAsyncData(
  () => (kind.value === 'com' ? 'com-home-profile' : 'hdr-skip-com-profile'),
  () =>
    kind.value === 'com'
      ? api.post<{ name?: string; logo?: string }>('/v1/mcenter/company/list', {}).catch(() => null)
      : Promise.resolve(null),
  reuseAsyncCache(),
)

const userLogo = computed(() => logoPc.value || mediaUrl(settings.value.sy_member_logo))
const comLogo = computed(() => logoPc.value || mediaUrl(settings.value.sy_unit_logo))
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
