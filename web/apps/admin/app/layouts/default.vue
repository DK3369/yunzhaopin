<script setup lang="ts">
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
const api = useApi()
const { data: menu } = await useAsyncData('admin-php-menu', () => api.post<MenuItem[]>('/v1/admin/menu', {}))
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
function leaves(id: number): MenuItem[] {
  const kids = children(id)
  const nested = kids.flatMap((k) => leaves(k.id))
  const self = kids.filter((k) => hrefOf(k))
  return [...self, ...nested]
}
function rootIdForPath(path: string): number {
  const hit = items.value.find((m) => hrefOf(m) === path)
  if (!hit) return 0
  let cur: MenuItem | undefined = hit
  const seen = new Set<number>()
  while (cur && cur.keyid !== 0 && !seen.has(cur.id)) {
    seen.add(cur.id)
    cur = byId.value.get(cur.keyid)
  }
  return cur && cur.keyid === 0 ? cur.id : hit.keyid === 0 ? hit.id : 0
}
const activeRoot = ref(0)
watch(
  () => route.path,
  (p) => {
    activeRoot.value = rootIdForPath(p)
  },
  { immediate: true },
)
async function logout() {
  await $fetch(bffUrl('/api/auth/logout'), { method: 'POST', credentials: 'include' }).catch(() => undefined)
  await navigateTo('/login')
}
</script>

<template>
  <el-container class="php-admin" style="min-height: 100vh">
    <el-header class="php-admin__header" height="56px">
      <nav class="php-admin__top">
        <a :class="{ on: activeRoot === 0 }" href="javascript:void(0)" @click="activeRoot = 0; navigateTo('/index')">
          {{ $t('common.home') }}
        </a>
        <a
          v-for="root in roots"
          :key="root.id"
          :class="{ on: activeRoot === root.id }"
          href="javascript:void(0)"
          @click="activeRoot = root.id"
        >
          {{ root.name }}
        </a>
      </nav>
      <div class="php-admin__tools">
        <LangSwitch cookie-key="admin_lang" />
        <el-button size="small" @click="logout">{{ $t('common.logout') }}</el-button>
      </div>
    </el-header>
    <el-container>
      <el-aside width="220px" class="php-admin__aside">
        <el-menu :key="activeRoot" router :default-active="route.path">
          <template v-if="activeRoot === 0">
            <el-menu-item index="/index">{{ $t('common.home') }}</el-menu-item>
          </template>
          <template v-else>
            <template v-for="sec in children(activeRoot)" :key="sec.id">
              <el-sub-menu v-if="leaves(sec.id).length" :index="'s-' + sec.id">
                <template #title>{{ sec.name }}</template>
                <el-menu-item v-for="leaf in leaves(sec.id)" :key="leaf.id" :index="hrefOf(leaf)">
                  {{ leaf.name }}
                </el-menu-item>
              </el-sub-menu>
              <el-menu-item v-else-if="hrefOf(sec)" :index="hrefOf(sec)">{{ sec.name }}</el-menu-item>
            </template>
          </template>
        </el-menu>
      </el-aside>
      <el-main>
        <slot />
      </el-main>
    </el-container>
  </el-container>
</template>

<style scoped>
.php-admin__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid #ebeef5;
  background: #fff;
}
.php-admin__top {
  display: flex;
  gap: 8px;
  overflow: auto;
}
.php-admin__top a {
  padding: 8px 12px;
  color: #303133;
  white-space: nowrap;
  text-decoration: none;
}
.php-admin__top a.on {
  color: #409eff;
  font-weight: 600;
  border-bottom: 2px solid #409eff;
}
.php-admin__tools {
  display: flex;
  align-items: center;
  gap: 8px;
}
.php-admin__aside {
  border-right: 1px solid #ebeef5;
}
</style>
