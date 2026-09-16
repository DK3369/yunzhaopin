/** Public settings from the shared `/v1/wap/initjobs` boot bundle. */
export function useSiteSettings() {
  const { data: boot } = useSiteBoot()
  const settings = computed(() => boot.value?.settings || {})
  const settingRows = computed(() =>
    Object.entries(settings.value).map(([key, value]) => ({ key, value })),
  )
  return { settingRows, settings }
}
