<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error, refresh } = await useAsyncData(
  () => `msgs-${page.value}`,
  () => api.post('/v1/mcenter/messages', { page: page.value, page_size: pageSize }),
)
const { data: dash } = await useAsyncData(
  'user-dash',
  () =>
    api
      .post<{ wkyqnum?: number; commsgnum?: number; sxnum?: number }>('/v1/mcenter/dashboard/full', {})
      .catch(() => null),
  reuseAsyncCache(),
)
const picked = ref<number[]>([])
const openId = ref(0)
const allPicked = computed({
  get: () => {
    const list = data.value?.list || []
    return list.length > 0 && picked.value.length === list.length
  },
  set: (v: boolean) => {
    picked.value = v ? (data.value?.list || []).map((r: { id: number }) => r.id) : []
  },
})
async function read(id: number) {
  await api.post('/v1/mcenter/messages/read', { id })
  refresh()
}
async function remove(id: number) {
  await api.post('/v1/mcenter/messages/delete', { id })
  refresh()
}
async function readAll() {
  await api.post('/v1/mcenter/messages/read-all', {})
  refresh()
}
async function removePicked() {
  for (const id of picked.value) await api.post('/v1/mcenter/messages/delete', { id })
  picked.value = []
  refresh()
}
async function readPicked() {
  for (const id of picked.value) await api.post('/v1/mcenter/messages/read', { id })
  picked.value = []
  refresh()
}
useSeoMeta({ title: t('common.message') })
const total = computed(() => inferTotal(data.value))
</script>

