<template>
  <div class="site-pc">
    <div class="about_content">
      <div class="about_left">
        <div class="about_left_h1">
          <span class="about_left_h1_span">{{ $t('common_02511') }}</span>
        </div>
        <div v-for="col in footerNav" :key="col.id" class="about_left_list">
          <div class="about_left_tit">
            <span class="bout_left_tit_span">{{ col.name }}</span>
          </div>
          <ul class="about_left_ul">
            <li v-for="item in col.list" :key="item.id" :class="{ about_left_ul_cur: isCurrent(item) }">
              <NuxtLink :to="item.to" :title="item.title">{{ item.title }}</NuxtLink>
            </li>
          </ul>
        </div>
      </div>
      <div class="about_right">
        <div class="about_right_h1">
          <span class="about_right_span">{{ title }}</span>
          <em class="about_right_cur">
            {{ $t('common_02137') }}：
            <NuxtLink to="/">{{ siteName || $t('common.home') }}</NuxtLink>
            &gt; {{ title }}
          </em>
        </div>
        <div class="about_right_p">
          <slot />
        </div>
      </div>
    </div>
    <div class="clear" />
  </div>
  <div class="site-h5 aboyt_cont">
    <h1>{{ title }}</h1>
    <slot />
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  title: string
  currentId?: number
  currentName?: string
}>()

const route = useRoute()
const { footerNav, siteName } = useSiteChrome()

function isCurrent(item: { id: number; title: string; to: string }) {
  if (props.currentId && item.id === props.currentId) return true
  if (route.path === item.to) return true
  if (props.currentName && item.title === props.currentName) return true
  return false
}
</script>
