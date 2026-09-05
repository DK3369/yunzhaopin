<script setup lang="ts">
import type { DictItem } from '~/utils/query'

const { t, locale } = useI18n()
const api = useApi()
const { applyToQuery } = useSubSite()
const form = reactive({
  username: '',
  sex: 1,
  exp: 1,
  job: '',
  mobile: '',
  password: '',
  province_id: 0,
  city_id: 0,
  three_city_id: 0,
  production: '',
})
const captcha = ref<{ cid: string; image: string } | null>(null)
const authcode = ref('')
const smsCode = ref('')
const msg = ref('')
const { data: provinces } = await useAsyncData(
  () => `dict-city-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/cities').catch(() => [] as DictItem[]),
)
const { data: cities, refresh: refreshCities } = await useAsyncData(
  () => `dict-city-child-${locale.value}-${form.province_id}`,
  () =>
    form.province_id
      ? api.get<DictItem[]>('/v1/wap/dict/cities/by-province', { province_id: form.province_id }).catch(() => [] as DictItem[])
      : Promise.resolve([] as DictItem[]),
)
const { data: districts, refresh: refreshDistricts } = await useAsyncData(
  () => `dict-city-dist-${locale.value}-${form.city_id}`,
  () =>
    form.city_id
      ? api.get<DictItem[]>('/v1/wap/dict/cities/by-province', { province_id: form.city_id }).catch(() => [] as DictItem[])
      : Promise.resolve([] as DictItem[]),
)
watch(
  () => form.province_id,
  () => {
    form.city_id = 0
    form.three_city_id = 0
    refreshCities()
  },
)
watch(
  () => form.city_id,
  () => {
    form.three_city_id = 0
    refreshDistricts()
  },
)
async function loadCaptcha() {
  captcha.value = await api.post('/v1/wap/captcha')
}
onMounted(loadCaptcha)
async function sendSms() {
  msg.value = ''
  try {
    await api.post('/v1/wap/sms/send', {
      moblie: form.mobile,
      scene: 'tiny',
      captcha_cid: captcha.value?.cid,
      authcode: authcode.value,
    })
    msg.value = t('ui.sms_sent')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.send_failed')
    loadCaptcha()
  }
}
async function submit() {
  msg.value = ''
  if (!form.city_id) {
    msg.value = t('common_02110')
    return
  }
  try {
    await api.post('/v1/wap/tiny-resumes', applyToQuery({
      ...form,
      captcha_cid: captcha.value?.cid || '',
      authcode: authcode.value,
      moblie_code: smsCode.value,
    }))
    msg.value = t('common.success')
    await navigateTo('/tiny')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
    loadCaptcha()
  }
}
useSeoMeta({ title: t('default_00331') })
</script>

<template>
  <section class="site-inner">
    <h1>{{ $t('default_00331') }}</h1>
    <form class="form" @submit.prevent="submit">
      <input v-model="form.username" :placeholder="$t('admin_user_00140')" required />
      <select v-model.number="form.sex">
        <option :value="1">{{ $t('common_02092') }}</option>
        <option :value="2">{{ $t('common_02069') }}</option>
      </select>
      <input v-model="form.job" :placeholder="$t('common.job')" required />
      <select v-model.number="form.province_id">
        <option :value="0">{{ $t('member_com_00378') }}</option>
        <option v-for="p in provinces || []" :key="p.id" :value="p.id">{{ p.name }}</option>
      </select>
      <select v-model.number="form.city_id" required>
        <option :value="0">{{ $t('common_02110') }}</option>
        <option v-for="c in cities || []" :key="c.id" :value="c.id">{{ c.name }}</option>
      </select>
      <select v-if="(districts || []).length" v-model.number="form.three_city_id">
        <option :value="0">{{ $t('member_com_00378') }}</option>
        <option v-for="d in districts || []" :key="d.id" :value="d.id">{{ d.name }}</option>
      </select>
      <input v-model="form.mobile" :placeholder="$t('wap_01619')" required />
      <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
      <input v-model="authcode" :placeholder="$t('ui.image_captcha')" />
      <button type="button" @click="sendSms">{{ $t('admin_user_00166') }}</button>
      <input v-model="smsCode" :placeholder="$t('wap_01371')" />
      <input v-model="form.password" type="password" :placeholder="$t('wap_user_00371')" required />
      <textarea v-model="form.production" rows="5" required />
      <button type="submit">{{ $t('common.submit') }}</button>
      <p v-if="msg">{{ msg }}</p>
    </form>
  </section>
</template>
