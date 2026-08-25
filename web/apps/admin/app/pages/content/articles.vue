<script setup lang="ts">
const api = useApi()
const { data } = await useAsyncData('admin-articles', () =>
  api.get('/v1/wap/articles', { page: 1, page_size: 20 }),
)
</script>

<template>
  <div>
    <h1>资讯（只读，复用公开接口）</h1>
    <el-table :data="data?.list || []">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="title" label="标题" />
    </el-table>
  </div>
</template>
