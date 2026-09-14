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
    <MemberUserSetTabs />
    <div class="site-pc account_settings">
      <div class="account_settings_list">
        <div class="account_settings_list_left">
          <i class="account_settings_list_left_icon account_settings_list_left_icon_user" />
          <div class="account_settings_tit">{{ $t('wap_user_00336') }}</div>
          {{ $t('member_user_00535') }}
        </div>
        <NuxtLink to="/user/account" class="account_settings_bth_hv">{{ $t('wap_js_00073') }}</NuxtLink>
      </div>
      <div class="account_settings_list">
        <div class="account_settings_list_left">
          <i class="account_settings_list_left_icon account_settings_list_left_icon_m" />
          <div class="account_settings_tit">{{ $t('member_user_00536') }}</div>
          {{ $t('member_user_00537') }}
        </div>
      </div>
      <div class="account_settings_list">
        <div class="account_settings_list_left">
          <i class="account_settings_list_left_icon account_settings_list_left_icon_yx" />
          <div class="account_settings_tit">{{ $t('wap_user_00179') }}</div>
        </div>
        <NuxtLink to="/user/binding" class="account_settings_bth_hv">{{ $t('member_user_00234') }}</NuxtLink>
      </div>
      <div class="account_settings_list">
        <div class="account_settings_list_left">
          <i class="account_settings_list_left_icon account_settings_list_left_icon_sj" />
          <div class="account_settings_tit">{{ $t('wap_user_00180') }}</div>
        </div>
        <NuxtLink to="/user/binding" class="account_settings_bth_hv">{{ $t('member_user_00234') }}</NuxtLink>
      </div>
    </div>
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
