<script setup lang="ts">
import { ApiError } from '~/utils/envelope'
import { qrSvgDataUri } from '~/utils/qr'

const { siteName, logoPc, settings, me, worktime, phone, refreshMe } = useSiteChrome()
const { t } = useI18n()
const api = useApi()
const smsLoginOn = computed(
  () => String(settings.value.sy_msg_isopen) === '1' && String(settings.value.sy_msg_login) === '1',
)
const needImageCaptcha = computed(() => {
  const web = String(settings.value.code_web || '')
  return web.includes('前台登录') || web.includes('wap_js_00062')
})
const panel = ref<'sms' | 'qr' | 'pass'>(smsLoginOn.value ? 'sms' : 'pass')
const role = ref<1 | 2>(1)
const username = ref('')
const password = ref('')
const agreed = ref(false)
const mobile = ref('')
const smsCode = ref('')
const smsWait = ref(0)
const nextFrom = ref('')
const { data: captcha } = await useAsyncData('login-captcha', () =>
  api.post<{ cid: string; image: string }>('/v1/wap/captcha').catch(() => null),
)
const authcode = ref('')
const err = ref('')
const oauth = ref<Array<{ name: string; path: string; provider: string }>>([])
let smsTimer: ReturnType<typeof setInterval> | null = null
const siteUrl = String(useRuntimeConfig().public.siteUrl || '').replace(/\/$/, '')
const wechatOauth = computed(() => oauth.value.find((o) => o.provider === 'wechat'))
const appTicket = ref<{ login_id: string; usertype: number; payload: string; scan_url: string } | null>(null)
const appQrSrc = ref('')
const appQrHint = ref('')
let appPoll: ReturnType<typeof setInterval> | null = null
let appWatchReady = false

function loginNext(): string {
  const q = String(useRoute().query.next || nextFrom.value || '')
  if (q.startsWith('/') && !q.startsWith('//')) return q
  return ''
}

function homeOf(usertype: number) {
  if (usertype === 2) return '/com'
  return '/'
}

function authFail(e: unknown): { key: string; msg: string } {
  if (e instanceof ApiError) return { key: e.key, msg: e.message }
  const ex = e as {
    statusMessage?: string
    data?: { key?: string; msg?: string; statusMessage?: string }
  }
  return {
    key: String(ex.data?.key || ''),
    msg: ex.data?.msg || ex.data?.statusMessage || ex.statusMessage || '',
  }
}

async function handleAuthFail(e: unknown) {
  const f = authFail(e)
  if (f.key === 'locked') {
    await navigateTo('/loginlock')
    return
  }
  if (f.key === 'need_register') {
    await navigateTo({
      path: '/register',
      query: {
        usertype: String(role.value),
        ...(mobile.value ? { moblie: mobile.value } : {}),
      },
    })
    return
  }
  err.value = f.msg || t('common_00888')
}

function goBack() {
  const n = loginNext()
  if (n) return navigateTo(n)
  if (import.meta.client && window.history.length > 1) {
    window.history.back()
    return
  }
  return navigateTo('/')
}

function rememberReferrer() {
  if (useRoute().query.next) return
  if (!import.meta.client) return
  try {
    const ref = document.referrer
    if (!ref) return
    const u = new URL(ref)
    if (u.origin !== window.location.origin) return
    if (!u.pathname || u.pathname === '/login' || u.pathname === '/register') return
    nextFrom.value = `${u.pathname}${u.search}`
  } catch {
    /* ignore */
  }
}

async function loadCaptcha() {
  try {
    captcha.value = await api.post('/v1/wap/captcha')
  } catch {
    captcha.value = null
  }
}

function startSmsWait() {
  smsWait.value = 60
  if (smsTimer) clearInterval(smsTimer)
  smsTimer = setInterval(() => {
    smsWait.value -= 1
    if (smsWait.value <= 0 && smsTimer) {
      clearInterval(smsTimer)
      smsTimer = null
    }
  }, 1000)
}

