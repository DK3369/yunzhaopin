<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'
import type { DictItem } from '~/utils/query'

const api = useApi()
const { t, locale } = useI18n()
const { data, error, refresh } = await useAsyncData('com-profile', () =>
  api.post('/v1/mcenter/company/list', {}),
)
const form = reactive({
  name: '',
  shortname: '',
  content: '',
  linkman: '',
  linkphone: '',
  linkmail: '',
  hy: 0,
  pr: 0,
  mun: 0,
  provinceid: 0,
  cityid: 0,
  three_cityid: 0,
  logo: '',
  x: '',
  y: '',
})
watch(
  data,
  (row) => {
    if (!row) return
    form.name = String(row.name || '')
    form.shortname = String(row.shortname || '')
    form.content = String(row.content || '')
    form.linkman = String(row.linkman || '')
    form.linkphone = String(row.linkphone || '')
    form.linkmail = String(row.linkmail || '')
    form.hy = Number(row.hy || 0)
    form.pr = Number(row.pr || 0)
    form.mun = Number(row.mun || 0)
    form.provinceid = Number(row.provinceid || 0)
    form.cityid = Number(row.cityid || 0)
    form.three_cityid = Number(row.three_cityid || 0)
    form.logo = String(row.logo || '')
    form.x = String(row.x || '')
    form.y = String(row.y || '')
  },
  { immediate: true },
)
const { data: industries } = await useAsyncData(
  () => `dict-hy-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/industries').catch(() => [] as DictItem[]),
)
const { data: natures } = await useAsyncData(
  () => `dict-pr-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/company-natures').catch(() => [] as DictItem[]),
)
const { data: sizes } = await useAsyncData(
  () => `dict-mun-${locale.value}`,
  () => api.get<DictItem[]>('/v1/wap/dict/company-sizes').catch(() => [] as DictItem[]),
)
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
  (n, o) => {
    if (o && n !== o) {
      form.cityid = 0
      form.three_cityid = 0
    }
    refreshCities()
  },
)
watch(
  () => form.cityid,
  (n, o) => {
    if (o && n !== o) form.three_cityid = 0
    refreshDistricts()
  },
)
const msg = ref('')
async function onLogo(ev: Event) {
  const file = (ev.target as HTMLInputElement).files?.[0]
  if (!file) return
  try {
    const r = await $fetch<{ key: string; url: string }>('/api/upload/company-logo', {
      method: 'POST',
      body: file,
      headers: { 'content-type': file.type || 'image/jpeg' },
    })
    form.logo = r.key || r.url
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function save() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company', { ...form })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('member_com_00378') })
</script>

<template>
  <section>
    <h1>{{ $t('member_com_00378') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <form v-else class="form" @submit.prevent="save">
      <input v-model="form.name" :placeholder="$t('wap_com_00157')" />
      <input v-model="form.shortname" :placeholder="$t('ui.shortname')" />
      <select v-model.number="form.hy">
        <option :value="0">{{ $t('common.all') }}</option>
        <option v-for="h in industries || []" :key="h.id" :value="h.id">{{ h.name }}</option>
      </select>
      <select v-model.number="form.pr">
        <option :value="0">{{ $t('wap_com_00159') }}</option>
        <option v-for="n in natures || []" :key="n.id" :value="n.id">{{ n.name }}</option>
      </select>
      <select v-model.number="form.mun">
        <option :value="0">{{ $t('member_com_00196') }}</option>
        <option v-for="s in sizes || []" :key="s.id" :value="s.id">{{ s.name }}</option>
      </select>
      <select v-model.number="form.provinceid">
        <option :value="0">{{ $t('member_com_00378') }}</option>
        <option v-for="p in provinces || []" :key="p.id" :value="p.id">{{ p.name }}</option>
      </select>
      <select v-model.number="form.cityid">
        <option :value="0">{{ $t('common_02110') }}</option>
        <option v-for="c in cities || []" :key="c.id" :value="c.id">{{ c.name }}</option>
      </select>
      <select v-if="(districts || []).length" v-model.number="form.three_cityid">
        <option :value="0">{{ $t('member_com_00378') }}</option>
        <option v-for="d in districts || []" :key="d.id" :value="d.id">{{ d.name }}</option>
      </select>
      <input type="file" accept="image/jpeg,image/png,image/webp" @change="onLogo" />
      <textarea v-model="form.content" :placeholder="$t('ui.desc')" rows="6" />
      <input v-model="form.linkman" :placeholder="$t('wap_01431')" />
      <input v-model="form.linkphone" :placeholder="$t('ui.linkphone')" />
      <input v-model="form.linkmail" :placeholder="$t('member_user_00282')" />
      <input v-model="form.x" placeholder="x" />
      <input v-model="form.y" placeholder="y" />
      <button type="submit">{{ $t('common.save') }}</button>
      <p v-if="msg">{{ msg }}</p>
    </form>
  </section>
</template>
