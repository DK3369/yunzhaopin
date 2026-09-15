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
    <div
      v-if="h5Kind !== 'none'"
      class="site-h5"
      :class="h5Kind === 'edu' ? 'resume_min_body_cord_education_experience' : 'resume_min_body_cord_work_experience'"
    >
      <div class="cord_work_experience_one">
        <div class="cord_intention_top_word">{{ title }}</div>
        <div class="cord_intention_top_icon" @click="$emit('toggle')">
          <img src="/legacy/h5/images/addition.png" alt="" width="100%" height="100%" />
        </div>
      </div>
      <slot name="h5" />
    </div>
    <div v-if="open" class="yun_createbox member-resume-section-form verification_form">
      <slot name="form" />
    </div>
  </div>
</template>

<script setup lang="ts">
withDefaults(
  defineProps<{
    title: string
    icon?: string
    open?: boolean
    h5Kind?: 'work' | 'edu' | 'none'
  }>(),
  { h5Kind: 'work' },
)
defineEmits<{ toggle: [] }>()
</script>
