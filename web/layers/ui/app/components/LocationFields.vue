<script setup lang="ts">
const country = defineModel<string>('country', { default: '' })
const provinceId = defineModel<number>('provinceId', { default: 0 })
const cityId = defineModel<number>('cityId', { default: 0 })
const districtId = defineModel<number>('districtId', { default: 0 })
const props = defineProps<{ required?: boolean }>()

const api = useApi()
const { countryItems, provinceItems, cityItems, districtItems, countryRegionId } = await useRegionCascade({
  country,
  provinceId,
  cityId,
})

watch(country, (n, o) => {
  if (o && n !== o) {
    provinceId.value = 0
    cityId.value = 0
    districtId.value = 0
  }
})
watch([country, provinceItems, countryRegionId], () => {
  if (!country.value || provinceItems.value.length) return
  const rid = countryRegionId.value
  if (rid && provinceId.value !== rid) provinceId.value = rid
})
watch(provinceId, (n, o) => {
  if (o && n !== o) {
    cityId.value = 0
    districtId.value = 0
  }
})
watch(cityId, (n, o) => {
  if (o && n !== o) districtId.value = 0
})

onMounted(async () => {
  if (provinceId.value && !country.value) {
    try {
      const node = await api.get<{ country_code?: string }>('/v1/wap/regions/get', {
        id: provinceId.value,
      })
      if (node?.country_code) country.value = node.country_code
    } catch {
      /* stored id is not a region node */
    }
  }
})
</script>

<template>
  <select v-model="country">
    <option value="">{{ $t('common.country') }}</option>
    <option v-for="c in countryItems" :key="c.code" :value="c.code">{{ c.name }}</option>
  </select>
  <select v-if="provinceItems.length" v-model.number="provinceId">
    <option :value="0">{{ $t('member_com_00378') }}</option>
    <option v-for="p in provinceItems" :key="p.id" :value="p.id">{{ p.name }}</option>
  </select>
  <select v-if="cityItems.length" v-model.number="cityId" :required="!!props.required">
    <option :value="0">{{ $t('common_02110') }}</option>
    <option v-for="c in cityItems" :key="c.id" :value="c.id">{{ c.name }}</option>
  </select>
  <select v-if="districtItems.length" v-model.number="districtId">
    <option :value="0">{{ $t('member_com_00378') }}</option>
    <option v-for="d in districtItems" :key="d.id" :value="d.id">{{ d.name }}</option>
  </select>
</template>
