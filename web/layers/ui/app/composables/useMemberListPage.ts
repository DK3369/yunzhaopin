export function useMemberListPage(opts?: { pageSize?: number }) {
  const route = useRoute()
  const router = useRouter()
  const pageSize = opts?.pageSize ?? 20
  const page = ref(Math.max(1, Number(route.query.page) || 1))
  const lastTotal = ref<number | undefined>()

  function inferTotal(data: { total?: number; list?: unknown[] } | null | undefined, listOverride?: unknown[]) {
    const list = listOverride ?? data?.list
    const listLen = Array.isArray(list) ? list.length : 0
    if (data?.total != null && Number(data.total) >= 0) {
      lastTotal.value = Number(data.total)
      return lastTotal.value
    }
    const inferred =
      listLen >= pageSize ? page.value * pageSize + 1 : (page.value - 1) * pageSize + listLen
    lastTotal.value = inferred
    return inferred
  }

  function clamp(n: number, total?: number) {
    let next = Math.max(1, Math.floor(n) || 1)
    if (total != null && total >= 0) {
      const maxPage = Math.max(1, Math.ceil(total / pageSize) || 1)
      next = Math.min(next, maxPage)
    }
    return next
  }

  function go(n: number, total?: number) {
    const next = clamp(n, total)
    if (next === page.value) return
    page.value = next
  }

  watch(page, (n) => {
    const q = Number(route.query.page) || 1
    if (q === n) return
    const query = { ...route.query } as Record<string, unknown>
    if (n > 1) query.page = String(n)
    else delete query.page
    router.replace({ query: query as typeof route.query })
  })

  watch(
    () => route.query.page,
    (p) => {
      const n = Math.max(1, Number(p) || 1)
      if (n !== page.value) page.value = n
    },
  )

  watch(lastTotal, (t) => {
    if (t == null) return
    const next = clamp(page.value, t)
    if (next !== page.value) page.value = next
  })

  return { page, pageSize, inferTotal, go, clamp }
}
