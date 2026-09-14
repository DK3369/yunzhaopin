<template>
  <div class="member-page" :class="kind === 'com' ? 'member-page-com' : 'member-page-user'">
    <div class="site-pc">
      <div v-if="kind === 'user'" class="user_new_tit">
        <span class="user_new_tit_n">{{ title }}</span>
        <span v-if="sub" class="user_new_tit_r">{{ sub }}</span>
      </div>
      <div v-else-if="!$slots.pcTabs" class="newmember_tit">
        <ul>
          <li class="newmember_titcur">
            <a href="javascript:;">{{ title }}</a>
          </li>
        </ul>
      </div>
      <slot name="pcTabs" />
    </div>
    <div class="site-h5">
      <slot name="h5Tabs" />
    </div>
    <div :class="kind === 'user' ? 'yun_m_rightbox member-page-body' : 'com_body member-page-body'">
      <p v-if="error" class="muted">{{ $t('ui.load_failed') }}</p>
      <slot />
      <div v-if="!error && empty" class="msg_no">
        <p>{{ emptyText || $t('ui.no_items') }}</p>
        <NuxtLink v-if="emptyTo" :to="emptyTo" class="msg_no_sq uesr_submit">{{ emptyAction || $t('common.more') }}</NuxtLink>
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
