import type { CatNode } from '../utils/site'

/** Shared job category tree. Same key = one request per locale. */
export function useJobCats() {
  const { data: boot } = useSiteBoot()
  const data = computed(() => (boot.value?.job_cats ?? []) as CatNode[])
  return { data }
}

/** Shared part-time category list. */
export function usePartCats() {
  const { data: boot } = useSiteBoot()
  const data = computed(() => (boot.value?.part_cats ?? []) as CatNode[])
  return { data }
}
