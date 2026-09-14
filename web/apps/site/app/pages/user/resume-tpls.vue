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
  pic?: string
  using?: boolean
  bought?: boolean
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
    <div class="site-pc resume_template_box">
      <dl v-for="row in list" :key="row.id" class="resume_template">
        <dt>
          <img v-if="row.pic" :alt="row.name" :src="row.pic" width="238" height="315" />
        </dt>
        <dd>
          <div class="resume_template_pd">
            <div class="resume_template_name">{{ row.name }}</div>
            <div class="resume_template_p">
              <span class="resume_template_jg">{{ row.price_yuan || row.price || $t('member_user_00572') }}</span>
            </div>
          </div>
          <div class="resume_template_cz">
            <span v-if="row.using" class="resume_template_bth_sy">{{ $t('member_user_00573') }}</span>
            <a v-else href="javascript:;" class="resume_template_bth" @click="apply(row.id)">{{ $t('member_user_00284') }}</a>
            <a
              v-if="Number(row.price || row.price_yuan || 0) > 0 && !row.bought"
              href="javascript:;"
              class="resume_template_ylbth"
              @click="buy(row.id)"
            >{{ $t('member_user_00285') }}</a>
          </div>
        </dd>
      </dl>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <div v-for="row in list" :key="'h5-' + row.id" class="issue_post_body_card">
          <div class="Posted_card_top">
            <div class="Posted_card_name">{{ row.name }}</div>
            <div class="Posted_card_pay">{{ row.price_yuan || row.price || $t('member_user_00572') }}</div>
          </div>
          <img v-if="row.pic" :alt="row.name" :src="row.pic" style="width: 100%; display: block" />
          <div class="Posted_card_bom">
            <span v-if="row.using" class="resume_template_bth_sy">{{ $t('member_user_00573') }}</span>
            <a v-else href="javascript:;" class="resume_template_bth" @click="apply(row.id)">{{ $t('member_user_00284') }}</a>
            <a
              v-if="Number(row.price || row.price_yuan || 0) > 0 && !row.bought"
              href="javascript:;"
              class="resume_template_ylbth"
              @click="buy(row.id)"
            >{{ $t('member_user_00285') }}</a>
          </div>
        </div>
      </div>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
