<script setup lang="ts">
const { siteName, logoPc, logoH5 } = useSiteChrome()
const { t } = useI18n()
const api = useApi()
const tab = ref<'pass' | 'sms'>('pass')
const username = ref('')
const password = ref('')
const mobile = ref('')
const smsCode = ref('')
const { data: captcha } = await useAsyncData('login-captcha', () =>
  api.post<{ cid: string; image: string }>('/v1/wap/captcha').catch(() => null),
)
const authcode = ref('')
const err = ref('')
const oauth = ref<Array<{ name: string; path: string }>>([])
const siteUrl = String(useRuntimeConfig().public.siteUrl || '').replace(/\/$/, '')

async function loadCaptcha() {
  try {
    captcha.value = await api.post('/v1/wap/captcha')
  } catch {
    captcha.value = null
  }
}
onMounted(async () => {
  if (!captcha.value) await loadCaptcha()
  const redirect_uri = `${siteUrl}/login`
  for (const [name, path] of [
    ['WeChat', '/v1/wap/oauth/wechat/authorize-url'],
    ['QQ', '/v1/wap/oauth/qq/authorize-url'],
    ['Weibo', '/v1/wap/oauth/weibo/authorize-url'],
  ] as const) {
    try {
      const r = await api.post<{ authorize_url?: string }>(path, { redirect_uri })
      if (r.authorize_url) oauth.value.push({ name, path: r.authorize_url })
    } catch {
      /* not configured */
    }
  }
})

