<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-tpls', () =>
  api.post<Array<Record<string, unknown>>>('/v1/mcenter/company-tpls', {}),
)
const msg = ref('')
async function apply(row: { id: number }) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company-tpls/apply', { id: row.id })
    msg.value = t('ui.apply_tpl')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('ui.com_tpl') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.com_tpl') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('ui.please_login_com') : $t('ui.load_failed') }}</p>
    <p v-if="msg">{{ msg }}</p>
    <div class="stack">
      <article v-for="row in data || []" :key="row.id">
        <h3>{{ row.name }}</h3>
        <p class="muted">kind {{ row.kind }} status {{ row.status }}</p>
        <button type="button" @click="apply(row)">{{ $t('ui.apply_tpl') }}</button>
      </article>
    </div>
    <p><NuxtLink to="/com">{{ $t('ui.back_com') }}</NuxtLink></p>
  </section>
</template>
