<template>
  <table v-if="rows.length" class="com_table site-pc">
    <tr>
      <th v-if="selectable" width="25">
        <label><input :checked="allPicked" type="checkbox" class="com_job_list_check" @change="toggleAll(($event.target as HTMLInputElement).checked)" /></label>
      </th>
      <th>{{ $t('wap_00456') }}</th>
      <th v-if="showJob">{{ $t('wap_com_00288') }}</th>
      <th>{{ $t('member_user_00106') }}</th>
      <th>{{ $t('member_user_00048') }}</th>
    </tr>
    <tr v-for="row in rows" :key="row.key">
      <td v-if="selectable" align="center">
        <input type="checkbox" class="com_job_list_check" :checked="isPicked(row.key)" @change="toggle(row.key)" />
      </td>
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
        <label v-if="selectable" class="com_job_list_check">
          <input type="checkbox" :checked="isPicked(row.key)" @click.stop @change="toggle(row.key)" />
        </label>
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

const props = withDefaults(
  defineProps<{
    rows: MemberHrResumeRow[]
    showJob?: boolean
    selectable?: boolean
    picked?: number[]
  }>(),
  { showJob: false, selectable: false, picked: () => [] },
)
const emit = defineEmits<{ 'update:picked': [number[]] }>()

function idOf(key: string | number) {
  return Number(key)
}
function isPicked(key: string | number) {
  return props.picked.includes(idOf(key))
}
const allPicked = computed(
  () => props.rows.length > 0 && props.rows.every((r) => props.picked.includes(idOf(r.key))),
)
function toggle(key: string | number) {
  const id = idOf(key)
  emit(
    'update:picked',
    props.picked.includes(id) ? props.picked.filter((x) => x !== id) : [...props.picked, id],
  )
}
function toggleAll(on: boolean) {
  emit('update:picked', on ? props.rows.map((r) => idOf(r.key)) : [])
}
</script>
