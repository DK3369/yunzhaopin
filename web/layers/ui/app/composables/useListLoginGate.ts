import { isUnauthErr } from '../utils/site'

export function useListLoginGate(error: Ref<unknown>) {
  const route = useRoute()
  watch(
    error,
    (e) => {
      if (!e || !isUnauthErr(e)) return
      navigateTo({ path: '/login', query: { next: route.fullPath } })
    },
    { immediate: true },
  )
}