async function afterLogin(user: { uid: number; usertype: number }) {
  await refreshMe()
  const next = loginNext()
  if (user.usertype === 0) {
    await navigateTo({ path: '/utype', query: next ? { next } : {} })
    return
  }
  if (next) {
    await navigateTo(next)
    return
  }
  await navigateTo(homeOf(user.usertype))
}

onMounted(async () => {
  rememberReferrer()
  const q = useRoute().query
  const code = typeof q.code === 'string' ? q.code : ''
  const state = typeof q.state === 'string' ? q.state : ''
  const oauthIntent = import.meta.client ? sessionStorage.getItem('oauth_intent') || '' : ''
  if (String(q.bind) === '1' && me.value && oauthIntent !== 'bind') {
    await $fetch('/api/auth/logout', { method: 'POST' }).catch(() => undefined)
    await refreshMe()
  } else if (me.value && Number(me.value.usertype) !== 0 && !(code && state)) {
    await afterLogin(me.value)
    return
  }
  if (!smsLoginOn.value) panel.value = 'pass'
  if (code && state) {
    const stored = sessionStorage.getItem('oauth_provider') || ''
    const provider = stored || (typeof q.provider === 'string' ? q.provider : 'wechat')
    const intent = sessionStorage.getItem('oauth_intent') || ''
    const bindNext = sessionStorage.getItem('oauth_bind_next') || '/user/binding'
    try {
      if (intent === 'bind') {
        await $fetch('/api/auth/oauth-bind', { method: 'POST', body: { provider, code, state } })
        sessionStorage.removeItem('oauth_provider')
        sessionStorage.removeItem('oauth_intent')
        sessionStorage.removeItem('oauth_bind_next')
        await navigateTo(bindNext)
        return
      }
      const logged = await $fetch<{ uid: number; usertype: number; need_bind?: boolean; ticket?: string }>(
        '/api/auth/oauth-login',
        { method: 'POST', body: { provider, code, state } },
      )
      sessionStorage.removeItem('oauth_provider')
      if (logged.need_bind && logged.ticket) {
        await navigateTo({ path: '/oauth-bind', query: { ticket: logged.ticket } })
        return
      }
      await afterLogin(logged)
      return
    } catch (e: unknown) {
      sessionStorage.removeItem('oauth_intent')
      sessionStorage.removeItem('oauth_bind_next')
      const ex = e as { data?: { statusMessage?: string }; statusMessage?: string }
      err.value = ex.data?.statusMessage || ex.statusMessage || t('common_00888')
    }
  }
  if (needImageCaptcha.value && !captcha.value) await loadCaptcha()
  const redirect_uri = `${siteUrl}/login`
  for (const [name, path, key] of [
    ['WeChat', '/v1/wap/oauth/wechat/authorize-url', 'wechat'],
    ['QQ', '/v1/wap/oauth/qq/authorize-url', 'qq'],
    ['Weibo', '/v1/wap/oauth/weibo/authorize-url', 'weibo'],
  ] as const) {
    try {
      const r = await api.post<{ authorize_url?: string }>(path, { redirect_uri })
      if (r.authorize_url) oauth.value.push({ name, path: r.authorize_url, provider: key })
    } catch {
      /* not configured */
    }
  }
})

function startOauth(o: { path: string; provider: string }) {
  sessionStorage.setItem('oauth_provider', o.provider)
  window.location.href = o.path
}

async function submitPass() {
  err.value = ''
  if (!agreed.value) {
    err.value = t('wap_00309')
    return
  }
  try {
    if (needImageCaptcha.value && !captcha.value) await loadCaptcha()
    const user = await $fetch<{ uid: number; usertype: number }>('/api/auth/login', {
      method: 'POST',
      body: {
        username: username.value,
        password: password.value,
        authcode: needImageCaptcha.value ? authcode.value : undefined,
        captcha_cid: needImageCaptcha.value ? captcha.value?.cid : undefined,
      },
    })
    await afterLogin(user)
  } catch (e: unknown) {
    await handleAuthFail(e)
    if (needImageCaptcha.value) loadCaptcha()
  }
}

