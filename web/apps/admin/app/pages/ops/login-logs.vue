<script setup lang="ts">
const api = useApi()
const usertype = ref(1)
const uid = ref<number | undefined>()
const { data, refresh } = await useAsyncData(
  () => `admin-login-logs-${usertype.value}-${uid.value || 0}`,
  () =>
    api.post<{ list: Array<Record<string, unknown>>; total: number }>('/v1/admin/login-logs', {
      page: 1,
      page_size: 20,
      usertype: usertype.value,
      uid: uid.value,
    }),
)
watch([usertype, uid], () => refresh())
</script>

<template>
  <div>
    <h1>{{ $t('ui.login_log') }}</h1>
    <el-form inline>
      <el-form-item>
        <el-select v-model="usertype" style="width: 140px">
          <el-option :value="1" :label="$t('ui.jobseeker')" />
          <el-option :value="2" :label="$t('common.company')" />
        </el-select>
      </el-form-item>
      <el-form-item>
        <el-input-number v-model="uid" :min="0" :placeholder="$t('ui.com_uid_label')" />
      </el-form-item>
      <el-button @click="refresh">{{ $t('ui.query') }}</el-button>
    </el-form>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="id" width="80" />
      <el-table-column prop="uid" label="uid" width="90" />
      <el-table-column prop="usertype" :label="$t('ui.type')" width="90" />
      <el-table-column prop="content" :label="$t('ui.content')" />
      <el-table-column prop="ip" label="ip" width="140" />
      <el-table-column prop="ctime" :label="$t('ui.time')" width="120" />
    </el-table>
  </div>
</template>
