<script setup lang="ts">
import { isUnauthErr, OAUTH_FRONT_PROVIDERS, oauthEnabledByAdmin } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { settings } = useSiteChrome()
const { data: dicts } = await usePublicDicts()
const { data, error, refresh } = await useAsyncData('com-oauth-bindings', () =>
  api.post<{ providers?: string[] }>('/v1/mcenter/oauth-bindings', {}),
)
const { data: me, refresh: refreshMe } = await useAsyncData('com-me-bind', () =>
  api.post<{ moblie?: string | null; email?: string | null }>('/v1/wap/me', {}).catch(() => null),
)
const mobile = ref(String(me.value?.moblie || ''))
const mobileCode = ref('')
const email = ref(String(me.value?.email || ''))
const msg = ref('')
function maskPhone(s: string) {
  const v = s.trim()
  if (v.length < 7) return v
  return `${v.slice(0, 3)}****${v.slice(-4)}`
}
function maskEmail(s: string) {
  const i = s.indexOf('@')
  if (i <= 1) return s
  return `${s[0]}***${s.slice(i)}`
}
const oauth = ref<Array<{ name: string; path: string; provider: string }>>([])
const bound = computed(() => new Set((data.value?.providers || []).map((p) => String(p).toLowerCase())))
const siteUrl = String(useRuntimeConfig().public.siteUrl || '').replace(/\/$/, '')

async function loadOauth() {
  const redirect_uri = `${siteUrl}/login`
  oauth.value = []
  for (const item of OAUTH_FRONT_PROVIDERS) {
    if (bound.value.has(item.key)) continue
    if (!oauthEnabledByAdmin(settings.value, item, dicts.value)) continue
    try {
      const r = await api.post<{ authorize_url?: string }>(item.path, { redirect_uri })
      if (r.authorize_url) oauth.value.push({ name: item.name, path: r.authorize_url, provider: item.key })
    } catch {
      /* not configured */
    }
  }
}
onMounted(loadOauth)
watch(bound, loadOauth)

function startBind(o: { path: string; provider: string }) {
  sessionStorage.setItem('oauth_provider', o.provider)
  sessionStorage.setItem('oauth_intent', 'bind')
  sessionStorage.setItem('oauth_bind_next', '/com/binding')
  window.location.href = o.path
}

function fail(e: unknown) {
  return e instanceof Error ? e.message : t('ui.failed')
}

async function unbind(provider: string) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/oauth-bindings/unbind', { provider })
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function sendMobile() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/cert/mobile/send', { moblie: mobile.value })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function bindMobile() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/cert/mobile/verify', { moblie: mobile.value, moblie_code: mobileCode.value })
    msg.value = t('common.success')
    await refreshMe()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function sendEmail() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/cert/email/send', { email: email.value })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}

useSeoMeta({ title: t('member_user_00059') })
</script>

<template>
  <MemberPanel :title="$t('member_user_00059')">
    <p v-if="error" class="muted">
      {{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}
    </p>
    <template v-else>
      <nav class="job_list_tit">
        <ul>
          <li><NuxtLink to="/com/profile">{{ $t('wap_user_00341') }}</NuxtLink></li>
          <li><NuxtLink to="/com/cert">{{ $t('wap_com_00075') }}</NuxtLink></li>
          <li><NuxtLink to="/com/password">{{ $t('member_com_00070') }}</NuxtLink></li>
          <li class="job_list_tit_cur"><a href="javascript:;">{{ $t('member_user_00059') }}</a></li>
        </ul>
      </nav>
      <h2>{{ $t('wap_00389') }}</h2>
      <p v-if="me?.moblie" class="muted">{{ $t('common.phone') }} {{ maskPhone(String(me.moblie)) }}</p>
      <p v-if="me?.email" class="muted">{{ $t('member_user_00282') }} {{ maskEmail(String(me.email)) }}</p>
      <ul class="bingding_box">
        <li v-for="p in data?.providers || []" :key="p">
          <div class="bingding_box_name">{{ p }}</div>
          <span class="bingding_box_bth_jc" @click="unbind(p)">{{ $t('wap_js_00065') }}</span>
        </li>
        <li v-for="o in oauth" :key="o.provider">
          <div class="bingding_box_name">{{ o.name }}</div>
          <span class="bingding_box_bth" @click="startBind(o)">{{ $t('wap_user_00119') }}</span>
        </li>
      </ul>
      <form class="form verification_form" @submit.prevent="bindMobile">
        <MemberField :label="$t('common.phone')"><input v-model="mobile" /></MemberField>
        <button type="button" class="verification_form_btn" @click="sendMobile">{{ $t('common.submit') }}</button>
        <MemberField :label="$t('wap_01371')"><input v-model="mobileCode" /></MemberField>
        <button type="submit" class="verification_form_btn">{{ $t('common.save') }}</button>
      </form>
      <form class="form verification_form" @submit.prevent="sendEmail">
        <MemberField :label="$t('member_user_00282')"><input v-model="email" /></MemberField>
        <button type="submit" class="verification_form_btn">{{ $t('common.submit') }}</button>
      </form>
      <p class="muted">{{ $t('ajax_00001') }}</p>
      <p><NuxtLink to="/email-verify">{{ $t('wap_user_00179') }}</NuxtLink></p>
      <p v-if="msg">{{ msg }}</p>
    </template>
  </MemberPanel>
</template>
