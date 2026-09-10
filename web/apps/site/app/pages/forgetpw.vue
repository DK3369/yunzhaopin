<script setup lang="ts">
const { siteName, logoPc } = useSiteChrome()
const api = useApi()
const { t } = useI18n()
const channel = ref<'sms' | 'email' | 'appeal'>('sms')
const form = reactive({
  moblie: '',
  email: '',
  captcha_cid: '',
  authcode: '',
  moblie_code: '',
  email_code: '',
  password: '',
  account: '',
  linkman: '',
  linkphone: '',
  linkemail: '',
})
const captcha = ref<{ cid: string; image: string } | null>(null)
const msg = ref('')
async function loadCaptcha() {
  captcha.value = await api.post('/v1/wap/captcha')
  form.captcha_cid = captcha.value?.cid || ''
}
onMounted(loadCaptcha)
async function sendSms() {
  msg.value = ''
  try {
    await api.post('/v1/wap/forgetpw/send-sms', {
      moblie: form.moblie,
      captcha_cid: form.captcha_cid,
      authcode: form.authcode,
    })
    msg.value = t('ui.sms_sent')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.send_failed')
    loadCaptcha()
  }
}
async function sendEmail() {
  msg.value = ''
  try {
    await api.post('/v1/wap/forgetpw/send-email', {
      email: form.email,
      captcha_cid: form.captcha_cid,
      authcode: form.authcode,
    })
    msg.value = t('ui.sms_sent')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.send_failed')
    loadCaptcha()
  }
}
async function resetPw() {
  msg.value = ''
  try {
    if (channel.value === 'appeal') {
      await api.post('/v1/wap/forgetpw/appeal', {
        account: form.account,
        linkman: form.linkman,
        linkphone: form.linkphone,
        linkemail: form.linkemail,
      })
      msg.value = t('wap_00756')
      return
    }
    if (channel.value === 'email') {
      await api.post('/v1/wap/forgetpw/reset-by-email', {
        email: form.email,
        email_code: form.email_code,
        password: form.password,
      })
    } else {
      await api.post('/v1/wap/forgetpw/reset', {
        moblie: form.moblie,
        moblie_code: form.moblie_code,
        password: form.password,
      })
    }
    msg.value = t('common_01549')
    await navigateTo('/login')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.reset_failed')
  }
}
useSeoMeta({ title: t('wap_js_00123') })
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
          <NuxtLink to="/login" class="logo_fh fr">{{ $t('common.login') }} ></NuxtLink>
          <span class="fr" style="margin-right: 16px; line-height: 60px"><LangSwitch /></span>
        </div>
      </div>
      <div class="logoin_cont_box">
        <div class="login_left">
          <form class="login_t_box" @submit.prevent="resetPw">
            <div class="login_box_h1_d">
              <ul class="login_box_h_list">
                <li :class="{ login_box_h_list_cur: channel === 'sms' }" @click="channel = 'sms'">{{ $t('wap_01619') }}</li>
                <li :class="{ login_box_h_list_cur: channel === 'email' }" @click="channel = 'email'">{{ $t('member_user_00282') }}</li>
                <li :class="{ login_box_h_list_cur: channel === 'appeal' }" @click="channel = 'appeal'">{{ $t('wap_00754') }}</li>
              </ul>
            </div>
            <template v-if="channel === 'appeal'">
              <p class="muted" style="padding: 8px 0">{{ $t('wap_00754') }}</p>
              <div class="login_box_list">
                <input v-model="form.account" class="login_box_bth" :placeholder="$t('wap_00208')" />
              </div>
              <div class="login_box_list">
                <input v-model="form.linkman" class="login_box_bth" :placeholder="$t('wap_com_00013')" />
              </div>
              <div class="login_box_list">
                <input v-model="form.linkphone" class="login_box_bth" :placeholder="$t('wap_com_00322')" />
              </div>
              <div class="login_box_list">
                <input v-model="form.linkemail" class="login_box_bth" :placeholder="$t('wap_com_00009')" />
              </div>
            </template>
            <template v-else>
              <div class="login_box_list">
                <input
                  v-if="channel === 'sms'"
                  v-model="form.moblie"
                  class="login_box_bth"
                  :placeholder="$t('wap_01619')"
                  autocomplete="tel"
                />
                <input
                  v-else
                  v-model="form.email"
                  class="login_box_bth"
                  :placeholder="$t('member_user_00282')"
                  autocomplete="email"
                />
              </div>
              <div class="login_box_list">
                <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
                <input v-model="form.authcode" class="login_box_bth" :placeholder="$t('wap_00110')" />
              </div>
              <div class="login_box_list">
                <input
                  v-if="channel === 'sms'"
                  v-model="form.moblie_code"
                  class="login_box_bth"
                  :placeholder="$t('wap_01371')"
                />
                <input
                  v-else
                  v-model="form.email_code"
                  class="login_box_bth"
                  :placeholder="$t('wap_01371')"
                />
                <button v-if="channel === 'sms'" type="button" @click="sendSms">{{ $t('admin_user_00166') }}</button>
                <button v-else type="button" @click="sendEmail">{{ $t('admin_user_00166') }}</button>
              </div>
              <div class="login_box_list">
                <input v-model="form.password" type="password" class="login_box_bth" :placeholder="$t('wap_user_00305')" autocomplete="new-password" />
              </div>
            </template>
            <div class="login_box_cz">
              <input type="submit" :value="$t('common_01878')" class="login_box_bth2" />
            </div>
            <p v-if="msg" class="muted">{{ msg }}</p>
          </form>
        </div>
      </div>
    </div>
  </div>
  <div class="site-h5">
    <div class="Back_to_the_previous_level">
      <NuxtLink to="/login" class="login_back">
        <img src="/legacy/h5/images/return.png" alt="" width="100%" height="100%" />
      </NuxtLink>
    </div>
    <div class="login_cont">
      <div style="text-align: right; padding: 0.24rem 0.32rem 0"><LangSwitch /></div>
      <div class="login_welcome">
        <div>{{ $t('wap_js_00123') }}</div>
      </div>
      <div class="login_otherfs" style="padding: 0.24rem 0">
        <span class="verification_code_word" @click="channel = 'sms'">{{ $t('wap_01619') }}</span>
        <span class="verification_code_word" @click="channel = 'email'">{{ $t('member_user_00282') }}</span>
        <span class="verification_code_word" @click="channel = 'appeal'">{{ $t('wap_00754') }}</span>
      </div>
      <form @submit.prevent="resetPw">
        <div class="The_login_subject">
          <template v-if="channel === 'appeal'">
            <p class="muted">{{ $t('wap_00754') }}</p>
            <div class="login_textbox">
              <input v-model="form.account" :placeholder="$t('wap_00208')" />
            </div>
            <div class="login_textbox">
              <input v-model="form.linkman" :placeholder="$t('wap_com_00013')" />
            </div>
            <div class="login_textbox">
              <input v-model="form.linkphone" :placeholder="$t('wap_com_00322')" />
            </div>
            <div class="login_textbox">
              <input v-model="form.linkemail" :placeholder="$t('wap_com_00009')" />
            </div>
          </template>
          <template v-else>
            <div class="login_textbox">
              <input v-if="channel === 'sms'" v-model="form.moblie" :placeholder="$t('wap_01619')" autocomplete="tel" />
              <input v-else v-model="form.email" :placeholder="$t('member_user_00282')" autocomplete="email" />
            </div>
            <div class="login_textbox">
              <input v-model="form.authcode" :placeholder="$t('wap_00110')" />
              <img v-if="captcha?.image" class="authcode" :src="captcha.image" alt="" @click="loadCaptcha" />
            </div>
            <div class="login_textbox">
              <input v-if="channel === 'sms'" v-model="form.moblie_code" :placeholder="$t('wap_01371')" />
              <input v-else v-model="form.email_code" :placeholder="$t('wap_01371')" />
              <div v-if="channel === 'sms'" class="dx_yz_hq" @click="sendSms">{{ $t('admin_user_00166') }}</div>
              <div v-else class="dx_yz_hq" @click="sendEmail">{{ $t('admin_user_00166') }}</div>
            </div>
            <div class="login_textbox">
              <input v-model="form.password" type="password" :placeholder="$t('wap_user_00305')" autocomplete="new-password" />
            </div>
          </template>
          <p v-if="msg" class="muted">{{ msg }}</p>
          <button type="submit" class="login_bth">{{ $t('common_01878') }}</button>
        </div>
      </form>
    </div>
  </div>
</template>
