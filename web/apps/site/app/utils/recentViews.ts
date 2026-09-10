export type RecentJob = { id: number; name: string; com_name?: string }
export type RecentResume = { uid: number; name: string }

const JOB_KEY = 'phpyun_recent_jobs'
const RESUME_KEY = 'phpyun_recent_resumes'
const LIMIT = 10

function readJson<T>(key: string): T[] {
  if (!import.meta.client) return []
  try {
    const raw = localStorage.getItem(key)
    if (!raw) return []
    const parsed = JSON.parse(raw) as unknown
    return Array.isArray(parsed) ? (parsed as T[]) : []
  } catch {
    return []
  }
}

function writeJson(key: string, value: unknown) {
  if (!import.meta.client) return
  try {
    localStorage.setItem(key, JSON.stringify(value))
  } catch {
    /* quota / private mode */
  }
}

export function readRecentJobs(): RecentJob[] {
  return readJson<RecentJob>(JOB_KEY)
    .filter((x) => Number(x?.id) > 0 && String(x?.name || ''))
    .slice(0, LIMIT)
}

export function pushRecentJob(item: RecentJob) {
  if (!item.id || !item.name) return
  const next = [item, ...readRecentJobs().filter((x) => x.id !== item.id)].slice(0, LIMIT)
  writeJson(JOB_KEY, next)
}

export function readRecentResumes(): RecentResume[] {
  return readJson<RecentResume>(RESUME_KEY)
    .filter((x) => Number(x?.uid) > 0 && String(x?.name || ''))
    .slice(0, LIMIT)
}

export function pushRecentResume(item: RecentResume) {
  if (!item.uid || !item.name) return
  const next = [item, ...readRecentResumes().filter((x) => x.uid !== item.uid)].slice(0, LIMIT)
  writeJson(RESUME_KEY, next)
}
