<script setup lang="ts">
import type { DictItem } from '~/utils/query'

const { t, locale } = useI18n()
const api = useApi()
const { applyToQuery } = useSubSite()
const form = reactive({
  title: '',
  salary: '',
  address: '',
  require: '',
  companyname: '',
  linkman: '',
  linktel: '',
  password: '',
  mans: '',
  oncepricegear: 0,
  province_id: 0,
  city_id: 0,
  three_city_id: 0,
})
const captcha = ref<{ cid: string; image: string } | null>(null)
const authcode = ref('')
const smsCode = ref('')
const msg = ref('')
const gears = ref<Array<{ id: number; days: number; price: number }>>([])
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
onMounted(async () => {
  await loadCaptcha()
  try {
    gears.value = await api.get('/v1/wap/once-jobs/gears')
    if (gears.value.length && !form.oncepricegear) form.oncepricegear = gears.value[0].id
  } catch {
    gears.value = []
  }
})
async function sendSms() {
  msg.value = ''
  try {
    await api.post('/v1/wap/sms/send', {
      moblie: form.linktel,
      scene: 'once',
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
  try {
    const r = await api.post<{ id: number }>('/v1/wap/once-jobs', applyToQuery({
      ...form,
      captcha_cid: captcha.value?.cid || '',
      authcode: authcode.value,
      moblie_code: smsCode.value,
    }))
    msg.value = t('common.success')
    await navigateTo(`/once/${r.id}`)
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
    loadCaptcha()
  }
}
useSeoMeta({ title: t('wap_01356') })
</script>

<template>
  <section class="site-inner">
    <h1>{{ $t('wap_01356') }}</h1>
    <form class="form" @submit.prevent="submit">
      <input v-model="form.title" :placeholder="$t('wap_01357')" required />
      <input v-model="form.salary" :placeholder="$t('wap_01359')" />
      <select v-model.number="form.province_id">
        <option :value="0">{{ $t('member_com_00378') }}</option>
        <option v-for="p in provinces || []" :key="p.id" :value="p.id">{{ p.name }}</option>
      </select>
      <select v-model.number="form.city_id">
        <option :value="0">{{ $t('common_02110') }}</option>
        <option v-for="c in cities || []" :key="c.id" :value="c.id">{{ c.name }}</option>
      </select>
      <select v-if="(districts || []).length" v-model.number="form.three_city_id">
        <option :value="0">{{ $t('member_com_00378') }}</option>
        <option v-for="d in districts || []" :key="d.id" :value="d.id">{{ d.name }}</option>
      </select>
      <input v-model="form.address" :placeholder="$t('wap_01363')" required />
      <textarea v-model="form.require" rows="5" required />
      <input v-model="form.companyname" :placeholder="$t('wap_01367')" required />
      <input v-model="form.linkman" :placeholder="$t('wap_01368')" required />
      <input v-model="form.linktel" :placeholder="$t('wap_01369')" required />
      <input v-model="form.mans" :placeholder="$t('wap_01360')" />
      <select v-model.number="form.oncepricegear">
        <option v-for="g in gears" :key="g.id" :value="g.id">{{ g.days }}{{ $t('wap_01375') }} · {{ g.price }}</option>
      </select>
      <img v-if="captcha?.image" :src="captcha.image" alt="captcha" @click="loadCaptcha" />
      <input v-model="authcode" :placeholder="$t('ui.image_captcha')" />
      <button type="button" @click="sendSms">{{ $t('wap_01373') }}</button>
      <input v-model="smsCode" :placeholder="$t('wap_01372')" />
      <input v-model="form.password" type="password" :placeholder="$t('wap_01380')" required />
      <button type="submit">{{ $t('wap_00354') }}</button>
      <p v-if="msg">{{ msg }}</p>
    </form>
  </section>
</template>
