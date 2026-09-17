<script setup lang="ts">
const route = useRoute()
const { t } = useI18n()
const api = useApi()
const { siteName, logoPc } = useSiteChrome()
const ticket = computed(() => String(route.query.ticket || ''))
const tab = ref<'bind' | 'reg'>('reg')
const username = ref('')
const password = ref('')
const mobile = ref('')
const smsCode = ref('')
const usertype = ref(1)
const { data: captcha } = await useAsyncData('oauth-bind-captcha', () =>
  api.post<{ cid: string; image: string }>('/v1/wap/captcha').catch(() => null),
)
const authcode = ref('')
const err = ref('')

async function loadCaptcha() {
  try {
    captcha.value = await api.post('/v1/wap/captcha')
  } catch {
    captcha.value = null
  }
}
onMounted(() => {
  if (!ticket.value) err.value = t('ui.oauth_ticket_missing')
})

async function afterLogin(me: { uid: number; usertype: number }) {
  await refreshNuxtData('auth-me')
  if (me.usertype === 0) {
    await navigateTo('/utype')
    return
  }
  await navigateTo(me.usertype === 2 ? '/com' : '/')
}

async function sendSms() {
  err.value = ''
  try {
    await api.post('/v1/wap/sms/send', {
      moblie: mobile.value,
      scene: 'register',
      captcha_cid: captcha.value?.cid,
      authcode: authcode.value,
    })
  } catch (e: unknown) {
    err.value = e instanceof Error ? e.message : t('common_00888')
    loadCaptcha()
  }
}

async function submitReg() {
  err.value = ''
  try {
    const me = await $fetch<{ uid: number; usertype: number }>('/api/auth/oauth-fast-reg', {
      method: 'POST',
      body: {
        ticket: ticket.value,
        moblie: mobile.value,
        moblie_code: smsCode.value,
        password: password.value,
        usertype: usertype.value,
      },
    })
    await afterLogin(me)
  } catch (e: unknown) {
    const ex = e as { data?: { statusMessage?: string }; statusMessage?: string }
    err.value = ex.data?.statusMessage || ex.statusMessage || t('common_00888')
    loadCaptcha()
  }
}

async function submitBind() {
  err.value = ''
  try {
    await $fetch('/api/auth/login', {
      method: 'POST',
      body: {
        username: username.value,
        password: password.value,
        authcode: authcode.value,
        captcha_cid: captcha.value?.cid,
      },
    })
    await api.post('/v1/wap/oauth/bind-pending', { ticket: ticket.value })
    const me = await $fetch<{ uid: number; usertype: number }>('/api/auth/me')
    await afterLogin(me)
  } catch (e: unknown) {
    const ex = e as { data?: { statusMessage?: string }; statusMessage?: string }
    err.value = ex.data?.statusMessage || ex.statusMessage || (e instanceof Error ? e.message : t('common_00888'))
    loadCaptcha()
  }
}

