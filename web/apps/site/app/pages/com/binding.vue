<script setup lang="ts">
import { isUnauthErr, OAUTH_FRONT_PROVIDERS, oauthEnabledByAdmin } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { settings } = useSiteChrome()
const { data: dicts } = await usePublicDicts()
const { data, error, refresh } = await useAsyncData('com-oauth-bindings', () =>
  api.post<{ providers?: string[] }>('/v1/mcenter/oauth-bindings', {}),
)
const { data: me, refresh: refreshMe } = await useAsyncData('com-me-bind', () =>
  api.post<{ moblie?: string | null; email?: string | null }>('/v1/wap/me', {}).catch(() => null),
)
const mobile = ref(String(me.value?.moblie || ''))
const mobileCode = ref('')
const email = ref(String(me.value?.email || ''))
const msg = ref('')
function maskPhone(s: string) {
  const v = s.trim()
  if (v.length < 7) return v
  return `${v.slice(0, 3)}****${v.slice(-4)}`
}
function maskEmail(s: string) {
  const i = s.indexOf('@')
  if (i <= 1) return s
  return `${s[0]}***${s.slice(i)}`
}
const oauth = ref<Array<{ name: string; path: string; provider: string }>>([])
const bound = computed(() => new Set((data.value?.providers || []).map((p) => String(p).toLowerCase())))
const siteUrl = String(useRuntimeConfig().public.siteUrl || '').replace(/\/$/, '')

async function loadOauth() {
  const redirect_uri = `${siteUrl}/login`
  oauth.value = []
  for (const item of OAUTH_FRONT_PROVIDERS) {
    if (bound.value.has(item.key)) continue
    if (!oauthEnabledByAdmin(settings.value, item, dicts.value)) continue
    try {
      const r = await api.post<{ authorize_url?: string }>(item.path, { redirect_uri })
      if (r.authorize_url) oauth.value.push({ name: item.name, path: r.authorize_url, provider: item.key })
    } catch {
      /* not configured */
    }
  }
}
onMounted(loadOauth)
watch(bound, loadOauth)

function startBind(o: { path: string; provider: string }) {
  sessionStorage.setItem('oauth_provider', o.provider)
  sessionStorage.setItem('oauth_intent', 'bind')
  sessionStorage.setItem('oauth_bind_next', '/com/binding')
  window.location.href = o.path
}

function fail(e: unknown) {
  return e instanceof Error ? e.message : t('ui.failed')
}

async function unbind(provider: string) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/oauth-bindings/unbind', { provider })
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function sendMobile() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/cert/mobile/send', { moblie: mobile.value })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function bindMobile() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/cert/mobile/verify', { moblie: mobile.value, moblie_code: mobileCode.value })
    msg.value = t('common.success')
    await refreshMe()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function sendEmail() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/cert/email/send', { email: email.value })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}

function iconClass(provider: string) {
  const p = provider.toLowerCase()
  if (p.includes('qq')) return { box: 'Bingding_icon', i: 'binding_qq_icon', h5: 'bingding_box_iconqq', h5bg: 'bingding_yx_qq' }
  if (p.includes('sina') || p.includes('weibo')) return { box: 'Bingding_sinaicon', i: 'binding_xl_icon', h5: 'bingding_box_iconxl', h5bg: 'bingding_yx_xl' }
  if (p.includes('wx') || p.includes('weixin') || p.includes('wechat')) return { box: 'Bingding_icon', i: 'binding_wx_icon', h5: 'bingding_box_iconwx', h5bg: 'bingding_yx_g' }
  return { box: 'Bingding_icon', i: 'binding_qq_icon', h5: 'bingding_box_iconqq', h5bg: '' }
}

useSeoMeta({ title: t('member_user_00059') })
const boundList = computed(() => data.value?.providers || [])
</script>

