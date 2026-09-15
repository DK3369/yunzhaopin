type SettingRow = { key: string; value: string }

/** One `/v1/wap/site/settings` per locale. Member nav and chrome share this. */
export function useSiteSettings() {
  const api = useApi()
  const { data: settingRows } = useAsyncData(
    localeAsyncKey('site-settings'),
    () => api.post<SettingRow[]>('/v1/wap/site/settings', {}).catch(() => [] as SettingRow[]),
    { default: () => [] as SettingRow[], ...reuseAsyncCache() },
  )
  const settings = computed(() => {
    const m: Record<string, string> = {}
    for (const row of settingRows.value || []) m[row.key] = row.value
    return m
  })
  return { settingRows, settings }
}
