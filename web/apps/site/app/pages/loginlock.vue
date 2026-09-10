<script setup lang="ts">
const { t } = useI18n()
const { phone } = useSiteChrome()
useSeoMeta({ title: t('wap_00733') })
</script>

<template>
  <section class="site-inner">
    <div class="prompt_message">
      <div class="prompt_message_cont">
        <div class="prompt_message_i"><i class="prompt_message_i_icon_sd" /></div>
        <div class="prompt_message_tit">{{ $t('wap_00733') }}</div>
        <div class="prompt_message_p">{{ $t('common_01912') }}，{{ $t('wap_00734') }}</div>
        <div class="prompt_message_box">
          <div class="prompt_message_p2">{{ $t('wap_00735') }}</div>
          <div v-if="phone" class="prompt_message_tel">{{ phone }}</div>
        </div>
        <p style="padding-top: 16px">
          <NuxtLink to="/login">{{ $t('common.login') }}</NuxtLink>
        </p>
      </div>
    </div>
  </section>
</template>