async function afterLogin(me: { uid: number; usertype: number }) {
  await navigateTo(me.usertype === 2 ? '/com' : '/user')
}
async function submitPass() {
  err.value = ''
  try {
    const me = await $fetch<{ uid: number; usertype: number }>('/api/auth/login', {
      method: 'POST',
      body: {
        username: username.value,
        password: password.value,
        authcode: authcode.value || undefined,
        captcha_cid: captcha.value?.cid,
      },
    })
    await afterLogin(me)
  } catch (e: unknown) {
    const ex = e as { data?: { statusMessage?: string }; statusMessage?: string }
    err.value = ex.data?.statusMessage || ex.statusMessage || t('common.no')
    loadCaptcha()
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
    err.value = e instanceof Error ? e.message : t('common.no')
    loadCaptcha()
  }
}
async function submitSms() {
  err.value = ''
  try {
    const me = await $fetch<{ uid: number; usertype: number }>('/api/auth/login-sms', {
      method: 'POST',
      body: { moblie: mobile.value, dynamiccode: smsCode.value },
    })
    await afterLogin(me)
  } catch (e: unknown) {
    const ex = e as { data?: { statusMessage?: string }; statusMessage?: string }
    err.value = ex.data?.statusMessage || ex.statusMessage || t('common.no')
  }
}
useSeoMeta({ title: t('common.login') })
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
                <li :class="{ login_box_h_list_cur: tab === 'sms' }" @click="tab = 'sms'">
                  {{ $t('ui.sms_login') }}
                </li>
              </ul>
            </div>
            <form v-if="tab === 'pass'" class="login_t_box" @submit.prevent="submitPass">
              <div class="login_box_cot">
                <div class="login_normal_box">
                  <div class="login_box_list">
                    <i class="login_box_icon login_box_username" />
                    <input v-model="username" class="login_box_bth placeholder loginname" autocomplete="username" :placeholder="$t('ui.username')" />
                  </div>
                  <div class="login_box_list">
                    <i class="login_box_icon loginpwd" />
                    <input v-model="password" type="password" class="login_box_bth placeholder loginname" autocomplete="current-password" :placeholder="$t('ui.password')" />
                  </div>
                  <div v-if="captcha?.image" class="login_box_list">
                    <img :src="captcha.image" alt="" @click="loadCaptcha" />
                    <input v-model="authcode" class="login_box_bth" :placeholder="$t('ui.captcha_ph')" />
                  </div>
                </div>
                <div class="login_box_cz">
                  <input type="submit" :value="$t('common.login')" class="login_box_bth2" />
                </div>
                <p v-if="err" class="muted" style="padding: 8px 0">{{ err }}</p>
                <div class="login_box_fw">
                  <span class="fl">{{ $t('common.register') }} <NuxtLink to="/register">{{ $t('common.register') }}</NuxtLink></span>
                  <NuxtLink to="/forgetpw" class="fr">{{ $t('ui.forget_pw') }}</NuxtLink>
                </div>
                <p v-if="oauth.length" style="padding: 12px 0">
                  <a v-for="o in oauth" :key="o.name" :href="o.path" style="margin-right: 12px">{{ o.name }}</a>
                </p>
              </div>
            </form>
            <form v-else class="login_t_box" @submit.prevent="submitSms">
              <div class="login_box_list">
                <input v-model="mobile" class="login_box_bth" :placeholder="$t('common.phone')" />
              </div>
              <div v-if="captcha?.image" class="login_box_list">
                <img :src="captcha.image" alt="" @click="loadCaptcha" />
                <input v-model="authcode" class="login_box_bth" />
              </div>
              <div class="login_box_list">
                <input v-model="smsCode" class="login_box_bth" :placeholder="$t('ui.sms_code')" />
                <button type="button" @click="sendSms">{{ $t('common.submit') }}</button>
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
      <NuxtLink to="/" class="login_back">
        <img src="/legacy/h5/images/return.png" alt="" width="100%" height="100%" />
      </NuxtLink>
    </div>
    <div class="login_cont">
      <div class="bottom_nav_bom" style="padding-top: 0; text-align: right">
        <LangSwitch />
        <NuxtLink to="/register" class="register_1" style="margin-left: 12px">{{ $t('common.register') }}</NuxtLink>
      </div>
      <div class="login_welcome">
        <div>{{ $t('common.login') }}</div>
        <div>{{ siteName }}</div>
      </div>
      <p>
        <a href="javascript:;" @click.prevent="tab = 'pass'">{{ $t('common.login') }}</a>
        ·
        <a href="javascript:;" @click.prevent="tab = 'sms'">{{ $t('common.phone') }}</a>
      </p>
      <form v-if="tab === 'pass'" @submit.prevent="submitPass">
        <div class="The_login_subject">
          <div class="login_textbox">
            <input v-model="username" type="text" class="account_number" autocomplete="username" />
          </div>
          <div class="login_textbox">
            <input v-model="password" type="password" autocomplete="current-password" />
          </div>
          <div v-if="captcha?.image" class="login_textbox" style="display: flex; gap: 0.16rem; align-items: center">
            <img :src="captcha.image" alt="" style="height: 0.8rem" @click="loadCaptcha" />
            <input v-model="authcode" :placeholder="$t('ui.captcha_ph')" />
          </div>
        </div>
        <p v-if="err" class="muted">{{ err }}</p>
        <div class="login_bthbox">
          <button type="submit" class="login_bth" style="width: 100%; height: 1.1rem; background: #2778f8; color: #fff; border: 0">
            {{ $t('common.login') }}
          </button>
        </div>
      </form>
      <form v-else @submit.prevent="submitSms">
        <div class="login_textbox"><input v-model="mobile" :placeholder="$t('common.phone')" /></div>
        <div class="login_textbox"><input v-model="smsCode" :placeholder="$t('ui.sms_code')" /><button type="button" @click="sendSms">{{ $t('common.submit') }}</button></div>
        <p v-if="err" class="muted">{{ err }}</p>
        <button type="submit" class="login_bth" style="width: 100%; height: 1.1rem; background: #2778f8; color: #fff; border: 0">
          {{ $t('common.login') }}
        </button>
      </form>
      <p v-if="oauth.length" style="padding: 0.32rem">
        <a v-for="o in oauth" :key="o.name" :href="o.path">{{ o.name }}</a>
      </p>
    </div>
  </div>
</template>