async function sendSms() {
  err.value = ''
  if (smsWait.value > 0) return
  try {
    await api.post('/v1/wap/sms/send', {
      moblie: mobile.value,
      scene: 'login',
      captcha_cid: captcha.value?.cid,
      authcode: authcode.value,
    })
    startSmsWait()
  } catch (e: unknown) {
    await handleAuthFail(e)
    loadCaptcha()
  }
}

async function submitSms() {
  err.value = ''
  if (!agreed.value) {
    err.value = t('wap_00309')
    return
  }
  try {
    const logged = await $fetch<{ uid: number; usertype: number }>('/api/auth/login-sms', {
      method: 'POST',
      body: { moblie: mobile.value, dynamiccode: smsCode.value },
    })
    await afterLogin(logged)
  } catch (e: unknown) {
    await handleAuthFail(e)
  }
}

function stopAppPoll() {
  if (appPoll) {
    clearInterval(appPoll)
    appPoll = null
  }
}

async function startAppQr() {
  stopAppPoll()
  appQrHint.value = ''
  appTicket.value = null
  appQrSrc.value = ''
  err.value = ''
  try {
    const ticket = await $fetch<{
      login_id: string
      usertype: number
      payload: string
      scan_url: string
    }>('/api/auth/login-app-qr', { method: 'POST', body: { usertype: role.value } })
    appTicket.value = ticket
    const content = siteUrl
      ? `${siteUrl}/app-login?login_id=${ticket.login_id}&usertype=${ticket.usertype}`
      : ticket.scan_url || ticket.payload
    appQrSrc.value = qrSvgDataUri(content)
    appPoll = setInterval(async () => {
      if (!appTicket.value?.login_id) return
      try {
        const st = await $fetch<{ status: string; uid?: number; usertype?: number }>(
          '/api/auth/login-app-status',
          { method: 'POST', body: { login_id: appTicket.value.login_id } },
        )
        if (st.status === 'ok' && st.uid) {
          stopAppPoll()
          await afterLogin({ uid: st.uid, usertype: Number(st.usertype || role.value) })
        } else if (st.status === 'mismatch') {
          appQrHint.value = t('loginPage.qr_mismatch')
        }
      } catch (e: unknown) {
        const f = authFail(e)
        if (f.key === 'applogin_expired') {
          stopAppPoll()
          appQrHint.value = f.msg || t('loginPage.qr_expired')
        }
      }
    }, 2000)
  } catch (e: unknown) {
    await handleAuthFail(e)
  }
}

function openPanel(next: 'sms' | 'qr' | 'pass') {
  panel.value = next
  if (next === 'qr') startAppQr()
  else stopAppPoll()
}

watch(role, () => {
  if (!appWatchReady) return
  if (panel.value === 'qr') startAppQr()
})

useSeoMeta({ title: t('common.login') })
onMounted(() => {
  appWatchReady = true
})
onUnmounted(() => {
  if (smsTimer) clearInterval(smsTimer)
  stopAppPoll()
})

</script>

