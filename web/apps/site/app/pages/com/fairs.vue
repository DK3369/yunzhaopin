<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-zph', () => api.post('/v1/mcenter/zph/my-reservation', {}))
const list = computed(() => data.value?.list || [])
useSeoMeta({ title: t('wap_00558') })
</script>

<template>
  <MemberPanel
    :title="$t('wap_00558')"
    :error="error && !isUnauthErr(error) ? error : undefined"
    :empty="!error && !list.length"
    empty-to="/fairs"
    :empty-action="$t('member_com_00670')"
  >
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('common_01153') }}</p>
    <table v-if="list.length" class="com_table site-pc">
      <tr>
        <th>{{ $t('member_com_00293') }}</th>
        <th>{{ $t('member_user_00106') }}</th>
        <th>{{ $t('member_user_00048') }}</th>
      </tr>
      <tr v-for="row in list" :key="row.id">
        <td>
          <NuxtLink v-if="row.zid" :to="`/fairs/${row.zid}?tab=reserve`" class="cblue">{{ row.title || row.name || row.zid }}</NuxtLink>
          <span v-else>{{ row.title || row.name || row.id }}</span>
        </td>
        <td>{{ row.start_at_n || row.datetime_n }}</td>
        <td>
          <NuxtLink v-if="row.zid" :to="`/fairs/${row.zid}?tab=reserve`" class="com_bth cblue">{{ $t('ui.detail') }}</NuxtLink>
        </td>
      </tr>
    </table>
    <div class="site-h5">
      <div v-for="row in list" :key="'h5-' + row.id" class="com_cardlist" @click="row.zid ? navigateTo(`/fairs/${row.zid}?tab=reserve`) : undefined">
        <div class="com_cardlist_tit">{{ row.title || row.name || row.zid || row.id }}</div>
        <div class="com_cardlist_p">
          <span class="com_cardlist_p_name">{{ $t('member_user_00106') }}</span>
          {{ row.start_at_n || row.datetime_n }}
        </div>
      </div>
    </div>
  </MemberPanel>
</template>
