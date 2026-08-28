<script setup lang="ts">
const { siteName, logoPc } = useSiteChrome()
const { t } = useI18n()
const api = useApi()
const form = reactive({
  username: '',
  password: '',
  captcha_cid: '',
  checkcode: '',
  usertype: 1,
  regway: 1,
})
const captcha = ref<{ cid: string; image: string } | null>(null)
async function loadCaptcha() {
  captcha.value = await api.post('/v1/wap/captcha')
  form.captcha_cid = captcha.value?.cid || ''
}
onMounted(loadCaptcha)
const err = ref('')
async function submit() {
  err.value = ''
  try {
    await api.post('/v1/wap/register', { ...form })
    await navigateTo('/login')
  } catch (e: unknown) {
    err.value = e instanceof Error ? e.message : t('common_06630')
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
          <form class="login_t_box" @submit.prevent="submit">
            <div class="login_box_list">
              <input v-model="form.username" class="login_box_bth" :placeholder="$t('admin_user_00140')" />
            </div>
            <div class="login_box_list">
              <input v-model="form.password" type="password" class="login_box_bth" :placeholder="$t('wap_user_00371')" />
            </div>
            <div class="login_box_list">
              <select v-model.number="form.usertype" class="login_box_bth">
                <option :value="1">{{ $t('wap_00686') }}</option>
                <option :value="2">{{ $t('ui.hire') }}</option>
              </select>
            </div>
            <div class="login_box_list">
              <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
              <input v-model="form.checkcode" class="login_box_bth" :placeholder="$t('wap_00110')" />
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
      <form @submit.prevent="submit">
        <div class="The_login_subject">
          <div class="login_textbox">
            <input v-model="form.username" :placeholder="$t('admin_user_00140')" />
          </div>
          <div class="login_textbox">
            <input v-model="form.password" type="password" :placeholder="$t('wap_user_00371')" />
          </div>
          <div class="login_textbox">
            <select v-model.number="form.usertype">
              <option :value="1">{{ $t('wap_00686') }}</option>
              <option :value="2">{{ $t('ui.hire') }}</option>
            </select>
          </div>
          <div class="login_textbox">
            <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
            <input v-model="form.checkcode" :placeholder="$t('wap_00110')" />
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