useSeoMeta({ title: t('ui.bind_account') })
function goBack() {
  if (import.meta.client && window.history.length > 1) {
    window.history.back()
    return
  }
  return navigateTo('/login')
}
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
                <li :class="{ login_box_h_list_cur: tab === 'reg' }" @click="tab = 'reg'">
                  {{ $t('ui.fast_reg') }}<i class="login_box_h_icon" />
                </li>
                <li :class="{ login_box_h_list_cur: tab === 'bind' }" @click="tab = 'bind'">
                  {{ $t('common.login') }}<i class="login_box_h_icon" />
                </li>
              </ul>
            </div>
            <div class="login_t_box">
              <form v-if="tab === 'reg'" @submit.prevent="submitReg">
                <div class="login_box_list">
                  <select v-model.number="usertype" class="login_box_bth">
                    <option :value="1">{{ $t('wap_00686') }}</option>
                    <option :value="2">{{ $t('wap_00688') }}</option>
                  </select>
                </div>
                <div class="login_box_list">
                  <input v-model="mobile" class="login_box_bth" autocomplete="tel" :placeholder="$t('wap_01619')" />
                </div>
                <div v-if="captcha?.image" class="login_box_list">
                  <input v-model="authcode" class="login_box_bth" :placeholder="$t('ui.image_captcha')" />
                  <img :src="captcha.image" alt="" @click="loadCaptcha" />
                </div>
                <div class="login_box_list">
                  <input v-model="smsCode" class="login_box_bth" :placeholder="$t('wap_01371')" />
                  <button type="button" @click="sendSms">{{ $t('admin_user_00166') }}</button>
                </div>
                <div class="login_box_list">
                  <input v-model="password" type="password" class="login_box_bth" autocomplete="new-password" :placeholder="$t('wap_user_00371')" />
                </div>
                <div class="login_box_cz">
                  <input type="submit" class="login_box_bth2" :value="$t('common.register')" />
                </div>
              </form>
              <form v-else @submit.prevent="submitBind">
                <div class="login_box_list">
                  <input v-model="username" class="login_box_bth" :placeholder="$t('admin_user_00140')" />
                </div>
                <div class="login_box_list">
                  <input v-model="password" type="password" class="login_box_bth" :placeholder="$t('wap_user_00371')" />
                </div>
                <div v-if="captcha?.image" class="login_box_list">
                  <input v-model="authcode" class="login_box_bth" :placeholder="$t('ui.image_captcha')" />
                  <img :src="captcha.image" alt="" @click="loadCaptcha" />
                </div>
                <div class="login_box_cz">
                  <input type="submit" class="login_box_bth2" :value="$t('ui.bind_account')" />
                </div>
              </form>
            </div>
            <p class="muted">{{ $t('ui.oauth_need_bind') }}</p>
            <p v-if="err" class="muted">{{ err }}</p>
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
      <div class="login_welcome">
        <div>{{ $t('ui.bind_account') }}</div>
        <div>{{ siteName }}</div>
      </div>
      <p class="muted">{{ $t('ui.oauth_need_bind') }}</p>
      <div class="The_login_subject">
        <div class="login_textbox">
          <button type="button" :class="{ on: tab === 'reg' }" @click="tab = 'reg'">{{ $t('ui.fast_reg') }}</button>
          <button type="button" :class="{ on: tab === 'bind' }" @click="tab = 'bind'">{{ $t('common.login') }}</button>
        </div>
      </div>
      <form v-if="tab === 'reg'" @submit.prevent="submitReg">
        <div class="The_login_subject">
          <div class="login_textbox">
            <select v-model.number="usertype">
              <option :value="1">{{ $t('wap_00686') }}</option>
              <option :value="2">{{ $t('wap_00688') }}</option>
            </select>
          </div>
          <div class="login_textbox">
            <input v-model="mobile" autocomplete="tel" :placeholder="$t('wap_01619')" />
          </div>
          <div v-if="captcha?.image" class="login_textbox">
            <input v-model="authcode" :placeholder="$t('ui.image_captcha')" />
            <img :src="captcha.image" alt="" class="authcode" @click="loadCaptcha" />
          </div>
          <div class="login_textbox">
            <input v-model="smsCode" :placeholder="$t('wap_01371')" />
            <div class="dx_yz_hq" @click="sendSms">{{ $t('admin_user_00166') }}</div>
          </div>
          <div class="login_textbox">
            <input v-model="password" type="password" autocomplete="new-password" :placeholder="$t('wap_user_00371')" />
          </div>
        </div>
        <p v-if="err" class="muted">{{ err }}</p>
        <div class="login_bthbox">
          <button type="submit" class="login_bth">{{ $t('common.register') }}</button>
        </div>
      </form>
      <form v-else @submit.prevent="submitBind">
        <div class="The_login_subject">
          <div class="login_textbox">
            <input v-model="username" :placeholder="$t('admin_user_00140')" />
          </div>
          <div class="login_textbox">
            <input v-model="password" type="password" :placeholder="$t('wap_user_00371')" />
          </div>
          <div v-if="captcha?.image" class="login_textbox">
            <input v-model="authcode" :placeholder="$t('ui.image_captcha')" />
            <img :src="captcha.image" alt="" class="authcode" @click="loadCaptcha" />
          </div>
        </div>
        <p v-if="err" class="muted">{{ err }}</p>
        <div class="login_bthbox">
          <button type="submit" class="login_bth">{{ $t('ui.bind_account') }}</button>
        </div>
      </form>
    </div>
  </div>
</template>
