<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const form = reactive({ uid: 1, kind: 'integral', amount: 100 })
const msg = ref('')
async function recharge() {
  msg.value = ''
  try {
    const r = await api.post<{ value: number }>('/v1/admin/finance/recharge', { ...form })
    msg.value = `${t('ui.success')} ${r.value}`
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
</script>

<template>
  <div>
    <h1>{{ $t('ui.recharge') }}</h1>
    <el-form label-width="100px" style="max-width: 420px">
      <el-form-item :label="$t('ui.com_uid_label')">
        <el-input-number v-model="form.uid" :min="1" />
      </el-form-item>
      <el-form-item :label="$t('ui.type')">
        <el-select v-model="form.kind" style="width: 200px">
          <el-option value="integral" :label="$t('ui.integral')" />
          <el-option value="vip_days" :label="$t('ui.vip_days')" />
        </el-select>
      </el-form-item>
      <el-form-item :label="$t('ui.qty')">
        <el-input-number v-model="form.amount" :min="1" />
      </el-form-item>
      <el-button type="primary" @click="recharge">{{ $t('ui.recharge_btn') }}</el-button>
    </el-form>
    <p v-if="msg">{{ msg }}</p>
  </div>
</template>
