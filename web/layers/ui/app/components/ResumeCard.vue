<template>
  <div class="resume_newlist site-pc">
    <div class="resume_newlist_jobname">
      <NuxtLink :to="`/resumes/${row.uid}`" class="resume_newlist_job">
        {{ title }}
      </NuxtLink>
    </div>
    <div class="resume_newlist_user">
      <NuxtLink :to="`/resumes/${row.uid}`" class="resume_newlist_username">{{ title }}</NuxtLink>
    </div>
    <div class="resume_newlist_city">{{ meta }}</div>
  </div>
  <NuxtLink class="site-h5 table-card" :to="`/resumes/${row.uid}`">
    <div class="card_post">
      <i class="table-card-word">{{ title }}</i>
    </div>
    <div class="table-card-require">
      <i class="requir-area">{{ meta }}</i>
    </div>
  </NuxtLink>
</template>

<script setup lang="ts">
const props = defineProps<{ row: Record<string, unknown> }>()
const title = computed(() =>
  String(props.row.display_name || props.row.name || props.row.uname || props.row.uid || ''),
)
const meta = computed(() =>
  [props.row.exp_n, props.row.edu_n || props.row.education_n, props.row.job_classid_n || props.row.expect]
    .filter(Boolean)
    .join(' · '),
)
</script>
