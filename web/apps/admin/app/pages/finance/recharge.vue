<script setup lang="ts">
const api = useApi()
const form = reactive({ uid: 1, kind: 'integral', amount: 100 })
const msg = ref('')
async function recharge() {
  msg.value = ''
  try {
    const r = await api.post<{ value: number }>('/v1/admin/finance/recharge', { ...form })
    msg.value = `完成，结果值 ${r.value}`
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : '失败'
  }
}
</script>

<template>
  <div>
    <h1>财务充值</h1>
    <p>PHP <code>finance_recharge</code>：写 <code>phpyun_company_statis</code> 积分或 <code>vip_etime</code>。不改 JWT。</p>
    <el-form label-width="100px" style="max-width: 420px">
      <el-form-item label="企业 uid">
        <el-input-number v-model="form.uid" :min="1" />
      </el-form-item>
      <el-form-item label="类型">
        <el-select v-model="form.kind" style="width: 200px">
          <el-option value="integral" label="积分 integral" />
          <el-option value="vip_days" label="套餐天数 vip_days" />
        </el-select>
      </el-form-item>
      <el-form-item label="数量">
        <el-input-number v-model="form.amount" :min="1" />
      </el-form-item>
      <el-button type="primary" @click="recharge">充值</el-button>
    </el-form>
    <p v-if="msg">{{ msg }}</p>
  </div>
</template>
