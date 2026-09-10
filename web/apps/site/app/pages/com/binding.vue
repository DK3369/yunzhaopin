<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
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
  for (const [name, path, key] of [
    ['WeChat', '/v1/wap/oauth/wechat/authorize-url', 'wechat'],
    ['QQ', '/v1/wap/oauth/qq/authorize-url', 'qq'],
    ['Weibo', '/v1/wap/oauth/weibo/authorize-url', 'weibo'],
  ] as const) {
    if (bound.value.has(key)) continue
    try {
      const r = await api.post<{ authorize_url?: string }>(path, { redirect_uri })
      if (r.authorize_url) oauth.value.push({ name, path: r.authorize_url, provider: key })
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
  <section>
    <h1>{{ $t('member_user_00059') }}</h1>
    <p v-if="error" class="muted">
      {{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}
    </p>
    <template v-else>
      <nav class="stack">
        <NuxtLink to="/com/profile" class="job-card">{{ $t('wap_user_00341') }}</NuxtLink>
        <NuxtLink to="/com/cert" class="job-card">{{ $t('wap_com_00075') }}</NuxtLink>
        <NuxtLink to="/com/password" class="job-card">{{ $t('member_com_00070') }}</NuxtLink>
        <NuxtLink to="/user/account" class="job-card">{{ $t('member_user_00220') }} / {{ $t('member_com_00538') }}</NuxtLink>
      </nav>
      <h2>{{ $t('wap_00389') }}</h2>
      <p v-if="me?.moblie" class="muted">{{ $t('common.phone') }} {{ maskPhone(String(me.moblie)) }}</p>
      <p v-if="me?.email" class="muted">{{ $t('member_user_00282') }} {{ maskEmail(String(me.email)) }}</p>
      <p v-if="!(data?.providers || []).length" class="muted">{{ $t('ui.no_binding') }}</p>
      <ul v-else class="stack">
        <li v-for="p in data?.providers || []" :key="p">
          {{ p }}
          <button type="button" @click="unbind(p)">{{ $t('wap_js_00065') }}</button>
        </li>
      </ul>
      <p v-if="oauth.length">
        <a
          v-for="o in oauth"
          :key="o.provider"
          :href="o.path"
          style="margin-right: 12px"
          @click.prevent="startBind(o)"
        >{{ o.name }}</a>
      </p>
      <form class="form" @submit.prevent="bindMobile">
        <input v-model="mobile" :placeholder="$t('common.phone')" />
        <button type="button" @click="sendMobile">{{ $t('common.submit') }}</button>
        <input v-model="mobileCode" :placeholder="$t('wap_01371')" />
        <button type="submit">{{ $t('common.save') }}</button>
      </form>
      <form class="form" @submit.prevent="sendEmail">
        <input v-model="email" :placeholder="$t('member_user_00282')" />
        <button type="submit">{{ $t('common.submit') }}</button>
      </form>
      <p class="muted">{{ $t('ajax_00001') }}</p>
      <p><NuxtLink to="/email-verify">{{ $t('wap_user_00179') }}</NuxtLink></p>
      <p v-if="msg">{{ msg }}</p>
    </template>
  </section>
</template>
