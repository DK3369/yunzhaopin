<script setup lang="ts">
const api = useApi()
const nid = ref<number | undefined>()
const keyword = ref('')
const form = reactive({
  id: 0,
  title: '',
  nid: 1,
  content: '',
  author: '',
  description: '',
  keyword: '',
})
const { data: groups, refresh: refreshGroups } = await useAsyncData('admin-news-groups', () =>
  api.post<Array<{ id: number; name: string }>>('/v1/admin/articles/groups', {}),
)
const { data, refresh } = await useAsyncData('admin-articles', () =>
  api.post<{ list: Array<Record<string, unknown>> }>('/v1/admin/articles/list', {
    page: 1,
    page_size: 20,
    nid: nid.value,
    keyword: keyword.value || undefined,
  }),
)
async function save() {
  await api.post('/v1/admin/articles', {
    id: form.id || undefined,
    title: form.title,
    nid: form.nid,
    content: form.content,
    author: form.author,
    description: form.description,
    keyword: form.keyword,
  })
  form.id = 0
  form.title = ''
  form.content = ''
  refresh()
  refreshGroups()
}
async function remove(row: { id: number }) {
  await api.post('/v1/admin/articles/delete', { id: row.id })
  refresh()
}
function edit(row: Record<string, unknown>) {
  form.id = Number(row.id || 0)
  form.title = String(row.title || '')
  form.nid = Number(row.nid || 1)
  form.content = String(row.content || '')
  form.author = String(row.author || '')
  form.description = String(row.summary || '')
  form.keyword = String(row.keyword || '')
}
</script>

<template>
  <div>
    <h1>资讯</h1>
    <el-form label-width="80px" style="max-width: 720px">
      <el-form-item label="分类 nid">
        <el-input-number v-model="form.nid" :min="1" />
        <span class="muted" style="margin-left: 8px">
          {{ (Array.isArray(groups) ? groups : []).find((g) => g.id === form.nid)?.name }}
        </span>
      </el-form-item>
      <el-form-item label="标题"><el-input v-model="form.title" /></el-form-item>
      <el-form-item label="作者"><el-input v-model="form.author" /></el-form-item>
      <el-form-item label="摘要"><el-input v-model="form.description" /></el-form-item>
      <el-form-item label="正文"><el-input v-model="form.content" type="textarea" :rows="6" /></el-form-item>
      <el-button type="primary" @click="save">保存</el-button>
    </el-form>
    <el-form inline style="margin-top: 16px">
      <el-form-item><el-input v-model="keyword" placeholder="标题" /></el-form-item>
      <el-button @click="refresh">筛选</el-button>
    </el-form>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="nid" label="nid" width="80" />
      <el-table-column prop="title" label="标题" />
      <el-table-column label="操作" width="180">
        <template #default="{ row }">
          <el-button size="small" @click="edit(row)">编辑</el-button>
          <el-button size="small" type="danger" @click="remove(row)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>
