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
    <h1>账号工具</h1>
    <p>注销申请、身份切换、个人账号并入企业。无列表接口，按 ID 操作。</p>
    <el-card style="margin-bottom: 16px">
      <h2>注销申请</h2>
      <el-input-number v-model="logoutId" :min="1" />
      <el-button @click="approveLogout">通过</el-button>
      <el-button type="danger" @click="rejectLogout">拒绝</el-button>
    </el-card>
    <el-card style="margin-bottom: 16px">
      <h2>身份切换申请</h2>
      <el-input-number v-model="usertypeId" :min="1" />
      <el-button @click="approveType">通过</el-button>
      <el-button type="danger" @click="rejectType">拒绝</el-button>
    </el-card>
    <el-card>
      <h2>账号合并</h2>
      <el-form inline>
        <el-form-item label="个人 UID"><el-input-number v-model="userUid" :min="1" /></el-form-item>
        <el-form-item label="企业 UID"><el-input-number v-model="companyUid" :min="1" /></el-form-item>
        <el-button type="primary" @click="merge">合并</el-button>
      </el-form>
    </el-card>
    <pre v-if="msg">{{ msg }}</pre>
  </div>
</template>