<template>
  <div class="lgp">
    <div class="lgp-top">
      <a href="javascript:;" class="lgp-back" @click.prevent="goBack">{{ $t('common.back') }}</a>
      <LangSwitch />
    </div>
    <div class="lgp-card">
      <aside class="lgp-side">
        <NuxtLink to="/" class="lgp-side-head">
          <span class="lgp-mark">
            <img v-if="logoPc" :src="logoPc" :alt="siteName" />
            <span v-else>{{ siteName.slice(0, 2) || 'JOB' }}</span>
          </span>
          <span class="lgp-side-copy">
            <strong>{{ $t('loginPage.side_job') }}</strong>
            <em>{{ $t('loginPage.side_talk', { site: siteName }) }}</em>
          </span>
        </NuxtLink>
        <ul v-if="role === 1" class="lgp-feat">
          <li>
            <span class="lgp-feat-ico" aria-hidden="true">
              <svg viewBox="0 0 24 24"><path fill="currentColor" d="M4 12h4v8H4v-8zm6-6h4v14h-4V6zm6 3h4v11h-4V9z" /></svg>
            </span>
            <span>
              <strong>{{ $t('loginPage.seek_f2_t') }}</strong>
              <em>{{ $t('loginPage.seek_f2_d') }}</em>
            </span>
          </li>
          <li>
            <span class="lgp-feat-ico" aria-hidden="true">
              <svg viewBox="0 0 24 24"><path fill="currentColor" d="M9 3h6l1 2h4v3H4V5h4l1-2zm-3 8h12l-1.5 9h-9L6 11z" /></svg>
            </span>
            <span>
              <strong>{{ $t('loginPage.seek_f3_t') }}</strong>
              <em>{{ $t('loginPage.seek_f3_d') }}</em>
            </span>
          </li>
        </ul>
        <ul v-else class="lgp-feat">
          <li>
            <span class="lgp-feat-ico" aria-hidden="true">
              <svg viewBox="0 0 24 24"><path fill="currentColor" d="M4 4h10v8H8l-4 3V4zm12 4h6v10l-4-3h-2V8z" /></svg>
            </span>
            <span>
              <strong>{{ $t('loginPage.hire_f1_t') }}</strong>
              <em>{{ $t('loginPage.hire_f1_d') }}</em>
            </span>
          </li>
          <li>
            <span class="lgp-feat-ico" aria-hidden="true">
              <svg viewBox="0 0 24 24"><path fill="currentColor" d="M9 11a3 3 0 1 0 0-6 3 3 0 0 0 0 6zm6.5 1a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5zM2 19c0-2.7 3.1-5 7-5s7 2.3 7 5v1H2v-1zm14 1v-1c0-1.3-.5-2.5-1.4-3.4 1.8.3 4.4 1.3 4.4 3.4V20h-3z" /></svg>
            </span>
            <span>
              <strong>{{ $t('loginPage.hire_f2_t') }}</strong>
              <em>{{ $t('loginPage.hire_f2_d') }}</em>
            </span>
          </li>
          <li>
            <span class="lgp-feat-ico" aria-hidden="true">
              <svg viewBox="0 0 24 24"><path fill="currentColor" d="M9 3h6l1 2h4v3H4V5h4l1-2zm-3 8h12l-1.5 9h-9L6 11z" /></svg>
            </span>
            <span>
              <strong>{{ $t('loginPage.hire_f3_t') }}</strong>
              <em>{{ $t('loginPage.hire_f3_d') }}</em>
            </span>
          </li>
        </ul>
      </aside>
      <div class="lgp-main">
        <button type="button" class="lgp-qr-btn" @click="openPanel(panel === 'qr' ? (smsLoginOn ? 'sms' : 'pass') : 'qr')">
          <svg v-if="panel !== 'qr'" class="lgp-qr-ico" viewBox="0 0 24 24" aria-hidden="true">
            <path fill="currentColor" d="M3 3h8v8H3V3zm2 2v4h4V5H5zm8-2h8v8h-8V3zm2 2v4h4V5h-4zM3 13h8v8H3v-8zm2 2v4h4v-4H5zm10 0h2v2h-2v-2zm4 0h2v2h-2v-2zm-4 4h2v2h-2v-2zm4 0h2v4h-4v-2h2v-2z" />
          </svg>
          <svg v-else class="lgp-qr-ico" viewBox="0 0 24 24" aria-hidden="true">
            <path fill="currentColor" d="M4 4h16v12H4V4zm2 2v8h12V6H6zm-2 12h16v2H4v-2z" />
          </svg>
          {{ panel === 'qr' ? $t('loginPage.sms_title') : $t('loginPage.tab_qr') }}
        </button>

        <template v-if="panel === 'qr'">
          <h1 class="lgp-h1">{{ $t('loginPage.qr_title', { site: siteName }) }}</h1>
          <div class="lgp-qr">
            <img v-if="appQrSrc" :src="appQrSrc" alt="" width="200" height="200" />
            <p v-else class="muted">{{ $t('common_02409') }}</p>
            <p v-if="appQrHint" class="lgp-err">{{ appQrHint }}</p>
            <button v-if="appQrHint" type="button" class="lgp-send" @click="startAppQr">{{ $t('loginPage.qr_refresh') }}</button>
          </div>
          <p v-if="err" class="lgp-err">{{ err }}</p>
          <p class="lgp-qr-links">
            <NuxtLink to="/download">{{ $t('ui.app_download') }}</NuxtLink>
            <span>{{ $t('loginPage.qr_help') }}</span>
          </p>
        </template>

        <template v-else>
          <h1 class="lgp-h1">{{ panel === 'pass' ? $t('common.login') : $t('loginPage.sms_title') }}</h1>
          <p v-if="panel !== 'pass'" class="lgp-sub">{{ $t('loginPage.sms_hint', { site: siteName }) }}</p>
          <div class="lgp-role">
            <button type="button" :class="{ on: role === 1 }" @click="role = 1">{{ $t('loginPage.seek') }}</button>
            <button type="button" :class="{ on: role === 2 }" @click="role = 2">{{ $t('loginPage.hire') }}</button>
          </div>

          <form v-if="panel === 'sms'" @submit.prevent="submitSms">
            <div class="lgp-field">
              <span class="lgp-cc">+86 <i /></span>
              <input v-model="mobile" type="tel" maxlength="11" autocomplete="tel" :placeholder="$t('loginPage.mobile_ph')" />
            </div>
            <div v-if="needImageCaptcha && captcha?.image" class="lgp-field">
              <input v-model="authcode" maxlength="8" autocomplete="off" :placeholder="$t('wap_00262')" />
              <img :src="captcha.image" alt="" class="lgp-captcha" @click="loadCaptcha" />
            </div>
            <div class="lgp-field">
              <input v-model="smsCode" maxlength="6" autocomplete="one-time-code" :placeholder="$t('loginPage.sms_code')" />
              <button type="button" class="lgp-send" :disabled="smsWait > 0" @click="sendSms">
                {{ smsWait > 0 ? `${smsWait}s` : $t('loginPage.send_code') }}
              </button>
            </div>
            <button type="submit" class="lgp-submit">{{ $t('loginPage.submit') }}</button>
          </form>

          <form v-else @submit.prevent="submitPass">
            <div class="lgp-field">
              <input v-model="username" autocomplete="username" :placeholder="$t('admin_user_00140')" />
            </div>
            <div class="lgp-field">
              <input v-model="password" type="password" autocomplete="current-password" :placeholder="$t('wap_user_00371')" />
            </div>
            <div v-if="needImageCaptcha && captcha?.image" class="lgp-field">
              <input v-model="authcode" maxlength="8" autocomplete="off" :placeholder="$t('wap_00262')" />
              <img :src="captcha.image" alt="" class="lgp-captcha" @click="loadCaptcha" />
            </div>
            <button type="submit" class="lgp-submit">{{ $t('common.login') }}</button>
            <p class="lgp-extra">
              <NuxtLink to="/forgetpw">{{ $t('wap_00680') }}</NuxtLink>
              <NuxtLink to="/register">{{ $t('common.register') }}</NuxtLink>
            </p>
          </form>

          <p v-if="err" class="lgp-err">{{ err }}</p>
          <div class="lgp-other">
            <a v-if="wechatOauth" href="javascript:;" @click.prevent="startOauth(wechatOauth)">
              <svg viewBox="0 0 24 24" aria-hidden="true"><path fill="#2aae67" d="M9.5 7.2c-3.7 0-6.7 2.4-6.7 5.4 0 1.7.9 3.2 2.4 4.3l-.6 1.8 2.1-1.1c.8.2 1.5.4 2.3.4.3 0 .6 0 .9-.1-.2-.5-.3-1.1-.3-1.7 0-3.2 2.9-5.7 6.5-5.7.2 0 .4 0 .6.1-1-2.1-3.4-3.4-6.2-3.4zm-1.7 2.2a.8.8 0 1 1 0 1.6.8.8 0 0 1 0-1.6zm3.5 0a.8.8 0 1 1 0 1.6.8.8 0 0 1 0-1.6zM16.8 11c-3.3 0-6 2.2-6 5s2.7 5 6 5c.6 0 1.2-.1 1.8-.3l1.7.9-.5-1.5c1.2-.9 2-2.2 2-3.6 0-2.8-2.7-5-5-5zm-1.5 1.9a.7.7 0 1 1 0 1.4.7.7 0 0 1 0-1.4zm3.1 0a.7.7 0 1 1 0 1.4.7.7 0 0 1 0-1.4z" /></svg>
              {{ $t('loginPage.wechat') }}
            </a>
            <NuxtLink v-if="role === 2" to="/download">
              <svg viewBox="0 0 24 24" aria-hidden="true"><path fill="currentColor" d="M4 5h16v11H4V5zm2 2v7h12V7H6zm-2 11h16v2H4v-2z" /></svg>
              {{ $t('loginPage.desktop') }}
            </NuxtLink>
          </div>
          <label class="lgp-agree">
            <input v-model="agreed" type="checkbox" />
            <span>
              {{ $t('loginPage.agree_prefix', { site: siteName }) }}
              <NuxtLink to="/pages/protocol">{{ $t('loginPage.protocol') }}</NuxtLink>
              <NuxtLink to="/pages/privacy">{{ $t('loginPage.privacy') }}</NuxtLink>
              {{ $t('loginPage.agree_suffix', { site: siteName }) }}
            </span>
          </label>
        </template>
        <p class="lgp-foot">
          {{ $t('loginPage.service', { tel: phone || settings.sy_freewebtel || '', time: worktime || '8:00-22:00' }) }}
          <br />
          {{ $t('loginPage.license') }}
        </p>
      </div>
    </div>
    <svg class="lgp-sky" viewBox="0 0 1440 220" preserveAspectRatio="xMidYMax meet" aria-hidden="true">
      <g fill="none" stroke="#0aa9a8" stroke-opacity="0.45" stroke-width="1.4">
        <path d="M0 210 H1440" />
        <path d="M20 210 V150 h28 v-22 h18 v32 h22 V210" />
        <path d="M100 210 V120 h50 V210" />
        <path d="M118 132 h14 v10 h-14z M118 148 h14 v10 h-14z M118 164 h14 v10 h-14z M118 180 h14 v10 h-14z" />
        <path d="M165 210 V88 h36 V210" />
        <path d="M220 210 V140 h70 V210" />
        <path d="M236 152 h14 v12 h-14z M258 152 h14 v12 h-14z M236 172 h14 v12 h-14z M258 172 h14 v12 h-14z" />
        <path d="M310 210 V100 h24 v-36 h18 v36 h24 V210" />
        <path d="M400 210 V70 l18-28 18 28 V210" />
        <path d="M460 210 V150 h90 V210" />
        <path d="M580 210 V110 h40 V210" />
        <path d="M1080 210 V130 h60 V210" />
        <path d="M1096 142 h12 v10 h-12z M1116 142 h12 v10 h-12z M1096 160 h12 v10 h-12z M1116 160 h12 v10 h-12z" />
        <path d="M1160 210 V80 h18 v-50 h12 v50 h18 V210" />
        <path d="M1220 210 V40 c8-28 18-48 22-70 4 22 14 42 22 70 V210" />
        <path d="M1288 210 V120 h70 V210" />
        <path d="M1368 210 V150 h52 V210" />
      </g>
    </svg>
  </div>
</template>
