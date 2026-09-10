<script setup lang="ts">
import { ApiError } from '~/utils/envelope'

const { siteName, logoPc, settings, me } = useSiteChrome()
const { t } = useI18n()
const api = useApi()
const smsLoginOn = computed(
  () => String(settings.value.sy_msg_isopen) === '1' && String(settings.value.sy_msg_login) === '1',
)
const needImageCaptcha = computed(() => {
  const web = String(settings.value.code_web || '')
  return web.includes('前台登录') || web.includes('wap_js_00062')
})
const panel = ref<'sms' | 'qr' | 'pass'>('sms')
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
const wxQr = ref<{ login_id: string; show_url: string } | null>(null)
const wxQrHint = ref('')
let wxPoll: ReturnType<typeof setInterval> | null = null
let smsTimer: ReturnType<typeof setInterval> | null = null
const siteUrl = String(useRuntimeConfig().public.siteUrl || '').replace(/\/$/, '')
const wechatOauth = computed(() => oauth.value.find((o) => o.provider === 'wechat'))

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
  try {
    wxQr.value = await $fetch('/api/auth/login-wx-qr', { method: 'POST' })
    if (wxQr.value?.login_id) {
      wxPoll = setInterval(async () => {
        try {
          const st = await $fetch<{ status: string; uid?: number; usertype?: number }>(
            '/api/auth/login-wx-status',
            { method: 'POST', body: { login_id: wxQr.value?.login_id } },
          )
          if (st.status === 'ok' && st.uid) {
            if (wxPoll) clearInterval(wxPoll)
            await afterLogin({ uid: st.uid, usertype: Number(st.usertype || 0) })
          } else if (st.status === 'unbound') {
            wxQrHint.value = t('common.register')
          }
        } catch {
          /* keep polling until expire */
        }
      }, 2000)
    }
  } catch {
    wxQr.value = null
  }
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

async function sendSmsCode(scene: 'login' | 'register') {
  await api.post('/v1/wap/sms/send', {
    moblie: mobile.value,
    scene,
    captcha_cid: captcha.value?.cid,
    authcode: authcode.value,
  })
}

async function sendSms() {
  err.value = ''
  if (!agreed.value) {
    err.value = t('wap_00309')
    return
  }
  if (smsWait.value > 0) return
  try {
    await sendSmsCode('login')
    startSmsWait()
  } catch (e: unknown) {
    const f = authFail(e)
    if (f.key === 'need_register') {
      try {
        await sendSmsCode('register')
        startSmsWait()
        return
      } catch (e2: unknown) {
        await handleAuthFail(e2)
        loadCaptcha()
        return
      }
    }
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
      body: { moblie: mobile.value, dynamiccode: smsCode.value, usertype: role.value },
    })
    await afterLogin(logged)
  } catch (e: unknown) {
    await handleAuthFail(e)
  }
}

