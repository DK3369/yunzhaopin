<template>
  <div>
    <div class="site-pc">
      <div class="user_new_tit">
        <span class="user_new_tit_n">{{ title }}</span>
      </div>
      <div class="yun_m_rightbox">
        <div class="resume_box_list">
          <div v-for="item in items" :key="item.to" class="jobnotice_list">
            <div class="user_new_job">
              <NuxtLink :to="item.to" class="user_new_jobname">{{ item.label }}</NuxtLink>
              <div v-if="item.hint" class="user_new_comname">{{ item.hint }}</div>
            </div>
            <div class="user_new_cz">
              <NuxtLink :to="item.to" class="user_new_yqh_sc">{{ $t('common.more') }}</NuxtLink>
            </div>
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
    items: Array<{ to: string; label: string; hint?: string }>
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
