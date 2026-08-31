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

function isOn(v: unknown): boolean {
  return v === true || v === 1 || v === '1'
}

/** PHP `set_config::index_action` computed fields on top of KV config. */
function configIndexShape(data: unknown): Record<string, unknown> {
  const m = settingsToMap(data)
  const mapKey = String(m.map_key || '')
  const weburl = String(m.sy_weburl || '')
  return {
    ...m,
    sy_ossurl: String(m.sy_ossurl || weburl),
    sy_web_online_status: isOn(m.sy_web_online),
    sy_iscsrf_status: isOn(m.sy_iscsrf),
    sy_istemplate_status: isOn(m.sy_istemplate),
    mapurl: mapKey ? `https://webapi.amap.com/maps?v=2.0&key=${mapKey}` : String(m.mapurl || ''),
    map_key: mapKey,
    map_secret: String(m.map_secret || ''),
  }
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

function csvList(v: unknown): string[] {
  if (Array.isArray(v)) return v.map((x) => String(x).trim()).filter(Boolean)
  return String(v || '')
    .split(/[,;\s]+/)
    .map((s) => s.trim())
    .filter(Boolean)
}

function parseJsonField(v: unknown): unknown {
  if (typeof v !== 'string') return v
  try {
    return JSON.parse(v)
  } catch {
    return v
  }
}

function pageQuery(body: Record<string, unknown>): Record<string, unknown> {
  const page = Number(body.page || body.currentPage || 1) || 1
  const page_size =
    Number(body.page_size || body.pageSize || body.limit || body.perPage || 20) || 20
  return { ...body, page, page_size }
}

function idFromDel(body: Record<string, unknown>): Record<string, unknown> {
  const ids = idsFromDel(body).ids as number[]
  return { id: ids[0] || Number(body.id || 0) }
}

function wrapList(data: unknown): Record<string, unknown> {
  if (Array.isArray(data)) return { list: data }
  return asRecord(data)
}

function evalGroupsToGetGroup(data: unknown): Record<string, unknown> {
  const list = Array.isArray(data) ? data : []
  const grouparr: { label: unknown; value: unknown }[] = []
  const show_group: Record<string, unknown> = {}
  for (const row of list) {
    const x = asRecord(row)
    grouparr.push({ label: x.name, value: x.id })
    show_group[String(x.id)] = x.name
  }
  return { grouparr, show_group, preview_url: '/index.php?m=evaluate&c=exampaper' }
}

function evalGroupsToRecordGroup(data: unknown): Record<string, unknown> {
  const list = Array.isArray(data) ? data : []
  const arr: Record<string, unknown> = {}
  for (const row of list) {
    const x = asRecord(row)
    arr[String(x.id)] = x.name
  }
  return { arr }
}

function wrapItems(body: Record<string, unknown>): Record<string, unknown> {
  if (body.items && typeof body.items === 'object' && !Array.isArray(body.items)) return body
  const skip = new Set(['page', 'page_size', 'm', 'c', 'a'])
  const items: Record<string, unknown> = {}
  for (const [k, v] of Object.entries(body)) {
    if (!skip.has(k)) items[k] = String(v ?? '')
  }
  return { items }
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
    mapurl: String(d.mapurl || ''),
    mapsecret: String(d.mapsecret || ''),
    map_key: String(d.map_key || d.mapkey || ''),
    mapkey: String(d.mapkey || d.map_key || ''),
  }
}

function phpPage(kind: string): PhpAction {
  return {
    path: '/v1/admin/cache/php-page',
    transformReq: (b) => ({ ...b, kind, pid: Number(b.pid || 0) }),
  }
}

function phpContent(mod: string, act: string): PhpAction {
  return { path: `/v1/admin/php-content/${mod}/${act}`, transformReq: pageQuery }
}

function phpContentRaw(mod: string, act: string): PhpAction {
  return { path: `/v1/admin/php-content/${mod}/${act}`, rawBody: true }
}

function catKind(kind: string): PhpAction {
  return { path: '/v1/admin/categories/list', transformReq: (b) => ({ ...b, kind }) }
}

function catClass(kind: string, act: string): PhpAction {
  return {
    path: `/v1/admin/php-content/cat-class/${act}`,
    transformReq: (b) => ({ ...pageQuery(b), kind }),
  }
}

