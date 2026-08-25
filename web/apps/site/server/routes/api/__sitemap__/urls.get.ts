export default defineEventHandler(async () => {
  const rustApi = useRuntimeConfig().rustApi
  try {
    const res = await $fetch<{ code: number; data?: { list?: { id: number }[] } }>(
      `${rustApi}/v1/wap/jobs`,
      { method: 'GET', query: { page: 1, page_size: 50 } },
    )
    const list = res.data?.list ?? []
    return list.map((j) => ({ loc: `/jobs/${j.id}` }))
  } catch {
    return [{ loc: '/' }, { loc: '/jobs' }, { loc: '/companies' }]
  }
})
