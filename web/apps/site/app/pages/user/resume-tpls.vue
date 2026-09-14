<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('resume-tpls', () => api.post('/v1/mcenter/resume-tpls', {}))
const msg = ref('')
const list = computed(() => (Array.isArray(data.value) ? data.value : data.value?.list || []) as Array<{
  id: number
  name?: string
  price?: number
  price_yuan?: number
}>)
async function apply(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume-tpls/apply', { id })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function buy(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume-tpls/buy', { id })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('wap_00328') })
</script>

<template>
  <MemberPanel :title="$t('wap_00328')" :error="error" :empty="!error && !list.length">
    <div v-for="row in list" :key="row.id" class="jobnotice_list site-pc">
      <div class="user_new_job">
        <span class="user_new_jobname">{{ row.name }}</span>
        <div v-if="row.price_yuan || row.price" class="user_new_jobxz">{{ row.price_yuan || row.price }}</div>
      </div>
      <div class="user_new_cz">
        <a href="javascript:;" class="user_new_yqh_a" @click="apply(row.id)">{{ $t('common.confirm') }}</a>
        <a v-if="Number(row.price || row.price_yuan || 0) > 0" href="javascript:;" class="user_new_bth" @click="buy(row.id)">{{ $t('common_01946') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in list"
          :key="'h5-' + row.id"
          :title="row.name || ''"
          :pay="String(row.price_yuan || row.price || '')"
        >
          <p>
            <a href="javascript:;" @click="apply(row.id)">{{ $t('common.confirm') }}</a>
          </p>
        </MemberPostedCard>
      </div>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
