<template>
  <div class="yun-empty">
    <div class="yun-empty__pc site-pc">
      <div class="new_notip">
        <div class="new_notip_img">
          <img :src="pcSrc" alt="" />
        </div>
        <div class="new_notip_tit">{{ titleText }}</div>
        <div v-if="hint" class="new_notip_p">{{ hint }}</div>
      </div>
    </div>
    <div class="yun-empty__h5 site-h5">
      <div class="no_data">
        <div class="no_data_img">
          <img :src="h5Src" alt="" />
        </div>
        <i class="no_data_word">{{ titleText }}</i>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  title?: string
  hint?: string
}>()

const { t } = useI18n()
const base = String(useRuntimeConfig().app.baseURL || '/')
const isAdmin = base === '/admin/' || base.startsWith('/admin')
const pcSrc = isAdmin ? '/admin/php-admin/images/notip.png' : '/legacy/pc/images/notip.png'
const h5Src = isAdmin ? pcSrc : '/legacy/h5/images/home_emptygraph.png'
const titleText = computed(() => props.title || t('wap_js_00113'))
</script>

<style scoped>
.yun-empty :deep(.new_notip) {
  float: none;
  width: 100%;
  margin: 0;
}
.yun-empty :deep(.no_data) {
  height: auto;
  margin: 0.4rem 0 0.8rem;
}
.yun-empty :deep(.no_data_img img) {
  width: 100%;
  display: block;
}
</style>
