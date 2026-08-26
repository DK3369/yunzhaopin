<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('oauth-bindings', () =>
  api.post<{ providers?: string[] }>('/v1/mcenter/oauth-bindings', {}),
)
const msg = ref('')
async function unbind(provider: string) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/oauth-bindings/unbind', { provider })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('ui.binding') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.binding') }}</h1>
    <p v-if="error" class="muted">{{ $t('wap_00376') }}</p>
    <p v-else-if="!(data?.providers || []).length" class="muted">{{ $t('ui.no_binding') }}</p>
    <ul v-else class="stack">
      <li v-for="p in data?.providers || []" :key="p">
        {{ p }}
        <button type="button" @click="unbind(p)">{{ $t('ui.unbind') }}</button>
      </li>
    </ul>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
