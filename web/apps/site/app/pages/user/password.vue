<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const form = reactive({ old_password: '', new_password: '', confirm: '' })
const msg = ref('')
const showPass = ref(false)
async function submit() {
  msg.value = ''
  if (form.new_password && form.confirm && form.new_password !== form.confirm) {
    msg.value = t('wap_01104')
    return
  }
  try {
    await api.post('/v1/mcenter/password', { old_password: form.old_password, new_password: form.new_password })
    msg.value = t('common.success')
    showPass.value = false
    form.old_password = ''
    form.new_password = ''
    form.confirm = ''
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('member_user_00226') })
</script>

<template>
  <MemberPanel :title="$t('member_user_00059')">
    <MemberUserSetTabs />
    <div class="site-pc account_settings">
      <div class="account_settings_list">
        <div class="account_settings_list_left">
          <i class="account_settings_list_left_icon account_settings_list_left_icon_user" />
          <div class="account_settings_tit">{{ $t('wap_user_00336') }}</div>
          <div class="account_settings_tip">{{ $t('member_user_00535') }}</div>
        </div>
        <NuxtLink to="/user/account" class="account_settings_bth">{{ $t('wap_js_00073') }}</NuxtLink>
      </div>
      <div class="account_settings_list">
        <div class="account_settings_list_left">
          <i class="account_settings_list_left_icon account_settings_list_left_icon_m" />
          <div class="account_settings_tit">{{ $t('member_user_00536') }}</div>
          <div class="account_settings_tip">{{ $t('member_user_00537') }}</div>
        </div>
        <a href="javascript:;" class="account_settings_bth_hv" @click.prevent="showPass = true">{{ $t('wap_js_00073') }}</a>
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
      <div class="account_settings_list">
        <div class="account_settings_list_left">
          <i class="account_settings_list_left_icon account_settings_list_left_icon_sf" />
          <div class="account_settings_tit">{{ $t('wap_01030') }}</div>
          <div class="account_settings_tip">{{ $t('member_user_00210') }}</div>
        </div>
        <NuxtLink to="/user/ident" class="account_settings_bth_hv">{{ $t('member_user_00235') }}</NuxtLink>
      </div>
    </div>
    <div v-if="showPass" class="site-pc Binding_pop_mask" @click.self="showPass = false">
      <form class="Binding_pop_box" @submit.prevent="submit">
        <div class="Binding_pop_box_msg">{{ $t('member_user_00537') }}</div>
        <div class="Binding_pop_box_list">
          <span class="Binding_pop_box_list_left"><i class="Binding_pop_box_list_left_i">*</i>{{ $t('member_user_00227') }}</span>
          <input v-model="form.old_password" type="password" class="Binding_pop_box_list_text Binding_pop_box_list_textw200" />
        </div>
        <div class="Binding_pop_box_list">
          <span class="Binding_pop_box_list_left"><i class="Binding_pop_box_list_left_i">*</i>{{ $t('wap_user_00305') }}</span>
          <input v-model="form.new_password" type="password" class="Binding_pop_box_list_text Binding_pop_box_list_textw200" />
        </div>
        <div class="Binding_pop_box_list">
          <span class="Binding_pop_box_list_left"><i class="Binding_pop_box_list_left_i">*</i>{{ $t('wap_com_00343') }}</span>
          <input v-model="form.confirm" type="password" class="Binding_pop_box_list_text Binding_pop_box_list_textw200" />
        </div>
        <div class="Binding_pop_sub">
          <button type="submit" class="layui-btn layui-btn-normal">{{ $t('common.confirm') }}</button>
          <button type="button" class="layui-btn layui-btn-primary" @click="showPass = false">{{ $t('common.cancel') }}</button>
        </div>
      </form>
    </div>
    <form class="site-h5 verification_form" @submit.prevent="submit">
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
