<script setup lang="ts">
import { ElMessage } from 'element-plus'
import { lc } from '~/utils/phpLc'
import { httpPost } from '~/utils/httpPost'

type MenuItem = {
  id: number
  keyid: number
  name: string
  route: string
  path: string
  classname: string
  menu: number
  sort: number
}

const route = useRoute()
const router = useRouter()
const { locale, setLocale } = useI18n()

const { data: menu } = await useAsyncData('admin-php-menu', () =>
  useApi().post<MenuItem[]>('/v1/admin/menu', {}),
)
const items = computed(() => (Array.isArray(menu.value) ? menu.value : []).filter((m) => m.id))
const byId = computed(() => {
  const map = new Map<number, MenuItem>()
  for (const m of items.value) map.set(m.id, m)
  return map
})
const roots = computed(() =>
  items.value.filter((m) => m.keyid === 0).sort((a, b) => a.sort - b.sort || a.id - b.id),
)
function children(id: number) {
  return items.value.filter((m) => m.keyid === id).sort((a, b) => a.sort - b.sort || a.id - b.id)
}
function hrefOf(m: MenuItem) {
  const p = (m.path || m.route || '').trim()
  if (!p) return ''
  return p.startsWith('/') ? p : `/${p}`
}
const shortcuts = computed(() => items.value.filter((m) => hrefOf(m) && m.menu === 1).slice(0, 20))

const subWidth = ref(false)
const curMenu = ref(0)
const curMenuOne = ref(0)
const curMenuTwo = ref(0)
const MenuOpen = ref<number[]>([0])
const msgNum = ref(0)
const msgNumLoad = ref(false)
const msgNumData = ref<Array<{ name: string; num: number; menudata: Record<string, unknown> }>>([])
const dialogLanguage = ref(false)
const dialogMap = ref(false)
const dialogShortcutMenu = ref(false)
const languageForm = reactive({ lang: 'zh_cn' })
const searchFormMap = reactive({ keyword: '' })
const tabList = ref<
  Array<{ nav_id: number; one_menu_id: number; two_menu_id: number; name: string; path: string; isdel: boolean; query?: Record<string, unknown> }>
>([
  { nav_id: 0, one_menu_id: 0, two_menu_id: 0, name: 'index', path: '/index', isdel: false },
])

const msgNumDef: Record<string, { name: string; path: string; query?: Record<string, unknown>; nval: number; oval: number; tval: number }> = {
  company_job: { name: 'admin_index_00055', path: '/companyjob', query: { state: '4' }, nval: 1, oval: 6, tval: 40 },
  partjob: { name: 'admin_index_00053', path: '/companyjob', query: { state: '4', tabs: 'partjob' }, nval: 1, oval: 6, tval: 40 },
  company: { name: 'admin_00316', path: '/companycrm', query: { status: '4' }, nval: 1, oval: 6, tval: 16 },
  company_cert: { name: 'admin_index_00027', path: '/companyrz', query: { status: '3' }, nval: 1, oval: 6, tval: 47 },
  resume_expect: { name: 'admin_index_00054', path: '/resume', query: { status: '4' }, nval: 1, oval: 7, tval: 33 },
  once_job: { name: 'admin_index_00033', path: '/weipin_once', query: { status: '3' }, nval: 1, oval: 6, tval: 48 },
  tiny: { name: 'admin_index_00034', path: '/weipin_tiny', query: { status: '2' }, nval: 1, oval: 156, tval: 36 },
  order: { name: 'admin_index_00016', path: '/chongzhidd', query: { order_state: '1' }, nval: 3, oval: 161, tval: 162 },
  reportjob: { name: 'admin_index_00014', path: '/reportjob', query: { status: '0' }, nval: 3, oval: 176, tval: 178 },
  warning: { name: 'admin_index_00057', path: '/warning', query: { status: '2' }, nval: 5, oval: 121, tval: 175 },
  handlenum: { name: 'admin_index_00017', path: '/feedback', query: { status: '1' }, nval: 5, oval: 198, tval: 65 },
}

function checkMenuOpen(id: number) {
  return MenuOpen.value.includes(id)
}
function MenuOpenChange(val: number) {
  const idx = MenuOpen.value.indexOf(val)
  if (idx > -1) MenuOpen.value.splice(idx, 1)
  else MenuOpen.value.push(val)
}
function checkMenu(val: number) {
  curMenu.value = val
  if (val === 0) navigateTo('/index')
}
function tabLabel(tab: { name: string }) {
  return lc(tab.name, null, tab.name)
}
function menuLabel(m: MenuItem) {
  return m.name
}
function filteredMapItems(parent: MenuItem) {
  const kw = searchFormMap.keyword.trim()
  const kids = children(parent.id)
  if (!kw) return kids
  return kids.filter((k) => (k.name || '').includes(kw) || hrefOf(k).includes(kw))
}

