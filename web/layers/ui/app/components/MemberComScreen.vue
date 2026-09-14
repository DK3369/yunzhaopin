<template>
  <div class="newmember_screenbox site-pc">
    <div class="newmember_screen">
      <ul v-if="tabs.length">
        <li v-for="tab in tabs" :key="String(tab.value)" :class="{ job_list_tit_cur: tab.on }">
          <a href="javascript:;" @click.prevent="tab.select()">
            {{ tab.label }}
            <span v-if="tab.count != null" class="job_list_tit_n">({{ tab.count }})</span>
          </a>
        </li>
      </ul>
      <div v-if="searchable" class="joblist_search">
        <div class="joblist_search_box">
          <input
            :value="keyword"
            type="text"
            class="joblist_search_box_text"
            :placeholder="searchPlaceholder || $t('admin_00149')"
            @input="$emit('update:keyword', ($event.target as HTMLInputElement).value)"
            @keydown.enter.prevent="$emit('search')"
          />
          <input type="button" class="joblist_search_bth" :value="$t('common.search')" @click="$emit('search')" />
        </div>
      </div>
      <div v-if="addTo" class="com_topbth_box">
        <NuxtLink :to="addTo" class="com_topbth">{{ addLabel || $t('wap_00322') }}</NuxtLink>
      </div>
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
withDefaults(
  defineProps<{
    tabs?: Array<{ value: unknown; label: string; on: boolean; count?: number; select: () => void }>
    keyword?: string
    searchable?: boolean
    searchPlaceholder?: string
    addTo?: string
    addLabel?: string
  }>(),
  { tabs: () => [] },
)
defineEmits<{ 'update:keyword': [string]; search: [] }>()
</script>
