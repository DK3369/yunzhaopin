export type AdBanner = {
  id?: number
  image_n?: string
  image?: string
  link?: string
  title?: string
  pic_content?: string
  html?: string
}

export type AdSlotNeed = { slot: string; limit?: number }

/** Shared PC/H5 fetch of `/v1/wap/initads`. One request for many slots. */
export function useAdsBundle(key: string, needs: AdSlotNeed[]) {
  const api = useApi()
  const empty: Record<string, AdBanner[]> = {}
  for (const n of needs) empty[n.slot] = []
  return useAsyncData(key, async () => {
    try {
      const data = await api.post<Record<string, AdBanner[]>>('/v1/wap/initads', {
        slots: needs.map((n) => ({ slot: n.slot, limit: n.limit ?? 10 })),
      } as Record<string, unknown>)
      return { ...empty, ...data }
    } catch {
      return { ...empty }
    }
  })
}
