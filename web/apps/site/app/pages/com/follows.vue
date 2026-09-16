<script setup lang="ts">
type FavItem = { kind?: number; target_id?: number; detail?: Record<string, unknown> }
type FavRow = { target_id: number; name: string }

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-follows', () =>
  api.post<{ list?: FavItem[] }>('/v1/mcenter/favorites/list', { kind: 3, page: 1, page_size: 20 }),
)
const rows = computed((): FavRow[] =>
  (data.value?.list || []).map((row) => {
    const d = row.detail && typeof row.detail === 'object' ? row.detail : {}
    const id = Number(row.target_id || d.uid || 0)
    return { target_id: id, name: String(d.name || d.username || d.nickname || id) }
  }),
)
async function toggle(row: FavRow) {
  await api.post('/v1/mcenter/favorites', { kind: 3, target_id: row.target_id })
  refresh()
}
useSeoMeta({ title: t('wap_01142') })
</script>

<template>
  <MemberPanel :title="$t('wap_01142')" :error="error" :empty="!error && !rows.length">
    <div v-if="rows.length" class="attention_enterprises_tit site-pc">
      <div class="attention_enterprises_span attention_enterprises_name">{{ $t('wap_com_00157') }}</div>
      <div class="attention_enterprises_span attention_enterprises_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in rows" :key="row.target_id" class="attention_enterprises_list site-pc">
      <div class="attention_enterprises_span attention_enterprises_name">{{ row.name || row.target_id }}</div>
      <div class="attention_enterprises_span attention_enterprises_cz">
        <a href="javascript:;" class="cblue" @click="toggle(row)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5">
      <div v-for="row in rows" :key="'h5-' + row.target_id" class="com_cardlist">
        <div class="com_cardlist_tit">{{ row.name || row.target_id }}</div>
        <div class="com_card_cz">
          <span class="com_card_delete" @click="toggle(row)" />
        </div>
      </div>
    </div>
  </MemberPanel>
</template>
