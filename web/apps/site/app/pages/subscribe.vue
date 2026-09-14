<script setup lang="ts">
const { t } = useI18n()
const { me, siteName } = useSiteChrome()
const next = '/user/searches'

watch(
  () => me.value,
  (v) => {
    if (v) navigateTo(next)
  },
  { immediate: true },
)

useSeoMeta({ title: () => t('ui.subscribe_svc') })
useHead({ link: [{ rel: 'canonical', href: '/subscribe' }] })
</script>

<template>
  <div class="container">
    <div class="subscribe_cont">
      <div class="subscribe_cont_h1">
        <span class="subscribe_cont_h1_p">{{ $t('ui.subscribe_svc') }}</span>
        {{ siteName }}
      </div>
      <div class="subscribe_cont_list">
        <p class="post_read_body_pp">
          {{ $t('wap_user_00205') }}：{{ $t('member_user_00108') }}
        </p>
        <NuxtLink class="post_btn_big" :to="me ? next : `/login?next=${encodeURIComponent(next)}`">
          {{ me ? $t('member_user_00108') : $t('common.login') }}
        </NuxtLink>
      </div>
    </div>
    <div class="clear" />
  </div>
</template>
