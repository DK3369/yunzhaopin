<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { me, settings } = useSiteChrome()
const form = reactive({
  infotype: 'advice',
  content: '',
  moblie: '',
  username: '',
  captcha_cid: '',
  captcha_input: '',
  moblie_code: '',
})
const captcha = ref<{ cid: string; image: string } | null>(null)
const msg = ref('')
async function loadCaptcha() {
  captcha.value = await api.post('/v1/wap/captcha')
  form.captcha_cid = captcha.value?.cid || ''
  form.captcha_input = ''
}
onMounted(async () => {
  if (me.value) {
    form.username = me.value.username || ''
  }
  await loadCaptcha()
})
async function sendSms() {
  msg.value = ''
  try {
    await api.post('/v1/wap/sms/send', {
      moblie: form.moblie,
      scene: 'advice',
      captcha_cid: form.captcha_cid,
      authcode: form.captcha_input,
    })
    msg.value = t('ui.sms_sent')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.send_failed')
    await loadCaptcha()
  }
}
async function submit() {
  msg.value = ''
  try {
    await api.post('/v1/wap/advice', { ...form })
    msg.value = t('common.success')
    form.content = ''
    await loadCaptcha()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common.no')
    await loadCaptcha()
  }
}
useSeoMeta({ title: t('wap_user_00203') })
</script>

<template>
  <MemberPanel :title="$t('wap_user_00203')">
    <div class="resume_Prompt_box">
      <div class="resume_Prompt"><i class="resume_Prompt_icon" />{{ $t('wap_js_00125') }}</div>
    </div>
    <form class="resume_fk_box" @submit.prevent="submit">
      <ul class="message_box">
        <li>
          <span class="msg_fk">{{ $t('admin_system_00199') }}：</span>
          <textarea v-model="form.content" cols="60" rows="6" class="message_box_text msg_bx_tt" required />
        </li>
        <li>
          <span class="msg_fk">{{ $t('wap_01619') }}：</span>
          <input v-model="form.moblie" class="yun_send_resume_txt" />
        </li>
        <li v-if="String(settings.sy_advice_mobilecode || '') === '1'">
          <button type="button" class="verification_form_btn" @click="sendSms">{{ $t('admin_user_00166') }}</button>
          <MemberField :label="$t('wap_01371')"><input v-model="form.moblie_code" /></MemberField>
        </li>
        <li>
          <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
          <MemberField :label="$t('wap_00110')"><input v-model="form.captcha_input" /></MemberField>
        </li>
        <li>
          <span>&nbsp;</span>
          <input type="submit" class="Verification_sc_bth2 uesr_submit" :value="$t('common.submit')" />
        </li>
      </ul>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
