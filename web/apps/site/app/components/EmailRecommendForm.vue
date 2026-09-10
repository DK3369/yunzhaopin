<script setup lang="ts">
const props = defineProps<{ kind: 'job' | 'resume'; id: number }>()
const api = useApi()
const { t } = useI18n()
const { me } = useSiteChrome()
const email = ref('')
const message = ref('')
const hint = ref('')
const quota = ref('')

async function loadQuota() {
  quota.value = ''
  if (!me.value) return
  try {
    const r = await api.post<{
      status?: number
      msg?: string
      used_today?: number
      day_cap?: number
    }>('/v1/mcenter/recommend/email/quota', {})
    quota.value = r.msg || `${r.used_today ?? 0}/${r.day_cap ?? 0}`
  } catch (e: unknown) {
    quota.value = e instanceof Error ? e.message : ''
  }
}

async function send() {
  hint.value = ''
  if (!me.value) {
    await navigateTo('/login')
    return
  }
  if (!props.id) {
    hint.value = t('ui.failed')
    return
  }
  try {
    await api.post('/v1/mcenter/recommend/email', {
      kind: props.kind,
      id: props.id,
      email: email.value,
      message: message.value || undefined,
    })
    hint.value = t('common.success')
    await loadQuota()
  } catch (e: unknown) {
    hint.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

onMounted(loadQuota)
</script>

<template>
  <form v-if="id" class="form" style="margin: 12px 0" @submit.prevent="send">
    <p>{{ $t('common.share') }} · {{ $t('member_user_00282') }}</p>
    <p v-if="quota" class="muted">{{ quota }}</p>
    <input v-model="email" type="email" required :placeholder="$t('member_user_00282')" />
    <textarea v-model="message" rows="2" />
    <button type="submit">{{ $t('common.submit') }}</button>
    <p v-if="hint" class="muted">{{ hint }}</p>
  </form>
</template>
