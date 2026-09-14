<template>
  <div class="member-page" :class="kind === 'com' ? 'member-page-com' : 'member-page-user'">
    <div v-if="kind === 'user'" class="site-pc">
      <div class="user_new_tit">
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
    <div :class="kind === 'user' ? 'yun_m_rightbox fltR mt20 re member-page-body' : 'com_body member-page-body'">
      <p v-if="error" class="muted">{{ $t('ui.load_failed') }}</p>
      <div v-if="kind === 'user'" class="resume_box_list">
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
const props = defineProps<{
  title: string
  sub?: string
  error?: unknown
  empty?: boolean
  emptyText?: string
  emptyTo?: string
  emptyAction?: string
  kind?: 'user' | 'com'
}>()
const route = useRoute()
const kind = computed(() => props.kind || (route.path.startsWith('/com') ? 'com' : 'user'))
</script>
