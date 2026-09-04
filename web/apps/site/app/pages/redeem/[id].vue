<script setup lang="ts">
import type { DictItem } from '~/utils/query'

const id = Number(useRoute().params.id)
const { t, locale } = useI18n()
const { me } = useSiteChrome()
const api = useApi()
const { data } = await useAsyncData(`reward-${id}`, () =>
  api.get('/v1/wap/redeem/rewards/detail', { id }),
)
const form = reactive({
  password: '',
  linkman: '',
  linktel: '',
  address: '',
  provinceid: 0,
  cityid: 0,
  three_cityid: 0,
  num: 1,
})
const { data: provinces } = await useAsyncData(
  () => `dict-city-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/cities').catch(() => [] as DictItem[]),
)
const { data: cities, refresh: refreshCities } = await useAsyncData(
  () => `dict-city-child-${locale.value}-${form.provinceid}`,
  () =>
    form.provinceid
      ? api.get<DictItem[]>('/v1/wap/dict/cities/by-province', { province_id: form.provinceid }).catch(() => [] as DictItem[])
      : Promise.resolve([] as DictItem[]),
)
const { data: districts, refresh: refreshDistricts } = await useAsyncData(
  () => `dict-city-dist-${locale.value}-${form.cityid}`,
  () =>
    form.cityid
      ? api.get<DictItem[]>('/v1/wap/dict/cities/by-province', { province_id: form.cityid }).catch(() => [] as DictItem[])
      : Promise.resolve([] as DictItem[]),
)
watch(
  () => form.provinceid,
  () => {
    form.cityid = 0
    form.three_cityid = 0
    refreshCities()
  },
)
watch(
  () => form.cityid,
  () => {
    form.three_cityid = 0
    refreshDistricts()
  },
)
const msg = ref('')
async function submit() {
  msg.value = ''
  if (!me.value) {
    await navigateTo('/login')
    return
  }
  try {
    await api.post('/v1/mcenter/redeem/rewards/redeem', { id, ...form })
    msg.value = t('model_00051')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
useSeoMeta({ title: () => String(data.value?.name || t('ui.redeem')) })
useHead({ link: [{ rel: 'canonical', href: `/redeem/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.name || $t('wap_00611') }}</h1>
    <p v-if="data?.integral" class="muted">{{ data.integral }} {{ $t('wap_user_00008') }} · {{ $t('admin_yunying_00118') }} {{ data.remaining }}</p>
    <div v-if="data?.content" v-html="data.content" />
    <p v-else-if="!data?.name" class="muted">{{ $t('wap_00611') }}</p>
    <form v-if="data?.name" class="form" @submit.prevent="submit">
      <input v-model="form.password" type="password" required :placeholder="$t('wap_01273')" />
      <input v-model="form.linkman" required :placeholder="$t('wap_01619')" />
      <input v-model="form.linktel" required />
      <select v-model.number="form.provinceid">
        <option :value="0">{{ $t('common.all') }}</option>
        <option v-for="p in provinces || []" :key="p.id" :value="p.id">{{ p.name }}</option>
      </select>
      <select v-model.number="form.cityid">
        <option :value="0">{{ $t('common.all') }}</option>
        <option v-for="c in cities || []" :key="c.id" :value="c.id">{{ c.name }}</option>
      </select>
      <select v-if="(districts || []).length" v-model.number="form.three_cityid">
        <option :value="0">{{ $t('common.all') }}</option>
        <option v-for="d in districts || []" :key="d.id" :value="d.id">{{ d.name }}</option>
      </select>
      <input v-model="form.address" />
      <input v-model.number="form.num" type="number" min="1" />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </article>
</template>
