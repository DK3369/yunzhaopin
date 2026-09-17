<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type NavItem = {
  key: string
  label_key: string
  to: string
  sort: number
  show: boolean
  target: string
}

const api = useApi()
const { t } = useI18n()
const { data: me } = await useAuthMe()
const canEdit = computed(() => !me.value?.is_sub)
const { data, error, refresh } = await useAsyncData('com-nav-editor', () =>
  api.post<{ is_nav: number; items: NavItem[] }>('/v1/mcenter/company/nav', {}),
)
const items = ref<NavItem[]>([])
watch(
  () => data.value?.items,
  (list) => {
    items.value = (list || []).map((i) => ({ ...i }))
  },
  { immediate: true },
)
const msg = ref('')

function move(i: number, dir: -1 | 1) {
  const j = i + dir
  if (j < 0 || j >= items.value.length) return
  const copy = [...items.value]
  const tmp = copy[i]
  copy[i] = copy[j]
  copy[j] = tmp
  items.value = copy.map((it, idx) => ({ ...it, sort: idx }))
}

async function save() {
  if (!canEdit.value) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company/nav/save', {
      items: items.value.map((it, idx) => ({ ...it, sort: idx })),
    })
    msg.value = t('common.success')
    await refresh()
    await refreshNuxtData('com-nav')
    await refreshNuxtData('com-nav-editor')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

async function reset() {
  if (!canEdit.value) return
  if (!window.confirm(t('common.confirm'))) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company/nav/reset', {})
    msg.value = t('common.success')
    await refresh()
    await refreshNuxtData('com-nav')
    await refreshNuxtData('com-nav-editor')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

useSeoMeta({ title: t('member_com_00397') })
</script>

<template>
  <MemberPanel :title="$t('member_com_00397')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <p class="muted site-h5">{{ $t('ui.pc_only') }}</p>
    <template v-if="!error">
      <table class="com_table site-pc">
        <tr>
          <th>{{ $t('member_com_00021') }}</th>
          <th>{{ $t('admin_00271') }}</th>
          <th>{{ $t('member_user_00048') }}</th>
        </tr>
        <tr v-for="(it, i) in items" :key="it.key">
          <td>{{ $t(it.label_key) }}</td>
          <td>
            <label><input v-model="it.show" type="checkbox" :disabled="!canEdit"> {{ $t('admin_00271') }}</label>
            <label>
              <input v-model="it.target" type="checkbox" true-value="_blank" false-value="_self" :disabled="!canEdit">
              {{ $t('admin_01152') }}
            </label>
          </td>
          <td>
            <a href="javascript:;" class="com_bth cblue" @click="move(i, -1)">↑</a>
            <a href="javascript:;" class="com_bth cblue" @click="move(i, 1)">↓</a>
          </td>
        </tr>
      </table>
      <p v-if="canEdit" class="site-pc">
        <button type="button" class="btn_01" @click="save">{{ $t('common.submit') }}</button>
        <button type="button" class="btn_02" @click="reset">{{ $t('ui.restore_default') }}</button>
      </p>
      <p v-if="msg">{{ msg }}</p>
    </template>
  </MemberPanel>
</template>