useSeoMeta({ title: t('common.login') })
onUnmounted(() => {
  if (wxPoll) clearInterval(wxPoll)
  if (smsTimer) clearInterval(smsTimer)
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
        <NuxtLink to="/" class="lgp-brand">
          <img v-if="logoPc" :src="logoPc" :alt="siteName" />
          <span v-else>{{ siteName }}</span>
        </NuxtLink>
        <p class="lgp-side-title">{{ role === 1 ? $t('loginPage.seek_tagline') : $t('loginPage.hire_tagline') }}</p>
        <ul v-if="role === 1" class="lgp-feat">
          <li>
            <strong>{{ $t('loginPage.seek_f1_t') }}</strong>
            <span>{{ $t('loginPage.seek_f1_d') }}</span>
          </li>
          <li>
            <strong>{{ $t('loginPage.seek_f2_t') }}</strong>
            <span>{{ $t('loginPage.seek_f2_d') }}</span>
          </li>
          <li>
            <strong>{{ $t('loginPage.seek_f3_t') }}</strong>
            <span>{{ $t('loginPage.seek_f3_d') }}</span>
          </li>
        </ul>
        <ul v-else class="lgp-feat">
          <li>
            <strong>{{ $t('loginPage.hire_f1_t') }}</strong>
            <span>{{ $t('loginPage.hire_f1_d') }}</span>
          </li>
          <li>
            <strong>{{ $t('loginPage.hire_f2_t') }}</strong>
            <span>{{ $t('loginPage.hire_f2_d') }}</span>
          </li>
          <li>
            <strong>{{ $t('loginPage.hire_f3_t') }}</strong>
            <span>{{ $t('loginPage.hire_f3_d') }}</span>
          </li>
        </ul>
      </aside>
      <div class="lgp-main">
        <div class="lgp-tabs">
          <button type="button" :class="{ on: panel === 'qr' }" @click="panel = 'qr'">{{ $t('loginPage.tab_qr') }}</button>
          <button type="button" :class="{ on: panel === 'sms' || panel === 'pass' }" @click="panel = smsLoginOn ? 'sms' : 'pass'">{{ $t('loginPage.tab_sms') }}</button>
        </div>

        <template v-if="panel === 'qr'">
          <h1 class="lgp-h1">{{ $t('loginPage.qr_title', { site: siteName }) }}</h1>
          <div class="lgp-qr">
            <img v-if="wxQr?.show_url" :src="wxQr.show_url" alt="" width="180" height="180" />
            <p v-else class="muted">{{ $t('common_02409') }}</p>
            <p v-if="wxQrHint" class="muted">{{ wxQrHint }}</p>
          </div>
          <p class="lgp-qr-links">
            <NuxtLink to="/download">{{ $t('ui.app_download') }}</NuxtLink>
            <span>·</span>
            <span>{{ $t('loginPage.qr_help') }}</span>
          </p>
        </template>

        <template v-else>
          <h1 class="lgp-h1">{{ $t('loginPage.sms_title') }}</h1>
          <p class="lgp-sub">{{ $t('loginPage.sms_hint') }}</p>
          <div class="lgp-role">
            <button type="button" :class="{ on: role === 1 }" @click="role = 1">{{ $t('loginPage.seek') }}</button>
            <button type="button" :class="{ on: role === 2 }" @click="role = 2">{{ $t('loginPage.hire') }}</button>
          </div>

          <form v-if="panel === 'sms'" @submit.prevent="submitSms">
            <div class="lgp-field">
              <span class="lgp-cc">+86</span>
              <input v-model="mobile" type="tel" maxlength="11" autocomplete="tel" :placeholder="$t('common.phone')" />
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
            <a v-if="wechatOauth" href="javascript:;" @click.prevent="startOauth(wechatOauth)">{{ $t('loginPage.wechat') }}</a>
            <NuxtLink to="/download">{{ $t('loginPage.desktop') }}</NuxtLink>
            <a v-if="smsLoginOn && panel === 'sms'" href="javascript:;" @click.prevent="panel = 'pass'">{{ $t('wap_00308') }}</a>
            <a v-else-if="smsLoginOn && panel === 'pass'" href="javascript:;" @click.prevent="panel = 'sms'">{{ $t('wap_00648') }}</a>
          </div>
          <label class="lgp-agree">
            <input v-model="agreed" type="checkbox" />
            <span>
              {{ $t('loginPage.agree_prefix') }}
              <NuxtLink to="/pages/protocol">{{ $t('wap_00678') }}</NuxtLink>
              <NuxtLink to="/pages/privacy">{{ $t('wap_00313') }}</NuxtLink>
            </span>
          </label>
        </template>
        <p class="lgp-foot">{{ $t('loginPage.service', { tel: settings.sy_freewebtel || '' }) }}</p>
      </div>
    </div>
    <svg class="lgp-sky" viewBox="0 0 1440 220" preserveAspectRatio="xMidYEnd meet" aria-hidden="true">
      <path fill="#0aa9a8" fill-opacity="0.35" d="M0 180 L80 140 L160 170 L240 110 L320 160 L400 90 L480 150 L560 100 L640 165 L720 80 L800 155 L880 95 L960 170 L1040 120 L1120 175 L1200 130 L1280 180 L1360 150 L1440 190 L1440 220 L0 220 Z" />
    </svg>
  </div>
</template>
