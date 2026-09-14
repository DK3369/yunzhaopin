<template>
  <div class="yun_w1200 member-shell" :class="kind === 'com' ? 'member-shell-com' : 'member-shell-user'">
    <template v-if="kind === 'user'">
      <div class="yun_m_leftsidebar site-pc">
        <div class="yun_m_leftsidebar_box">
          <ul class="yun_m_leftsidebar_list">
            <li v-for="item in userMain" :key="item.to" :class="{ yun_m_left_cur: active(item.to) }">
              <NuxtLink :to="item.to" class="nava">
                <i class="left_navicon" :class="item.icon" />
                <span>{{ item.label }}</span>
                <i v-if="item.badge === 'interview' && interviewBadge" class="left_sidebar_msg_icon">{{ interviewBadge }}</i>
              </NuxtLink>
            </li>
            <li
              v-if="userMore.length"
              class="member-nav-more"
              :class="{ yun_m_left_cur: moreActive(userMore) }"
              @mouseenter="userMoreOpen = true"
              @mouseleave="userMoreOpen = false"
            >
              <a href="javascript:;" class="nava" @click.prevent>
                <i class="left_navicon left_navicon_i9" />
                <span>{{ userMoreTitle }}</span>
              </a>
              <div v-show="userMoreOpen" class="user_more">
                <div v-for="g in userMore" :key="g.title" class="user_more_list">
                  <span class="user_more_name">{{ g.title }}</span>
                  <NuxtLink v-for="it in g.items" :key="it.to" :to="it.to" class="user_more_a">{{ it.label }}</NuxtLink>
                </div>
              </div>
            </li>
          </ul>
        </div>
        <div v-if="wxQr" class="left_wx_box">
          <div class="left_wx_box_tip">{{ $t('member_user_00182') }}</div>
          <dl class="left_wx_box_dl">
            <dt><img :src="wxQr" alt="" width="150" height="150" /></dt>
            <dd class="left_wx_box_tit">{{ $t('wap_user_00191') }}</dd>
          </dl>
        </div>
      </div>
      <div class="yun_m_rightsidebar">
        <div class="wap_member member-shell-slot">
          <slot />
        </div>
      </div>
    </template>
    <div v-else class="memberSubject">
      <div class="admin_mainbody member-shell-com-body">
        <div class="sidebar site-pc">
          <div class="left_box">
            <ul class="left_nav_ul">
              <li v-for="item in comMain" :key="item.to" :class="{ left_nav_newcur: active(item.to) }">
                <span>
                  <NuxtLink :to="item.to" class="new_com_nav_a">
                    <i class="com_left_icon" :class="item.icon" />
                    {{ item.label }}
                    <i v-if="item.badge === 'resume' && resumeBadge" class="com_icon com_icon_new">{{ resumeBadge }}</i>
                  </NuxtLink>
                </span>
              </li>
              <li
                v-if="comMore.length"
                class="more_box member-nav-more"
                :class="{ left_nav_newcur: moreActive(comMore) }"
                @mouseenter="comMoreOpen = true"
                @mouseleave="comMoreOpen = false"
              >
                <span>
                  <a href="javascript:;" class="new_com_nav_a" @click.prevent>
                    <i class="com_left_icon com_left_icon9" />
                    {{ comMoreTitle }}
                  </a>
                </span>
                <div v-show="comMoreOpen" class="user_more">
                  <div v-for="g in comMore" :key="g.title" class="user_more_list">
                    <span class="user_more_name">{{ g.title }}</span>
                    <NuxtLink v-for="it in g.items" :key="it.to" :to="it.to" class="user_more_a">{{ it.label }}</NuxtLink>
                  </div>
                </div>
              </li>
            </ul>
          </div>
        </div>
        <div class="memberSubCont">
          <div class="memberSubRight memberSubRight--full">
            <slot />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { MemberMoreGroup } from '../composables/useMemberNav'

const props = withDefaults(defineProps<{ kind?: 'user' | 'com' }>(), { kind: 'user' })

const {
  userMain,
  userMore,
  userMoreTitle,
  comMain,
  comMore,
  comMoreTitle,
} = useMemberNav()
const { wxQr } = useSiteChrome()
const api = useApi()
const route = useRoute()
const userMoreOpen = ref(false)
const comMoreOpen = ref(false)

const { data: userDash } = useAsyncData(
  () => `member-shell-user-dash-${props.kind}`,
  () =>
    props.kind === 'user'
      ? api.post<{ wkyqnum?: number }>('/v1/mcenter/dashboard', {}).catch(() => null)
      : Promise.resolve(null),
)
const { data: comDash } = useAsyncData(
  () => `member-shell-com-dash-${props.kind}`,
  () =>
    props.kind === 'com'
      ? api.post<{ applies_unread?: number }>('/v1/mcenter/com-dashboard', {}).catch(() => null)
      : Promise.resolve(null),
)

const interviewBadge = computed(() => Number(userDash.value?.wkyqnum || 0))
const resumeBadge = computed(() => Number(comDash.value?.applies_unread || 0))

function active(to: string) {
  if (to === '/user' || to === '/com') return route.path === to
  if (to === '/com/applications') {
    return ['/com/applications', '/com/downloads', '/com/looks', '/com/views', '/com/fans', '/com/talent'].some(
      (p) => route.path === p || route.path.startsWith(`${p}/`),
    )
  }
  return route.path === to || route.path.startsWith(`${to}/`)
}

function moreActive(groups: MemberMoreGroup[]) {
  return groups.some((g) => g.items.some((it) => route.path === it.to || route.path.startsWith(`${it.to}/`)))
}
</script>
