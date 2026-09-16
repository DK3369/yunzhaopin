export type { CountryView, PublicDictBundle } from './useSiteBoot'
export { emptyPublicDictBundle } from './useSiteBoot'

/** Shared PC/H5 fetch of `/v1/wap/initjobs`. Same key = one request per locale. */
export function usePublicDicts() {
  return useSiteBoot()
}
