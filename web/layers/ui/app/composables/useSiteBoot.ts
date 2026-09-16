import type { DictItem } from '../utils/query'
import type { CatNode } from '../utils/site'

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
  marriages: DictItem[]
  langs: DictItem[]
  tags: DictItem[]
  job_categories: DictItem[]
  sy_googlelogin?: string
  sy_facebooklogin?: string
}

export const SITE_BOOT_WITH = 'site,nav,footer,cats,register,map,subscribe,stats,hot'

export type SiteBootNav = {
  id: number
  label: string
  url: string
  icon?: string
  icon_n?: string
  parent_id?: number
  sort?: number
  config?: string
}

export type SiteBootReason = { id?: number; code: string; name: string }

export type SiteBootRegister = {
  registration_open?: boolean
  reg_user?: boolean
  reg_moblie?: boolean
  reg_email?: boolean
}

export type SiteBootMap = { map_x?: string; map_y?: string }

export type SiteBootSubscribe = { jionly?: number; cionly?: number; cycles?: number[] }

export type SiteBootHot = { keyword: string }

export type SiteBoot = PublicDictBundle & {
  settings?: Record<string, string>
  report_reasons?: SiteBootReason[]
  nav?: SiteBootNav[]
  footer_classes?: Array<{ id: number; name: string }>
  footer_pages?: Array<{
    id: number
    class_id: number
    name?: string
    title: string
    is_nav?: number
    link_url?: string
    is_type?: number
  }>
  job_cats?: CatNode[]
  part_cats?: CatNode[]
  hot_job_class?: CatNode[]
  register?: SiteBootRegister
  map?: SiteBootMap
  subscribe?: SiteBootSubscribe
  stats?: Record<string, number>
  hot_searches?: SiteBootHot[]
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
    marriages: [],
    langs: [],
    tags: [],
    job_categories: [],
    sy_googlelogin: '',
    sy_facebooklogin: '',
  }
}

export function emptySiteBoot(): SiteBoot {
  return emptyPublicDictBundle()
}

/** One `/v1/wap/initjobs?with=` per locale for PC/H5 chrome + public config. */
export function useSiteBoot() {
  const api = useApi()
  return useAsyncData(
    localeAsyncKey('wap-initjobs'),
    () =>
      api
        .get<SiteBoot>('/v1/wap/initjobs', { with: SITE_BOOT_WITH })
        .catch(() => emptySiteBoot()),
    { default: () => emptySiteBoot(), ...reuseAsyncCache() },
  )
}
