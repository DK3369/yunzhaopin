<script setup lang="ts">
const { siteName, logoPc, settings } = useSiteChrome()
const { t } = useI18n()
const api = useApi()
const { data: cfg } = await useAsyncData('register-config', () =>
  api.post<{ registration_open?: boolean }>('/v1/wap/register/config', {}).catch(() => ({
    registration_open: true,
  })),
)
const registrationOpen = computed(() => {
  if (String(settings.value.reg_user_stop || '1') !== '1') return false
  return cfg.value?.registration_open !== false
})
const form = reactive({
  username: '',
  password: '',
  captcha_cid: '',
  checkcode: '',
  usertype: 1,
  regway: 1,
  moblie: '',
  moblie_code: '',
  email: '',
})
const captcha = ref<{ cid: string; image: string } | null>(null)
async function loadCaptcha() {
  captcha.value = await api.post('/v1/wap/captcha')
  form.captcha_cid = captcha.value?.cid || ''
}
onMounted(loadCaptcha)
const err = ref('')
async function sendSms() {
  err.value = ''
  try {
    await api.post('/v1/wap/sms/send', {
      moblie: form.moblie,
      scene: 'register',
      captcha_cid: form.captcha_cid,
      authcode: form.checkcode,
    })
  } catch (e: unknown) {
    err.value = e instanceof Error ? e.message : t('common_06630')
    loadCaptcha()
  }
}
async function submit() {
  err.value = ''
  try {
    const me = await $fetch<{ uid: number; usertype: number }>('/api/auth/register', {
      method: 'POST',
      body: {
        ...form,
        email: form.regway === 3 ? form.email : undefined,
        moblie_code: form.regway === 2 ? form.moblie_code : '',
        referrer_uid: Number(useRoute().query.uid || 0) || 0,
      },
    })
    if (me.usertype === 0) {
      await navigateTo('/utype')
      return
    }
    await navigateTo(me.usertype === 2 ? '/com' : '/user')
  } catch (e: unknown) {
    const ex = e as { data?: { statusMessage?: string }; statusMessage?: string }
    err.value = ex.data?.statusMessage || ex.statusMessage || t('common_06630')
    loadCaptcha()
  }
}
useSeoMeta({ title: t('common.register') })
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
          <NuxtLink to="/" class="logo_fh fr">{{ $t('member_user_00119') }} ></NuxtLink>
          <span class="fr" style="margin-right: 16px; line-height: 60px"><LangSwitch /></span>
        </div>
      </div>
      <div class="logoin_cont_box">
        <div class="login_left">
          <p v-if="!registrationOpen" class="muted">{{ $t('ui.registration_closed') }}</p>
          <form v-else class="login_t_box" @submit.prevent="submit">
            <div class="login_box_list">
              <select v-model.number="form.regway" class="login_box_bth">
                <option :value="1">{{ $t('admin_user_00140') }}</option>
                <option :value="2">{{ $t('wap_01619') }}</option>
                <option :value="3">{{ $t('ui.email_addr') }}</option>
              </select>
            </div>
            <div v-if="form.regway === 1" class="login_box_list">
              <input v-model="form.username" class="login_box_bth" :placeholder="$t('admin_user_00140')" />
            </div>
            <div v-if="form.regway === 2" class="login_box_list">
              <input v-model="form.moblie" class="login_box_bth" :placeholder="$t('wap_01619')" />
            </div>
            <div v-if="form.regway === 3" class="login_box_list">
              <input v-model="form.email" class="login_box_bth" :placeholder="$t('ui.email_addr')" />
            </div>
            <div class="login_box_list">
              <input v-model="form.password" type="password" class="login_box_bth" :placeholder="$t('wap_user_00371')" />
            </div>
            <div class="login_box_list">
              <select v-model.number="form.usertype" class="login_box_bth">
                <option :value="1">{{ $t('wap_00686') }}</option>
                <option :value="2">{{ $t('wap_00688') }}</option>
              </select>
            </div>
            <div class="login_box_list">
              <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
              <input v-model="form.checkcode" class="login_box_bth" :placeholder="$t('wap_00110')" />
            </div>
            <div v-if="form.regway === 2" class="login_box_list">
              <input v-model="form.moblie_code" class="login_box_bth" :placeholder="$t('wap_01371')" />
              <button type="button" @click="sendSms">{{ $t('admin_user_00166') }}</button>
            </div>
            <div class="login_box_cz">
              <input type="submit" :value="$t('common.register')" class="login_box_bth2" />
            </div>
            <p v-if="err" class="muted">{{ err }}</p>
            <div class="login_box_fw">
              {{ $t('ui.have_account') }} <NuxtLink to="/login">{{ $t('ui.go_login') }}</NuxtLink>
            </div>
          </form>
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
      <div style="text-align: right; padding: 0.24rem 0.32rem 0"><LangSwitch /></div>
      <div class="login_welcome">
        <div>{{ $t('common.register') }}</div>
        <div>{{ siteName }}</div>
      </div>
      <p v-if="!registrationOpen" class="muted">{{ $t('ui.registration_closed') }}</p>
      <form v-else @submit.prevent="submit">
        <div class="The_login_subject">
          <div class="login_textbox">
            <select v-model.number="form.regway">
              <option :value="1">{{ $t('admin_user_00140') }}</option>
              <option :value="2">{{ $t('wap_01619') }}</option>
              <option :value="3">{{ $t('ui.email_addr') }}</option>
            </select>
          </div>
          <div v-if="form.regway === 1" class="login_textbox">
            <input v-model="form.username" :placeholder="$t('admin_user_00140')" />
          </div>
          <div v-if="form.regway === 2" class="login_textbox">
            <input v-model="form.moblie" :placeholder="$t('wap_01619')" />
          </div>
          <div v-if="form.regway === 3" class="login_textbox">
            <input v-model="form.email" :placeholder="$t('ui.email_addr')" />
          </div>
          <div class="login_textbox">
            <input v-model="form.password" type="password" :placeholder="$t('wap_user_00371')" />
          </div>
          <div class="login_textbox">
            <select v-model.number="form.usertype">
              <option :value="1">{{ $t('wap_00686') }}</option>
              <option :value="2">{{ $t('wap_00688') }}</option>
            </select>
          </div>
          <div class="login_textbox">
            <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
            <input v-model="form.checkcode" :placeholder="$t('wap_00110')" />
          </div>
          <div v-if="form.regway === 2" class="login_textbox">
            <input v-model="form.moblie_code" :placeholder="$t('wap_01371')" />
            <button type="button" @click="sendSms">{{ $t('admin_user_00166') }}</button>
          </div>
        </div>
        <p v-if="err" class="muted">{{ err }}</p>
        <div class="login_bthbox">
          <button type="submit" class="login_bth" style="width: 100%; height: 1.1rem; background: #2778f8; color: #fff; border: 0; border-radius: 0.12rem">
            {{ $t('common.register') }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>
