<script setup lang="ts">
const api = useApi()
const logoutId = ref(1)
const usertypeId = ref(1)
const userUid = ref(1)
const companyUid = ref(1)
const msg = ref('')
async function approveLogout() {
  msg.value = JSON.stringify(await api.post('/v1/admin/account-logouts/approve', { id: logoutId.value }))
}
async function rejectLogout() {
  msg.value = JSON.stringify(await api.post('/v1/admin/account-logouts/reject', { id: logoutId.value }))
}
async function approveType() {
  msg.value = JSON.stringify(await api.post('/v1/admin/usertype-changes/approve', { id: usertypeId.value }))
}
async function rejectType() {
  msg.value = JSON.stringify(await api.post('/v1/admin/usertype-changes/reject', { id: usertypeId.value }))
}
async function merge() {
  msg.value = JSON.stringify(
    await api.post('/v1/admin/account-merge', { user_uid: userUid.value, company_uid: companyUid.value }),
  )
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.accounts') }}</h1>
    <p>{{ $t('ui.accounts_hint') }}</p>
    <el-card style="margin-bottom: 16px">
      <h2>{{ $t('ui.logout_apply') }}</h2>
      <el-input-number v-model="logoutId" :min="1" />
      <el-button @click="approveLogout">{{ $t('ui.approved') }}</el-button>
      <el-button type="danger" @click="rejectLogout">{{ $t('ui.reject') }}</el-button>
    </el-card>
    <el-card style="margin-bottom: 16px">
      <h2>{{ $t('ui.type_switch') }}</h2>
      <el-input-number v-model="usertypeId" :min="1" />
      <el-button @click="approveType">{{ $t('ui.approved') }}</el-button>
      <el-button type="danger" @click="rejectType">{{ $t('ui.reject') }}</el-button>
    </el-card>
    <el-card>
      <h2>{{ $t('ui.merge_acc') }}</h2>
      <el-form inline>
        <el-form-item :label="$t('ui.user_uid')"><el-input-number v-model="userUid" :min="1" /></el-form-item>
        <el-form-item :label="$t('ui.com_uid')"><el-input-number v-model="companyUid" :min="1" /></el-form-item>
        <el-button type="primary" @click="merge">{{ $t('ui.merge') }}</el-button>
      </el-form>
    </el-card>
    <pre v-if="msg">{{ msg }}</pre>
  </div>
</template>
