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
    <h1>{{ $t('ui.geo') }}</h1>
    <p>{{ $t('ui.geo_hint') }}</p>
    <el-card style="margin-bottom: 16px">
      <h2>{{ $t('ui.region') }}</h2>
      <el-form inline>
        <el-form-item><el-input v-model="region.country_code" placeholder="CN" style="width: 80px" /></el-form-item>
        <el-form-item><el-input v-model="region.code" placeholder="CN-BJ" /></el-form-item>
        <el-form-item><el-input-number v-model="region.level" :min="0" :max="3" /></el-form-item>
        <el-form-item><el-input v-model="region.name" :placeholder="$t('ui.name')" /></el-form-item>
        <el-button type="primary" @click="run('create_region', () => api.post('/v1/admin/regions', { ...region }))">{{
          $t('ui.create')
        }}</el-button>
      </el-form>
      <el-form inline>
        <el-form-item><el-input-number v-model="regionPatch.id" :min="1" /></el-form-item>
        <el-form-item><el-input v-model="regionPatch.name" :placeholder="$t('ui.new_name')" /></el-form-item>
        <el-button @click="run('patch_region', () => api.post('/v1/admin/regions/patch', { ...regionPatch }))">{{
          $t('common.edit')
        }}</el-button>
        <el-button type="danger" @click="run('delete_region', () => api.post('/v1/admin/regions/delete', { id: regionId }))">
          {{ $t('ui.delete_id') }}
        </el-button>
        <el-input-number v-model="regionId" :min="1" />
        <el-button @click="run('reload_region', () => api.post('/v1/admin/regions/reload', {}))">{{ $t('ui.reload_region') }}</el-button>
      </el-form>
    </el-card>
    <el-card style="margin-bottom: 16px">
      <h2>{{ $t('ui.country') }}</h2>
      <el-form inline>
        <el-form-item><el-input v-model="country.code" placeholder="code" style="width: 80px" /></el-form-item>
        <el-form-item><el-input v-model="country.code3" placeholder="code3" style="width: 90px" /></el-form-item>
        <el-form-item><el-input v-model="country.name_zh" :placeholder="$t('ui.name_zh')" /></el-form-item>
        <el-form-item><el-input v-model="country.name_en" placeholder="English" /></el-form-item>
        <el-button type="primary" @click="run('create_country', () => api.post('/v1/admin/countries', { ...country }))">{{
          $t('ui.create')
        }}</el-button>
      </el-form>
      <el-form inline>
        <el-form-item><el-input-number v-model="countryPatch.id" :min="1" /></el-form-item>
        <el-form-item><el-input v-model="countryPatch.name_zh" :placeholder="$t('ui.new_name_zh')" /></el-form-item>
        <el-button @click="run('patch_country', () => api.post('/v1/admin/countries/patch', { ...countryPatch }))">{{
          $t('common.edit')
        }}</el-button>
        <el-input-number v-model="countryId" :min="1" />
        <el-button type="danger" @click="run('delete_country', () => api.post('/v1/admin/countries/delete', { id: countryId }))">
          {{ $t('common.delete') }}
        </el-button>
        <el-button @click="run('reload_country', () => api.post('/v1/admin/countries/reload', {}))">{{ $t('ui.reload_country') }}</el-button>
      </el-form>
    </el-card>
    <el-card>
      <h2>{{ $t('ui.dict_i18n') }}</h2>
      <el-button @click="run('reload_dict', () => api.post('/v1/admin/dict-i18n/reload', {}))">{{ $t('ui.dict_reload') }}</el-button>
    </el-card>
    <pre v-if="msg" style="margin-top: 16px">{{ msg }}</pre>
  </div>
</template>
