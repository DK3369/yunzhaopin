<template>
  <div class="member-resume-section">
    <div class="site-pc yun_resume_joblist">
      <div class="yun_resume_h1">
        <span class="yun_resume_h1_s">
          <i v-if="icon" class="yun_resume_h1_icon" :class="icon" />
          {{ title }}
        </span>
      </div>
      <a href="javascript:;" class="yun_resume_handle" @click="$emit('toggle')">{{ $t('common.edit') }}</a>
      <slot name="pc" />
    </div>
    <div class="site-h5 resume_min_body_cord_work_experience">
      <div class="cord_work_experience_one">
        <div class="cord_intention_top_word">{{ title }}</div>
        <div class="cord_intention_top_icon" @click="$emit('toggle')">
          <img src="/legacy/h5/images/addition.png" alt="" width="100%" height="100%" />
        </div>
      </div>
      <slot name="h5" />
    </div>
    <div v-if="open" class="verification_form member-resume-section-form">
      <slot name="form" />
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  title: string
  icon?: string
  open?: boolean
}>()
defineEmits<{ toggle: [] }>()
</script>
