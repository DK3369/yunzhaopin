<script setup lang="ts">
const route = useRoute()
const uid = computed(() => Number(route.query.uid || 0))
const api = useApi()
const { data: works } = await useAsyncData(
  () => `resume-works-${uid.value}`,
  () =>
    uid.value
      ? api.post<Array<Record<string, unknown>>>('/v1/admin/resumes/works', { uid: uid.value })
      : Promise.resolve([]),
)
const { data: edus } = await useAsyncData(
  () => `resume-edus-${uid.value}`,
  () =>
    uid.value
      ? api.post<Array<Record<string, unknown>>>('/v1/admin/resumes/edus', { uid: uid.value })
      : Promise.resolve([]),
)
const { data: trainings } = await useAsyncData(
  () => `resume-trainings-${uid.value}`,
  () =>
    uid.value
      ? api.post<Array<Record<string, unknown>>>('/v1/admin/resumes/trainings', { uid: uid.value })
      : Promise.resolve([]),
)
</script>

<template>
  <div>
    <h1>简历经历树</h1>
    <p>PHP <code>users_resume</code> 工作/教育/培训：列名 <code>id,uid,eid,name,sdate,edate</code>。</p>
    <el-form inline>
      <el-form-item>
        <el-input-number :model-value="uid" :min="0" @change="(v: number) => navigateTo({ query: { uid: v } })" />
      </el-form-item>
    </el-form>
    <h2>工作经历 works</h2>
    <el-table :data="Array.isArray(works) ? works : []">
      <el-table-column prop="id" label="id" width="80" />
      <el-table-column prop="name" label="name" />
      <el-table-column prop="title" label="title" />
      <el-table-column prop="department" label="department" />
      <el-table-column prop="sdate" label="sdate" width="120" />
      <el-table-column prop="edate" label="edate" width="120" />
    </el-table>
    <h2>教育经历 edus</h2>
    <el-table :data="Array.isArray(edus) ? edus : []">
      <el-table-column prop="id" label="id" width="80" />
      <el-table-column prop="name" label="name" />
      <el-table-column prop="specialty" label="specialty" />
      <el-table-column prop="education" label="education" width="110" />
      <el-table-column prop="sdate" label="sdate" width="120" />
      <el-table-column prop="edate" label="edate" width="120" />
    </el-table>
    <h2>培训经历 trainings</h2>
    <el-table :data="Array.isArray(trainings) ? trainings : []">
      <el-table-column prop="id" label="id" width="80" />
      <el-table-column prop="name" label="name" />
      <el-table-column prop="title" label="title" />
      <el-table-column prop="sdate" label="sdate" width="120" />
      <el-table-column prop="edate" label="edate" width="120" />
    </el-table>
  </div>
</template>