function sysmsgQuery(body: Record<string, unknown>): Record<string, unknown> {
  const q = pageQuery(body)
  const typeNum = Number(q.type)
  return {
    page: q.page,
    page_size: q.page_size,
    keyword: q.keyword || undefined,
    ...(Number.isFinite(typeNum) && typeNum > 0 ? { type: typeNum } : {}),
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
  'user/company_job/add': { path: '/v1/admin/jobs/php-add-form' },
  'common/cache': { path: '/v1/admin/cache/php-dicts' },
  'common/cache/getCityClass': { path: '/v1/admin/cache/php-dicts' },
  'common/cache/getJobClass': { path: '/v1/admin/cache/php-dicts' },
  'common/cache/getCity': { path: '/v1/admin/cache/php-dicts' },
  'common/cache/getDname': { path: '/v1/admin/cache/php-dicts' },
  'common/cache/getlthy': { path: '/v1/admin/cache/php-dicts' },
  'common/cache/getltjob': { path: '/v1/admin/cache/php-dicts' },
  'common/cache/poi': { path: '/v1/admin/cache/php-dicts' },
  'common/cache/getJobChildIds': phpPage('job_child_ids'),
  'common/cache/getCityChildIds': phpPage('city_child_ids'),

  'user/company/companyNum': phpContent('user-gap', 'company-num'),
  'user/users_resume/resumeNum': phpContentRaw('user-gap', 'resume-num'),
  'user/users_resume/getConfig': phpContent('user-gap', 'resume-config'),
  'user/users_member/userNum': phpContentRaw('user-gap', 'user-num'),
  'user/users_member/getConfigData': phpContent('user-gap', 'user-config'),
  'user/company/reset_companypassword': phpContent('user-gap', 'reset-password'),
  'user/company_job/matching': phpContent('user-gap', 'matching'),
  'user/users_resume/resumeAudit': phpContent('user-gap', 'resume-audit'),
  'user/users_member/reset_pw': phpContent('user-gap', 'reset-password'),
  'user/company/index': { path: '/v1/admin/companies', transformReq: pageQuery },
  'user/company/xls': { path: '/v1/admin/companies/export' },
  'user/company/getCache': { path: '/v1/admin/companies/php-cache' },
  'user/company/add': { path: '/v1/admin/companies/php-add-form' },
  'user/company/checkUsername': { path: '/v1/admin/companies/check-username' },
  'user/company/checkComName': { path: '/v1/admin/companies/check-com-name' },
  'user/company/edit': { path: '/v1/admin/companies/php-edit' },
  'user/company/comeditsave': { path: '/v1/admin/companies/php-comeditsave' },
  'user/company/getinfo': { path: '/v1/admin/companies/php-getinfo' },
  'user/company/saveUser': { path: '/v1/admin/companies/php-save-user' },
  'user/company/Imitate': { path: '/v1/admin/companies/php-imitate' },
  'user/company/getrating': { path: '/v1/admin/companies/php-getrating' },
  'user/company/getstatis': { path: '/v1/admin/companies/php-getstatis' },
  'user/company/uprating': { path: '/v1/admin/companies/php-uprating' },
  'user/company/companyAudit': { path: '/v1/admin/companies/php-audit' },
  'user/company/suspend': { path: '/v1/admin/companies/php-suspend' },
  'user/company/comcert': { path: '/v1/admin/companies/php-comcert' },
  'user/users_resume/checkUsername': { path: '/v1/admin/companies/check-username' },
  'user/users_resume/add': { path: '/v1/admin/resumes/php-add' },
  'user/users_resume/editResume': { path: '/v1/admin/resumes/php-edit' },
  'user/users_resume/saveExpect': { path: '/v1/admin/resumes/php-save-expect' },
  'user/users_resume/saveTag': { path: '/v1/admin/resumes/php-save-tag' },
  'user/partjob/getCacheData': {
    path: '/v1/admin/cache/php-dicts',
    transformRes: cacheDataShape,
  },
  'user/users_resume/getCache': phpPage('resume_getCache'),
  'user/weipin_tiny/getCache': phpPage('tiny_getCache'),
  'user/weipin_once/getCache': phpPage('once_getCache'),
  'user/admin_member/getCache': phpPage('admin_member_getCache'),
  'system/set_friendlink/getCache': phpPage('friendlink_getCache'),
  'neirong/news/getCache': phpPage('news_getCache'),
  'tool/dataCollection/getCache': phpPage('dataCollection_getCache'),
  'system/domain_group/getAdminCache': phpPage('getAdminCache'),
  'user/users_userset/indexBaseData': phpPage('userset_indexBaseData'),
  'user/users_userset/index': phpPage('userset_index'),
  'user/users_userset': phpPage('userset_index'),
  'user/company_comset/index': phpPage('comset_index'),
  'user/company_comset': phpPage('comset_index'),
  'tool/emailset/index': phpPage('emailset_index'),
  'tool/emailset': phpPage('emailset_index'),
  'yunying/special_special/get_base_data': phpPage('special_base'),
  'yunying/ad/get_base_data': phpContent('ads', 'get_base_data'),
  'yunying/shop_reward/index_base_data': phpPage('shop_reward_base'),
  'yunying/shop_list/index_base_data': phpPage('shop_list_base'),
  'yunying/report_resume/index_base_data': phpPage('report_resume_base'),
  'tool/messagelog/index_base_data': phpPage('messagelog_base'),
  'tool/dataCall/index_base_data': phpPage('dataCall_base'),
  'user/users_member/getSearchData': phpPage('member_search'),
  'user/hotjob/getSearchData': phpPage('hotjob_search'),
  'user/users_trust/getSearchData': phpPage('trust_search'),

  'user/users_resume': { path: '/v1/admin/resumes', transformReq: pageQuery },
  'user/users_resume/index': { path: '/v1/admin/resumes', transformReq: pageQuery },
  'user/users_resume/cstatus': { path: '/v1/admin/resumes/status' },
  'user/users_resume/xls': { path: '/v1/admin/resumes/export' },

  'system/set_config': { path: '/v1/admin/site-settings/list', transformRes: configIndexShape },
  'system/set_config/index': { path: '/v1/admin/site-settings/list', transformRes: configIndexShape },
  'system/set_config/save': { path: '/v1/admin/site-settings/batch' },
  'system/set_payset': { path: '/v1/admin/site-settings/payset' },
  'system/set_payset/index': { path: '/v1/admin/site-settings/payset' },
  'system/set_payset/save': { path: '/v1/admin/site-settings/batch' },
  'system/set_payset/alipay': { path: '/v1/admin/site-settings/payset/alipay' },
  'system/set_payset/tenpay': { path: '/v1/admin/site-settings/payset/tenpay' },
  'system/set_payset/bank': { path: '/v1/admin/site-settings/payset/bank' },
  'system/set_payset/del': {
    path: '/v1/admin/site-settings/payset/bank-delete',
    transformReq: idFromDel,
  },
  'system/set_seo': { path: '/v1/admin/site-settings/php-seo' },
  'system/set_seo/index': { path: '/v1/admin/site-settings/php-seo' },
  'system/set_seo/seoadd': { path: '/v1/admin/site-settings/php-seo-add' },
  'system/set_seo/save': { path: '/v1/admin/site-settings/php-seo-save' },
  'system/set_seo/del': { path: '/v1/admin/site-settings/php-seo-del', transformReq: idFromDel },
  'system/seoset': { path: '/v1/admin/site-settings/php-seo' },
  'system/set_regset': { path: '/v1/admin/site-settings/php-regset' },
  'system/set_regset/index': { path: '/v1/admin/site-settings/php-regset' },
  'system/set_regset/save': { path: '/v1/admin/site-settings/php-regset-save' },
  'tool/messageset': { path: '/v1/admin/site-settings/php-messageset' },
  'tool/messageset/index': { path: '/v1/admin/site-settings/php-messageset' },
  'yunying/yingxiao_hbconfig': { path: '/v1/admin/site-settings/php-hbconfig' },
  'yunying/yingxiao_hbconfig/index': { path: '/v1/admin/site-settings/php-hbconfig' },
  'yunying/yingxiao_hbconfig/saveSet': { path: '/v1/admin/site-settings/php-hb-saveset' },
  'yunying/yingxiao_hbconfig/job': {
    path: '/v1/admin/site-settings/php-hb-list',
    transformReq: (b) => ({ ...b, type: 1 }),
  },
  'yunying/yingxiao_hbconfig/com': {
    path: '/v1/admin/site-settings/php-hb-list',
    transformReq: (b) => ({ ...b, type: 2 }),
  },
  'yunying/yingxiao_hbconfig/inviteReg': {
    path: '/v1/admin/site-settings/php-hb-list',
    transformReq: (b) => ({ ...b, type: 3 }),
  },
  'yunying/yingxiao_hbconfig/gongzhao': {
    path: '/v1/admin/site-settings/php-hb-list',
    transformReq: (b) => ({ ...b, type: 4 }),
  },
  'yunying/yingxiao_hbconfig/saveWhbConfig': { path: '/v1/admin/site-settings/php-hb-save-open' },
  'neirong/announcement/add': { path: '/v1/admin/announcements/php-add' },

  'user/part': { path: '/v1/admin/parts', transformReq: pageQuery },
  'user/part/index': { path: '/v1/admin/parts', transformReq: pageQuery },
  'user/partjob': { path: '/v1/admin/parts', transformReq: pageQuery },
  'user/partjob/index': { path: '/v1/admin/parts', transformReq: pageQuery },
  'user/partjob/status': { path: '/v1/admin/parts/state' },
  'user/partjob/show': phpContent('part', 'show'),
  'user/partjob/partAudit': phpContent('part', 'partAudit'),
  'user/partjob/recommend': phpContent('part', 'recommend'),
  'user/partjob/ctime': phpContent('part', 'ctime'),
  'user/partjob/refresh': phpContent('part', 'refresh'),
  'user/partjob/del': phpContent('part', 'del'),
  'user/partjob/checkstate': phpContent('part', 'checkstate'),
  'user/once': { path: '/v1/admin/once-jobs', transformReq: pageQuery },
  'user/weipin_once': { path: '/v1/admin/once-jobs', transformReq: pageQuery },
  'user/weipin_once/price_gear': phpContent('once', 'price_gear'),
  'user/weipin_once/price_gear_add': phpContent('once', 'price_gear_add'),
  'user/weipin_once/price_gear_ajax': phpContent('once', 'price_gear_ajax'),
  'user/weipin_once/price_gear_del': phpContent('once', 'price_gear_del'),
  'user/weipin_once/set': phpContent('once', 'set'),
  'user/weipin_once/onceset': phpContent('once', 'onceset'),
  'user/weipin_once/edit': phpContent('once', 'edit'),
  'user/weipin_once/save': phpContent('once', 'save'),
  'user/weipin_once/del': phpContent('once', 'del'),
  'user/weipin_once/ctime': phpContent('once', 'ctime'),
  'user/weipin_once/refresh_job': phpContent('once', 'refresh_job'),
  'user/weipin_tiny': { path: '/v1/admin/tiny', transformReq: pageQuery },
  'user/weipin_tiny/set': phpContent('tiny', 'set'),
  'user/weipin_tiny/tinyset': phpContent('tiny', 'tinyset'),
  'user/weipin_tiny/save': phpContent('tiny', 'save'),
  'user/weipin_tiny/del': phpContent('tiny', 'del'),
  'user/weipin_tiny/refresh': phpContent('tiny', 'refresh'),
  'user/tiny': { path: '/v1/admin/tiny', transformReq: pageQuery },
  'user/hotjob/save': phpContent('hotjob', 'save'),
  'user/hotjob/getComList': phpContent('hotjob', 'getComList'),
  'user/hotjob/gethotjob': phpContent('hotjob', 'gethotjob'),
  'user/hotjob/hotjobinfo': phpContent('hotjob', 'hotjobinfo'),
  'user/hotjob/hotNum': phpContent('hotjob', 'hotNum'),
  'user/users_member/Imitate': { path: '/v1/admin/users/impersonate' },
  'user/users_member/add': { path: '/v1/admin/users/php-add' },
  'user/users_member/edit': { path: '/v1/admin/users/php-edit' },
  'user/users_member/editSave': { path: '/v1/admin/users/php-editsave' },
  'user/users_member/saveUser': { path: '/v1/admin/users/php-save-user' },
  'user/users_resume/status': { path: '/v1/admin/resumes/status' },
  'user/users_resume/work': { path: '/v1/admin/resumes/works' },
  'user/users_resume/edu': { path: '/v1/admin/resumes/edus' },
  'user/users_resume/training': { path: '/v1/admin/resumes/trainings' },
  'user/users_resume/skill': phpContent('resume', 'skill'),
  'user/users_resume/project': phpContent('resume', 'project'),
  'user/users_resume/other': phpContent('resume', 'other'),

  'neirong/question': phpContent('question', 'index'),
  'neirong/question/index': phpContent('question', 'index'),
  'neirong/question/getGroup': phpContent('question', 'getGroup'),
  'neirong/question/add': phpContent('question', 'add'),
  'neirong/question/save': phpContent('question', 'save'),
  'neirong/question/del': phpContent('question', 'delete'),
  'neirong/question/status': { path: '/v1/admin/questions/state' },
  'neirong/question/recommend': phpContent('question', 'recommend'),
  'neirong/question/getanswer': phpContent('question', 'getanswer'),
  'neirong/question/statusAnswer': phpContent('question', 'statusAnswer'),
  'neirong/question/save_answer': phpContent('question', 'save_answer'),
  'neirong/question/delanswer': phpContent('question', 'delanswer'),
  'neirong/question/getcomment': phpContent('question', 'getcomment'),
  'neirong/question/statusAnswerReview': phpContent('question', 'statusAnswerReview'),
  'neirong/question/save_review': phpContent('question', 'save_review'),
  'neirong/question/delreview': phpContent('question', 'delreview'),
  'neirong/question/config': phpContent('question', 'config'),
  'neirong/question/configSave': phpContent('question', 'configSave'),
  'neirong/zhaopinhui': phpContent('fairs', 'index'),
  'neirong/zhaopinhui/index': phpContent('fairs', 'index'),
  'neirong/zhaopinhui/getGroup': phpContent('fairs', 'get-group'),
  'neirong/zhaopinhui/add': phpContent('fairs', 'add'),
  'neirong/zhaopinhui/del': phpContent('fairs', 'delete'),
  'neirong/zhaopinhui/com': phpContent('fairs', 'com'),
  'neirong/zhaopinhui/status': phpContent('fairs', 'status'),
  'neirong/zhaopinhui/audit': phpContent('fairs', 'audit'),
  'neirong/zhaopinhui/getjoblist': phpContent('fairs', 'getjoblist'),
  'neirong/zhaopinhui/upjob': phpContent('fairs', 'upjob'),
  'neirong/zhaopinhui/comadd': phpContent('fairs', 'comadd'),
  'neirong/zhaopinhui/getcomlist': phpContent('fairs', 'getcomlist'),
  'neirong/zhaopinhui/getzhanwei': phpContent('fairs', 'getzhanwei'),
  'neirong/zhaopinhui/upzhanwei': phpContent('fairs', 'upzhanwei'),
  'neirong/zhaopinhui/comaddsave': phpContent('fairs', 'comaddsave'),
  'neirong/zhaopinhui/delcom': phpContent('fairs', 'delcom'),
  'neirong/zhaopinhui/ajaxsort': phpContent('fairs', 'ajaxsort'),
  'neirong/zhaopinhui/upisopen': phpContent('fairs', 'upisopen'),
  'neirong/zhaopinhui/checksitedid': phpContent('fairs', 'checksitedid'),
  'neirong/zhaopinhui/comxlscheck': phpContent('fairs', 'comxlscheck'),
  'neirong/zhaopinhui/comxls': phpContent('fairs', 'comxls'),
  'neirong/zhaopinhui/upload': phpContent('fairs', 'upload'),
  'neirong/zhaopinhui/uploadsave': phpContent('fairs', 'uploadsave'),
  'neirong/zhaopinhui/setthemb': phpContent('fairs', 'setthemb'),
  'neirong/zhaopinhui/delpic': phpContent('fairs', 'delpic'),
  'neirong/news': phpContent('news', 'index'),
  'neirong/news/index': phpContent('news', 'index'),
  'neirong/news/addnews': phpContent('news', 'addnews'),
  'neirong/news/delnews': phpContent('news', 'delete'),
  'neirong/news/group': phpContent('news', 'group'),
  'neirong/news/addgroup': phpContent('news', 'addgroup'),
  'neirong/news/delgroup': phpContent('news', 'delgroup'),
  'neirong/news/ajax': phpContent('news', 'ajax'),
  'neirong/news/recommend': phpContent('news', 'recommend'),
  'neirong/news/changeClass': phpContent('news', 'changeClass'),
  'neirong/news/checksitedid': phpContent('news', 'checksitedid'),
  'neirong/news/savepro': phpContent('news', 'savepro'),
  'neirong/news/type': phpContent('news', 'type'),
  'neirong/news/property': phpContent('news', 'property'),
  'neirong/news/delpro': phpContent('news', 'delpro'),
  'neirong/news/delmenu': phpContent('news', 'delmenu'),
  'neirong/news/changeSon': phpContent('news', 'changeSon'),
  'neirong/gongzhao': phpContent('gongzhao', 'index'),
  'neirong/gongzhao/index': phpContent('gongzhao', 'index'),
  'neirong/gongzhao/getGroup': phpContent('gongzhao', 'getGroup'),
  'neirong/gongzhao/add': phpContent('gongzhao', 'add'),
  'neirong/gongzhao/del': phpContent('gongzhao', 'delete'),
  'neirong/gongzhao/checksitedid': phpContent('gongzhao', 'checksitedid'),
  'neirong/gongzhao/setRec': phpContent('gongzhao', 'setRec'),
  'neirong/gongzhao/whb': phpContent('gongzhao', 'whb'),
  'neirong/announcement/getGroup': phpContent('announce', 'getGroup'),
  'neirong/announcement/checksitedid': phpContent('announce', 'checksitedid'),

  'neirong/evaluate': { path: '/v1/admin/evaluate/papers/list', transformReq: pageQuery },
  'neirong/evaluate/index': { path: '/v1/admin/evaluate/papers/list', transformReq: pageQuery },
  'neirong/evaluate/getGroup': { path: '/v1/admin/evaluate/groups/list', transformRes: evalGroupsToGetGroup },
  'neirong/evaluate/add': {
    path: '/v1/admin/evaluate/papers',
    transformReq: (b) => {
      if (Number(b.add) === 1) {
        return { add: 1, id: b.id ? Number(b.id) : undefined }
      }
      return {
        ...b,
        add: 0,
        id: b.id ? Number(b.id) : undefined,
        keyid: Number(b.keyid || 0),
        sort: Number(b.sort || 0),
        top: Number(b.top || 0),
        hot: Number(b.hot || 0),
        recommend: Number(b.recommend || 0),
        pj_arr: parseJsonField(b.pj_arr) || [],
        ask_arr: parseJsonField(b.ask_arr) || [],
      }
    },
  },
  'neirong/evaluate/delevaluate': { path: '/v1/admin/evaluate/papers/delete', transformReq: idsFromDel },
  'neirong/evaluate/ajaxsave': { path: '/v1/admin/evaluate/questions' },
  'neirong/evaluate/delquestion': { path: '/v1/admin/evaluate/questions/delete' },
  'neirong/evaluate/group': { path: '/v1/admin/evaluate/groups/list' },
  'neirong/evaluate/addgroup': { path: '/v1/admin/evaluate/groups' },
  'neirong/evaluate/ajax': { path: '/v1/admin/evaluate/groups/patch' },
  'neirong/evaluate/delgroup': { path: '/v1/admin/evaluate/groups/delete', transformReq: idFromDel },
  'neirong/evaluate/message': { path: '/v1/admin/evaluate/messages', transformReq: pageQuery },
  'neirong/evaluate/delmsg': { path: '/v1/admin/evaluate/messages/delete', transformReq: idsFromDel },
  'neirong/evaluate/record': { path: '/v1/admin/evaluate/logs', transformReq: pageQuery },
  'neirong/evaluate/recordGroup': { path: '/v1/admin/evaluate/groups/list', transformRes: evalGroupsToRecordGroup },
  'neirong/evaluate/delevaluatelog': { path: '/v1/admin/evaluate/logs/delete', transformReq: idsFromDel },

  'neirong/toolbox_doc': { path: '/v1/admin/toolbox/docs/list', transformReq: pageQuery },
  'neirong/toolbox_doc/index': { path: '/v1/admin/toolbox/docs/list', transformReq: pageQuery },
  'neirong/toolbox_doc/save': { path: '/v1/admin/toolbox/docs' },
  'neirong/toolbox_doc/del': { path: '/v1/admin/toolbox/docs/delete', transformReq: idsFromDel },
  'neirong/toolbox_doc/show': { path: '/v1/admin/toolbox/docs/show' },
  'neirong/toolbox_doc/getGroup': { path: '/v1/admin/toolbox/docs/meta' },
  'neirong/toolbox_doc/add': { path: '/v1/admin/toolbox/docs/detail' },
  'neirong/toolbox_class': { path: '/v1/admin/toolbox/classes/list', transformRes: wrapList },
  'neirong/toolbox_class/index': { path: '/v1/admin/toolbox/classes/list', transformRes: wrapList },
  'neirong/toolbox_class/save': { path: '/v1/admin/toolbox/classes' },
  'neirong/toolbox_class/del': { path: '/v1/admin/toolbox/classes/delete', transformReq: idsFromDel },

  'neirong/question_class': { path: '/v1/admin/question-classes/list', transformReq: pageQuery },
  'neirong/question_class/index': { path: '/v1/admin/question-classes/list', transformReq: pageQuery },
  'neirong/question_class/add': { path: '/v1/admin/question-classes/detail' },
  'neirong/question_class/save': { path: '/v1/admin/question-classes' },
  'neirong/question_class/del': { path: '/v1/admin/question-classes/delete', transformReq: idsFromDel },

  'user/users_pic': { path: '/v1/admin/user-photos', transformReq: pageQuery },
  'user/users_pic/index': { path: '/v1/admin/user-photos', transformReq: pageQuery },
  'user/users_pic/status': { path: '/v1/admin/user-photos/status' },
  'user/users_pic/getStatist': { path: '/v1/admin/user-photos/statist' },
  'user/users_pic/getStatusBody': { path: '/v1/admin/user-photos/status-body' },
  'user/users_pic/savePhoto': {
    path: '/v1/admin/user-photos/save',
    transformReq: (b) => ({
      uid: Number(b.id || b.uid || 0),
      photo: String(b.photo || b.url || b.pic || ''),
    }),
  },
  'user/users_pic/delPhoto': { path: '/v1/admin/user-photos/delete', transformReq: idsFromDel },
  'user/users_pic/show': { path: '/v1/admin/resume-shows', transformReq: pageQuery },
  'user/users_pic/showStatus': { path: '/v1/admin/resume-shows/status' },
  'user/users_pic/getShowStatusBody': { path: '/v1/admin/resume-shows/status-body' },
  'user/users_pic/saveShow': { path: '/v1/admin/resume-shows/save' },
  'user/users_pic/delShow': { path: '/v1/admin/resume-shows/delete', transformReq: idsFromDel },
  'user/users_usercert': { path: '/v1/admin/user-certs', transformReq: pageQuery },
  'user/users_usercert/index': { path: '/v1/admin/user-certs', transformReq: pageQuery },
  'user/users_usercert/status': { path: '/v1/admin/user-certs/status' },
  'user/users_usercert/getStatist': { path: '/v1/admin/user-certs/statist' },
  'user/users_usercert/getStatusBody': { path: '/v1/admin/user-certs/status-body' },
  'user/users_msg/getStatist': { path: '/v1/admin/user-msgs/statist' },
  'user/users_msg': { path: '/v1/admin/user-msgs', transformReq: pageQuery },
  'user/users_msg/index': { path: '/v1/admin/user-msgs', transformReq: pageQuery },
  'user/users_msg/del': { path: '/v1/admin/user-msgs/delete', transformReq: idsFromDel },
  'user/users_userlog': { path: '/v1/admin/user-logs/down', transformReq: pageQuery },
  'user/users_userlog/index': { path: '/v1/admin/user-logs/down', transformReq: pageQuery },
  'user/users_userlog/down': { path: '/v1/admin/user-logs/down', transformReq: pageQuery },
  'user/users_userlog/freedown': { path: '/v1/admin/user-logs/freedown', transformReq: pageQuery },
  'user/users_userlog/lookresume': { path: '/v1/admin/user-logs/look-resume', transformReq: pageQuery },
  'user/users_userlog/talentpool': { path: '/v1/admin/user-logs/talent-pool', transformReq: pageQuery },
  'user/users_userlog/trust': { path: '/v1/admin/user-logs/trust', transformReq: pageQuery },
  'user/users_userlog/sxLog': { path: '/v1/admin/user-logs/refresh', transformReq: pageQuery },
  'user/company_pic': { path: '/v1/admin/company-photos', transformReq: pageQuery },
  'user/company_pic/index': { path: '/v1/admin/company-photos', transformReq: pageQuery },
  'user/company_pic/status': { path: '/v1/admin/company-photos/status' },
  'user/company_pic/getStatist': { path: '/v1/admin/company-photos/statist' },
  'user/company_pic/getStatusBody': { path: '/v1/admin/company-photos/status-body' },
  'user/company_pic/savePhoto': {
    path: '/v1/admin/company-photos/save',
    transformReq: (b) => ({
      uid: Number(b.id || b.uid || 0),
      photo: String(b.photo || b.url || b.pic || ''),
    }),
  },
  'user/company_pic/del': {
    path: '/v1/admin/company-photos/delete',
    transformReq: (b) => ({ ...idsFromDel(b), type: String(b.type || 'logo') }),
  },
  'user/company_pic/show': { path: '/v1/admin/company-shows', transformReq: pageQuery },
  'user/company_pic/showStatus': { path: '/v1/admin/company-shows/status' },
  'user/company_pic/getShowStatusBody': { path: '/v1/admin/company-shows/status-body' },
  'user/company_pic/banner': { path: '/v1/admin/company-banners', transformReq: pageQuery },
  'user/company_pic/bannerStatus': {
    path: '/v1/admin/company-banners/status',
    transformReq: (b) => ({
      ...b,
      ids: idsFromDel({ ...b, id: b.sid ?? b.id ?? b.del }).ids,
      status: Number(b.status || 0),
      statusbody: String(b.statusbody || ''),
    }),
  },
  'user/company_pic/uploadsave': {
    path: '/v1/admin/company-banners/save',
    transformReq: (b) => ({
      id: Number(b.id || 0) || undefined,
      uid: Number(b.uid || b.id || 0),
      type: String(b.type || 'banner'),
      pic: String(b.pic || b.photo || b.url || ''),
      title: String(b.title || ''),
    }),
  },
  'user/company_product': { path: '/v1/admin/company-products', transformReq: pageQuery },
  'user/company_product/index': { path: '/v1/admin/company-products', transformReq: pageQuery },
  'user/company_product/status': { path: '/v1/admin/company-products/status' },
  'user/company_news': { path: '/v1/admin/company-news', transformReq: pageQuery },
  'user/company_news/index': { path: '/v1/admin/company-news', transformReq: pageQuery },
  'user/company_news/status': { path: '/v1/admin/company-news/status' },
  'user/company_interview': { path: '/v1/admin/company-interviews', transformReq: pageQuery },
  'user/company_interview/index': { path: '/v1/admin/company-interviews', transformReq: pageQuery },
  'user/company_comlog': { path: '/v1/admin/company-logs/userid-job', transformReq: pageQuery },
  'user/company_comlog/index': { path: '/v1/admin/company-logs/userid-job', transformReq: pageQuery },
  'user/company_comlog/useridmsg': { path: '/v1/admin/company-logs/userid-msg', transformReq: pageQuery },
  'user/company_comlog/lookjob': { path: '/v1/admin/company-logs/look-job', transformReq: pageQuery },
  'user/company_comlog/partapply': { path: '/v1/admin/company-logs/part-apply', transformReq: pageQuery },
  'user/company_comlog/favjob': { path: '/v1/admin/company-logs/fav-job', transformReq: pageQuery },
  'user/company_comlog/jobtellog': { path: '/v1/admin/company-logs/job-tellog', transformReq: pageQuery },
  'user/company_job_refresh_log': { path: '/v1/admin/job-refresh-logs', transformReq: pageQuery },
  'user/company_job_refresh_log/index': { path: '/v1/admin/job-refresh-logs', transformReq: pageQuery },
  'user/company_comrating': { path: '/v1/admin/rating-packages/list', transformReq: pageQuery },
  'user/company_comrating/index': { path: '/v1/admin/rating-packages/list', transformReq: pageQuery },
  'user/company_comrating/rating': { path: '/v1/admin/rating-packages/detail' },
  'user/company_comrating/saveclass': {
    path: '/v1/admin/rating-packages',
    transformReq: (b) => ({
      ...b,
      id: b.id ? Number(b.id) : undefined,
      youhui: Number(b.youhui || 0),
      time: Array.isArray(b.time) ? (b.time as unknown[]).join('~') : String(b.time || ''),
    }),
  },
  'user/company_comrating/delrating': { path: '/v1/admin/rating-packages/delete', transformReq: idsFromDel },
  'user/company_comrating/baseData': { path: '/v1/admin/rating-packages/base-data' },
  'user/company_comrating/server': { path: '/v1/admin/rating-services/list' },
  'user/company_comrating/save': { path: '/v1/admin/rating-services' },
  'user/company_comrating/opera': { path: '/v1/admin/rating-services/opera' },
  'user/company_comrating/delserver': { path: '/v1/admin/rating-services/delete', transformReq: idsFromDel },
  'user/company_comrating/list': { path: '/v1/admin/rating-services/details', transformReq: pageQuery },
  'user/company_comrating/saves': { path: '/v1/admin/rating-services/details/save' },

  'system/set_guanjianci': { path: '/v1/admin/keywords/list', transformReq: pageQuery },
  'system/set_guanjianci/index': { path: '/v1/admin/keywords/list', transformReq: pageQuery },
  'system/set_guanjianci/keyWord': phpContent('keyword', 'map'),
  'system/set_guanjianci/save': { path: '/v1/admin/keywords' },
  'system/set_guanjianci/del': { path: '/v1/admin/keywords/delete', transformReq: idsFromDel },
  'system/set_guanjianci/recup': { path: '/v1/admin/keywords/recup' },
  'system/set_guanjianci/status': { path: '/v1/admin/keywords/status' },
  'system/set_web_config/index': phpContent('web-config', 'index'),
  'system/set_web_config/city': phpContent('web-config', 'city'),
  'system/set_web_config/save': { path: '/v1/admin/site-settings/batch' },
  'system/domain_list': { path: '/v1/admin/domains', transformReq: pageQuery },
  'system/domain_list/index': { path: '/v1/admin/domains', transformReq: pageQuery },
  'system/domain_list/save': { path: '/v1/admin/domains/upsert' },
  'system/domain_list/saveDomain': { path: '/v1/admin/domains/upsert' },
  'system/domain_list/del': { path: '/v1/admin/domains/delete', transformReq: idsFromDel },
  'system/domain_list/delDomain': { path: '/v1/admin/domains/delete', transformReq: idsFromDel },
  'system/domain_list/domainInfo': { path: '/v1/admin/domains/detail' },
  'system/domain_list/config': { path: '/v1/admin/domains/config' },
  'system/domain_group': { path: '/v1/admin/domain-admins', transformReq: pageQuery },
  'system/domain_group/adminList': { path: '/v1/admin/domain-admins', transformReq: pageQuery },
  'system/domain_group/save': { path: '/v1/admin/domain-admins/save' },
  'system/domain_group/saveAdmin': { path: '/v1/admin/domain-admins/save' },
  'system/domain_group/del': {
    path: '/v1/admin/domain-admins/delete',
    transformReq: (b) => idsFromDel({ ...b, id: b.uid ?? b.id ?? b.del }),
  },
  'system/domain_group/delAdmin': {
    path: '/v1/admin/domain-admins/delete',
    transformReq: (b) => idsFromDel({ ...b, id: b.uid ?? b.id ?? b.del }),
  },
  'system/set_cron': { path: '/v1/admin/cron/table', transformReq: pageQuery },
  'system/set_cron/index': { path: '/v1/admin/cron/table', transformReq: pageQuery },
  'system/set_cron/save': { path: '/v1/admin/cron/save' },
  'system/set_cron/del': { path: '/v1/admin/cron/delete', transformReq: idsFromDel },
  'system/set_cron/info': { path: '/v1/admin/cron/info' },
  'system/set_cron/run': { path: '/v1/admin/cron/run' },
  'system/set_cron/cronLog': { path: '/v1/admin/cron/logs', transformReq: pageQuery },
  'system/info_errorlog': { path: '/v1/admin/error-logs', transformReq: pageQuery },
  'system/info_errorlog/index': { path: '/v1/admin/error-logs', transformReq: pageQuery },
  'system/info_errorlog/del': { path: '/v1/admin/error-logs/delete', transformReq: idsFromDel },
  'system/info_systeminfo': { path: '/v1/admin/sysmsgs', transformReq: sysmsgQuery },
  'system/info_systeminfo/index': { path: '/v1/admin/sysmsgs', transformReq: sysmsgQuery },
  'system/info_systeminfo/sendSys': { path: '/v1/admin/sysmsgs/send' },
  'system/set_navmap': { path: '/v1/admin/navmap', transformReq: pageQuery },
  'system/set_navmap/index': { path: '/v1/admin/navmap', transformReq: pageQuery },
  'system/set_navmap/save': { path: '/v1/admin/navmap/save' },
  'system/set_navmap/del': { path: '/v1/admin/navmap/delete', transformReq: idsFromDel },
  'system/role_myuser': { path: '/v1/admin/rbac/myuser' },
  'system/role_myuser/index': { path: '/v1/admin/rbac/myuser' },
  'system/role_myuser/savePass': { path: '/v1/admin/rbac/me/password' },
  'system/set_tplset': { path: '/v1/admin/tpl/comtpl' },
  'system/set_tplset/comtpl': { path: '/v1/admin/tpl/comtpl' },
  'system/set_tplset/check_style': { path: '/v1/admin/tpl/style' },
  'system/set_module': { path: '/v1/admin/modules' },
  'system/set_module/index': { path: '/v1/admin/modules' },
  'system/set_module/save': { path: '/v1/admin/modules/save' },
  'system/category_userclass': catClass('userclass', 'list'),
  'system/category_userclass/index': catClass('userclass', 'list'),
  'system/category_userclass/ajax': catClass('userclass', 'ajax'),
  'system/category_userclass/save': catClass('userclass', 'save'),
  'system/category_userclass/add': catClass('userclass', 'add'),
  'system/category_userclass/del': catClass('userclass', 'del'),
  'system/category_userclass/up': catClass('userclass', 'up'),
  'system/category_comclass': catClass('comclass', 'list'),
  'system/category_comclass/index': catClass('comclass', 'list'),
  'system/category_comclass/ajax': catClass('comclass', 'ajax'),
  'system/category_comclass/save': catClass('comclass', 'save'),
  'system/category_comclass/add': catClass('comclass', 'add'),
  'system/category_comclass/del': catClass('comclass', 'del'),
  'system/category_comclass/up': catClass('comclass', 'up'),
  'system/category_partclass': catClass('partclass', 'list'),
  'system/category_partclass/index': catClass('partclass', 'list'),
  'system/category_partclass/ajax': catClass('partclass', 'ajax'),
  'system/category_partclass/save': catClass('partclass', 'save'),
  'system/category_partclass/add': catClass('partclass', 'add'),
  'system/category_partclass/del': catClass('partclass', 'del'),
  'system/category_partclass/up': catClass('partclass', 'up'),
  'system/category_reason': catClass('reason', 'list'),
  'system/category_reason/index': catClass('reason', 'list'),
  'system/category_reason/ajax': catClass('reason', 'ajax'),
  'system/category_reason/save': catClass('reason', 'save'),
  'system/category_reason/del': catClass('reason', 'del'),
  'system/category_industry': catClass('industry', 'list'),
  'system/category_industry/index': catClass('industry', 'list'),
  'system/category_industry/ajax': catClass('industry', 'ajax'),
  'system/category_industry/add': catClass('industry', 'add'),
  'system/category_industry/save': catClass('industry', 'save'),
  'system/category_industry/del': catClass('industry', 'del'),
  'system/category_city': catClass('city', 'list'),
  'system/category_city/index': catClass('city', 'list'),
  'system/category_city/ajax': catClass('city', 'ajax'),
  'system/category_city/del': catClass('city', 'del'),
  'system/category_city/get_city_children': catClass('city', 'children'),
  'system/category_city/add_single': catClass('city', 'add_single'),
  'system/category_city/up_single': catClass('city', 'up_single'),
  'system/category_city/upp': catClass('city', 'upp'),
  'system/category_city/ajaxpinyin': catClass('city', 'ajaxpinyin'),
  'system/category_city/clearpinyin': catClass('city', 'clearpinyin'),
  'system/category_city/ajaxchachong': catClass('city', 'ajaxchachong'),
  'system/category_schoolclass': catClass('schoolclass', 'list'),
  'system/category_schoolclass/index': catClass('schoolclass', 'list'),
  'system/category_schoolclass/ajax': catClass('schoolclass', 'ajax'),
  'system/category_schoolclass/save': catClass('schoolclass', 'save'),
  'system/category_schoolclass/del': catClass('schoolclass', 'del'),
  'system/category_schoolclass/up': catClass('schoolclass', 'up'),
  'system/category_px_subject_class': catClass('px_subject', 'list'),
  'system/category_px_subject_class/index': catClass('px_subject', 'list'),
  'system/category_px_subject_class/ajax': catClass('px_subject', 'ajax'),
  'system/category_px_subject_class/save': catClass('px_subject', 'save'),
  'system/category_px_subject_class/del': catClass('px_subject', 'del'),
  'system/category_px_subject_class/up': catClass('px_subject', 'up'),
  'system/category_introduce_class': catClass('introduce', 'list'),
  'system/category_introduce_class/index': catClass('introduce', 'list'),
  'system/category_introduce_class/ajax': catClass('introduce', 'ajax'),
  'system/category_introduce_class/save': catClass('introduce', 'save'),
  'system/category_introduce_class/del': catClass('introduce', 'del'),
  'system/category_introduce_class/classadd': catClass('introduce', 'classadd'),

  'yunying/yingxiao_tuiguang': { path: '/v1/admin/marketing/email-status' },
  'yunying/yingxiao_tuiguang/index': { path: '/v1/admin/marketing/email-status' },
  'yunying/yingxiao_tuiguang/msgtg': { path: '/v1/admin/marketing/sms-status' },
  'yunying/yingxiao_tuiguang/send': {
    path: '/v1/admin/marketing/email-send',
    transformReq: (b) => ({
      emails: csvList(b.email_user || b.emails),
      title: String(b.email_title || b.title || ''),
      content: String(b.content || ''),
      utype: Number(b.utype || 0),
    }),
  },
  'yunying/yingxiao_tuiguang/msgsave': {
    path: '/v1/admin/marketing/sms-send',
    transformReq: (b) => ({
      mobiles: csvList(b.userarr || b.mobiles),
      content: String(b.content || ''),
      utype: Number(b.utype || 0),
    }),
  },
  'yunying/yingxiao_tuiguang/sendPromotion': {
    path: '/v1/admin/marketing/promote',
    transformReq: (b) => ({
      ...b,
      emails: csvList(b.email_user || b.emails),
      mobiles: csvList(b.userarr || b.mobiles),
      title: String(b.email_title || b.title || ''),
      content: String(b.content || ''),
      utype: Number(b.utype || 0),
    }),
  },
  'yunying/yingxiao_tuiguang/xls': { path: '/v1/admin/marketing/export' },
  'yunying/yingxiao_tuiguang/finish': { path: '/v1/admin/marketing/finish' },
  'yunying/yingxiao_tuiguang/job': { path: '/v1/admin/marketing/job' },
  'yunying/yingxiao_tuiguang/resume': { path: '/v1/admin/marketing/resume' },
  'yunying/yingxiao_hrlog': { path: '/v1/admin/hr-logs', transformReq: pageQuery },
  'yunying/yingxiao_hrlog/index': { path: '/v1/admin/hr-logs', transformReq: pageQuery },
  'yunying/special_special/com': {
    path: '/v1/admin/specials/companies',
    transformReq: (b) => ({ ...pageQuery(b), sid: Number(b.id || b.sid || 0) }),
  },
  'yunying/special_special/statuscom': {
    path: '/v1/admin/specials/companies/status',
    transformReq: (b) => ({
      ...b,
      pid: String(b.pid || b.id || ''),
      status: Number(b.status || 0),
      statusbody: String(b.statusbody || ''),
    }),
  },
  'yunying/special_special/delcom': { path: '/v1/admin/specials/companies/delete', transformReq: idsFromDel },

  'tool/weixinrecord': { path: '/v1/admin/weixin-records', transformReq: pageQuery },
  'tool/weixinrecord/index': { path: '/v1/admin/weixin-records', transformReq: pageQuery },
  'tool/fabutool': { path: '/v1/admin/wxpub-temps/list', transformReq: pageQuery },
  'tool/fabutool/index': { path: '/v1/admin/wxpub-temps/list', transformReq: pageQuery },
  'tool/fabutool/wxPubTempDel': { path: '/v1/admin/wxpub-temps/delete', transformReq: idsFromDel },
  'tool/fabutool/wxPubTempSave': { path: '/v1/admin/wxpub-temps' },
  'tool/gsdConfig': { path: '/v1/admin/gsd-config' },
  'tool/gsdConfig/index': { path: '/v1/admin/gsd-config' },
  'tool/gsdConfig/setIpAddressConfig': { path: '/v1/admin/gsd-config/save', transformReq: wrapItems },
  'tool/dataOss': { path: '/v1/admin/oss-config' },
  'tool/dataOss/index': { path: '/v1/admin/oss-config' },
  'tool/dataOss/setOssConfig': { path: '/v1/admin/oss-config/save', transformReq: wrapItems },
  'tool/fastlogin': { path: '/v1/admin/fastlogin-config' },
  'tool/fastlogin/index': { path: '/v1/admin/fastlogin-config' },
  'tool/fastlogin/save': { path: '/v1/admin/fastlogin-config/save', transformReq: wrapItems },
  'tool/dataCall': { path: '/v1/admin/data-call/list', transformReq: pageQuery },
  'tool/dataCall/index': { path: '/v1/admin/data-call/list', transformReq: pageQuery },
  'tool/dataCall/save': { path: '/v1/admin/data-call' },
  'tool/dataCall/del': { path: '/v1/admin/data-call/delete', transformReq: idsFromDel },

  'yunying/special_special': phpContent('special', 'index'),
  'yunying/special_special/index': phpContent('special', 'index'),
  'yunying/special_special/add': phpContent('special', 'add'),
  'yunying/special_special/del': phpContent('special', 'delete'),
  'yunying/special_special/setOrder': phpContent('special', 'setOrder'),
  'yunying/special_special/recommend': phpContent('special', 'recommend'),
  'yunying/special_special/ajaxsort': phpContent('special', 'ajaxsort'),
  'yunying/special_special/setFamous': phpContent('special', 'setFamous'),
  'yunying/special_special/addlist': phpContent('special', 'addlist'),
  'yunying/special_special/set_comaddsearch': phpContent('special', 'set_comaddsearch'),
  'yunying/special_special/audit': phpContent('special', 'audit'),
  'yunying/special_special/comjob': phpContent('special', 'comjob'),
  'yunying/ad': phpContent('ads', 'index'),
  'yunying/ad/index': phpContent('ads', 'index'),
  'yunying/ad/info': phpContent('ads', 'info'),
  'yunying/ad/ad_saveadd': phpContent('ads', 'ad_saveadd'),
  'yunying/ad/del': phpContent('ads', 'delete'),
  'yunying/ad/preview': phpContent('ads', 'preview'),
  'yunying/ad/check': phpContent('ads', 'check'),
  'yunying/ad/cache_ad': phpContent('ads', 'cache_ad'),
  'yunying/ad/ctime': phpContent('ads', 'ctime'),
  'yunying/ad/upsort': phpContent('ads', 'upsort'),
  'yunying/ad_class': phpContent('ad-class', 'index'),
  'yunying/ad_class/index': phpContent('ad-class', 'index'),
  'yunying/ad_class/info': phpContent('ad-class', 'info'),
  'yunying/ad_class/addclass': phpContent('ad-class', 'addclass'),
  'yunying/ad_class/del': phpContent('ad-class', 'delete'),
  'yunying/ad_class/delbuy': phpContent('ad-class', 'delbuy'),
  'yunying/ad_class/upsort': phpContent('ad-class', 'upsort'),
  'yunying/finance_company_order': phpContent('finance-order', 'index'),
  'yunying/finance_company_order/index': phpContent('finance-order', 'index'),
  'yunying/finance_company_order/searchType': phpContent('finance-order', 'searchType'),
  'yunying/finance_company_order/edit': phpContent('finance-order', 'edit'),
  'yunying/finance_company_order/save': phpContent('finance-order', 'save'),
  'yunying/finance_company_order/setpay': phpContent('finance-order', 'setpay'),
  'yunying/finance_company_order/del': phpContent('finance-order', 'delete'),
  'yunying/finance_company_order/xls': phpContent('finance-order', 'xls'),
  'yunying/finance_company_order/multiupload': phpContent('finance-order', 'multiupload'),
  'yunying/finance_company_order/uploadsave': phpContent('finance-order', 'uploadsave'),
  'yunying/finance_company_order/htpic_del': phpContent('finance-order', 'htpic_del'),
  'yunying/finance_company_pay': phpContent('finance-pay', 'index'),
  'yunying/finance_company_pay/index': phpContent('finance-pay', 'index'),
  'yunying/finance_company_pay/del': phpContent('finance-pay', 'delete'),
  'yunying/finance_recharge': phpContent('finance-recharge', 'index'),
  'yunying/finance_recharge/index': phpContent('finance-recharge', 'index'),
  'yunying/finance_recharge/jifenSave': phpContent('finance-recharge', 'jifenSave'),
  'yunying/finance_recharge/comvip': phpContent('finance-recharge', 'comvip'),
  'yunying/finance_recharge/comservice': phpContent('finance-recharge', 'comservice'),
  'yunying/finance_recharge/getservice': phpContent('finance-recharge', 'getservice'),
  'yunying/finance_recharge/searchname': {
    path: '/v1/admin/php-content/finance-recharge/searchname',
    rawBody: true,
  },
  'yunying/finance_recharge/searchcom': {
    path: '/v1/admin/php-content/finance-recharge/searchcom',
    rawBody: true,
  },
  'yunying/report': { path: '/v1/admin/reports' },
  'yunying/report/index': { path: '/v1/admin/reports' },

  'system/category_job_class': catKind('job'),
  'system/category_job_class/index': catKind('job'),
  'system/category_job_class/ajax': phpContent('job-class', 'ajax'),
  'system/category_job_class/setrec': phpContent('job-class', 'setrec'),
  'system/category_job_class/get_class': phpContent('job-class', 'get_class'),
  'system/category_job_class/up': phpContent('job-class', 'up'),
  'system/category_job_class/getJobClass': phpContent('job-class', 'getJobClass'),
  'system/category_job_class/classadd': phpContent('job-class', 'classadd'),
  'system/category_job_class/ajaxchachong': phpContent('job-class', 'ajaxchachong'),
  'system/category_job_class/ajaxpinyin': phpContent('job-class', 'ajaxpinyin'),
  'system/category_job_class/move': phpContent('job-class', 'move'),
  'system/singlepage': phpContent('pages', 'index'),
  'system/singlepage/index': phpContent('pages', 'index'),
  'system/singlepage/add': phpContent('pages', 'add'),
  'system/singlepage/save': phpContent('pages', 'save'),
  'system/singlepage/del': phpContent('pages', 'delete'),
  'system/singlepage/make': phpContent('pages', 'make'),
  'system/singlepage/ajax': phpContent('pages', 'ajax'),
  'tool/weixinmenu/index': phpContent('wx-nav', 'config'),
  'tool/weixinmenu/save': { path: '/v1/admin/site-settings/batch' },
  'tool/weixinmenu/wxnav': phpContent('wx-nav', 'wxnav'),
  'tool/weixinmenu/savenav': phpContentRaw('wx-nav', 'savenav'),
  'tool/weixinmenu/delnav': phpContent('wx-nav', 'delnav'),
  'tool/weixinmenu/ajaxnav': phpContent('wx-nav', 'ajaxnav'),
  'tool/weixinmenu/creatnav': phpContentRaw('wx-nav', 'creatnav'),
  'tool/weixinmenu/zdkeyword': phpContent('wx-nav', 'zdkeyword'),
  'tool/weixinmenu/delkeyword': phpContent('wx-nav', 'delkeyword'),
  'tool/weixinmenu/getzdkeyword': phpContent('wx-nav', 'getzdkeyword'),
  'tool/weixinmenu/saveZdKeyword': phpContent('wx-nav', 'save-zdkeyword'),
  'tool/emailset/ceshi': phpContent('email-set', 'ceshi'),
  'tool/emailset/gettpl': phpContent('email-set', 'gettpl'),
  'tool/emailset/savetpl': phpContent('email-set', 'savetpl'),
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
  'user/weipin_once': { list: '/v1/admin/once-jobs', status: '/v1/admin/once-jobs/status' },
  'user/weipin_tiny': { list: '/v1/admin/tiny', status: '/v1/admin/tiny/status' },
  'user/users_resume': { list: '/v1/admin/resumes', status: '/v1/admin/resumes/status' },
  'user/users_member': { list: '/v1/admin/users', status: '/v1/admin/users/status', del: '/v1/admin/users/status' },
  'user/users_usercert': { list: '/v1/admin/user-certs', status: '/v1/admin/user-certs/status' },
  'neirong/question': { list: '/v1/admin/questions', del: '/v1/admin/questions/delete', status: '/v1/admin/questions/state' },
  'neirong/question_class': { list: '/v1/admin/question-classes/list', del: '/v1/admin/question-classes/delete', save: '/v1/admin/question-classes' },
  'neirong/zhaopinhui': { list: '/v1/admin/fairs' },
  'neirong/zph_space': { list: '/v1/admin/fairs/spaces', del: '/v1/admin/fairs/spaces/delete', save: '/v1/admin/fairs/spaces/upsert' },
  'neirong/gongzhao': { list: '/v1/admin/gongzhao/list', del: '/v1/admin/gongzhao/delete', save: '/v1/admin/gongzhao' },
  'neirong/announcement': { list: '/v1/admin/announcements/list', del: '/v1/admin/announcements/delete', save: '/v1/admin/announcements' },
  'neirong/news': { list: '/v1/admin/articles/list', del: '/v1/admin/articles/delete', save: '/v1/admin/articles' },
  'yunying/special_special': { list: '/v1/admin/specials' },
  'yunying/ad': { list: '/v1/admin/php-content/ads/index' },
  'yunying/report': { list: '/v1/admin/reports', status: '/v1/admin/reports/status' },
  'yunying/report_job': { list: '/v1/admin/reports', status: '/v1/admin/reports/status', del: '/v1/admin/reports/status' },
  'yunying/report_resume': { list: '/v1/admin/reports', status: '/v1/admin/reports/status' },
  'yunying/report_ask': { list: '/v1/admin/reports', status: '/v1/admin/reports/status' },
  'yunying/report_advise': { list: '/v1/admin/reports', status: '/v1/admin/reports/status' },
  'yunying/shop_reward': { list: '/v1/admin/rewards/list', del: '/v1/admin/rewards/delete', status: '/v1/admin/rewards/status', save: '/v1/admin/rewards' },
  'yunying/shop_class': { list: '/v1/admin/redeem-classes/list', del: '/v1/admin/redeem-classes/delete', save: '/v1/admin/redeem-classes' },
  'yunying/shop_list': { list: '/v1/admin/redeem-orders', status: '/v1/admin/redeem-orders/approve' },
  'system/category_job_class': { list: '/v1/admin/categories/list', save: '/v1/admin/categories', del: '/v1/admin/categories/update' },
  'system/role_user': { list: '/v1/admin/rbac/users', status: '/v1/admin/rbac/users/status' },
  'system/role_ugroup': { list: '/v1/admin/rbac/groups' },
  'system/role_myuser': { list: '/v1/admin/rbac/myuser' },
  'system/set_navigation': { list: '/v1/admin/nav/list', save: '/v1/admin/nav', del: '/v1/admin/nav/update' },
  'system/admin_nav': { list: '/v1/admin/nav/list', save: '/v1/admin/nav' },
  'system/set_cron': { list: '/v1/admin/cron/table', save: '/v1/admin/cron/save', del: '/v1/admin/cron/delete' },
  'system/warning': { list: '/v1/admin/warnings/list' },
  'system/info_feedback': { list: '/v1/admin/feedback', status: '/v1/admin/feedback/status', del: '/v1/admin/feedback/status' },
  'system/set_friendlink': { list: '/v1/admin/friend-links/list', del: '/v1/admin/friend-links/delete', save: '/v1/admin/friend-links', status: '/v1/admin/friend-links' },
  'system/set_config': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'system/set_payset': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'system/set_seo': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'system/seoset': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'system/set_regset': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'system/set_module': { list: '/v1/admin/modules', save: '/v1/admin/modules/save' },
  'tool/dataRecycle': { list: '/v1/admin/recycle-bin', del: '/v1/admin/recycle-bin/purge' },
  'tool/emaillog': { list: '/v1/admin/email-logs' },
  'tool/messagelog': { list: '/v1/admin/sms-logs' },
  'tool/weixinmenu': { list: '/v1/admin/wx-navs', save: '/v1/admin/wx-navs/upsert', del: '/v1/admin/wx-navs/delete' },
  'tool/emailset': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'tool/messageset': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'common/cache': { list: '/v1/admin/cache/php-dicts' },
  'user/company_comrating': { list: '/v1/admin/rating-packages/list', save: '/v1/admin/rating-packages', del: '/v1/admin/rating-packages/delete' },
  'user/company_pic': { list: '/v1/admin/company-photos', status: '/v1/admin/company-photos/status' },
  'user/users_pic': { list: '/v1/admin/user-photos', status: '/v1/admin/user-photos/status' },
  'user/users_msg': { list: '/v1/admin/user-msgs', del: '/v1/admin/user-msgs/delete' },
  'user/users_trust': { list: '/v1/admin/resumes' },
  'user/users_userset': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'user/company_comset': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'user/company_news': { list: '/v1/admin/company-news', status: '/v1/admin/company-news/status' },
  'user/company_product': { list: '/v1/admin/company-products', status: '/v1/admin/company-products/status' },
  'user/company_interview': { list: '/v1/admin/company-interviews' },
  'user/company_pay': { list: '/v1/admin/orders' },
  'user/company_job_refresh_log': { list: '/v1/admin/job-refresh-logs' },
  'user/company_company': { list: '/v1/admin/companies' },
  'user/admin_member': { list: '/v1/admin/rbac/users' },
  'yunying/yingxiao_tuiguang': { list: '/v1/admin/marketing/email-status' },
  'yunying/yingxiao_hbconfig': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'yunying/yingxiao_hrlog': { list: '/v1/admin/hr-logs' },
  'yunying/shop_set': { list: '/v1/admin/site-settings/list', save: '/v1/admin/site-settings/batch' },
  'neirong/evaluate': { list: '/v1/admin/evaluate/papers/list', del: '/v1/admin/evaluate/papers/delete', save: '/v1/admin/evaluate/papers' },
  'neirong/toolbox_doc': { list: '/v1/admin/toolbox/docs/list', del: '/v1/admin/toolbox/docs/delete', save: '/v1/admin/toolbox/docs', status: '/v1/admin/toolbox/docs/show' },
  'neirong/toolbox_class': { list: '/v1/admin/toolbox/classes/list', del: '/v1/admin/toolbox/classes/delete', save: '/v1/admin/toolbox/classes' },
  'system/set_tplset': { list: '/v1/admin/tpl/comtpl', save: '/v1/admin/tpl/style' },
  'system/domain_group': { list: '/v1/admin/domain-admins' },
  'system/domain_list': { list: '/v1/admin/domains', save: '/v1/admin/domains/upsert', del: '/v1/admin/domains/delete' },
  'system/singlepage': { list: '/v1/admin/php-content/pages/index' },
  'system/singleclass': { list: '/v1/admin/categories/list' },
  'system/category_introduce_class': { list: '/v1/admin/desc-classes/list' },
  'system/set_navmap': { list: '/v1/admin/navmap', save: '/v1/admin/navmap/save', del: '/v1/admin/navmap/delete' },
  'system/category_reason': { list: '/v1/admin/categories/list' },
  'system/info_systeminfo': { list: '/v1/admin/sysmsgs' },
  'system/info_errorlog': { list: '/v1/admin/error-logs', del: '/v1/admin/error-logs/delete' },
  'system/role_logrecord': { list: '/v1/admin/admin-logs' },
  'tool/fabutool': { list: '/v1/admin/wxpub-temps/list', del: '/v1/admin/wxpub-temps/delete', save: '/v1/admin/wxpub-temps' },
  'tool/database': { list: '/v1/admin/recycle-bin' },
  'tool/generate_page': { list: '/v1/admin/cache/clear' },
  'tool/dataCall': { list: '/v1/admin/data-call/list', save: '/v1/admin/data-call', del: '/v1/admin/data-call/delete' },
  'tool/dataCollection': { list: '/v1/admin/articles/list' },
  'tool/weixinrecord': { list: '/v1/admin/weixin-records' },
  'tool/dataBoard': { list: '/v1/admin/dashboard/overview' },
  'tool/gsdConfig': { list: '/v1/admin/gsd-config', save: '/v1/admin/gsd-config/save' },
  'tool/admin_uc': { list: '/v1/admin/site-settings/list' },
  'tool/dataOss': { list: '/v1/admin/oss-config', save: '/v1/admin/oss-config/save' },
  'tool/fastlogin': { list: '/v1/admin/fastlogin-config', save: '/v1/admin/fastlogin-config/save' },
  'tool/generate_cache': { list: '/v1/admin/cache/clear' },
  'tool/generate_xml': { list: '/v1/admin/cache/clear' },
  'index/getIpAddress': { list: '/v1/admin/cache/php-dicts' },
  'index/getMobileAddress': { list: '/v1/admin/cache/php-dicts' },
  'index/getwxbindstatus': { list: '/v1/admin/cache/php-dicts' },
  'index/wxbind': { list: '/v1/admin/cache/php-dicts' },
}

function isIndexAction(a: string): boolean {
  const act = (a || '').toLowerCase()
  return !act || act === 'index'
}

function moduleAction(mod: ModuleRoutes, a: string): PhpAction | undefined {
  const act = (a || 'index').toLowerCase()
  // Exact verbs only. Prefix matches (delStatisDetail, configSave) used to hit the
  // wrong table and show up as 400/500. Named PHP actions must be in PHP_ADMIN_MAP.
  if ((act === 'del' || act === 'delete') && mod.del) {
    return { path: mod.del, transformReq: idsFromDel }
  }
  if ((act === 'status' || act === 'audit' || act === 'checkstate') && mod.status) {
    return { path: mod.status }
  }
  if (act === 'save' && mod.save) {
    return { path: mod.save }
  }
  if (act === 'add' && mod.save) {
    return { path: mod.save }
  }
  if (isIndexAction(act)) {
    return { path: mod.list, transformReq: pageQuery }
  }
  return undefined
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
  // Named PHP actions must match exactly (`m/c/a`). Do not fall back to `m/c`
  // list, which previously sent getCache/add onto companies/parts lists.
  if (!isIndexAction(a)) {
    const exact = PHP_ADMIN_MAP[`${m}/${c}/${a}`]
    if (exact) return exact
    const mod = MODULE_ROUTES[`${m}/${c}`]
    if (mod) return moduleAction(mod, a)
    return undefined
  }
  const keys = [`${m}/${c}/index`, `${m}/${c}`]
  for (const k of keys) {
    if (PHP_ADMIN_MAP[k]) return PHP_ADMIN_MAP[k]
  }
  const mod = MODULE_ROUTES[`${m}/${c}`]
  if (mod) return moduleAction(mod, a || 'index')
  return undefined
}
