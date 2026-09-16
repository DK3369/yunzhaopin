<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-tpls', () =>
  api.post<Array<Record<string, unknown>>>('/v1/mcenter/company-tpls', {}),
)
const msg = ref('')
async function apply(row: { id: number }) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company-tpls/apply', { id: row.id })
    msg.value = t('ui.apply_tpl')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('ui.com_tpl') })
</script>

<template>
  <MemberPanel :title="$t('ui.com_tpl')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <p v-if="msg">{{ msg }}</p>
    <div class="resume_template_box site-pc">
      <dl v-for="row in data || []" :key="row.id" class="resume_template">
        <dd>
          <div class="resume_template_pd">
            <div class="resume_template_name">{{ row.name }}</div>
            <div class="resume_template_p">kind {{ row.kind }}</div>
          </div>
          <div class="resume_template_cz">
            <a href="javascript:;" class="resume_template_bth" @click="apply(row)">{{ $t('ui.apply_tpl') }}</a>
          </div>
        </dd>
      </dl>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <div v-for="row in data || []" :key="'h5-' + row.id" class="issue_post_body_card">
          <div class="Posted_card_top">
            <div class="Posted_card_name">{{ row.name }}</div>
            <div class="Posted_card_pay">kind {{ row.kind }}</div>
          </div>
          <div class="Posted_card_bom">
            <a href="javascript:;" class="resume_template_bth" @click="apply(row)">{{ $t('ui.apply_tpl') }}</a>
          </div>
        </div>
      </div>
    </div>
    <p><NuxtLink to="/com">{{ $t('ui.back_com') }}</NuxtLink></p>
  </MemberPanel>
</template>
