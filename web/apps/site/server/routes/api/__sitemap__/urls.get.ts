type Loc = { loc: string }
type Row = { id?: number; uid?: number }

const STATIC: Loc[] = [
  { loc: '/' },
  { loc: '/jobs' },
  { loc: '/companies' },
  { loc: '/resumes' },
  { loc: '/search' },
  { loc: '/articles' },
  { loc: '/announcements' },
  { loc: '/parts' },
  { loc: '/fairs' },
  { loc: '/gongzhao' },
  { loc: '/specials' },
  { loc: '/questions' },
  { loc: '/once' },
  { loc: '/tiny' },
  { loc: '/redeem' },
  { loc: '/hr' },
  { loc: '/links' },
  { loc: '/map' },
  { loc: '/login' },
  { loc: '/register' },
  { loc: '/forgetpw' },
  { loc: '/pages/about' },
  { loc: '/pages/privacy' },
  { loc: '/pages/protocol' },
  { loc: '/pages/contact' },
]

async function pagedList(rustApi: string, path: string): Promise<Row[]> {
  try {
    const res = await $fetch<{ data?: { list?: Row[] } }>(`${rustApi}${path}`, {
      method: 'GET',
      query: { page: 1, page_size: 100 },
    })
    return res.data?.list ?? []
  } catch {
    return []
  }
}

export default defineEventHandler(async () => {
  const rustApi = useRuntimeConfig().rustApi
  const [jobs, companies, articles, parts, fairs, questions] = await Promise.all([
    pagedList(rustApi, '/v1/wap/jobs'),
    pagedList(rustApi, '/v1/wap/companies'),
    pagedList(rustApi, '/v1/wap/articles'),
    pagedList(rustApi, '/v1/wap/parts'),
    pagedList(rustApi, '/v1/wap/zph'),
    pagedList(rustApi, '/v1/wap/questions'),
  ])
  const locs = new Set<string>(STATIC.map((s) => s.loc))
  for (const j of jobs) {
    if (j.id) locs.add(`/jobs/${j.id}`)
    if (j.uid) locs.add(`/companies/${j.uid}`)
  }
  for (const c of companies) {
    if (c.uid) locs.add(`/companies/${c.uid}`)
  }
  for (const a of articles) {
    if (a.id) locs.add(`/articles/${a.id}`)
  }
  for (const p of parts) {
    if (p.id) locs.add(`/parts/${p.id}`)
  }
  for (const z of fairs) {
    if (z.id) locs.add(`/fairs/${z.id}`)
  }
  for (const q of questions) {
    if (q.id) locs.add(`/questions/${q.id}`)
  }
  return [...locs].map((loc) => ({ loc }))
})
