<template>
  <table v-if="rows.length" class="com_table site-pc">
    <tr>
      <th>{{ $t('wap_00456') }}</th>
      <th v-if="showJob">{{ $t('wap_com_00288') }}</th>
      <th>{{ $t('member_user_00106') }}</th>
      <th>{{ $t('member_user_00048') }}</th>
    </tr>
    <tr v-for="row in rows" :key="row.key">
      <td>
        <div class="newcom_user_info">
          <div v-if="row.photo" class="newcom_user_pic">
            <img :src="row.photo" alt="" />
          </div>
          <div>
            <NuxtLink v-if="row.to" :to="row.to" class="newcom_user_name">{{ row.name }}</NuxtLink>
            <a v-else href="javascript:;" class="newcom_user_name">{{ row.name }}</a>
            <span v-if="row.stateText" class="newcom_user_zt">{{ row.stateText }}</span>
            <span v-if="row.invited" class="hr_yyy">{{ $t('wap_user_00216') }}</span>
            <span v-if="row.downloaded" class="hr_yxz">{{ $t('wap_00451') }}</span>
            <div v-if="row.info && row.info.length" class="newcom_user_infop">{{ row.info.join(' · ') }}</div>
            <div v-if="row.salary">{{ $t('wap_00925') }}：{{ row.salary }}</div>
          </div>
        </div>
      </td>
      <td v-if="showJob">
        <a class="newcom_user_td">{{ row.job }}</a>
      </td>
      <td>{{ row.time }}</td>
      <td>
        <slot name="pc-acts" :row="row" />
      </td>
    </tr>
  </table>
  <div class="site-h5 resume_management_body_card">
    <div class="management_body_card_content">
      <MemberHrUserCard
        v-for="row in rows"
        :key="'h5-' + row.key"
        :name="row.name"
        :photo="row.photo"
        :job="row.job"
        :time="row.time"
        :invited="row.invited"
        :downloaded="row.downloaded"
        :state-text="row.stateText"
        :info="row.info || []"
        @open="row.to ? navigateTo(row.to) : undefined"
      >
        <slot name="h5-acts" :row="row" />
      </MemberHrUserCard>
    </div>
  </div>
</template>

<script setup lang="ts">
export type MemberHrResumeRow = {
  key: string | number
  name: string
  job?: string
  time?: string
  to?: string
  invited?: boolean
  downloaded?: boolean
  info?: string[]
  photo?: string
  salary?: string
  stateText?: string
}

withDefaults(
  defineProps<{
    rows: MemberHrResumeRow[]
    showJob?: boolean
  }>(),
  { showJob: false },
)
</script>
