<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type Row = {
  id: number
  link_man: string
  link_moblie: string
  link_phone?: string | null
  email?: string | null
  link_address?: string | null
  province_id: number
  city_id: number
  three_city_id: number
  x?: string | null
  y?: string | null
}

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-addresses', () =>
  api.post<{ list: Row[]; total: number }>('/v1/mcenter/company-addresses', {
    page: 1,
    page_size: 20,
  }),
)

const blank = () => ({
  id: 0,
  link_man: '',
  link_moblie: '',
  link_phone: '',
  email: '',
  link_address: '',
  province_id: 0,
  city_id: 0,
  three_city_id: 0,
  x: '',
  y: '',
})
const form = reactive(blank())
const editing = computed(() => form.id > 0)
const msg = ref('')
const list = computed<Row[]>(() => data.value?.list || [])

function reset() {
  Object.assign(form, blank())
}

function fail(e: unknown) {
  return e instanceof Error ? e.message : t('ui.failed')
}

async function save() {
  msg.value = ''
  try {
    const path = editing.value
      ? '/v1/mcenter/company-addresses/update'
      : '/v1/mcenter/company-addresses/create'
    await api.post(path, { ...form })
    reset()
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}

// 地区字段列表里不可见，编辑时必须原样带回，否则 update 会把已有省市清零
function edit(row: Row) {
  form.id = row.id
  form.link_man = row.link_man || ''
  form.link_moblie = row.link_moblie || ''
  form.link_phone = row.link_phone || ''
  form.email = row.email || ''
  form.link_address = row.link_address || ''
  form.province_id = Number(row.province_id || 0)
  form.city_id = Number(row.city_id || 0)
  form.three_city_id = Number(row.three_city_id || 0)
  form.x = row.x || ''
  form.y = row.y || ''
}

async function remove(row: Row) {
  if (!window.confirm(t('member_com_00083'))) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company-addresses/delete', { ids: [row.id] })
    if (form.id === row.id) reset()
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}

useSeoMeta({ title: t('ui.map_addr') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.map_addr') }}</h1>
    <p v-if="error" class="muted">
      {{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}
    </p>
    <template v-else>
      <form class="form" @submit.prevent="save">
        <input v-model="form.link_man" required :placeholder="$t('wap_01431')" />
        <input v-model="form.link_moblie" required :placeholder="$t('common.phone')" />
        <input v-model="form.link_phone" :placeholder="$t('wap_com_00014')" />
        <input v-model="form.email" :placeholder="$t('member_user_00282')" />
        <input v-model="form.link_address" :placeholder="$t('ui.map_addr')" />
        <input v-model="form.x" :placeholder="$t('ui.lng')" />
        <input v-model="form.y" :placeholder="$t('ui.lat')" />
        <div class="row">
          <button type="submit">{{ editing ? $t('common.save') : $t('ui.add') }}</button>
          <button v-if="editing" type="button" @click="reset">{{ $t('common.cancel') }}</button>
        </div>
      </form>
      <p v-if="msg">{{ msg }}</p>
      <p v-if="!list.length" class="muted">{{ $t('ui.no_addr') }}</p>
      <div class="stack">
        <article v-for="row in list" :key="row.id" class="job-card">
          <h3>{{ row.link_man }} · {{ row.link_address }}</h3>
          <p class="muted">{{ row.link_moblie }}</p>
          <p class="muted">x {{ row.x }} y {{ row.y }}</p>
          <div class="row">
            <button type="button" @click="edit(row)">{{ $t('common.edit') }}</button>
            <button type="button" @click="remove(row)">{{ $t('common.delete') }}</button>
          </div>
        </article>
      </div>
    </template>
  </section>
</template>

<style scoped>
.row {
  display: flex;
  gap: 0.5rem;
  align-items: center;
  flex-wrap: wrap;
}
</style>