<template>
  <MemberPanel :title="$t('member_user_00059')">
    <p v-if="error" class="muted">
      {{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}
    </p>
    <template v-else>
      <nav class="job_list_tit">
        <ul>
          <li><NuxtLink to="/com/profile">{{ $t('wap_user_00341') }}</NuxtLink></li>
          <li><NuxtLink to="/com/cert">{{ $t('wap_com_00075') }}</NuxtLink></li>
          <li><NuxtLink to="/com/password">{{ $t('member_com_00070') }}</NuxtLink></li>
          <li class="job_list_tit_cur"><a href="javascript:;">{{ $t('member_user_00059') }}</a></li>
        </ul>
      </nav>
      <div class="resume_Prompt_box">
        <div class="resume_Prompt"><i class="resume_Prompt_icon" />{{ $t('member_user_00474') }}</div>
      </div>
      <div class="site-pc">
        <div v-for="p in boundList" :key="p" class="Binding_list">
          <div class="Binding_list_left">
            <div :class="[iconClass(p).box, 'Bingding_icon_cur']"><i :class="iconClass(p).i" /></div>
            <span class="bingding_yx_wr">{{ p }}</span>
          </div>
          <div class="Binding_list_text Binding_list_text_mt">{{ $t('wap_user_00127') }}</div>
          <div class="Binding_oper">
            <a href="javascript:;" class="Binding_submit_qx" @click="unbind(p)">{{ $t('member_user_00054') }}</a>
          </div>
        </div>
        <div v-for="o in oauth" :key="o.provider" class="Binding_list">
          <div class="Binding_list_left">
            <div :class="iconClass(o.provider).box"><i :class="iconClass(o.provider).i" /></div>
            <span class="bingding_yx_wr">{{ o.name }}</span>
          </div>
          <div class="Binding_list_text">
            <span class="Binding_list_text_zt"><i class="Binding_list_text_zticon" />{{ $t('wap_user_00181') }}</span>
          </div>
          <div class="Binding_oper">
            <a href="javascript:;" class="Binding_submit" @click.prevent="startBind(o)">{{ $t('wap_user_00119') }}</a>
          </div>
        </div>
      </div>
      <ul class="site-h5 bingding_box">
        <li v-for="p in boundList" :key="'h5b-' + p">
          <div class="bingding_box_iconbg" :class="iconClass(p).h5bg"><i :class="iconClass(p).h5" /></div>
          <div class="bingding_box_name">{{ p }}</div>
          <div class="bingding_box_p">{{ $t('wap_user_00127') }}</div>
          <span class="bingding_box_bth_jc" @click="unbind(p)">{{ $t('wap_user_00138') }}</span>
        </li>
        <li v-for="o in oauth" :key="'h5o-' + o.provider">
          <div class="bingding_box_iconbg"><i :class="iconClass(o.provider).h5" /></div>
          <div class="bingding_box_name">{{ o.name }}</div>
          <div class="bingding_box_p">{{ $t('wap_user_00135') }}</div>
          <span class="bingding_box_bth" @click="startBind(o)">{{ $t('wap_user_00119') }}</span>
        </li>
      </ul>
      <form class="form verification_form" @submit.prevent="bindMobile">
        <MemberField :label="$t('common.phone')"><input v-model="mobile" /></MemberField>
        <button type="button" class="verification_form_btn" @click="sendMobile">{{ $t('common.submit') }}</button>
        <MemberField :label="$t('wap_01371')"><input v-model="mobileCode" /></MemberField>
        <button type="submit" class="verification_form_btn">{{ $t('common.save') }}</button>
      </form>
      <form class="form verification_form" @submit.prevent="sendEmail">
        <MemberField :label="$t('member_user_00282')"><input v-model="email" /></MemberField>
        <button type="submit" class="verification_form_btn">{{ $t('common.submit') }}</button>
      </form>
      <p class="muted">{{ $t('ajax_00001') }}</p>
      <p><NuxtLink to="/email-verify">{{ $t('wap_user_00179') }}</NuxtLink></p>
      <p v-if="msg">{{ msg }}</p>
    </template>
  </MemberPanel>
</template>
