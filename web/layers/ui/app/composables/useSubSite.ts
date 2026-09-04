type SubSiteRow = {
  id?: number
  did?: number
  province?: number | null
  city_id?: number | null
  three_city_id?: number | null
  hy?: number | null
  fz_type?: number
  web_name?: string | null
  web_title?: string | null
  web_logo?: string | null
  mode?: number
  domain?: string
  indexdir?: string | null
}

export function useSubSite() {
  const did = useCookie('did', { path: '/', maxAge: 60 * 60 * 24 * 30 })
  const province = useCookie('province', { path: '/', maxAge: 60 * 60 * 24 * 30 })
  const cityid = useCookie('cityid', { path: '/', maxAge: 60 * 60 * 24 * 30 })
  const threeCityid = useCookie('three_cityid', { path: '/', maxAge: 60 * 60 * 24 * 30 })
  const hyclass = useCookie('hyclass', { path: '/', maxAge: 60 * 60 * 24 * 30 })
  const fzType = useCookie('fz_type', { path: '/', maxAge: 60 * 60 * 24 * 30 })
  const gotocity = useCookie('gotocity', { path: '/', maxAge: 60 * 60 * 24 * 30 })
  const syWebname = useCookie('sy_webname', { path: '/', maxAge: 60 * 60 * 24 * 30 })
  const syWebtitle = useCookie('sy_webtitle', { path: '/', maxAge: 60 * 60 * 24 * 30 })
  const syLogo = useCookie('sy_logo', { path: '/', maxAge: 60 * 60 * 24 * 30 })

  const didNum = computed(() => Number(did.value || 0) || 0)

  function applyToQuery(q: Record<string, unknown>) {
    if (didNum.value > 0) q.did = didNum.value
    if (Number(fzType.value) === 1) {
      if (q.province_id == null && Number(province.value) > 0) q.province_id = Number(province.value)
      if (q.city_id == null && Number(cityid.value) > 0) q.city_id = Number(cityid.value)
      if (q.three_city_id == null && Number(threeCityid.value) > 0) {
        q.three_city_id = Number(threeCityid.value)
      }
    } else if (Number(fzType.value) === 2) {
      if (q.hy == null && Number(hyclass.value) > 0) q.hy = Number(hyclass.value)
    }
    return q
  }

  function saveSite(row: SubSiteRow) {
    const id = Number(row.did || row.id || 0)
    did.value = id > 0 ? String(id) : ''
    province.value = row.province ? String(row.province) : ''
    cityid.value = row.city_id ? String(row.city_id) : ''
    threeCityid.value = row.three_city_id ? String(row.three_city_id) : ''
    hyclass.value = row.hy ? String(row.hy) : ''
    fzType.value = row.fz_type ? String(row.fz_type) : ''
    gotocity.value = '1'
    syWebname.value = row.web_name ? String(row.web_name) : ''
    syWebtitle.value = row.web_title ? String(row.web_title) : ''
    syLogo.value = row.web_logo ? String(row.web_logo) : ''
  }

  function clearSite() {
    did.value = ''
    province.value = ''
    cityid.value = ''
    threeCityid.value = ''
    hyclass.value = ''
    fzType.value = ''
    gotocity.value = '1'
    syWebname.value = ''
    syWebtitle.value = ''
    syLogo.value = ''
  }

  return { did, didNum, gotocity, applyToQuery, saveSite, clearSite, syWebname, syWebtitle, syLogo }
}
