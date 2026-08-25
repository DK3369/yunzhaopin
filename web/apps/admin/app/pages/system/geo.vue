<script setup lang="ts">
const api = useApi()
const msg = ref('')
const region = reactive({
  parent_id: undefined as number | undefined,
  country_code: 'CN',
  code: '',
  level: 1,
  name: '',
  continent: '',
  sort: 0,
})
const regionPatch = reactive({ id: 1, name: '', sort: 0, continent: '' })
const regionId = ref(1)
const country = reactive({
  code: 'XX',
  code3: 'XXX',
  numeric_code: 0,
  name_en: '',
  name_zh: '',
  continent: 'AS',
  phone_code: '0',
  currency: 'XXX',
  flag: '🏳️',
  sort: 0,
})
const countryId = ref(1)
const countryPatch = reactive({ id: 1, name_zh: '' })
async function run(label: string, fn: () => Promise<unknown>) {
  msg.value = `${label}: ${JSON.stringify(await fn())}`
}
</script>

<template>
  <div>
    <h1>地区 / 国家 / 字典缓存</h1>
    <p>无列表接口，只提供创建、修改、删除与热加载。</p>
    <el-card style="margin-bottom: 16px">
      <h2>地区</h2>
      <el-form inline>
        <el-form-item><el-input v-model="region.country_code" placeholder="CN" style="width: 80px" /></el-form-item>
        <el-form-item><el-input v-model="region.code" placeholder="CN-BJ" /></el-form-item>
        <el-form-item><el-input-number v-model="region.level" :min="0" :max="3" /></el-form-item>
        <el-form-item><el-input v-model="region.name" placeholder="名称" /></el-form-item>
        <el-button type="primary" @click="run('创建地区', () => api.post('/v1/admin/regions', { ...region }))">创建</el-button>
      </el-form>
      <el-form inline>
        <el-form-item><el-input-number v-model="regionPatch.id" :min="1" /></el-form-item>
        <el-form-item><el-input v-model="regionPatch.name" placeholder="新名称" /></el-form-item>
        <el-button @click="run('修改地区', () => api.post('/v1/admin/regions/patch', { ...regionPatch }))">修改</el-button>
        <el-button type="danger" @click="run('删除地区', () => api.post('/v1/admin/regions/delete', { id: regionId }))">
          删除 ID
        </el-button>
        <el-input-number v-model="regionId" :min="1" />
        <el-button @click="run('重载地区', () => api.post('/v1/admin/regions/reload', {}))">重载缓存</el-button>
      </el-form>
    </el-card>
    <el-card style="margin-bottom: 16px">
      <h2>国家</h2>
      <el-form inline>
        <el-form-item><el-input v-model="country.code" placeholder="code" style="width: 80px" /></el-form-item>
        <el-form-item><el-input v-model="country.code3" placeholder="code3" style="width: 90px" /></el-form-item>
        <el-form-item><el-input v-model="country.name_zh" placeholder="中文名" /></el-form-item>
        <el-form-item><el-input v-model="country.name_en" placeholder="English" /></el-form-item>
        <el-button type="primary" @click="run('创建国家', () => api.post('/v1/admin/countries', { ...country }))">创建</el-button>
      </el-form>
      <el-form inline>
        <el-form-item><el-input-number v-model="countryPatch.id" :min="1" /></el-form-item>
        <el-form-item><el-input v-model="countryPatch.name_zh" placeholder="新中文名" /></el-form-item>
        <el-button @click="run('修改国家', () => api.post('/v1/admin/countries/patch', { ...countryPatch }))">修改</el-button>
        <el-input-number v-model="countryId" :min="1" />
        <el-button type="danger" @click="run('删除国家', () => api.post('/v1/admin/countries/delete', { id: countryId }))">
          删除
        </el-button>
        <el-button @click="run('重载国家', () => api.post('/v1/admin/countries/reload', {}))">重载缓存</el-button>
      </el-form>
    </el-card>
    <el-card>
      <h2>字典翻译</h2>
      <el-button @click="run('重载字典', () => api.post('/v1/admin/dict-i18n/reload', {}))">立即重载 dict-i18n</el-button>
    </el-card>
    <pre v-if="msg" style="margin-top: 16px">{{ msg }}</pre>
  </div>
</template>
