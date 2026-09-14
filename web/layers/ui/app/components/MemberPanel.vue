<template>
  <div class="member-page" :class="kind === 'com' ? 'member-page-com' : 'member-page-user'">
    <div v-if="kind === 'user'" class="site-pc">
      <div v-if="userShell === 'plain'" class="member_right_index_h1 fltL">
        <span class="member_right_h1_span fltL">{{ title }}</span>
        <i class="member_right_h1_icon user_bg" />
        <slot name="titExtra" />
      </div>
      <div v-else class="user_new_tit">
        <span class="user_new_tit_n">{{ title }}</span>
        <span v-if="sub" class="user_new_tit_r">{{ sub }}</span>
        <slot name="titExtra" />
        <slot name="pcFilters" />
      </div>
    </div>
    <div v-else class="site-pc">
      <slot name="pcTabs" />
      <div v-if="!$slots.pcTabs" class="newmember_tit">
        <ul>
          <li class="newmember_titcur">
            <a href="javascript:;">{{ title }}</a>
          </li>
        </ul>
      </div>
    </div>
    <slot name="h5Tabs" />
    <div :class="bodyClass">
      <p v-if="error" class="muted">{{ $t('ui.load_failed') }}</p>
      <div v-if="kind === 'user' && userShell === 'list'" class="resume_box_list">
        <slot />
      </div>
      <div v-else-if="kind === 'user' && userShell === 'resume'" class="user_resume_list">
        <slot />
      </div>
      <slot v-else />
      <div v-if="!error && empty" class="site-pc">
        <div class="msg_no">
          <p>{{ emptyText || $t('ui.no_items') }}</p>
          <NuxtLink v-if="emptyTo" :to="emptyTo" class="msg_no_sq uesr_submit">{{ emptyAction || $t('common.more') }}</NuxtLink>
        </div>
      </div>
      <div v-if="!error && empty" class="site-h5">
        <div class="wap_member_bgcar">
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
export type MemberUserShell = 'list' | 'resume' | 'plain'

const USER_LIST_PREFIXES = [
  '/user/applications',
  '/user/interviews',
  '/user/views',
  '/user/favorites',
  '/user/follows',
  '/user/looks',
  '/user/messages',
  '/user/consults',
  '/user/parts',
  '/user/inbox',
  '/user/reports',
  '/user/eval-logs',
  '/user/recommend',
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
}>()
const route = useRoute()
const kind = computed(() => props.kind || (route.path.startsWith('/com') ? 'com' : 'user'))

function inferUserShell(path: string): MemberUserShell {
  const p = path.split('?')[0]
  if (p === '/user/resume' || p.startsWith('/user/resume/') || p === '/user/expects') return 'resume'
  if (USER_LIST_PREFIXES.some((x) => p === x || p.startsWith(`${x}/`))) return 'list'
  return 'plain'
}

const userShell = computed(() => props.shell || inferUserShell(route.path))
const bodyClass = computed(() =>
  kind.value === 'user' ? 'yun_m_rightbox fltR mt20 re member-page-body' : 'com_body member-page-body',
)
</script>
