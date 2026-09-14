<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'
import { runPackedTranslate } from '../../../../../layers/base/app/utils/packedText'

const api = useApi()
const { t, te, locale, getLocaleMessage } = useI18n()
function packed(text: unknown) {
  const loc = String(locale.value).toLowerCase().startsWith('en') ? 'en' : 'zh'
  const zhRoot = getLocaleMessage('zh') as Record<string, unknown>
  return runPackedTranslate(text, {
    locale: loc,
    lc: (k) => (te(k) ? String(t(k)) : k),
    zhRoot,
  })
}
const { data: bal, error } = await useAsyncData('user-finance-bal', () =>
  api.post('/v1/mcenter/integral/balance', {}),
)
const { data: pays } = await useAsyncData('user-finance-pay', () =>
  api.post('/v1/mcenter/integral/consumes', { page: 1, page_size: 20 }).catch(() => ({ list: [] })),
)
const { data: rewards } = await useAsyncData('user-finance-ex', () =>
  api.post('/v1/mcenter/integral/history', { page: 1, page_size: 20 }).catch(() => ({ list: [] })),
)
const { data: signSt, refresh: refreshSign } = await useAsyncData('user-finance-sign', () =>
  api.post<{ signed_today?: boolean }>('/v1/mcenter/sign/status', {}).catch(() => null),
)
const msg = ref('')
async function sign() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/sign', {})
    msg.value = t('common.success')
    await refreshSign()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('wap_user_00213') })
</script>

<template>
  <MemberPanel :title="$t('wap_user_00213')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <p v-else>{{ $t('ui.balance') }} {{ bal?.balance ?? 0 }}</p>
    <p>
      <button type="button" :disabled="signSt?.signed_today" @click="sign">{{ $t('wap_01023') }}</button>
    </p>
    <div class="stack">
      <NuxtLink to="/user/integral" class="jobnotice_list">{{ $t('wap_user_00008') }}</NuxtLink>
      <NuxtLink to="/user/pay" class="jobnotice_list">{{ $t('common_01946') }}</NuxtLink>
      <NuxtLink to="/redeem" class="jobnotice_list">{{ $t('wap_user_00170') }}</NuxtLink>
      <NuxtLink to="/invite" class="jobnotice_list">{{ $t('wap_user_00253') }}</NuxtLink>
    </div>
    <h2>{{ $t('wap_user_00213') }}</h2>
    <article v-for="row in pays?.list || []" :key="row.id" class="jobnotice_list">
      <p>{{ packed(row.detail) }} · {{ row.delta }}</p>
      <p class="muted">{{ row.ctime_n || row.ctime }}</p>
    </article>
    <h2>{{ $t('member_user_00190') }}</h2>
    <article v-for="row in rewards?.list || []" :key="row.id" class="jobnotice_list">
      <p>{{ row.item_name || row.item_id }} · {{ row.cost }}</p>
      <p class="muted">{{ row.created_at_n || row.created_at }}</p>
    </article>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