function checkMenuTwo(
  nval: number,
  oval: number,
  tval: number,
  name: string,
  path: string,
  query?: Record<string, unknown>,
) {
  curMenu.value = Number(nval) || 0
  curMenuOne.value = Number(oval) || 0
  curMenuTwo.value = Number(tval) || 0
  const p = path.startsWith('/') ? path : `/${path}`
  const exist = tabList.value.findIndex((t) => t.path === p)
  if (exist === -1) {
    tabList.value.push({
      nav_id: curMenu.value,
      one_menu_id: curMenuOne.value,
      two_menu_id: curMenuTwo.value,
      name,
      path: p,
      isdel: true,
      query,
    })
  }
  navigateTo({ path: p, query: (query || {}) as Record<string, string> })
}

function checkTab(idx: number) {
  const tab = tabList.value[idx]
  if (!tab) return
  curMenu.value = tab.nav_id
  curMenuOne.value = tab.one_menu_id
  curMenuTwo.value = tab.two_menu_id
  navigateTo({ path: tab.path, query: (tab.query || {}) as Record<string, string> })
}
function closeTab(idx: number) {
  if (!tabList.value[idx]?.isdel) return
  tabList.value.splice(idx, 1)
  const last = tabList.value[tabList.value.length - 1]
  if (last) checkTab(tabList.value.length - 1)
}
function closeTabOther() {
  tabList.value = tabList.value.filter((t) => !t.isdel || t.path === route.path)
}
function closeTabAll() {
  tabList.value = tabList.value.filter((t) => !t.isdel)
  navigateTo('/index')
}

async function getMsgNum() {
  msgNumLoad.value = true
  const res = await httpPost('m=index&c=msgNum', {})
  const raw = (res.data || {}) as Record<string, number>
  msgNum.value = Number(raw.msgNum || raw.msg_num || 0)
  const rows: typeof msgNumData.value = []
  for (const [k, def] of Object.entries(msgNumDef)) {
    const n = Number(raw[k] || 0)
    if (n > 0) {
      rows.push({
        name: lc(def.name),
        num: n,
        menudata: { nval: def.nval, oval: def.oval, tval: def.tval, name: def.name, path: def.path, query: def.query },
      })
    }
  }
  msgNumData.value = rows
  msgNumLoad.value = false
}

async function logout() {
  await $fetch(bffUrl('/api/auth/logout'), { method: 'POST', credentials: 'include' }).catch(() => undefined)
  if (import.meta.client) localStorage.removeItem('indexPath')
  await navigateTo('/login')
}
async function clearCache() {
  const res = await httpPost('m=index&c=del_cache', {})
  const body = res.data as { error?: number }
  if (body.error) ElMessage.error(lc('admin_index_00051'))
  else ElMessage.success(lc('admin_index_00052'))
}
function saveLanguage() {
  const lang = languageForm.lang || 'zh_cn'
  localStorage.setItem('lang', lang)
  document.cookie = `admin_lang=${lang.startsWith('en') ? 'en' : 'zh'}; path=/; max-age=31536000`
  setLocale(lang.startsWith('en') ? 'en' : 'zh')
  dialogLanguage.value = false
  location.reload()
}
function openPage(url: string) {
  window.open(url || '/', '_blank')
}

watch(
  () => route.path,
  (p) => {
    const hit = items.value.find((m) => hrefOf(m) === p)
    if (!hit) {
      if (p === '/index' || p === '/') curMenu.value = 0
      return
    }
    let cur: MenuItem | undefined = hit
    const seen = new Set<number>()
    while (cur && cur.keyid !== 0 && !seen.has(cur.id)) {
      seen.add(cur.id)
      cur = byId.value.get(cur.keyid)
    }
    if (cur && cur.keyid === 0) curMenu.value = cur.id
    curMenuTwo.value = hit.id
  },
  { immediate: true },
)

if (import.meta.client) {
  const homeapp = {
    $route: route,
    $router: router,
    checkMenuTwo,
    checkMenu,
    getMsgNum,
    msgNumData,
    msgNumLoad,
    lc,
  }
  window.homeapp = homeapp as unknown as Record<string, unknown>
  try {
    ;(window.parent as Window).homeapp = window.homeapp
  } catch {
    /* ignore cross-origin */
  }
}
onMounted(() => {
  getMsgNum()
})
</script>

