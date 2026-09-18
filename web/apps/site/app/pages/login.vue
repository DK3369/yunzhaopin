<script setup lang="ts">
import { ApiError } from '~/utils/envelope'
import { qrSvgDataUri } from '~/utils/qr'
import { OAUTH_FRONT_PROVIDERS, oauthEnabledByAdmin, safeLoginNext } from '~/utils/site'

const { siteName, logoPc, settings, me, refreshMe } = useSiteChrome()
const { data: dicts } = await usePublicDicts()
const { t } = useI18n()
const api = useApi()
const smsLoginOn = computed(
  () => String(settings.value.sy_msg_isopen) === '1' && String(settings.value.sy_msg_login) === '1',
)
const needImageCaptcha = computed(() => {
  const parts = String(settings.value.code_web || '')
    .split(/[,，]/)
    .map((s) => s.trim())
    .filter(Boolean)
  return parts.includes('wap_js_00062') || parts.includes('前台登录')
})
const panel = ref<'sms' | 'qr' | 'pass'>(smsLoginOn.value ? 'sms' : 'pass')
const role = ref<1 | 2>(1)
const username = ref('')
const password = ref('')
const agreed = ref(false)
const remember = ref(false)
const mobile = ref('')
const smsCode = ref('')
const smsWait = ref(0)
const nextFrom = ref('')
const { data: captcha } = await useAsyncData('login-captcha', () =>
  api.post<{ cid: string; image: string }>('/v1/wap/captcha').catch(() => null),
)
const authcode = ref('')
const err = ref('')
const submitting = ref(false)
const oauth = ref<Array<{ name: string; path: string; provider: string }>>([])
let smsTimer: ReturnType<typeof setInterval> | null = null
const siteUrl = String(useRuntimeConfig().public.siteUrl || '').replace(/\/$/, '')
const appTicket = ref<{ login_id: string; usertype: number; payload: string; scan_url: string } | null>(null)
const appQrSrc = ref('')
const appQrHint = ref('')
let appPoll: ReturnType<typeof setInterval> | null = null
let appWatchReady = false

