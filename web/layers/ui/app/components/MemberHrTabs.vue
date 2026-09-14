<template>
  <div class="site-pc">
    <div class="newmember_tit">
      <ul>
        <li v-for="item in items" :key="item.to" :class="{ newmember_titcur: on(item.to) }">
          <NuxtLink :to="item.to">{{ item.label }}</NuxtLink>
        </li>
      </ul>
    </div>
  </div>
  <div class="site-h5">
    <div class="m_tab">
      <div class="m_tabbox category">
        <ul>
          <li v-for="item in items" :key="item.to" :class="{ m_tabactive: on(item.to) }" @click="go(item.to)">
            {{ item.label }}
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const { t } = useI18n()
const route = useRoute()
const items = computed(() => [
  { to: '/com/applications', label: t('member_com_00454') },
  { to: '/com/downloads', label: t('wap_00451') },
  { to: '/com/looks', label: t('member_com_00007') },
  { to: '/com/fans', label: t('wap_com_00407') },
  { to: '/com/views', label: t('member_com_00006') },
  { to: '/com/talent', label: t('member_com_00597') },
])
function on(to: string) {
  return route.path === to
}
function go(to: string) {
  return navigateTo(to)
}
</script>
