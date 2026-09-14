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
        <NuxtLink v-if="row.to" :to="row.to">{{ row.name }}</NuxtLink>
        <span v-else>{{ row.name }}</span>
      </td>
      <td v-if="showJob">{{ row.job }}</td>
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
        :job="row.job"
        :time="row.time"
        :invited="row.invited"
        :downloaded="row.downloaded"
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
}

withDefaults(
  defineProps<{
    rows: MemberHrResumeRow[]
    showJob?: boolean
  }>(),
  { showJob: false },
)
</script>