function loginNext(): string {
  const raw = useRoute().query.next
  const q = Array.isArray(raw) ? String(raw[0] || '') : String(raw || nextFrom.value || '')
  return safeLoginNext(q)
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
    const next = loginNext()
    if (next) {
      await navigateTo(next)
      return
    }
    goBack()
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
  for (const item of OAUTH_FRONT_PROVIDERS) {
    if (!oauthEnabledByAdmin(settings.value, item, dicts.value)) continue
    try {
      const r = await api.post<{ authorize_url?: string }>(item.path, { redirect_uri })
      if (r.authorize_url) oauth.value.push({ name: item.name, path: r.authorize_url, provider: item.key })
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
  if (submitting.value) return
  if (!agreed.value) {
    err.value = t('wap_00309')
    return
  }
  submitting.value = true
  try {
    if (needImageCaptcha.value && !captcha.value) await loadCaptcha()
    const user = await $fetch<{ uid: number; usertype: number }>('/api/auth/login', {
      method: 'POST',
      body: {
        username: username.value,
        password: password.value,
        authcode: needImageCaptcha.value ? authcode.value : undefined,
        captcha_cid: needImageCaptcha.value ? captcha.value?.cid : undefined,
        remember: remember.value,
      },
    })
    await afterLogin(user)
  } catch (e: unknown) {
    await handleAuthFail(e)
    if (needImageCaptcha.value) loadCaptcha()
  } finally {
    submitting.value = false
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
  if (submitting.value) return
  if (!agreed.value) {
    err.value = t('wap_00309')
    return
  }
  submitting.value = true
  try {
    const logged = await $fetch<{ uid: number; usertype: number }>('/api/auth/login-sms', {
      method: 'POST',
      body: { moblie: mobile.value, dynamiccode: smsCode.value, remember: remember.value },
    })
    await afterLogin(logged)
  } catch (e: unknown) {
    await handleAuthFail(e)
  } finally {
    submitting.value = false
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
  <div class="site-pc">
    <div class="login_cont">
      <div class="login_w960">
        <div class="login_header">
          <div class="logo fl">
            <NuxtLink to="/">
              <img v-if="logoPc" :src="logoPc" :alt="siteName" />
              <span v-else class="site-wordmark">{{ siteName }}</span>
            </NuxtLink>
          </div>
          <a href="javascript:;" class="logo_fh fr" @click.prevent="goBack">{{ $t('member_user_00116') }} ></a>
          <span class="fr" style="margin-right: 16px; line-height: 60px"><LangSwitch /></span>
        </div>
      </div>
      <div class="logoin_cont_box">
        <div class="login_left">
          <div class="login_box_cont">
            <div class="login_box_h1_d">
              <ul class="login_box_h_list">
                <li :class="{ login_box_h_list_cur: panel === 'pass' }" @click="openPanel('pass')">
                  {{ $t('default_00062') }}<i class="login_box_h_icon" />
                </li>
                <li
                  v-if="smsLoginOn"
                  :class="{ login_box_h_list_cur: panel === 'sms' }"
                  @click="openPanel('sms')"
                >
                  {{ $t('default_00259') }}<i class="login_box_h_icon" />
                </li>
              </ul>
              <div
                class="wxcode_login"
                :title="$t('loginPage.tab_qr')"
                :class="{ none: panel === 'qr' }"
                @click="openPanel('qr')"
              />
              <div
                class="normal_login"
                :class="{ none: panel !== 'qr' }"
                :title="$t('common.login')"
                @click="openPanel(smsLoginOn ? 'sms' : 'pass')"
              />
            </div>
            <div v-if="panel === 'qr'" class="wx_login_show">
              <div class="wx_login_show_new">
                <div class="wxlogintext">
                  <img v-if="appQrSrc" :src="appQrSrc" alt="" width="180" height="180" />
                  <p v-else class="muted">{{ $t('common_02409') }}</p>
                </div>
                <div class="wxlogintxt">{{ $t('loginPage.qr_title', { site: siteName }) }}</div>
                <p class="muted" style="padding: 8px 0">
                  <button type="button" :class="{ on: role === 1 }" @click="role = 1">{{ $t('loginPage.seek') }}</button>
                  <button type="button" :class="{ on: role === 2 }" @click="role = 2">{{ $t('loginPage.hire') }}</button>
                </p>
                <p v-if="appQrHint" class="muted">{{ appQrHint }}</p>
                <button v-if="appQrHint" type="button" class="login_box_bth2" @click="startAppQr">{{ $t('loginPage.qr_refresh') }}</button>
                <p>
                  <NuxtLink to="/download">{{ $t('ui.app_download') }}</NuxtLink>
                </p>
              </div>
            </div>
            <div v-else class="login_t_box">
              <form v-if="panel === 'sms'" @submit.prevent="submitSms">
                <div class="login_box_list">
                  <i class="login_box_icon login_box_usersj" />
                  <input
                    v-model="mobile"
                    required
                    type="tel"
                    maxlength="11"
                    class="login_box_bth"
                    autocomplete="tel"
                    :placeholder="$t('loginPage.mobile_ph')"
                  />
                </div>
                <div v-if="needImageCaptcha && captcha?.image" class="login_box_list">
                  <input v-model="authcode" maxlength="8" class="login_box_bth" autocomplete="off" :placeholder="$t('wap_00262')" />
                  <img :src="captcha.image" alt="" @click="loadCaptcha" />
                </div>
                <div class="login_box_list">
                  <input
                    v-model="smsCode"
                    required
                    maxlength="6"
                    class="login_box_bth"
                    autocomplete="one-time-code"
                    :placeholder="$t('loginPage.sms_code')"
                  />
                  <button type="button" :disabled="smsWait > 0" @click="sendSms">
                    {{ smsWait > 0 ? `${smsWait}s` : $t('loginPage.send_code') }}
                  </button>
                </div>
                <div class="login_xy">
                  <label>
                    <input v-model="agreed" type="checkbox" />
                    <span>
                      <i class="policy">{{ $t('wap_00309') }}</i>
                      <NuxtLink to="/pages/protocol" class="Privacy">{{ $t('wap_00678') }}</NuxtLink>
                      <i class="policy">{{ $t('wap_00679') }}</i>
                      <NuxtLink to="/pages/privacy" class="Privacy">{{ $t('wap_00313') }}</NuxtLink>
                    </span>
                  </label>
                </div>
                <div class="login_box_cz">
                  <input type="submit" class="login_box_bth2" :disabled="submitting" :value="$t('common.login')" />
                </div>
                <div class="login_box_fw">
                  <label class="login_remember">
                    <input v-model="remember" type="checkbox" />
                    {{ $t('loginPage.remember') }}
                  </label>
                </div>
              </form>
              <form v-else @submit.prevent="submitPass">
                <div class="login_box_list">
                  <i class="login_box_icon login_box_username" />
                  <input
                    v-model="username"
                    required
                    class="login_box_bth"
                    autocomplete="username"
                    :placeholder="$t('admin_user_00140')"
                  />
                </div>
                <div class="login_box_list">
                  <i class="login_box_icon loginpwd" />
                  <input
                    v-model="password"
                    required
                    type="password"
                    class="login_box_bth"
                    autocomplete="current-password"
                    :placeholder="$t('wap_user_00371')"
                  />
                </div>
                <div v-if="needImageCaptcha && captcha?.image" class="login_box_list">
                  <input v-model="authcode" maxlength="8" class="login_box_bth" autocomplete="off" :placeholder="$t('wap_00262')" />
                  <img :src="captcha.image" alt="" @click="loadCaptcha" />
                </div>
                <div class="login_xy">
                  <label>
                    <input v-model="agreed" type="checkbox" />
                    <span>
                      <i class="policy">{{ $t('wap_00309') }}</i>
                      <NuxtLink to="/pages/protocol" class="Privacy">{{ $t('wap_00678') }}</NuxtLink>
                      <i class="policy">{{ $t('wap_00679') }}</i>
                      <NuxtLink to="/pages/privacy" class="Privacy">{{ $t('wap_00313') }}</NuxtLink>
                    </span>
                  </label>
                </div>
                <div class="login_box_cz">
                  <input type="submit" class="login_box_bth2" :disabled="submitting" :value="$t('common.login')" />
                </div>
                <div class="login_box_fw">
                  <label class="login_remember">
                    <input v-model="remember" type="checkbox" />
                    {{ $t('loginPage.remember') }}
                  </label>
                  <NuxtLink to="/forgetpw">{{ $t('wap_00680') }}</NuxtLink>
                  <NuxtLink to="/register">{{ $t('common.register') }}</NuxtLink>
                </div>
              </form>
            </div>
            <p v-if="err" class="muted">{{ err }}</p>
            <div v-if="oauth.length" class="login_other">
              <a v-for="o in oauth" :key="o.provider" href="javascript:;" class="l-icon" @click.prevent="startOauth(o)">
                {{
                  o.provider === 'wechat'
                    ? $t('loginPage.wechat')
                    : o.provider === 'qq'
                      ? $t('loginPage.qq')
                      : o.provider === 'weibo'
                        ? $t('loginPage.weibo')
                        : o.provider === 'google'
                          ? $t('loginPage.google')
                          : o.provider === 'facebook'
                            ? $t('loginPage.facebook')
                            : o.name
                }}
              </a>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
  <div class="site-h5">
    <div class="Back_to_the_previous_level">
      <a href="javascript:;" class="login_back" @click.prevent="goBack">
        <img src="/legacy/h5/images/return.png" alt="" width="100%" height="100%" />
      </a>
    </div>
    <div class="login_cont">
      <div style="text-align: right; padding: 0.24rem 0.32rem 0"><LangSwitch /></div>
      <div class="bottom_nav_bom" style="padding-top: 0; text-align: right">
        <i class="bottom_nav_bom_word">{{ $t('wap_00672') }}</i>
        <NuxtLink to="/register" class="register_1">{{ $t('wap_00673') }}</NuxtLink>
      </div>
      <div class="login_welcome">
        <div>{{ $t('common.login') }}</div>
        <div>{{ siteName }}</div>
      </div>
      <form v-if="panel === 'sms'" @submit.prevent="submitSms">
        <div class="The_login_subject">
          <div class="login_textbox">
            <input v-model="mobile" required type="tel" maxlength="11" autocomplete="tel" :placeholder="$t('loginPage.mobile_ph')" />
          </div>
          <div v-if="needImageCaptcha && captcha?.image" class="login_textbox">
            <input v-model="authcode" maxlength="8" autocomplete="off" :placeholder="$t('wap_00262')" />
            <img :src="captcha.image" alt="" class="authcode" @click="loadCaptcha" />
          </div>
          <div class="login_textbox">
            <input v-model="smsCode" required maxlength="6" autocomplete="one-time-code" :placeholder="$t('loginPage.sms_code')" />
            <div class="dx_yz_hq" :class="{ muted: smsWait > 0 }" @click="smsWait > 0 ? undefined : sendSms()">
              {{ smsWait > 0 ? `${smsWait}s` : $t('loginPage.send_code') }}
            </div>
          </div>
          <div class="login_xy">
            <div class="login_xy_zx"><input v-model="agreed" type="checkbox" /></div>
            <div>
              <i class="policy">{{ $t('wap_00309') }}</i>
              <NuxtLink to="/pages/protocol" class="Privacy">{{ $t('wap_00678') }}</NuxtLink>
              <i class="policy">{{ $t('wap_00679') }}</i>
              <NuxtLink to="/pages/privacy" class="Privacy">{{ $t('wap_00313') }}</NuxtLink>
            </div>
          </div>
        </div>
        <p v-if="err" class="muted">{{ err }}</p>
        <div class="login_bthbox">
          <button type="submit" class="login_bth" :disabled="submitting">{{ $t('common.login') }}</button>
        </div>
      </form>
      <form v-else @submit.prevent="submitPass">
        <div class="The_login_subject">
          <div class="login_textbox">
            <input v-model="username" required autocomplete="username" :placeholder="$t('admin_user_00140')" />
          </div>
          <div class="login_textbox">
            <input v-model="password" required type="password" autocomplete="current-password" :placeholder="$t('wap_user_00371')" />
          </div>
          <div v-if="needImageCaptcha && captcha?.image" class="login_textbox">
            <input v-model="authcode" maxlength="8" autocomplete="off" :placeholder="$t('wap_00262')" />
            <img :src="captcha.image" alt="" class="authcode" @click="loadCaptcha" />
          </div>
          <div class="login_xy">
            <div class="login_xy_zx"><input v-model="agreed" type="checkbox" /></div>
            <div>
              <i class="policy">{{ $t('wap_00309') }}</i>
              <NuxtLink to="/pages/protocol" class="Privacy">{{ $t('wap_00678') }}</NuxtLink>
              <i class="policy">{{ $t('wap_00679') }}</i>
              <NuxtLink to="/pages/privacy" class="Privacy">{{ $t('wap_00313') }}</NuxtLink>
            </div>
          </div>
        </div>
        <p v-if="err" class="muted">{{ err }}</p>
        <div class="login_bthbox">
          <button type="submit" class="login_bth" :disabled="submitting">{{ $t('common.login') }}</button>
        </div>
      </form>
      <div class="login_otherfs">
        <label class="login_remember">
          <input v-model="remember" type="checkbox" />
          {{ $t('loginPage.remember') }}
        </label>
        <div v-if="smsLoginOn && panel !== 'sms'" class="verification_code_word" @click="openPanel('sms')">{{ $t('wap_00648') }}</div>
        <div v-if="panel === 'sms'" class="verification_code_word" @click="openPanel('pass')">{{ $t('wap_00308') }}</div>
        <NuxtLink to="/forgetpw" class="login_wjmm">{{ $t('wap_00680') }}</NuxtLink>
      </div>
    </div>
    <div v-if="oauth.length" class="bottom_nav">
      <div class="bottom_nav_top">{{ $t('wap_00681') }}</div>
      <div class="bottom_nav_center">
        <a v-for="o in oauth" :key="'h5-' + o.provider" href="javascript:;" class="bottom_nav_center_logo" @click.prevent="startOauth(o)">
          {{
            o.provider === 'wechat'
              ? $t('loginPage.wechat')
              : o.provider === 'qq'
                ? $t('loginPage.qq')
                : o.provider === 'weibo'
                  ? $t('loginPage.weibo')
                  : o.provider === 'google'
                    ? $t('loginPage.google')
                    : o.provider === 'facebook'
                      ? $t('loginPage.facebook')
                      : o.name
          }}
        </a>
      </div>
    </div>
  </div>
</template>
