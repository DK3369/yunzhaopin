<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const form = reactive({ email: '', content: t('ui.invite_body') })
const msg = ref('')
async function send() {
  msg.value = ''
  try {
    const r = await api.post<{ invite_id: number }>('/v1/mcenter/invite-reg', { ...form })
    msg.value = `${t('ui.send')} invite_id ${r.invite_id}`
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('ui.invite_reg') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.invite_reg') }}</h1>
    <form class="form" @submit.prevent="send">
      <input v-model="form.email" type="email" :placeholder="$t('member_user_00282')" required />
      <textarea v-model="form.content" rows="4" :placeholder="$t('wap_user_00102')" />
      <button type="submit">{{ $t('ui.send_invite_reg') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
