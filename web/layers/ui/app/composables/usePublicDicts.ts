import type { DictItem } from '../utils/query'

export type CountryView = {
  id: number
  code: string
  code3?: string
  numeric_code?: number
  name: string
  name_en?: string
  name_zh?: string
  continent?: string
  phone_code?: string
  currency?: string
  flag?: string
  sort?: number
}

export type PublicDictBundle = {
  countries: CountryView[]
  educations: DictItem[]
  educations_user: DictItem[]
  experiences: DictItem[]
  experiences_user: DictItem[]
  salaries: DictItem[]
  industries: DictItem[]
  welfares: DictItem[]
  reports: DictItem[]
  reports_user: DictItem[]
  job_types: DictItem[]
  job_types_user: DictItem[]
  company_natures: DictItem[]
  company_sizes: DictItem[]
}

export function emptyPublicDictBundle(): PublicDictBundle {
  return {
    countries: [],
    educations: [],
    educations_user: [],
    experiences: [],
    experiences_user: [],
    salaries: [],
    industries: [],
    welfares: [],
    reports: [],
    reports_user: [],
    job_types: [],
    job_types_user: [],
    company_natures: [],
    company_sizes: [],
  }
}

/** Shared PC/H5 fetch of `/v1/wap/initjobs`. Same key = one request per locale. */
export function usePublicDicts() {
  const api = useApi()
  const { locale } = useI18n()
  return useAsyncData(
    () => `wap-initjobs-${locale.value}`,
    () => api.get<PublicDictBundle>('/v1/wap/initjobs').catch(() => emptyPublicDictBundle()),
  )
}
