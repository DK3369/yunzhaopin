<template>
  <div v-if="surface !== 'h5'" class="yun_resume_exp_list site-pc">
    <div class="yun_resume_exp_timt">{{ time || '—' }}</div>
    <div class="yun_resume_exp_r">
      <div class="yun_resume_exp_name">
        {{ title }}
        <span v-if="sub" class="yun_resume_exp_name_job">{{ sub }}</span>
      </div>
      <div v-if="body" class="yun_resume_exp_p">{{ body }}</div>
    </div>
    <a href="javascript:;" class="compile_delete" @click.prevent="$emit('edit')">{{ $t('common.edit') }}</a>
    <a href="javascript:;" class="compile_delete" @click.prevent="$emit('remove')">{{ $t('common.delete') }}</a>
  </div>
  <div v-if="surface !== 'pc'" class="work_list site-h5" @click="$emit('edit')">
    <div class="cord_work_experience_two">
      <div class="cord_work_experience_two_word">{{ title }}</div>
      <div class="cord_work_experience_two_icon">
        <img src="/legacy/h5/images/icon_more.png" alt="" width="100%" height="100%" />
      </div>
    </div>
    <div class="cord_work_experience_three">
      <div v-if="time" class="experience_three_timeline">{{ time }}</div>
      <div v-if="sub" class="experience_three_job">{{ sub }}</div>
    </div>
    <div v-if="body" class="cord_work_experience_four">{{ body }}</div>
  </div>
</template>

<script setup lang="ts">
withDefaults(
  defineProps<{
    title: string
    sub?: string
    time?: string
    body?: string
    surface?: 'pc' | 'h5' | 'both'
  }>(),
  { surface: 'both' },
)
defineEmits<{ edit: []; remove: [] }>()
</script>
