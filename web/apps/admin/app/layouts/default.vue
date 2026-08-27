<script setup lang="ts">
type MenuItem = {
  id: number
  keyid: number
  name: string
  route: string
  path: string
  menu: number
  sort: number
}
const api = useApi()
const { data: menu } = await useAsyncData('admin-php-menu', () => api.post<MenuItem[]>('/v1/admin/menu', {}))
const items = computed(() => (Array.isArray(menu.value) ? menu.value : []).filter((m) => m.route))
const roots = computed(() => items.value.filter((m) => m.keyid === 0))
function children(id: number) {
  return items.value.filter((m) => m.keyid === id)
}
function leaves(id: number): MenuItem[] {
  const kids = children(id)
  const nested = kids.flatMap((k) => leaves(k.id))
  const self = kids.filter((k) => k.route)
  return [...self, ...nested]
}
async function logout() {
  await $fetch(bffUrl('/api/auth/logout'), { method: 'POST', credentials: 'include' }).catch(() => undefined)
  await navigateTo('/login')
}
</script>

<template>
  <el-container style="min-height: 100vh">
    <el-aside width="220px" style="border-right: 1px solid #eee">
      <h2 style="padding: 1rem 1rem 0; font-size: 16px">{{ $t('admin_index_00072') }}</h2>
      <div style="padding: 0 1rem 0.5rem">
        <LangSwitch cookie-key="admin_lang" />
      </div>
      <el-menu router :default-active="$route.path">
        <el-menu-item index="/">{{ $t('ui.dashboard') }}</el-menu-item>
        <template v-for="root in roots" :key="root.id">
          <el-sub-menu v-if="leaves(root.id).length" :index="'g-' + root.id">
            <template #title>{{ root.name }}</template>
            <el-menu-item v-for="leaf in leaves(root.id)" :key="leaf.id" :index="leaf.route">
              {{ leaf.name }}
            </el-menu-item>
          </el-sub-menu>
        </template>
      </el-menu>
      <div style="padding: 1rem">
        <el-button size="small" @click="logout">{{ $t('common.logout') }}</el-button>
      </div>
    </el-aside>
    <el-main>
      <slot />
    </el-main>
  </el-container>
</template>
