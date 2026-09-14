<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-follows', () =>
  api.post('/v1/mcenter/follows/list', { page: 1, page_size: 20 }),
)
async function toggle(row: { target_uid?: number; uid?: number }) {
  await api.post('/v1/mcenter/follows', { target_kind: 1, target_uid: row.target_uid || row.uid })
  refresh()
}
useSeoMeta({ title: t('wap_01142') })
</script>

<template>
  <MemberPanel :title="$t('wap_01142')" :error="error" :empty="!error && !(data?.list || []).length">
    <div v-if="(data?.list || []).length" class="attention_enterprises_tit site-pc">
      <div class="attention_enterprises_span attention_enterprises_name">{{ $t('wap_com_00157') }}</div>
      <div class="attention_enterprises_span attention_enterprises_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in data?.list || []" :key="row.target_uid || row.uid" class="attention_enterprises_list site-pc">
      <div class="attention_enterprises_span attention_enterprises_name">{{ row.name || row.target_uid }}</div>
      <div class="attention_enterprises_span attention_enterprises_cz">
        <a href="javascript:;" class="cblue" @click="toggle(row)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <MemberSxNewsCard
        v-for="row in data?.list || []"
        :key="'h5-' + (row.target_uid || row.uid)"
        :title="String(row.name || row.target_uid || '')"
      />
    </div>
  </MemberPanel>
</template>
