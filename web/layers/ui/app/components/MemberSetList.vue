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
      <div class="member_right_index_h1 fltL">
        <span class="member_right_h1_span fltL">{{ title }}</span>
        <i class="member_right_h1_icon user_bg" />
      </div>
      <div class="yun_m_rightbox">
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
        <div class="issue_post_body_card" style="padding-top: 0; margin-top: 0">
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
        <div class="issue_post_body_card">
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
  </div>
</template>

<script setup lang="ts">
withDefaults(
  defineProps<{
    title: string
    items: Array<{ to: string; label: string; hint?: string; icon?: string }>
    logoutable?: boolean
    kind?: 'user' | 'com'
  }>(),
  { kind: 'user' },
)

async function logout() {
  await $fetch('/api/auth/logout', { method: 'POST' })
  await refreshNuxtData('auth-me')
  await navigateTo('/login')
}
</script>
