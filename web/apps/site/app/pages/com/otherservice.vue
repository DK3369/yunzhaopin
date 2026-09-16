<script setup lang="ts">
import { isMemberModuleOn } from '~/utils/site'

const { t } = useI18n()
const { settings } = useSiteChrome()
useSeoMeta({ title: t('wap_user_00196') })
const items = computed(() =>
  [
    { to: '/com/parts', label: t('wap_user_00271'), icon: '/legacy/h5/images/jobhunter_top.png' },
    { to: '/com/fairs', label: t('member_com_00293'), icon: '/legacy/h5/images/resume_index.png' },
    { to: '/com/specials', label: t('wap_com_00310'), icon: '/legacy/h5/images/job_training.png' },
    { to: '/questions', label: t('wap_01141'), icon: '/legacy/h5/images/fk.png' },
    { to: '/com/jobs', label: t('wap_com_00106'), icon: '/legacy/h5/images/yun_cy_icon6.png' },
    { to: '/com/jobs/new', label: t('wap_00322'), icon: '/legacy/h5/images/yun_cy_icon6.png' },
    { to: '/com/pay', label: t('common_01946'), icon: '/legacy/h5/images/caiwuegl.png' },
    { to: '/com/cert', label: t('wap_com_00075'), icon: '/legacy/h5/images/sz.png' },
    { to: '/com/map', label: t('ui.map_addr'), icon: '/legacy/h5/images/yun_cy_icon3.png' },
    { to: '/com/job-messages', label: t('member_user_00115'), icon: '/legacy/h5/images/fk.png' },
    { to: '/com/messages', label: t('common.message'), icon: '/legacy/h5/images/fk.png' },
    { to: '/com/news', label: t('ui.com_news'), icon: '/legacy/h5/images/job_training.png' },
    { to: '/com/products', label: t('ui.com_products'), icon: '/legacy/h5/images/job_training.png' },
    { to: '/com/banners', label: t('ui.com_banner'), icon: '/legacy/h5/images/job_training.png' },
    { to: '/com/tpls', label: t('ui.com_tpl'), icon: '/legacy/h5/images/resume_index.png' },
    { to: '/com/finder', label: t('member_com_00086'), icon: '/legacy/h5/images/userpp.png' },
    { to: '/com/stats', label: t('admin_tool_00224'), icon: '/legacy/h5/images/yun_cy_icon3.png' },
    { to: '/com/report', label: t('member_com_00148'), icon: '/legacy/h5/images/fk.png' },
    { to: '/advice', label: t('wap_user_00203'), icon: '/legacy/h5/images/fk.png' },
  ].filter((item) => isMemberModuleOn(settings.value, item.to)),
)
</script>

<template>
  <div>
    <div class="site-pc">
      <MemberSetList kind="com" :title="$t('wap_user_00196')" :items="items" />
    </div>
    <div class="site-h5 position_management_body">
      <div class="management_body_card" style="margin-top: 0">
        <ul>
          <li v-for="item in items" :key="item.to" @click="navigateTo(item.to)">
            <div class="management_body_card_icon">
              <img :src="item.icon" alt="" />
            </div>
            <div class="management_body_card_name">{{ item.label }}</div>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>
