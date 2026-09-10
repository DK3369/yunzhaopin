<script setup lang="ts">
import { ApiError } from '~/utils/envelope'

const { siteName, logoPc, settings, me } = useSiteChrome()
const { t } = useI18n()
const api = useApi()
const { data: cfg } = await useAsyncData('register-config', () =>
  api
    .post<{
      registration_open?: boolean
      reg_user?: boolean
      reg_moblie?: boolean
      reg_email?: boolean
    }>('/v1/wap/register/config', {})
    .catch(() => ({ registration_open: true, reg_user: true, reg_moblie: true, reg_email: true })),
)
const registrationOpen = computed(() => {
  if (String(settings.value.reg_user_stop || '1') !== '1') return false
  return cfg.value?.registration_open !== false
})
const allowUser = computed(() => cfg.value?.reg_user !== false)
const allowMobile = computed(() => cfg.value?.reg_moblie !== false)
const allowEmail = computed(() => cfg.value?.reg_email !== false)
const alreadyIn = computed(() => Boolean(me.value && Number(me.value.usertype) !== 0))
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
  c_name: '',
})
watch(
  () => [allowUser.value, allowMobile.value, allowEmail.value],
  () => {
    if (form.regway === 1 && !allowUser.value) form.regway = allowMobile.value ? 2 : 3
    else if (form.regway === 2 && !allowMobile.value) form.regway = allowUser.value ? 1 : 3
    else if (form.regway === 3 && !allowEmail.value) form.regway = allowMobile.value ? 2 : 1
  },
  { immediate: true },
)
const captcha = ref<{ cid: string; image: string } | null>(null)
async function loadCaptcha() {
  captcha.value = await api.post('/v1/wap/captcha')
  form.captcha_cid = captcha.value?.cid || ''
}
onMounted(async () => {
  if (alreadyIn.value) {
    await navigateTo(Number(me.value?.usertype) === 2 ? '/com' : '/user')
    return
  }
  await loadCaptcha()
})
const err = ref('')
const comNameHint = ref('')
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
async function checkComName() {
  comNameHint.value = ''
  const name = form.c_name.trim()
  if (!name || form.usertype !== 2) return
  try {
    const r = await api.post<{ taken?: boolean }>('/v1/wap/register/check-com-name', { c_name: name })
    if (r.taken) comNameHint.value = t('wap_js_00054')
  } catch (e: unknown) {
    comNameHint.value = e instanceof Error ? e.message : ''
  }
}
async function submit() {
  err.value = ''
  if (alreadyIn.value) {
    err.value = t('wap_00416')
    return
  }
  if (form.usertype === 2 && form.c_name.trim() && comNameHint.value) return
  try {
    const logged = await $fetch<{ uid: number; usertype: number }>('/api/auth/register', {
      method: 'POST',
      body: {
        ...form,
        email: form.regway === 3 ? form.email : undefined,
        moblie_code: form.regway === 2 ? form.moblie_code : '',
        referrer_uid: Number(useRoute().query.uid || 0) || 0,
      },
    })
    if (logged.usertype === 0) {
      await navigateTo('/utype')
      return
    }
    await navigateTo(logged.usertype === 2 ? '/com' : '/user')
  } catch (e: unknown) {
    if (e instanceof ApiError) err.value = e.message
    else {
      const ex = e as { data?: { statusMessage?: string }; statusMessage?: string }
      err.value = ex.data?.statusMessage || ex.statusMessage || t('common_06630')
    }
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
          <p v-if="alreadyIn" class="muted">{{ $t('wap_00416') }}</p>
          <p v-else-if="!registrationOpen" class="muted">{{ $t('ui.registration_closed') }}</p>
          <form v-else class="login_t_box" @submit.prevent="submit">
            <div class="login_box_list">
              <select v-model.number="form.regway" class="login_box_bth">
                <option v-if="allowUser" :value="1">{{ $t('admin_user_00140') }}</option>
                <option v-if="allowMobile" :value="2">{{ $t('wap_01619') }}</option>
                <option v-if="allowEmail" :value="3">{{ $t('member_user_00282') }}</option>
              </select>
            </div>
            <div v-if="form.regway === 1" class="login_box_list">
              <input v-model="form.username" class="login_box_bth" :placeholder="$t('admin_user_00140')" />
            </div>
            <div v-if="form.regway === 2" class="login_box_list">
              <input v-model="form.moblie" class="login_box_bth" :placeholder="$t('wap_01619')" />
            </div>
            <div v-if="form.regway === 3" class="login_box_list">
              <input v-model="form.email" class="login_box_bth" :placeholder="$t('member_user_00282')" />
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
            <div v-if="form.usertype === 2" class="login_box_list">
              <input
                v-model="form.c_name"
                class="login_box_bth"
                :placeholder="$t('ui.company_name')"
                @blur="checkComName"
              />
            </div>
            <p v-if="comNameHint" class="muted">{{ comNameHint }}</p>
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
      <p v-if="alreadyIn" class="muted">{{ $t('wap_00416') }}</p>
      <p v-else-if="!registrationOpen" class="muted">{{ $t('ui.registration_closed') }}</p>
      <form v-else @submit.prevent="submit">
        <div class="The_login_subject">
          <div class="login_textbox">
            <select v-model.number="form.regway">
              <option v-if="allowUser" :value="1">{{ $t('admin_user_00140') }}</option>
              <option v-if="allowMobile" :value="2">{{ $t('wap_01619') }}</option>
              <option v-if="allowEmail" :value="3">{{ $t('member_user_00282') }}</option>
            </select>
          </div>
          <div v-if="form.regway === 1" class="login_textbox">
            <input v-model="form.username" :placeholder="$t('admin_user_00140')" />
          </div>
          <div v-if="form.regway === 2" class="login_textbox">
            <input v-model="form.moblie" :placeholder="$t('wap_01619')" />
          </div>
          <div v-if="form.regway === 3" class="login_textbox">
            <input v-model="form.email" :placeholder="$t('member_user_00282')" />
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
          <div v-if="form.usertype === 2" class="login_textbox">
            <input v-model="form.c_name" :placeholder="$t('ui.company_name')" @blur="checkComName" />
          </div>
          <p v-if="comNameHint" class="muted">{{ comNameHint }}</p>
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
