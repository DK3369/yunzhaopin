import type { DictItem } from '../utils/query'

export type CountryOpt = { code: string; name: string }

type CountryView = { code: string; name: string; flag?: string }
type RegionView = { id: number; name: string; country_code?: string }

function asId(v: unknown): number {
  const n = Number(v || 0)
  return Number.isFinite(n) && n > 0 ? n : 0
}

/** Country from `/v1/wap/countries`, cities from `/v1/wap/regions` — not city_class.
 *  All `useAsyncData` must be registered before any `await` (Nuxt E1001).
 */
export async function useRegionCascade(opts: {
  country: { value: string }
  provinceId: { value: number }
  cityId: { value: number }
}) {
  const { locale } = useI18n()
  const api = useApi()
  const country = computed(() => String(opts.country.value || '').trim().toUpperCase())
  const provinceId = computed(() => asId(opts.provinceId.value))
  const cityId = computed(() => asId(opts.cityId.value))

  const countriesAsync = useAsyncData(
    () => `countries-${locale.value}`,
    () => api.get<CountryView[]>('/v1/wap/countries').catch(() => [] as CountryView[]),
  )
  const countryRootsAsync = useAsyncData(
    () => `regions-l0-${locale.value}-${country.value}`,
    () =>
      country.value
        ? api
            .get<RegionView[]>('/v1/wap/regions', { country: country.value, level: 0 })
            .catch(() => [] as RegionView[])
        : Promise.resolve([] as RegionView[]),
  )
  const provincesAsync = useAsyncData(
    () => `regions-l1-${locale.value}-${country.value}`,
    () =>
      country.value
        ? api
            .get<RegionView[]>('/v1/wap/regions', { country: country.value, level: 1 })
            .catch(() => [] as RegionView[])
        : Promise.resolve([] as RegionView[]),
  )
  const citiesAsync = useAsyncData(
    () => `regions-child-${locale.value}-${provinceId.value}`,
    () =>
      provinceId.value
        ? api
            .get<RegionView[]>('/v1/wap/regions/children', { id: provinceId.value })
            .catch(() => [] as RegionView[])
        : Promise.resolve([] as RegionView[]),
  )
  const districtsAsync = useAsyncData(
    () => `regions-dist-${locale.value}-${cityId.value}`,
    () =>
      cityId.value
        ? api
            .get<RegionView[]>('/v1/wap/regions/children', { id: cityId.value })
            .catch(() => [] as RegionView[])
        : Promise.resolve([] as RegionView[]),
  )
  const [
    { data: countries },
    { data: countryRoots },
    { data: provinces },
    { data: cities },
    { data: districts },
  ] = await Promise.all([countriesAsync, countryRootsAsync, provincesAsync, citiesAsync, districtsAsync])

  const countryItems = computed<CountryOpt[]>(() =>
    (countries.value || []).map((c) => ({
      code: c.code,
      name: c.flag ? `${c.flag} ${c.name}` : c.name,
    })),
  )
  const countryDictItems = computed<DictItem[]>(() =>
    countryItems.value.map((c, i) => ({ id: i + 1, name: c.name, code: c.code })),
  )
  const provinceItems = computed<DictItem[]>(() =>
    (provinces.value || []).map((p) => ({ id: p.id, name: p.name })),
  )
  const cityItems = computed<DictItem[]>(() =>
    (cities.value || []).map((p) => ({ id: p.id, name: p.name })),
  )
  const districtItems = computed<DictItem[]>(() =>
    (districts.value || []).map((p) => ({ id: p.id, name: p.name })),
  )
  const countryRegionId = computed(() => asId(countryRoots.value?.[0]?.id))

  return { countryItems, countryDictItems, provinceItems, cityItems, districtItems, countryRegionId }
}
