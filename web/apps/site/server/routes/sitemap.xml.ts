type Row = { id?: number; uid?: number }

const STATIC = [
  '/',
  '/jobs',
  '/companies',
  '/resumes',
  '/search',
  '/articles',
  '/announcements',
  '/parts',
  '/fairs',
  '/gongzhao',
  '/specials',
  '/questions',
  '/once',
  '/tiny',
  '/redeem',
  '/hr',
  '/links',
  '/map',
  '/login',
  '/register',
  '/forgetpw',
  '/pages/about',
  '/pages/privacy',
  '/pages/protocol',
  '/pages/contact',
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

function xmlEscape(value: string) {
  return value.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/"/g, '&quot;')
}

export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()
  const origin = String(config.public.siteUrl || 'http://127.0.0.1:3001').replace(/\/$/, '')
  const rustApi = String(config.rustApi || 'http://127.0.0.1:3003')
  const [jobs, companies, articles, parts, fairs, questions] = await Promise.all([
    pagedList(rustApi, '/v1/wap/jobs'),
    pagedList(rustApi, '/v1/wap/companies'),
    pagedList(rustApi, '/v1/wap/articles'),
    pagedList(rustApi, '/v1/wap/parts'),
    pagedList(rustApi, '/v1/wap/zph'),
    pagedList(rustApi, '/v1/wap/questions'),
  ])
  const locs = new Set<string>(STATIC)
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

  const urls = [...locs]
    .sort()
    .map((path) => `  <url><loc>${xmlEscape(`${origin}${path}`)}</loc></url>`)
    .join('\n')
  setHeader(event, 'content-type', 'application/xml; charset=utf-8')
  return `<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n${urls}\n</urlset>\n`
})
