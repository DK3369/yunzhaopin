/** PHP `m=&c=&a=` → explicit `/v1/admin/*` (no invoke). */

export type PhpAction = {
  path: string
  queryFrom?: string[]
  transformReq?: (body: Record<string, unknown>) => Record<string, unknown>
  transformRes?: (data: unknown) => unknown
  /** PHP msgNum echoes the object as the whole body */
  rawBody?: boolean
}

function asRecord(data: unknown): Record<string, unknown> {
  return data && typeof data === 'object' && !Array.isArray(data) ? (data as Record<string, unknown>) : {}
}

function settingsToMap(data: unknown): Record<string, unknown> {
  const list = Array.isArray(data) ? data : []
  const out: Record<string, unknown> = {}
  for (const row of list) {
    const r = asRecord(row)
    const k = String(r.key || '')
    if (k) out[k] = r.value
  }
  return out
}

function jobStatsToPhp(data: unknown): Record<string, unknown> {
  const d = asRecord(data)
  return {
    jobAllNum: d.total || 0,
    jobStatusNum1: d.dsh || 0,
    jobStatusNum2: d.wtg || 0,
    jobStatusNum3: d.xj || 0,
  }
}

function idsFromDel(body: Record<string, unknown>): Record<string, unknown> {
  const del = body.del ?? body.id ?? body.ids
  const ids = Array.isArray(del) ? del : del != null ? [del] : []
  return { ids: ids.map((x) => Number(x)).filter((n) => n > 0) }
}

function pageQuery(body: Record<string, unknown>): Record<string, unknown> {
  const page = Number(body.page || body.currentPage || 1) || 1
  const page_size =
    Number(body.page_size || body.pageSize || body.limit || body.perPage || 20) || 20
  return { ...body, page, page_size }
}

function cacheDataShape(data: unknown): Record<string, unknown> {
  const d = asRecord(data)
  const search = asRecord(d.search_list)
  const source = asRecord(search.source)
  return {
    cache: d.cache ?? {},
    comdata: d.comdata ?? {},
    comclass_name: d.comclass_name ?? {},
    job_name: d.job_name ?? {},
    city_name: d.city_name ?? {},
    jionly: d.jionly ?? 0,
    job_types: Array.isArray(d.job_types) ? d.job_types : [],
    city_types: Array.isArray(d.city_types) ? d.city_types : [],
    curr_time: d.curr_time ?? Math.floor(Date.now() / 1000),
    search_list: {
      ...search,
      source: { name: source.name ?? 'admin_yunying_00139', value: source.value ?? {} },
    },
    hbNum: d.hbNum ?? 0,
    hb_isopen: d.hb_isopen ?? '0',
  }
}