<template>
  <MemberPanel :title="$t('common.message')" :error="error" :empty="!error && !(data?.list || []).length" :empty-text="$t('wap_00129')">
    <div class="site-pc job_list_tit">
      <ul>
        <li class="job_list_tit_cur">
          <a href="javascript:;">{{ $t('common.message') }}</a>
        </li>
        <li>
          <NuxtLink to="/user/interviews">{{ $t('wap_user_00216') }}<span v-if="dash?.wkyqnum">({{ dash.wkyqnum }})</span></NuxtLink>
        </li>
        <li>
          <NuxtLink to="/user/consults">{{ $t('wap_user_00364') }}<span v-if="dash?.commsgnum">({{ dash.commsgnum }})</span></NuxtLink>
        </li>
      </ul>
    </div>
    <div class="site-h5">
      <div class="chatnewcardbg">
        <div class="chatnewcard">
          <ul>
            <li @click="navigateTo('/user/interviews')">
              <div class="card_logo">
                <img src="/legacy/h5/images/resume.png" alt="" width="100%" height="100%" />
                <div v-if="dash?.wkyqnum" class="card_logo_circle">{{ dash.wkyqnum }}</div>
              </div>
              <i class="card_word">{{ $t('wap_user_00216') }}</i>
            </li>
            <li @click="navigateTo('/user/applications')">
              <div class="card_logo">
                <img src="/legacy/h5/images/copy.png" alt="" width="100%" height="100%" />
              </div>
              <i class="card_word">{{ $t('wap_01133') }}</i>
            </li>
            <li @click="navigateTo('/user/consults')">
              <div class="card_logo">
                <img src="/legacy/h5/images/genius_consult.png" alt="" width="100%" height="100%" />
                <div v-if="dash?.commsgnum" class="card_logo_circle">{{ dash.commsgnum }}</div>
              </div>
              <i class="card_word">{{ $t('wap_user_00364') }}</i>
            </li>
            <li @click="navigateTo('/user/chat')">
              <div class="card_logo">
                <img src="/legacy/h5/images/sixin.png" alt="" width="100%" height="100%" />
                <div v-if="dash?.sxnum" class="card_logo_circle">{{ dash.sxnum }}</div>
              </div>
              <i class="card_word">{{ $t('wap_user_00363') }}</i>
            </li>
          </ul>
        </div>
      </div>
      <div class="m_cardbox">
        <MemberSxNewsCard
          v-for="row in data?.list || []"
          :key="'h5-' + row.id"
          :kicker="$t('wap_user_00361')"
          :parts="row.parts"
          :title="String(row.body || row.content || row.title || row.id)"
          :time="row.created_at_n || row.datetime_n"
          :on-delete="() => remove(row.id)"
          @click="read(row.id)"
        />
      </div>
    </div>
    <p class="user_czbth site-pc">
      <a href="javascript:;" class="user_new_yqh_a" @click="readAll">{{ $t('common.confirm') }}</a>
    </p>
    <div v-if="(data?.list || []).length" class="sysynews_tit site-pc">
      <div class="sysynews_span">&nbsp;</div>
      <div class="sysynews_span sysynews_name">{{ $t('common.message') }}</div>
      <div class="sysynews_span sysynews_time">{{ $t('member_user_00104') }}</div>
      <div class="sysynews_span sysynews_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in data?.list || []" :key="row.id" class="sysynews_list site-pc">
      <div class="sysynews_span">
        <input type="checkbox" :checked="picked.includes(row.id)" @change="picked = picked.includes(row.id) ? picked.filter((x) => x !== row.id) : [...picked, row.id]" />
      </div>
      <div class="sysynews_span sysynews_name" :style="row.remind_status === 0 ? 'font-weight:bold' : ''">
        <template v-for="(p, i) in (row.parts || [])" :key="'pc-' + row.id + '-' + i"><NuxtLink v-if="p.to" :to="p.to" class="sys_a">{{ p.n }}</NuxtLink><span v-else>{{ p.n }}</span></template>
        <template v-if="!(row.parts || []).length">{{ row.body || row.content || row.title || row.id }}</template>
        <span v-if="row.remind_status === 0" class="sysynews_span_nolook">{{ $t('wap_user_00260') }}</span>
      </div>
      <div class="sysynews_span sysynews_time">{{ row.created_at_n || row.datetime_n }}</div>
      <div class="sysynews_span sysynews_cz">
        <a href="javascript:;" class="cblue" @click="read(row.id); openId = row.id">{{ $t('wap_00071') }}</a>
        <span class="jobnotice_cz_line">|</span>
        <a href="javascript:;" class="List_dete cblue" @click="remove(row.id)">{{ $t('common.delete') }}</a>
      </div>
      <div v-if="openId === row.id" class="sys_tm">
        <p><i>{{ $t('member_user_00104') }}：</i><span>{{ row.created_at_n || row.datetime_n }}</span></p>
        <p>
          <i>{{ $t('common.message') }}：</i>
          <span>
            <template v-for="(p, i) in (row.parts || [])" :key="'dt-' + row.id + '-' + i"><NuxtLink v-if="p.to" :to="p.to" class="sys_a">{{ p.n }}</NuxtLink><span v-else>{{ p.n }}</span></template>
            <template v-if="!(row.parts || []).length">{{ row.body || row.content || row.title }}</template>
          </span>
        </p>
        <div class="sys_bot">
          <a href="javascript:;" class="sys_bot_del" @click="remove(row.id)">{{ $t('common.delete') }}</a>
          <a href="javascript:;" class="sys_bot_qx" @click="openId = 0">{{ $t('common.cancel') }}</a>
        </div>
      </div>
    </div>
    <div v-if="(data?.list || []).length" class="checkall_toyota site-pc">
      <label><input v-model="allPicked" type="checkbox" /> {{ $t('common.all') }}</label>
      <input type="button" class="job_operation_bth" :value="$t('common.delete')" @click="removePicked" />
      <input type="button" class="job_operation_bth" :value="$t('common.confirm')" @click="readPicked" />
      <input type="button" class="job_operation_bth" :value="$t('wap_user_00260')" @click="readAll" />
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
  </MemberPanel>
</template>
