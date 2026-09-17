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
          <li
            v-for="item in items"
            :key="'h5-' + item.to"
            :class="{ m_tabactive: on(item.to) }"
            @click="go(item.to)"
          >
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
const api = useApi()
const { data: current } = useAsyncData('com-vip-current', () =>
  api.post<{ rating_type?: number }>('/v1/mcenter/vip/current', {}).catch(() => null),
)
const { settings } = useSiteChrome()
const hideAdded = computed(
  () => Number(current.value?.rating_type) === 2 || String(settings.value.com_integral_online || '') === '4',
)
const items = computed(() =>
  [
    { to: '/com/member-right', label: t('wap_com_00380') },
    ...(hideAdded.value ? [] : [{ to: '/com/added', label: t('wap_com_00393') }]),
    { to: '/com/pay', label: t('common_01946') },
    { to: '/com/orders', label: t('common_02029') },
  ],
)
function on(to: string) {
  return route.path === to
}
function go(to: string) {
  return navigateTo(to)
}
</script>