export const PHP_ADMIN_MAP: Record<string, PhpAction> = {
  'index/homeData': { path: '/v1/admin/dashboard/home-data' },
  'index/ajax_statis': { path: '/v1/admin/dashboard/ajax-statis' },
  'index/monthStatis': { path: '/v1/admin/dashboard/month-statis' },
  'index/ajax_right': { path: '/v1/admin/dashboard/ajax-right' },
  'index/getweb': { path: '/v1/admin/dashboard/chart', transformReq: (b) => ({ ...b, kind: 'getweb' }) },
  'index/comtj': { path: '/v1/admin/dashboard/chart', transformReq: (b) => ({ ...b, kind: 'comtj' }) },
  'index/resumetj': { path: '/v1/admin/dashboard/chart', transformReq: (b) => ({ ...b, kind: 'resumetj' }) },
  'index/jobtj': { path: '/v1/admin/dashboard/chart', transformReq: (b) => ({ ...b, kind: 'jobtj' }) },
  'index/ujobtj': { path: '/v1/admin/dashboard/chart', transformReq: (b) => ({ ...b, kind: 'ujobtj' }) },
  'index/yqmstj': { path: '/v1/admin/dashboard/chart', transformReq: (b) => ({ ...b, kind: 'yqmstj' }) },
  'index/downresumetj': { path: '/v1/admin/dashboard/chart', transformReq: (b) => ({ ...b, kind: 'downresumetj' }) },
  'index/adtj': { path: '/v1/admin/dashboard/chart', transformReq: (b) => ({ ...b, kind: 'adtj' }) },
  'index/wxbdtj': { path: '/v1/admin/dashboard/chart', transformReq: (b) => ({ ...b, kind: 'wxbdtj' }) },
  'index/msgNum': {
    path: '/v1/admin/dashboard/msg-num',
    rawBody: true,
    transformRes: (data) => {
      const d = asRecord(data)
      return {
        ...d,
        msgNum: d.msg_num ?? d.msgNum ?? 0,
        usercertNum: d.usercert_num ?? d.usercertNum ?? 0,
        linkNum: d.link_num ?? d.linkNum ?? 0,
      }
    },
  },
  'index/del_cache': { path: '/v1/admin/cache/clear' },
  'index/get_navigation': { path: '/v1/admin/menu' },
  'index/logout': { path: '/v1/admin/logout' },

  'user/company_job': { path: '/v1/admin/jobs', transformReq: pageQuery },
  'user/company_job/index': { path: '/v1/admin/jobs', transformReq: pageQuery },
  'user/company_job/jobNum': { path: '/v1/admin/jobs/stats', transformRes: jobStatsToPhp },
  'user/company_job/status': { path: '/v1/admin/jobs/state' },
  'user/company_job/checkstate': { path: '/v1/admin/jobs/state' },
  'user/company_job/del': { path: '/v1/admin/jobs/delete', transformReq: idsFromDel },
  'user/company_job/refresh': { path: '/v1/admin/jobs/refresh', transformReq: idsFromDel },
  'user/company_job/getCacheData': {
    path: '/v1/admin/cache/php-dicts',
    transformRes: cacheDataShape,
  },
  'user/company_job/getHbData': {
    path: '/v1/admin/cache/php-dicts',
    transformRes: cacheDataShape,
  },
  'user/company_job/xuanshang': { path: '/v1/admin/jobs/promote', transformReq: (b) => ({ ids: idsFromDel(b).ids, kind: 'top', on: true, days: Number(b.days || 0) }) },
  'user/company_job/recommend': { path: '/v1/admin/jobs/promote', transformReq: (b) => ({ ids: idsFromDel(b).ids, kind: 'rec', on: true, days: Number(b.days || 0) }) },
  'user/company_job/urgent': { path: '/v1/admin/jobs/promote', transformReq: (b) => ({ ids: idsFromDel(b).ids, kind: 'urgent', on: true, days: Number(b.days || 0) }) },
  'common/cache': { path: '/v1/admin/cache/php-dicts' },
  'common/cache/getCityClass': { path: '/v1/admin/cache/php-dicts' },
  'common/cache/getJobClass': { path: '/v1/admin/cache/php-dicts' },
  'common/cache/getCity': { path: '/v1/admin/cache/php-dicts' },
  'common/cache/getDname': { path: '/v1/admin/cache/php-dicts' },
  'common/cache/getlthy': { path: '/v1/admin/cache/php-dicts' },
  'common/cache/getltjob': { path: '/v1/admin/cache/php-dicts' },
  'common/cache/poi': { path: '/v1/admin/cache/php-dicts' },



  'user/company': { path: '/v1/admin/companies', transformReq: pageQuery },
  'user/company/index': { path: '/v1/admin/companies', transformReq: pageQuery },
  'user/company/xls': { path: '/v1/admin/companies/export' },

  'user/users_resume': { path: '/v1/admin/resumes', transformReq: pageQuery },
  'user/users_resume/index': { path: '/v1/admin/resumes', transformReq: pageQuery },
  'user/users_resume/cstatus': { path: '/v1/admin/resumes/status' },
  'user/users_resume/xls': { path: '/v1/admin/resumes/export' },

  'system/set_config': { path: '/v1/admin/site-settings/list', transformRes: settingsToMap },
  'system/set_config/index': { path: '/v1/admin/site-settings/list', transformRes: settingsToMap },
  'system/set_config/save': { path: '/v1/admin/site-settings/batch' },
  'system/set_payset': { path: '/v1/admin/site-settings/list', transformRes: settingsToMap },
  'system/set_payset/index': { path: '/v1/admin/site-settings/list', transformRes: settingsToMap },
  'system/set_payset/save': { path: '/v1/admin/site-settings/batch' },
  'system/set_seo': { path: '/v1/admin/site-settings/list', transformRes: settingsToMap },
  'system/seoset': { path: '/v1/admin/site-settings/list', transformRes: settingsToMap },

  'user/part': { path: '/v1/admin/parts', transformReq: pageQuery },
  'user/part/index': { path: '/v1/admin/parts', transformReq: pageQuery },
  'user/partjob': { path: '/v1/admin/parts', transformReq: pageQuery },
  'user/partjob/index': { path: '/v1/admin/parts', transformReq: pageQuery },
  'user/partjob/status': { path: '/v1/admin/parts/state' },
  'user/partjob/checkstate': { path: '/v1/admin/parts/state' },
  'user/once': { path: '/v1/admin/once-jobs', transformReq: pageQuery },
  'user/weipin_once': { path: '/v1/admin/once-jobs', transformReq: pageQuery },
  'user/weipin_tiny': { path: '/v1/admin/tiny', transformReq: pageQuery },
  'user/tiny': { path: '/v1/admin/tiny', transformReq: pageQuery },
  'user/users_member/Imitate': { path: '/v1/admin/users/impersonate' },
  'user/users_resume/status': { path: '/v1/admin/resumes/status' },
  'user/users_resume/work': { path: '/v1/admin/resumes/works' },
  'user/users_resume/edu': { path: '/v1/admin/resumes/edus' },
  'user/users_resume/training': { path: '/v1/admin/resumes/trainings' },

  'neirong/question': { path: '/v1/admin/questions', transformReq: pageQuery },
  'neirong/question/del': { path: '/v1/admin/questions/delete' },
  'neirong/question/status': { path: '/v1/admin/questions/state' },
  'neirong/zhaopinhui': { path: '/v1/admin/fairs' },
  'neirong/zhaopinhui/index': { path: '/v1/admin/fairs' },
  'neirong/gongzhao': { path: '/v1/admin/gongzhao/list' },

  'yunying/special_special': { path: '/v1/admin/specials' },
  'yunying/special_special/index': { path: '/v1/admin/specials' },
  'yunying/ad': { path: '/v1/admin/ads/list' },
  'yunying/report': { path: '/v1/admin/reports' },
  'yunying/report/index': { path: '/v1/admin/reports' },

  'system/category_job_class': { path: '/v1/admin/categories/list' },
  'system/category_job_class/index': { path: '/v1/admin/categories/list' },
  'system/role_user': { path: '/v1/admin/rbac/users' },
  'system/role_ugroup': { path: '/v1/admin/rbac/groups' },
  'system/nav': { path: '/v1/admin/nav/list' },
  'system/cron': { path: '/v1/admin/cron' },
  'system/warning': { path: '/v1/admin/warnings/list' },
  'system/feedback': { path: '/v1/admin/feedback' },
}