<template>
  <section id="homeapp" class="subjectDome">
    <div class="subContent">
      <div class="subContNav" :class="{ subContNavCur: subWidth }">
        <div class="subHeadtop">
          <div class="subHeadLogo">
            <img src="/admin/php-admin/images/admin_new_logo.png" alt="" />
          </div>
          <div class="widthButn" :class="{ widthButnCur: subWidth }" @click="subWidth = !subWidth">
            <i class="el-icon-s-fold iconone" />
            <i class="el-icon-s-unfold icontwo" />
          </div>
        </div>
        <div class="subContNavTite">
          <div class="subNavTite">
            <i /><span>{{ lc('admin_index_00065') }}</span><i />
          </div>
          <div class="subNavLogo">
            <img src="/admin/php-admin/images/navimg.png" alt="" />
          </div>
        </div>
        <div class="subContNavLink">
          <ul v-if="curMenu == 0">
            <li :class="{ subContLinkCur: curMenuOne == 0 }">
              <div class="subNavLinkTite" @click="MenuOpenChange(0)">
                <div class="subNavLinkImg kjcz">
                  <span>{{ lc('admin_index_00068') }}</span>
                </div>
                <div class="subNavLinkIcon" :class="{ subNavLinkIconCur: checkMenuOpen(0) }">
                  <i class="el-icon-arrow-up iconup" /><i class="el-icon-arrow-down icondwon" />
                </div>
              </div>
              <div v-show="checkMenuOpen(0)" class="subNavLinkText">
                <a
                  v-for="m in shortcuts"
                  :key="m.id"
                  href="javascript:void(0);"
                  @click="checkMenuTwo(0, 0, m.id, m.name, hrefOf(m))"
                >
                  <span :class="{ curspan: curMenuTwo == m.id }">{{ m.name }}</span>
                </a>
              </div>
            </li>
          </ul>
          <ul v-for="root in roots" v-else :key="'nav-' + root.id" v-show="curMenu == root.id">
            <li v-for="sec in children(root.id)" :key="sec.id" :class="{ subContLinkCur: curMenuOne == sec.id }">
              <div class="subNavLinkTite" @click="MenuOpenChange(sec.id)">
                <div class="subNavLinkImg" :class="sec.classname">
                  <span>{{ sec.name }}</span>
                </div>
                <div class="subNavLinkIcon" :class="{ subNavLinkIconCur: checkMenuOpen(sec.id) }">
                  <i class="el-icon-arrow-up iconup" /><i class="el-icon-arrow-down icondwon" />
                </div>
              </div>
              <div v-show="checkMenuOpen(sec.id)" class="subNavLinkText">
                <a
                  v-for="leaf in children(sec.id)"
                  :key="leaf.id"
                  href="javascript:void(0);"
                  @click="checkMenuTwo(root.id, sec.id, leaf.id, leaf.name, hrefOf(leaf))"
                >
                  <span :class="{ curspan: curMenuTwo == leaf.id }">{{ leaf.name }}</span>
                </a>
              </div>
            </li>
          </ul>
        </div>
      </div>
      <div class="subContPage" :class="{ subContPageCur: subWidth }">
        <div class="subHeader">
          <div class="subHeaderLeft">
            <div class="subHeadNavs">
              <ul>
                <li :class="{ subHeadNavCue: curMenu == 0 }">
                  <a href="javascript:void(0)" @click="checkMenu(0)">{{ lc('wap_00191') }}</a>
                </li>
                <li v-for="root in roots" :key="root.id" :class="{ subHeadNavCue: curMenu == root.id }">
                  <a href="javascript:void(0)" @click="checkMenu(root.id)">{{ root.name }}</a>
                </li>
              </ul>
            </div>
          </div>
          <div class="subHeadRight">
            <div class="subHeadRigNumer">
              <el-popover placement="bottom" :width="450" trigger="hover" @show="getMsgNum">
                <div class="subHeaNumerDomes" v-loading="msgNumLoad">
                  <div class="subHeaNumerName"><span>{{ lc('admin_00093') }}</span></div>
                  <div v-if="msgNumData.length > 0" class="subHeaNumLibos">
                    <ul>
                      <li
                        v-for="item in msgNumData"
                        :key="item.name"
                        @click="
                          checkMenuTwo(
                            Number(item.menudata.nval),
                            Number(item.menudata.oval),
                            Number(item.menudata.tval),
                            String(item.menudata.name),
                            String(item.menudata.path),
                            item.menudata.query as Record<string, unknown>,
                          )
                        "
                      >
                        <div class="subHeaNumminc"><a href="javascript:void(0);">{{ item.name }}</a></div>
                        <div class="subHeaNumData"><a href="javascript:void(0);">( {{ item.num }} )</a></div>
                      </li>
                    </ul>
                  </div>
                  <div v-else class="subHeaNumNones">
                    <el-empty :description="lc('admin_00092')" />
                  </div>
                </div>
                <template #reference>
                  <el-button>
                    <el-badge :value="msgNum" :max="99" :hidden="msgNum === 0">
                      <el-button size="small">
                        <div class="subHeadRigIcon">
                          <img src="/admin/php-admin/images/head2.png" alt="" />
                        </div>
                      </el-button>
                    </el-badge>
                  </el-button>
                </template>
              </el-popover>
            </div>
            <div class="subHeadRigList" @click="openPage('/')">
              <div class="subHeadRigIcon">
                <img src="/admin/php-admin/images/head3.png" alt="" />
              </div>
            </div>
            <div class="subHeadRigList" @click="dialogMap = true">
              <div class="subHeadRigIcon">
                <img src="/admin/php-admin/images/head4.png" alt="" />
              </div>
            </div>
            <div class="subHeadRigList" @click="clearCache">
              <div class="subHeadRigIcon">
                <img src="/admin/php-admin/images/head5.png" alt="" />
              </div>
            </div>
            <div class="subHeadRigUser">
              <el-popover placement="top-start" :width="140" trigger="hover">
                <div class="subHeadlogout">
                  <div class="subjeHeadFlex" @click="dialogLanguage = true">
                    <span>{{ lc('admin_index_00075') }}</span>
                  </div>
                  <div class="subjeHeadTuichus">
                    <div @click="logout">
                      <img src="/admin/php-admin/images/admin_navicon7.png" alt="" />
                      <span>{{ lc('wap_user_00177') }}</span>
                    </div>
                  </div>
                </div>
                <template #reference>
                  <el-button>
                    <div class="subHeadRigIcon">
                      <img src="/admin/php-admin/images/head6.png" alt="" />
                    </div>
                  </el-button>
                </template>
              </el-popover>
            </div>
          </div>
        </div>
        <div class="subContPageTips">
          <div class="subContPageWidth">
            <ul>
              <li
                v-for="(tabItem, tabIndex) in tabList"
                :key="tabIndex"
                :class="{ subContTipsCur: route.path === tabItem.path }"
                @click="checkTab(tabIndex)"
              >
                <div class="spana">
                  <span class="curspan">{{ tabLabel(tabItem) }}</span>
                  <i v-if="tabItem.isdel" class="el-icon-close" @click.stop="closeTab(tabIndex)" />
                </div>
              </li>
            </ul>
          </div>
          <div class="subContPageCose">
            <el-popover placement="bottom" :width="100" trigger="hover">
              <div class="subPageBurt">
                <el-button link @click="router.go(0)">{{ lc('admin_00194') }}</el-button>
                <el-button link @click="closeTabOther">{{ lc('admin_index_00064') }}</el-button>
                <el-button link @click="closeTabAll">{{ lc('admin_index_00063') }}</el-button>
              </div>
              <template #reference>
                <el-button>
                  <i class="el-icon-menu" />
                </el-button>
              </template>
            </el-popover>
          </div>
        </div>
        <div class="subContPageInfo">
          <slot />
        </div>
      </div>
    </div>
    <el-dialog v-model="dialogMap" :title="lc('admin_index_00066')" width="680px">
      <el-input v-model="searchFormMap.keyword" :placeholder="lc('admin_user_00158')" />
      <div class="homeDiaCaidan" style="height: 420px; overflow: auto">
        <div v-for="root in roots" :key="root.id" class="homeDiaCaiConts">
          <div class="homeDiaCaiOntite"><span>{{ menuLabel(root) }}</span></div>
          <div v-for="sec in children(root.id)" :key="sec.id" class="homeDiaCaiLis">
            <div class="homeCaiTwoTite"><span>{{ menuLabel(sec) }}</span></div>
            <div class="homeCaiTwoNeir">
              <div v-for="leaf in filteredMapItems(sec)" :key="leaf.id" class="homeCaiTwocheck">
                <a
                  href="javascript:void(0);"
                  @click="checkMenuTwo(root.id, sec.id, leaf.id, leaf.name, hrefOf(leaf)); dialogMap = false"
                >{{ menuLabel(leaf) }}</a>
              </div>
            </div>
          </div>
        </div>
      </div>
    </el-dialog>
    <el-dialog v-model="dialogLanguage" :title="lc('admin_index_00075')" width="360px">
      <el-radio-group v-model="languageForm.lang">
        <el-radio value="en_us">English</el-radio>
        <el-radio value="zh_cn">{{ lc('admin_index_00071') }}</el-radio>
      </el-radio-group>
      <template #footer>
        <el-button @click="dialogLanguage = false">{{ lc('admin_user_weipin_00043') }}</el-button>
        <el-button type="primary" @click="saveLanguage">{{ lc('wap_com_00019') }}</el-button>
      </template>
    </el-dialog>
  </section>
</template>
