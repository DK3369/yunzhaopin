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
const tab = ref<'pass' | 'sms' | 'email'>('pass')
const username = ref('')
const password = ref('')
const showPwd = ref(false)
const agreed = ref(true)
const mobile = ref('')
const smsCode = ref('')
const email = ref('')
const emailCode = ref('')
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
const siteUrl = String(useRuntimeConfig().public.siteUrl || '').replace(/\/$/, '')

function loginNext(): string {
  const q = String(useRoute().query.next || nextFrom.value || '')
  if (q.startsWith('/') && !q.startsWith('//')) return q
  return ''
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
    await navigateTo('/register')
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
  if (smsLoginOn.value && String(settings.value.sy_login_type) === '2') {
    tab.value = 'sms'
  }
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
      const me = await $fetch<{ uid: number; usertype: number; need_bind?: boolean; ticket?: string }>(
        '/api/auth/oauth-login',
        {
          method: 'POST',
          body: { provider, code, state },
        },
      )
      sessionStorage.removeItem('oauth_provider')
      if (me.need_bind && me.ticket) {
        await navigateTo({ path: '/oauth-bind', query: { ticket: me.ticket } })
        return
      }
      await afterLogin(me)
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

async function afterLogin(me: { uid: number; usertype: number }) {
  const next = loginNext()
  if (me.usertype === 0) {
    await navigateTo({ path: '/utype', query: next ? { next } : {} })
    return
  }
  if (next) {
    await navigateTo(next)
    return
  }
  await navigateTo(me.usertype === 2 ? '/com' : '/user')
}
async function submitPass() {
  err.value = ''
  if (!agreed.value) {
    err.value = t('wap_00309')
    return
  }
  try {
    if (needImageCaptcha.value && !captcha.value) await loadCaptcha()
    const me = await $fetch<{ uid: number; usertype: number }>('/api/auth/login', {
      method: 'POST',
      body: {
        username: username.value,
        password: password.value,
        authcode: needImageCaptcha.value ? authcode.value : undefined,
        captcha_cid: needImageCaptcha.value ? captcha.value?.cid : undefined,
      },
    })
    await afterLogin(me)
  } catch (e: unknown) {
    await handleAuthFail(e)
    if (needImageCaptcha.value) loadCaptcha()
  }
}
async function sendSms() {
  err.value = ''
  try {
    await api.post('/v1/wap/sms/send', {
      moblie: mobile.value,
      scene: 'login',
      captcha_cid: captcha.value?.cid,
      authcode: authcode.value,
    })
  } catch (e: unknown) {
    await handleAuthFail(e)
    loadCaptcha()
  }
}
async function submitSms() {
  err.value = ''
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
async function sendEmail() {
  err.value = ''
  try {
    await api.post('/v1/wap/login/email/code', {
      email: email.value,
      captcha_cid: captcha.value?.cid,
      authcode: authcode.value,
    })
  } catch (e: unknown) {
    err.value = e instanceof Error ? e.message : t('common_00888')
    loadCaptcha()
  }
}
async function submitEmail() {
  err.value = ''
  try {
    const me = await $fetch<{ uid: number; usertype: number }>('/api/auth/login-email', {
      method: 'POST',
      body: { email: email.value, code: emailCode.value },
    })
    await afterLogin(me)
  } catch (e: unknown) {
    await handleAuthFail(e)
  }
}
useSeoMeta({ title: t('common.login') })
onUnmounted(() => {
  if (wxPoll) clearInterval(wxPoll)
})
</script>

<template>
  <div class="site-pc">
    <div class="login_cont">
      <div class="login_w960">
        <div class="login_header">
          <div class="logo fl" style="position: relative">
            <NuxtLink to="/">
              <img v-if="logoPc" :src="logoPc" class="png" :alt="siteName" />
              <span v-else class="site-wordmark">{{ siteName }}</span>
            </NuxtLink>
          </div>
          <NuxtLink to="/" class="logo_fh fr">{{ $t('common.home') }} ></NuxtLink>
          <span class="fr" style="margin-right: 16px; line-height: 60px"><LangSwitch /></span>
        </div>
      </div>
      <div class="clear" />
      <div class="logoin_cont_box">
        <div class="login_left">
          <div class="login_box_cont">
            <div class="login_box_h1_d">
              <ul class="login_box_h_list">
                <li :class="{ login_box_h_list_cur: tab === 'pass' }" @click="tab = 'pass'">
                  {{ $t('common.login') }}<i class="login_box_h_icon" />
                </li>
                <li v-if="smsLoginOn" :class="{ login_box_h_list_cur: tab === 'sms' }" @click="tab = 'sms'">
                  {{ $t('wap_00648') }}
                </li>
                <li :class="{ login_box_h_list_cur: tab === 'email' }" @click="tab = 'email'">
                  {{ $t('member_user_00282') }}
                </li>
              </ul>
            </div>
            <form v-if="tab === 'pass'" class="login_t_box" @submit.prevent="submitPass">
              <div class="login_box_cot">
                <div class="login_normal_box">
                  <div class="login_box_list">
                    <i class="login_box_icon login_box_username" />
                    <input v-model="username" class="login_box_bth placeholder loginname" autocomplete="username" :placeholder="$t('admin_user_00140')" />
                  </div>
                  <div class="login_box_list">
                    <i class="login_box_icon loginpwd" />
                    <input v-model="password" type="password" class="login_box_bth placeholder loginname" autocomplete="current-password" :placeholder="$t('wap_user_00371')" />
                  </div>
                  <div v-if="needImageCaptcha && captcha?.image" class="login_box_list">
                    <img :src="captcha.image" alt="" @click="loadCaptcha" />
                    <input v-model="authcode" class="login_box_bth" :placeholder="$t('wap_00262')" autocomplete="off" />
                  </div>
                </div>
                <div class="login_xy" style="padding: 8px 0">
                  <label class="login_xy_zx">
                    <input id="xieyicheck-pc" v-model="agreed" type="checkbox" />
                    <i class="policy">{{ $t('wap_00309') }}</i>
                    <NuxtLink to="/pages/protocol" class="Privacy">{{ $t('wap_00678') }}</NuxtLink>
                    <i class="policy">{{ $t('wap_00679') }}</i>
                    <NuxtLink to="/pages/privacy" class="Privacy">{{ $t('wap_00313') }}</NuxtLink>
                  </label>
                </div>
                <div class="login_box_cz">
                  <input type="submit" :value="$t('common.login')" class="login_box_bth2" />
                </div>
                <p v-if="err" class="muted" style="padding: 8px 0">{{ err }}</p>
                <div class="login_box_fw">
                  <span class="fl">{{ $t('common.register') }} <NuxtLink to="/register">{{ $t('common.register') }}</NuxtLink></span>
                  <NuxtLink to="/forgetpw" class="fr">{{ $t('wap_00680') }}</NuxtLink>
                </div>
                <p v-if="oauth.length" style="padding: 12px 0">
                  <a v-for="o in oauth" :key="o.name" :href="o.path" style="margin-right: 12px" @click="sessionStorage.setItem('oauth_provider', o.provider)">{{ o.name }}</a>
                </p>
                <div v-if="wxQr?.show_url" style="padding: 12px 0; text-align: center">
                  <p class="muted">{{ $t('common_02228') }}</p>
                  <img :src="wxQr.show_url" alt="" width="160" height="160" />
                  <p v-if="wxQrHint" class="muted">{{ wxQrHint }}</p>
                </div>
              </div>
            </form>
            <form v-else-if="tab === 'sms'" class="login_t_box" @submit.prevent="submitSms">
              <div class="login_box_list">
                <input v-model="mobile" class="login_box_bth" :placeholder="$t('common.phone')" />
              </div>
              <div v-if="captcha?.image" class="login_box_list">
                <img :src="captcha.image" alt="" @click="loadCaptcha" />
                <input v-model="authcode" class="login_box_bth" />
              </div>
              <div class="login_box_list">
                <input v-model="smsCode" class="login_box_bth" :placeholder="$t('wap_01371')" />
                <button type="button" @click="sendSms">{{ $t('common.submit') }}</button>
              </div>
              <div class="login_box_cz">
                <input type="submit" :value="$t('common.login')" class="login_box_bth2" />
              </div>
              <p v-if="err" class="muted">{{ err }}</p>
            </form>
            <form v-else class="login_t_box" @submit.prevent="submitEmail">
              <div class="login_box_list">
                <input v-model="email" class="login_box_bth" :placeholder="$t('member_user_00282')" />
              </div>
              <div v-if="captcha?.image" class="login_box_list">
                <img :src="captcha.image" alt="" @click="loadCaptcha" />
                <input v-model="authcode" class="login_box_bth" :placeholder="$t('wap_00110')" />
              </div>
              <div class="login_box_list">
                <input v-model="emailCode" class="login_box_bth" :placeholder="$t('wap_01371')" />
                <button type="button" @click="sendEmail">{{ $t('common.submit') }}</button>
              </div>
              <div class="login_box_cz">
                <input type="submit" :value="$t('common.login')" class="login_box_bth2" />
              </div>
              <p v-if="err" class="muted">{{ err }}</p>
            </form>
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
      <div class="bottom_nav_bom" style="padding-top: 0; text-align: right">
        <LangSwitch />
        <i class="bottom_nav_bom_word">{{ $t('wap_00672') }}</i>
        <NuxtLink to="/register" class="register_1" style="margin-left: 8px">{{ $t('wap_00673') }}</NuxtLink>
      </div>
      <div class="login_welcome">
        <div>{{ $t('wap_00674') }}</div>
        <div>{{ $t('wap_00675') }}</div>
      </div>
      <form v-if="tab === 'pass'" @submit.prevent="submitPass">
        <div class="The_login_subject">
          <div class="login_textbox">
            <input
              v-model="username"
              type="text"
              class="account_number"
              autocomplete="username"
              :placeholder="`${$t('wap_00208')}/${$t('member_user_00282')}/${$t('wap_user_00180')}`"
            />
          </div>
          <div class="login_textbox">
            <input
              v-model="password"
              :type="showPwd ? 'text' : 'password'"
              autocomplete="current-password"
              :placeholder="$t('wap_js_00139')"
            />
            <div class="close_open" @click="showPwd = !showPwd">
              <img
                :src="showPwd ? '/legacy/h5/images/conceal_1.png' : '/legacy/h5/images/conceal.png'"
                alt=""
                width="100%"
                height="100%"
              />
            </div>
          </div>
          <div v-if="needImageCaptcha" class="login_textbox">
            <input
              v-model="authcode"
              class="inputitemtxt"
              type="text"
              maxlength="6"
              autocomplete="off"
              :placeholder="$t('wap_00262')"
            />
            <img
              v-if="captcha?.image"
              class="authcode"
              :src="captcha.image"
              alt=""
              @click="loadCaptcha"
            />
          </div>
          <div class="login_xy">
            <div class="login_xy_zx">
              <input id="xieyicheck" v-model="agreed" type="checkbox" />
            </div>
            <div>
              <i class="policy">{{ $t('wap_00309') }}</i>
              <NuxtLink to="/pages/protocol" class="Privacy">{{ $t('wap_00678') }}</NuxtLink>
              <i class="policy">{{ $t('wap_00679') }}</i>
              <NuxtLink to="/pages/privacy" class="Privacy">{{ $t('wap_00313') }}</NuxtLink>
            </div>
          </div>
          <p v-if="err" class="muted" style="padding-top: 0.16rem">{{ err }}</p>
          <button type="submit" class="login_bth">{{ $t('common.login') }}</button>
        </div>
      </form>
      <form v-else-if="tab === 'sms'" @submit.prevent="submitSms">
        <div class="The_login_subject">
          <div class="login_textbox">
            <input v-model="mobile" type="tel" :placeholder="$t('wap_user_00180')" />
          </div>
          <div v-if="needImageCaptcha" class="login_textbox">
            <input v-model="authcode" class="inputitemtxt" maxlength="6" autocomplete="off" :placeholder="$t('wap_00262')" />
            <img v-if="captcha?.image" class="authcode" :src="captcha.image" alt="" @click="loadCaptcha" />
          </div>
          <div class="login_textbox">
            <input v-model="smsCode" maxlength="6" :placeholder="$t('wap_00677')" />
            <div class="dx_yz_hq" @click="sendSms">{{ $t('wap_user_00144') }}</div>
          </div>
          <p v-if="err" class="muted">{{ err }}</p>
          <button type="submit" class="login_bth">{{ $t('common.login') }}</button>
        </div>
      </form>
      <div class="login_otherfs">
        <div
          v-if="smsLoginOn && tab === 'pass'"
          class="verification_code_word"
          @click="tab = 'sms'"
        >
          {{ $t('wap_00648') }}
        </div>
        <div
          v-else-if="smsLoginOn && tab === 'sms'"
          class="verification_code_word"
          @click="tab = 'pass'"
        >
          {{ $t('wap_00308') }}
        </div>
        <NuxtLink to="/forgetpw" class="login_wjmm">{{ $t('wap_00680') }}</NuxtLink>
      </div>
      <p v-if="oauth.length" class="bottom_nav_top" style="padding: 0.48rem 0">
        {{ $t('wap_00681') }}
      </p>
      <div v-if="oauth.length" class="bottom_nav_center">
        <a
          v-for="o in oauth"
          :key="o.name"
          class="bottom_nav_center_logo"
          :href="o.path"
          @click="sessionStorage.setItem('oauth_provider', o.provider)"
        >
          {{ o.name }}
        </a>
      </div>
    </div>
  </div>
</template>