type ModuleRoutes = { list: string; del?: string; status?: string; save?: string }

/** Controller → existing explicit `/v1/admin/*` (still no invoke). */
const MODULE_ROUTES: Record<string, ModuleRoutes> = {
  'user/company_job': { list: '/v1/admin/jobs', del: '/v1/admin/jobs/delete', status: '/v1/admin/jobs/state' },
  'user/company': { list: '/v1/admin/companies', del: '/v1/admin/companies/status', status: '/v1/admin/companies/status' },
  'user/company_cert': { list: '/v1/admin/company-certs', status: '/v1/admin/company-certs/review' },
  'user/company_expire': { list: '/v1/admin/company-expire' },
  'user/company_order': { list: '/v1/admin/orders', status: '/v1/admin/orders/status' },
  'user/hotjob': { list: '/v1/admin/hotjobs/list', del: '/v1/admin/hotjobs/delete' },
  'user/partjob': { list: '/v1/admin/parts', status: '/v1/admin/parts/state' },
  'user/part': { list: '/v1/admin/parts', status: '/v1/admin/parts/state' },
  'user/weipin_once': { list: '/v1/admin/once-jobs', status: '/v1/admin/once-jobs/status', del: '/v1/admin/once-jobs/status' },
  'user/weipin_tiny': { list: '/v1/admin/tiny', status: '/v1/admin/tiny/status' },
  'user/users_resume': { list: '/v1/admin/resumes', status: '/v1/admin/resumes/status' },
  'user/users_member': { list: '/v1/admin/users', status: '/v1/admin/users/status', del: '/v1/admin/users/status' },
  'user/users_usercert': { list: '/v1/admin/company-certs' },
  'neirong/question': { list: '/v1/admin/questions', del: '/v1/admin/questions/delete', status: '/v1/admin/questions/state' },
  'neirong/question_class': { list: '/v1/admin/categories/list' },
  'neirong/zhaopinhui': { list: '/v1/admin/fairs' },
  'neirong/zph_space': { list: '/v1/admin/fairs/spaces', del: '/v1/admin/fairs/spaces/delete', save: '/v1/admin/fairs/spaces/upsert' },
  'neirong/gongzhao': { list: '/v1/admin/gongzhao/list', del: '/v1/admin/gongzhao/delete', save: '/v1/admin/gongzhao' },
  'neirong/announcement': { list: '/v1/admin/announcements/list', del: '/v1/admin/announcements/delete', save: '/v1/admin/announcements' },
  'neirong/news': { list: '/v1/admin/articles/list', del: '/v1/admin/articles/delete', save: '/v1/admin/articles' },
  'yunying/special_special': { list: '/v1/admin/specials' },
  'yunying/ad': { list: '/v1/admin/ads/list', save: '/v1/admin/ads', del: '/v1/admin/ads' },
  'yunying/report': { list: '/v1/admin/reports', status: '/v1/admin/reports/status' },
  'yunying/report_job': { list: '/v1/admin/reports', status: '/v1/admin/reports/status', del: '/v1/admin/reports/status' },
  'yunying/report_resume': { list: '/v1/admin/reports', status: '/v1/admin/reports/status' },
  'yunying/report_ask': { list: '/v1/admin/reports', status: '/v1/admin/reports/status' },
  'yunying/report_advise': { list: '/v1/admin/reports', status: '/v1/admin/reports/status' },
  'yunying/shop_reward': { list: '/v1/admin/rewards/list', del: '/v1/admin/rewards/delete', status: '/v1/admin/rewards/status', save: '/v1/admin/rewards' },
  'yunying/shop_class': { list: '/v1/admin/redeem-classes/list', del: '/v1/admin/redeem-classes/delete', save: '/v1/admin/redeem-classes' },
  'yunying/shop_list': { list: '/v1/admin/redeem-orders', status: '/v1/admin/redeem-orders/approve' },
  'system/category_job_class': { list: '/v1/admin/categories/list', save: '/v1/admin/categories', del: '/v1/admin/categories/update' },
  'system/category_city': { list: '/v1/admin/regions', save: '/v1/admin/regions', del: '/v1/admin/regions/delete' },
  'system/category_industry': { list: '/v1/admin/categories/list' },
  'system/category_partclass': { list: '/v1/admin/categories/list' },
  'system/category_userclass': { list: '/v1/admin/categories/list' },
  'system/category_comclass': { list: '/v1/admin/categories/list' },
  'system/role_user': { list: '/v1/admin/rbac/users', status: '/v1/admin/rbac/users/status' },
  'system/role_ugroup': { list: '/v1/admin/rbac/groups' },
  'system/role_myuser': { list: '/v1/admin/rbac/users' },
  'system/set_navigation': { list: '/v1/admin/nav/list', save: '/v1/admin/nav', del: '/v1/admin/nav/update' },
  'system/admin_nav': { list: '/v1/admin/nav/list', save: '/v1/admin/nav' },
  'system/set_cron': { list: '/v1/admin/cron' },
  'system/warning': { list: '/v1/admin/warnings/list' },
  'system/info_feedback': { list: '/v1/admin/feedback', status: '/v1/admin/feedback/status', del: '/v1/admin/feedback/status' },
  'system/set_friendlink': { list: '/v1/admin/friend-links/list', del: '/v1/admin/friend-links/delete', save: '/v1/admin/friend-links', status: '/v1/admin/friend-links' },
  'system/set_config': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'system/set_payset': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'system/set_seo': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'system/seoset': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'system/set_regset': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'system/set_module': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'tool/dataRecycle': { list: '/v1/admin/recycle-bin', del: '/v1/admin/recycle-bin/purge' },
  'tool/emaillog': { list: '/v1/admin/email-logs' },
  'tool/messagelog': { list: '/v1/admin/sms-logs' },
  'tool/weixinmenu': { list: '/v1/admin/wx-navs', save: '/v1/admin/wx-navs/upsert', del: '/v1/admin/wx-navs/delete' },
  'tool/emailset': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'tool/messageset': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'common/cache': { list: '/v1/admin/cache/php-dicts' },
  'user/company_comrating': { list: '/v1/admin/companies/ratings' },
  'user/company_pic': { list: '/v1/admin/companies' },
  'user/users_pic': { list: '/v1/admin/resumes' },
  'user/users_userlog': { list: '/v1/admin/login-logs' },
  'user/company_comlog': { list: '/v1/admin/login-logs' },
  'user/users_msg': { list: '/v1/admin/sms-logs' },
  'user/users_trust': { list: '/v1/admin/resumes' },
  'user/users_userset': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'user/company_comset': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'user/company_news': { list: '/v1/admin/articles/list' },
  'user/company_product': { list: '/v1/admin/articles/list' },
  'user/company_interview': { list: '/v1/admin/jobs' },
  'user/company_pay': { list: '/v1/admin/orders' },
  'user/company_job_refresh_log': { list: '/v1/admin/jobs' },
  'user/company_company': { list: '/v1/admin/companies' },
  'user/admin_member': { list: '/v1/admin/rbac/users' },
  'yunying/yingxiao_tuiguang': { list: '/v1/admin/specials' },
  'yunying/yingxiao_hbconfig': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'yunying/yingxiao_hrlog': { list: '/v1/admin/login-logs' },
  'yunying/ad_class': { list: '/v1/admin/ads/list', save: '/v1/admin/ads' },
  'yunying/shop_set': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'neirong/evaluate': { list: '/v1/admin/questions' },
  'neirong/toolbox_doc': { list: '/v1/admin/articles/list' },
  'neirong/toolbox_class': { list: '/v1/admin/categories/list' },
  'system/set_tplset': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'system/domain_group': { list: '/v1/admin/rbac/groups' },
  'system/domain_list': { list: '/v1/admin/nav/list' },
  'system/singlepage': { list: '/v1/admin/articles/list' },
  'system/singleclass': { list: '/v1/admin/categories/list' },
  'system/category_introduce_class': { list: '/v1/admin/desc-classes/list' },
  'system/set_navmap': { list: '/v1/admin/nav/list' },
  'system/category_reason': { list: '/v1/admin/categories/list' },
  'system/info_systeminfo': { list: '/v1/admin/dashboard/overview' },
  'system/info_errorlog': { list: '/v1/admin/audit-log' },
  'system/role_logrecord': { list: '/v1/admin/admin-logs' },
  'tool/fabutool': { list: '/v1/admin/wx-navs' },
  'tool/database': { list: '/v1/admin/recycle-bin' },
  'tool/generate_page': { list: '/v1/admin/cache/clear' },
  'tool/dataCall': { list: '/v1/admin/articles/list' },
  'tool/dataCollection': { list: '/v1/admin/articles/list' },
  'tool/weixinrecord': { list: '/v1/admin/login-logs' },
  'tool/dataBoard': { list: '/v1/admin/dashboard/overview' },
  'tool/gsdConfig': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'tool/admin_uc': { list: '/v1/admin/site-settings/list' },
  'tool/dataOss': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'tool/fastlogin': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'tool/generate_cache': { list: '/v1/admin/cache/clear' },
  'tool/generate_xml': { list: '/v1/admin/cache/clear' },
  'index/getIpAddress': { list: '/v1/admin/cache/php-dicts' },
  'index/getMobileAddress': { list: '/v1/admin/cache/php-dicts' },
  'index/getwxbindstatus': { list: '/v1/admin/cache/php-dicts' },
  'index/wxbind': { list: '/v1/admin/cache/php-dicts' },
}

