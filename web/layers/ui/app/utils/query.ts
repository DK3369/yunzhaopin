import type { LocationQuery } from 'vue-router'

export type DictItem = { id: number; name: string }

export function numQuery(q: unknown): number | undefined {
  const n = Number(q)
  return Number.isFinite(n) && n > 0 ? n : undefined
}

export function mergeQuery(
  query: LocationQuery,
  patch: Record<string, string | number | undefined | null>,
): LocationQuery {
  const next: LocationQuery = { ...query, page: '1' }
  for (const [key, value] of Object.entries(patch)) {
    if (value === undefined || value === null || value === '' || value === 0) {
      delete next[key]
    } else {
      next[key] = String(value)
    }
  }
  return next
}

export const SALARY_BOUNDS: Record<number, { min_salary?: number; max_salary?: number }> = {
  1: { max_salary: 1000 },
  2: { min_salary: 1000, max_salary: 2000 },
  3: { min_salary: 2000, max_salary: 3000 },
  4: { min_salary: 3000, max_salary: 5000 },
  5: { min_salary: 5000, max_salary: 8000 },
  6: { min_salary: 8000, max_salary: 12000 },
  7: { min_salary: 12000, max_salary: 20000 },
  8: { min_salary: 20000 },
}
