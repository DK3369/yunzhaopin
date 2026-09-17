<script setup lang="ts">
const props = defineProps<{ kind: 'user' | 'com' }>()
const { t } = useI18n()
const { settings } = useSiteChrome()
const priceName = computed(() => String(settings.value.integral_pricename || t('wap_user_00008')))

const rows = computed(() => {
  const s = settings.value
  if (props.kind === 'user') {
    return [
      { k: t('wap_user_00207'), v: s.integral_resume_top },
    ]
  }
  return [
    { k: t('wap_com_00106'), v: s.integral_job },
    { k: t('wap_com_00029'), v: s.integral_jobefresh },
    { k: t('wap_00451'), v: s.integral_down_resume },
    { k: t('wap_user_00216'), v: s.integral_interview },
    { k: t('wap_com_00238'), v: s.integral_job_top },
    { k: t('member_com_00613'), v: s.com_urgent },
    { k: t('wap_com_00237'), v: s.com_recjob },
    { k: t('wap_com_00239'), v: s.job_auto },
  ]
})

useSeoMeta({ title: priceName.value })
</script>

<template>
  <MemberPanel :title="priceName">
    <div v-for="row in rows" :key="row.k" class="site-pc paylist_list">
      <span class="paylist_span paylist_dh">{{ row.k }}</span>
      <span class="paylist_span paylist_money">{{ row.v || 0 }} {{ priceName }}</span>
    </div>
    <div class="site-h5">
      <div v-for="row in rows" :key="'h5-' + row.k" class="com_cardlist">
        <div class="com_cardlist_tit">{{ row.k }}</div>
        <div class="com_cardlist_p">{{ row.v || 0 }} {{ priceName }}</div>
      </div>
    </div>
  </MemberPanel>
</template>
