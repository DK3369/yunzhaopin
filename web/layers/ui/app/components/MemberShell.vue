<template>
  <div>
    <div class="site-pc">
      <div class="yun_w1200 member-shell">
        <div v-if="kind === 'user'" class="yun_m_leftsidebar">
          <div class="yun_m_leftsidebar_box">
            <ul class="yun_m_leftsidebar_list">
              <li v-for="(item, i) in items" :key="item.to" :class="{ yun_m_left_cur: active(item.to) }">
                <NuxtLink :to="item.to" class="nava">
                  <i class="left_navicon" :class="`left_navicon_i${(i % 9) + 1}`" />
                  <span>{{ item.label }}</span>
                </NuxtLink>
              </li>
            </ul>
          </div>
        </div>
        <div v-else class="sidebar">
          <div class="left_box">
            <ul class="left_nav_ul">
              <li v-for="(item, i) in items" :key="item.to" :class="{ left_nav_newcur: active(item.to) }">
                <span>
                  <NuxtLink :to="item.to" class="new_com_nav_a">
                    <i class="com_left_icon" :class="`com_left_icon${(i % 12) + 1}`" />
                    {{ item.label }}
                  </NuxtLink>
                </span>
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
withDefaults(
  defineProps<{ items: Array<{ to: string; label: string }>; kind?: 'user' | 'com' }>(),
  { kind: 'user' },
)
const route = useRoute()
function active(to: string) {
  if (to === '/user' || to === '/com') return route.path === to
  return route.path === to || route.path.startsWith(`${to}/`)
}
</script>
