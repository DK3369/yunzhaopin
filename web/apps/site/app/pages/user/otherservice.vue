<script setup lang="ts">
import { isMemberModuleOn } from '~/utils/site'

const { t } = useI18n()
const { settings } = useSiteChrome()
useSeoMeta({ title: t('wap_user_00196') })
const items = computed(() =>
  [
    { to: '/user/parts', label: t('wap_user_00220'), icon: '/legacy/h5/images/jobhunter_top.png' },
    { to: '/user/member-right', label: t('wap_com_00097'), icon: '/legacy/h5/images/financial_management.png' },
    { to: '/user/pay', label: t('common_01946'), icon: '/legacy/h5/images/financial_management.png' },
    { to: '/user/integral', label: t('wap_user_00008'), icon: '/legacy/h5/images/job_training.png' },
    { to: '/user/eval-logs', label: t('wap_00194'), icon: '/legacy/h5/images/job_training.png' },
    { to: '/user/looks', label: t('wap_user_00275'), icon: '/legacy/h5/images/jobhunter_preview.png' },
    { to: '/user/follows', label: t('wap_01142'), icon: '/legacy/h5/images/userpp.png' },
    { to: '/user/recommend', label: t('wap_user_00211'), icon: '/legacy/h5/images/userpp.png' },
    { to: '/user/searches', label: t('member_user_00108'), icon: '/legacy/h5/images/job_training.png' },
    { to: '/user/outbox', label: t('member_user_00188'), icon: '/legacy/h5/images/resume_index.png' },
    { to: '/user/resume-tpls', label: t('member_user_00189'), icon: '/legacy/h5/images/resume_index.png' },
    { to: '/user/expects', label: t('wap_00460'), icon: '/legacy/h5/images/resume_index.png' },
    { to: '/user/consults', label: t('member_user_00115'), icon: '/legacy/h5/images/fk.png' },
    { to: '/user/messages', label: t('common.message'), icon: '/legacy/h5/images/fk.png' },
    { to: '/user/inbox', label: t('admin_user_00263'), icon: '/legacy/h5/images/resume_index.png' },
    { to: '/user/invite', label: t('wap_user_00253'), icon: '/legacy/h5/images/job_training.png' },
    { to: '/user/reports', label: t('ui.my_reports'), icon: '/legacy/h5/images/fk.png' },
    { to: '/user/blacklist', label: t('member_user_00044'), icon: '/legacy/h5/images/ys.png' },
    { to: '/questions', label: t('wap_00331'), icon: '/legacy/h5/images/fk.png' },
    { to: '/advice', label: t('wap_user_00203'), icon: '/legacy/h5/images/fk.png' },
  ].filter((item) => isMemberModuleOn(settings.value, item.to)),
)
</script>

<template>
  <div>
    <div class="site-pc">
      <MemberSetList :title="$t('wap_user_00196')" :items="items" />
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
