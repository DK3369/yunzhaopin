<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const form = reactive({ old_password: '', new_password: '', confirm: '' })
const msg = ref('')
async function submit() {
  msg.value = ''
  if (form.new_password && form.confirm && form.new_password !== form.confirm) {
    msg.value = t('wap_01104')
    return
  }
  try {
    await api.post('/v1/mcenter/password', { old_password: form.old_password, new_password: form.new_password })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('member_user_00226') })
</script>

<template>
  <MemberPanel :title="$t('member_user_00226')">
    <form class="form verification_form" @submit.prevent="submit">
      <MemberField :label="$t('wap_01096')">
        <input v-model="form.old_password" type="password" :placeholder="$t('wap_01097')" />
      </MemberField>
      <MemberField :label="$t('wap_user_00305')">
        <input v-model="form.new_password" type="password" :placeholder="$t('wap_01099')" />
      </MemberField>
      <MemberField :label="$t('wap_01098')">
        <input v-model="form.confirm" type="password" :placeholder="$t('wap_01099')" />
      </MemberField>
      <button type="submit" class="verification_form_btn">{{ $t('wap_js_00094') }}</button>
    </form>
    <p v-if="msg" class="muted">{{ msg }}</p>
  </MemberPanel>
</template>
