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
    <div v-if="h5Kind !== 'none'" class="site-h5" :class="h5Wrap">
      <div :class="h5Head">
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
const props = withDefaults(
  defineProps<{
    title: string
    icon?: string
    open?: boolean
    h5Kind?: 'work' | 'edu' | 'skill' | 'show' | 'none'
  }>(),
  { h5Kind: 'work' },
)
defineEmits<{ toggle: [] }>()

const h5Wrap = computed(() => {
  if (props.h5Kind === 'edu') return 'resume_min_body_cord_education_experience'
  if (props.h5Kind === 'show') return 'resume_min_body_Individual_works'
  if (props.h5Kind === 'skill') return ''
  return 'resume_min_body_cord_work_experience'
})
const h5Head = computed(() =>
  props.h5Kind === 'skill' || props.h5Kind === 'show' ? 'cord_intention_top' : 'cord_work_experience_one',
)
</script>
