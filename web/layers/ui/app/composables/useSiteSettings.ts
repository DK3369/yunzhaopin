/** One `/v1/wap/initjobs?with=site` per locale. Member nav and chrome share this. */
export function useSiteSettings() {
  const { data: boot } = useSiteBoot()
  const settings = computed(() => boot.value?.settings || {})
  const settingRows = computed(() =>
    Object.entries(settings.value).map(([key, value]) => ({ key, value })),
  )
  return { settingRows, settings }
}
