export function useMemberListPage(opts?: { pageSize?: number }) {
  const page = ref(1)
  const pageSize = opts?.pageSize ?? 20

  function inferTotal(data: { total?: number; list?: unknown[] } | null | undefined, listOverride?: unknown[]) {
    const list = listOverride ?? data?.list
    const listLen = Array.isArray(list) ? list.length : 0
    if (data?.total != null && Number(data.total) >= 0) return Number(data.total)
    if (listLen >= pageSize) return page.value * pageSize + 1
    return (page.value - 1) * pageSize + listLen
  }

  function go(n: number) {
    const next = Math.max(1, Math.floor(n) || 1)
    if (next === page.value) return
    page.value = next
  }

  return { page, pageSize, inferTotal, go }
}
