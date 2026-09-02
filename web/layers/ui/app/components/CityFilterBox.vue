<template>
  <div class="Search_citybox">
    <div class="Search_cityboxname">{{ label }}：</div>
    <div class="Search_citybox_right">
      <div class="Search_cityall" :class="{ none: !allOpen }">
        <NuxtLink
          :to="{ path, query: mergeQuery(route.query, { province_id: undefined, city_id: undefined, three_city_id: undefined }) }"
          class="city_name"
          :class="{ city_name_active: !provinceId }"
          @click="allOpen = false"
        >
          {{ allLabel }}
        </NuxtLink>
        <NuxtLink
          v-for="item in provinces"
          :key="'all-' + item.id"
          :to="{ path, query: mergeQuery(route.query, { province_id: item.id, city_id: undefined, three_city_id: undefined }) }"
          class="city_name"
          :class="{ city_name_active: provinceId === item.id && !cityId }"
          @click="allOpen = false"
        >
          {{ item.name }}
        </NuxtLink>
      </div>
      <div class="Search_cityboxright">
        <a
          class="search_city_list_cur acity_two"
          :class="{
            none: !provinceId,
            search_city_active: !!provinceId && (!cityId || !districts.length),
          }"
          href="javascript:;"
          @click.prevent="clearCity"
        >
          <span class="search_city_p">{{ provinceName }}</span>
          <i class="search_city_p_jt" />
          <i class="search_city_list_line" />
        </a>
        <a
          class="search_city_list_cur acity_three"
          :class="{
            none: !(provinceId && cityId && districts.length),
            search_city_active: !!cityId && !!districts.length,
          }"
          href="javascript:;"
          @click.prevent="clearDistrict"
        >
          <span class="search_city_p">{{ cityName || unlimitedLabel }}</span>
          <i class="search_city_list_line" />
        </a>
        <NuxtLink
          :to="{ path, query: mergeQuery(route.query, { province_id: undefined, city_id: undefined, three_city_id: undefined }) }"
          class="search_city_list_all"
          :class="{ city_name_active: !provinceId }"
        >
          {{ allLabel }}
        </NuxtLink>
        <div class="search_city_list">
          <NuxtLink
            v-for="(item, idx) in provinces"
            v-show="allOpen || idx < visibleLimit"
            :key="item.id"
            :to="{ path, query: mergeQuery(route.query, { province_id: item.id, city_id: undefined, three_city_id: undefined }) }"
            class="city_name"
            :class="{ city_name_active: provinceId === item.id && !cityId }"
          >
            {{ item.name }}
          </NuxtLink>
        </div>
        <a href="javascript:;" class="search_city_list_more" :class="{ showcheck: allOpen }" @click.prevent="allOpen = !allOpen">{{
          moreLabel
        }}</a>
      </div>
      <div class="Search_cityboxclose" :class="{ none: !showCities }">
        <NuxtLink
          :to="{ path, query: mergeQuery(route.query, { city_id: undefined, three_city_id: undefined }) }"
          class="city_name"
          :class="{ city_name_active: !!provinceId && !cityId }"
        >
          {{ unlimitedLabel }}
        </NuxtLink>
        <NuxtLink
          v-for="item in cities"
          :key="'c-' + item.id"
          :to="{ path, query: mergeQuery(route.query, { city_id: item.id, three_city_id: undefined }) }"
          class="city_name"
          :class="{ city_name_active: cityId === item.id }"
        >
          {{ item.name }}
        </NuxtLink>
      </div>
      <div class="Search_cityboxclose" :class="{ none: !showDistricts }">
        <NuxtLink
          :to="{ path, query: mergeQuery(route.query, { three_city_id: undefined }) }"
          class="city_name"
          :class="{ city_name_active: !!cityId && !threeCityId }"
        >
          {{ unlimitedLabel }}
        </NuxtLink>
        <NuxtLink
          v-for="item in districts"
          :key="'d-' + item.id"
          :to="{ path, query: mergeQuery(route.query, { three_city_id: item.id }) }"
          class="city_name"
          :class="{ city_name_active: threeCityId === item.id }"
        >
          {{ item.name }}
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { DictItem } from '../utils/query'

const props = defineProps<{
  label: string
  path: string
  allLabel: string
  unlimitedLabel: string
  moreLabel: string
  provinces: DictItem[]
  cities: DictItem[]
  districts: DictItem[]
  provinceId?: number
  cityId?: number
  threeCityId?: number
}>()

const route = useRoute()
const allOpen = ref(false)

const visibleLimit = computed(() => {
  if (props.cityId) return 13
  if (props.provinceId) return 14
  return 15
})

const provinceName = computed(() => props.provinces.find((x) => x.id === props.provinceId)?.name || '')
const cityName = computed(() => props.cities.find((x) => x.id === props.cityId)?.name || '')
const showDistricts = computed(() => !!(props.cityId && props.districts.length))
const showCities = computed(() => !!(props.provinceId && props.cities.length && !showDistricts.value))

function clearCity() {
  return navigateTo({
    query: mergeQuery(route.query, { city_id: undefined, three_city_id: undefined }),
  })
}

function clearDistrict() {
  return navigateTo({
    query: mergeQuery(route.query, { three_city_id: undefined }),
  })
}

watch(
  () => [props.provinceId, props.cityId] as const,
  () => {
    allOpen.value = false
  },
)
</script>
