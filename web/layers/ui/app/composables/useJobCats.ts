import type { CatNode } from '../utils/site'

/** Shared job category tree. Same key = one request per locale. */
export function useJobCats() {
  const api = useApi()
  return useAsyncData(
    localeAsyncKey('job-cats'),
    () => api.get<CatNode[]>('/v1/wap/categories', { kind: 'job' }).catch(() => [] as CatNode[]),
    { default: () => [] as CatNode[] },
  )
}

/** Shared part-time category list. */
export function usePartCats() {
  const api = useApi()
  return useAsyncData(
    localeAsyncKey('part-cats'),
    () => api.get<CatNode[]>('/v1/wap/categories', { kind: 'part' }).catch(() => [] as CatNode[]),
    { default: () => [] as CatNode[] },
  )
}
