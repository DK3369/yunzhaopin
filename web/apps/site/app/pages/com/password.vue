<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const form = reactive({ old_password: '', new_password: '' })
const msg = ref('')
async function submit() {
  try {
    await api.post('/v1/mcenter/password', { ...form })
    msg.value = t('common.confirm')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common.no')
  }
}
useSeoMeta({ title: t('member_user_00226') })
</script>

<template>
  <MemberPanel :title="$t('member_user_00226')">
    <form class="form" @submit.prevent="submit">
      <input v-model="form.old_password" type="password" :placeholder="$t('ui.old_password')" />
      <input v-model="form.new_password" type="password" :placeholder="$t('ui.new_password')" />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
