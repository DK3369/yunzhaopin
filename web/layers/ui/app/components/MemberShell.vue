<template>
  <div>
    <div class="site-pc">
      <div class="yun_w1200 member-shell">
        <div class="yun_m_leftsidebar">
          <div class="yun_m_leftsidebar_box">
            <ul class="yun_m_leftsidebar_list">
              <li v-for="item in items" :key="item.to" :class="{ yun_m_left_cur: active(item.to) }">
                <NuxtLink :to="item.to" class="nava">
                  <span>{{ item.label }}</span>
                </NuxtLink>
              </li>
            </ul>
          </div>
        </div>
        <div class="yun_m_rightsidebar">
          <slot />
        </div>
      </div>
    </div>
    <div class="site-h5">
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{ items: Array<{ to: string; label: string }> }>()
const route = useRoute()
function active(to: string) {
  if (to === '/user' || to === '/com') return route.path === to
  return route.path === to || route.path.startsWith(`${to}/`)
}
</script>
