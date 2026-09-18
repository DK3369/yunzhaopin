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
const props = withDefaults(defineProps<{ kind?: 'com' | 'user' }>(), { kind: 'com' })
const { t } = useI18n()
const route = useRoute()
const items = computed(() =>
  props.kind === 'user'
    ? [
        { to: '/user/member-right', label: t('wap_com_00097') },
        { to: '/redeem', label: t('wap_00398') },
        { to: '/user/orders', label: t('common_02029') },
      ]
    : [
        { to: '/com/member-right', label: t('wap_com_00097') },
        { to: '/redeem', label: t('wap_00398') },
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
