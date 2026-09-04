<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data: bal, error, refresh: refreshBal } = await useAsyncData('integral-bal', () =>
  api.post('/v1/mcenter/integral/balance', {}),
)
const { data: hist } = await useAsyncData('integral-hist', () =>
  api.post('/v1/mcenter/integral/history', { page: 1, page_size: 20 }),
)
const { data: signSt, refresh: refreshSign } = await useAsyncData('sign-status', () =>
  api.post<{ signed_today?: boolean; signday?: number; signdays?: number }>('/v1/mcenter/sign/status', {}).catch(() => null),
)
const msg = ref('')
async function sign() {
  msg.value = ''
  try {
    const r = await api.post<{ reward?: number }>('/v1/mcenter/sign', {})
    msg.value = `${t('common.success')} ${r?.reward ?? ''}`
    await refreshSign()
    await refreshBal()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('wap_user_00008') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_user_00008') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <p v-else>{{ $t('ui.balance') }} {{ bal?.balance ?? 0 }}</p>
    <p>
      <button type="button" :disabled="signSt?.signed_today" @click="sign">{{ $t('wap_01023') }}</button>
      <span v-if="signSt" class="muted"> {{ signSt.signday ?? 0 }} / {{ signSt.signdays ?? 0 }}</span>
    </p>
    <h2>{{ $t('ui.flow') }}</h2>
    <p v-if="!(hist?.list || []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <div class="stack">
      <article v-for="(row, i) in hist?.list || []" :key="row.id || i" class="job-card">
        <h3>{{ row.item_id || row.id }}</h3>
        <p class="muted">{{ row.cost ?? row.delta ?? '' }} · {{ row.status ?? '' }} · {{ row.created_at || row.ctime }}</p>
      </article>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