function moduleAction(mod: ModuleRoutes, a: string): PhpAction {
  const act = (a || 'index').toLowerCase()
  if ((act === 'del' || act.startsWith('del') || act === 'delete') && mod.del) {
    return { path: mod.del, transformReq: idsFromDel }
  }
  if ((act === 'status' || act.includes('status') || act === 'audit' || act === 'checkstate') && mod.status) {
    return { path: mod.status }
  }
  if ((act === 'save' || act === 'add' || act.endsWith('save')) && mod.save) {
    return { path: mod.save }
  }
  return { path: mod.list, transformReq: pageQuery }
}

export function parsePhpUrl(url: string): { m: string; c: string; a: string } {
  const q = url.startsWith('m=') || url.includes('=') ? url : ''
  const params = new URLSearchParams(q.includes('?') ? q.slice(q.indexOf('?') + 1) : q)
  if (!params.get('m') && url.includes('&') === false && url.includes('=') === false) {
    return { m: '', c: '', a: '' }
  }
  return {
    m: params.get('m') || '',
    c: params.get('c') || '',
    a: params.get('a') || '',
  }
}

export function resolvePhpAction(url: string): PhpAction | undefined {
  const { m, c, a } = parsePhpUrl(url)
  if (!m && !c) return undefined
  const keys = a ? [`${m}/${c}/${a}`, `${m}/${c}`] : [`${m}/${c}`]
  for (const k of keys) {
    if (PHP_ADMIN_MAP[k]) return PHP_ADMIN_MAP[k]
  }
  const mod = MODULE_ROUTES[`${m}/${c}`]
  if (mod) return moduleAction(mod, a)
  return undefined
}
