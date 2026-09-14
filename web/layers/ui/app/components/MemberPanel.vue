<template>
  <div class="member-page" :class="kind === 'com' ? 'member-page-com' : 'member-page-user'">
    <div v-if="kind === 'user' && userTitle === 'user_new_tit'" class="site-pc">
      <div class="user_new_tit">
        <span class="user_new_tit_n">{{ title }}</span>
        <span v-if="sub" class="user_new_tit_r">{{ sub }}</span>
        <slot name="titExtra" />
        <slot name="pcFilters" />
      </div>
    </div>
    <slot name="h5Tabs" />
    <div :class="bodyClass">
      <div v-if="kind === 'user' && userTitle === 'h1'" class="site-pc member_right_index_h1 fltL">
        <span class="member_right_h1_span fltL">{{ title }}</span>
        <i class="member_right_h1_icon user_bg" />
        <slot name="titExtra" />
      </div>
      <div v-if="kind === 'com'" class="site-pc">
        <slot name="pcTabs" />
        <div v-if="!$slots.pcTabs" class="newmember_tit">
          <ul>
            <li class="newmember_titcur">
              <a href="javascript:;">{{ title }}</a>
            </li>
          </ul>
        </div>
      </div>
      <p v-if="error" class="muted">{{ $t('ui.load_failed') }}</p>
      <div v-if="kind === 'user' && userWrap === 'resume_box_list'" class="resume_box_list">
        <slot />
      </div>
      <div v-else-if="kind === 'user' && userWrap === 'user_resume_list'" class="user_resume_list">
        <slot />
      </div>
      <slot v-else />
      <div v-if="!error && empty" class="site-pc">
        <div v-if="kind === 'com'" class="com_msg_no">
          <p class="com_msg_no_name">{{ emptyText || $t('ui.no_items') }}</p>
          <NuxtLink v-if="emptyTo" :to="emptyTo" class="com_msg_no_bth com_submit">{{ emptyAction || $t('common.more') }}</NuxtLink>
        </div>
        <div v-else class="msg_no">
          <p>{{ emptyText || $t('ui.no_items') }}</p>
          <NuxtLink v-if="emptyTo" :to="emptyTo" class="msg_no_sq uesr_submit">{{ emptyAction || $t('common.more') }}</NuxtLink>
        </div>
      </div>
      <div v-if="!error && empty" class="site-h5">
        <div v-if="kind === 'com'" class="none_position_body">
          <div class="none_position_body_text">{{ emptyText || $t('ui.no_items') }}</div>
          <NuxtLink v-if="emptyTo" :to="emptyTo" class="com_msg_no_bth com_submit">{{ emptyAction || $t('common.more') }}</NuxtLink>
        </div>
        <div v-else class="wap_member_bgcar">
          <div class="wap_member_no">
            {{ emptyText || $t('ui.no_items') }}
            <div v-if="emptyTo">
              <NuxtLink :to="emptyTo" class="wap_member_no_submit">{{ emptyAction || $t('common.more') }}</NuxtLink>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/** @deprecated 标题与包层已拆开；仍可传，会映射到 title+wrap */
export type MemberUserShell = 'list' | 'resume' | 'plain'
export type MemberUserTitle = 'user_new_tit' | 'h1'
export type MemberUserWrap = 'resume_box_list' | 'user_resume_list' | 'none'

const USER_NEW_TIT_LIST = [
  '/user/applications',
  '/user/interviews',
  '/user/views',
  '/user/favorites',
  '/user/looks',
  '/user/recommend',
]

const H1_WITH_LIST = [
  '/user/follows',
  '/user/messages',
  '/user/consults',
  '/user/parts',
  '/user/inbox',
  '/user/reports',
  '/user/eval-logs',
  '/user/privacy',
  '/user/binding',
  '/user/pay',
  '/user/finance',
  '/user/searches',
  '/user/outbox',
  '/user/account',
  '/advice',
]

const props = defineProps<{
  title: string
  sub?: string
  error?: unknown
  empty?: boolean
  emptyText?: string
  emptyTo?: string
  emptyAction?: string
  kind?: 'user' | 'com'
  shell?: MemberUserShell
  userTitle?: MemberUserTitle
  userWrap?: MemberUserWrap
}>()
const route = useRoute()
const kind = computed(() => props.kind || (route.path.startsWith('/com') ? 'com' : 'user'))

function pathMatches(path: string, prefixes: string[]) {
  return prefixes.some((x) => path === x || path.startsWith(`${x}/`))
}

function inferUserTitle(path: string): MemberUserTitle {
  if (path === '/user/resume' || path.startsWith('/user/resume/') || path === '/user/expects') return 'user_new_tit'
  if (pathMatches(path, USER_NEW_TIT_LIST)) return 'user_new_tit'
  return 'h1'
}

function inferUserWrap(path: string): MemberUserWrap {
  if (path === '/user/resume' || path.startsWith('/user/resume/') || path === '/user/expects') return 'user_resume_list'
  if (pathMatches(path, USER_NEW_TIT_LIST) || pathMatches(path, H1_WITH_LIST)) return 'resume_box_list'
  return 'none'
}

function fromShell(shell: MemberUserShell): { title: MemberUserTitle; wrap: MemberUserWrap } {
  if (shell === 'resume') return { title: 'user_new_tit', wrap: 'user_resume_list' }
  if (shell === 'list') return { title: 'user_new_tit', wrap: 'resume_box_list' }
  return { title: 'h1', wrap: 'none' }
}

const userTitle = computed<MemberUserTitle>(() => {
  if (props.userTitle) return props.userTitle
  if (props.shell) return fromShell(props.shell).title
  return inferUserTitle(route.path.split('?')[0])
})
const userWrap = computed<MemberUserWrap>(() => {
  if (props.userWrap) return props.userWrap
  if (props.shell) return fromShell(props.shell).wrap
  return inferUserWrap(route.path.split('?')[0])
})
const bodyClass = computed(() =>
  kind.value === 'user' ? 'yun_m_rightbox fltR mt20 re member-page-body' : 'com_body member-page-body',
)
</script>
