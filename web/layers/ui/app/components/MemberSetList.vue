<template>
  <div>
    <div v-if="kind === 'com'" class="site-pc">
      <div class="newmember_tit">
        <ul>
          <li class="newmember_titcur">
            <a href="javascript:;">{{ title }}</a>
          </li>
        </ul>
      </div>
      <div class="com_body">
        <NuxtLink v-for="item in items" :key="item.to" :to="item.to" class="com_set_list">
          <div class="com_set_listname">{{ item.label }}</div>
          <div v-if="item.hint" class="com_set_listp">{{ item.hint }}</div>
        </NuxtLink>
      </div>
    </div>
    <div v-else class="site-pc">
      <div class="yun_m_rightbox mt20 re">
        <div class="member_right_index_h1 fltL">
          <span class="member_right_h1_span fltL">{{ title }}</span>
          <i class="member_right_h1_icon user_bg" />
        </div>
        <div class="account_settings">
          <div v-for="item in items" :key="item.to" class="account_settings_list">
            <div class="account_settings_list_left">
              <i class="account_settings_list_left_icon" :class="item.icon || 'account_settings_list_left_icon_user'" />
              <div class="account_settings_tit">{{ item.label }}</div>
              <div v-if="item.hint" class="account_settings_tip">{{ item.hint }}</div>
            </div>
            <NuxtLink :to="item.to" class="account_settings_bth_hv">{{ $t('wap_js_00073') }}</NuxtLink>
          </div>
        </div>
      </div>
    </div>
    <div class="site-h5">
      <div v-if="kind === 'com'" class="issue_post_body_new">
        <div class="issue_post_body_card member-set-list" style="padding-top: 0; margin-top: 0">
          <div class="com_set_list member-set-lang" @click="langOpen = true">
            <div class="com_set_listname">{{ $t('ui.language') }}</div>
            <div class="com_set_listp">{{ langLabel }}</div>
            <div class="com_set_listicon">
              <img src="/legacy/h5/images/issue_add.png" alt="" width="100%" height="100%" />
            </div>
          </div>
          <NuxtLink v-for="item in items" :key="item.to" :to="item.to" class="com_set_list">
            <div class="com_set_listname">{{ item.label }}</div>
            <div v-if="item.hint" class="com_set_listp">{{ item.hint }}</div>
            <div class="com_set_listicon">
              <img src="/legacy/h5/images/issue_add.png" alt="" width="100%" height="100%" />
            </div>
          </NuxtLink>
        </div>
        <div v-if="logoutable" class="logout_btn" @click="logout">{{ $t('wap_user_00342') }}</div>
      </div>
      <div v-else class="issue_post_body">
        <div class="issue_post_body_card member-set-list">
          <div class="post_body_card_job member-set-lang" @click="langOpen = true">
            <div class="body_card_job_box">
              <div class="card_job_box_post">{{ $t('ui.language') }}</div>
              <div class="card_job_box_name">{{ langLabel }}</div>
            </div>
            <div class="body_card_job_icon">
              <img src="/legacy/h5/images/issue_add.png" alt="" width="100%" />
            </div>
          </div>
          <NuxtLink v-for="item in items" :key="item.to" :to="item.to" class="post_body_card_job">
            <div class="body_card_job_box">
              <div class="card_job_box_post">{{ item.label }}</div>
              <div v-if="item.hint" class="card_job_box_name">{{ item.hint }}</div>
            </div>
            <div class="body_card_job_icon">
              <img src="/legacy/h5/images/issue_add.png" alt="" width="100%" />
            </div>
          </NuxtLink>
        </div>
        <div v-if="logoutable" class="logout_btn" @click="logout">{{ $t('wap_user_00342') }}</div>
      </div>
    </div>
    <Teleport to="body">
      <div v-if="langOpen" id="Common_language" style="display: block" @click="langOpen = false">
        <div id="Common_language_box" @click.stop>
          <div class="Common_language_box_header">
            <div class="Common_language_box_header_left" />
            <div class="Common_language_box_header_center">{{ $t('ui.language') }}</div>
            <div class="Common_language_box_header_right" :title="$t('common.close')" @click="langOpen = false">×</div>
          </div>
          <div
            v-for="opt in langOpts"
            :key="opt.code"
            class="Common_language_box_header_textandfall"
            @click="pickLang(opt.code)"
          >
            <div class="Common_language_box_header_text">{{ $t(opt.label) }}</div>
            <div v-if="locale === opt.code" class="Common_language_box_header_text_fall">✓</div>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { persistWebLocale } from '../../../base/app/utils/locale'

withDefaults(
  defineProps<{
    title: string
    items: Array<{ to: string; label: string; hint?: string; icon?: string }>
    logoutable?: boolean
    kind?: 'user' | 'com'
  }>(),
  { kind: 'user' },
)

const { locale, t } = useI18n()
const scope = useLocaleScope()
const langOpen = ref(false)
const langOpts = [
  { code: 'zh' as const, label: 'ui.lang_zh' },
  { code: 'en' as const, label: 'ui.lang_en' },
]
const langLabel = computed(() => (locale.value === 'zh' ? t('ui.lang_zh') : t('ui.lang_en')))

function pickLang(code: 'zh' | 'en') {
  if (locale.value === code) {
    langOpen.value = false
    return
  }
  persistWebLocale(code, scope.key)
  if (import.meta.client) location.reload()
}

async function logout() {
  await $fetch('/api/auth/logout', { method: 'POST' })
  await refreshNuxtData('auth-me')
  await navigateTo('/login')
}
</script>
