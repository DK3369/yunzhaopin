<template>
  <div class="hr_userbox" @click="$emit('open')">
    <div class="hr_userlist">
      <div v-if="photo" class="hr_userlist_pic">
        <img :src="photo" alt="" />
      </div>
      <div class="hr_userlist_name">
        {{ name }}
        <div v-if="iconSrc" class="hr_userlist_icon">
          <img :src="iconSrc" alt="" />
        </div>
        <div v-if="invited" class="hr_userlist_p"><span class="hr_yyy">{{ $t('wap_user_00216') }}</span></div>
        <div v-if="downloaded" class="hr_userlist_p"><span class="hr_yxz">{{ $t('wap_00451') }}</span></div>
      </div>
      <div v-if="info.length" class="hr_userlist_info">
        <span v-for="(item, i) in info" :key="i">{{ item }}</span>
      </div>
      <div v-if="stateText" class="hr_userlist_wsh">{{ stateText }}</div>
    </div>
    <div v-if="job" class="hr_userlist_p">
      <span class="hr_userlist_t">{{ $t('wap_com_00288') }}</span>
      {{ job }}
    </div>
    <div v-if="time" class="hr_userlist_p">
      <span class="hr_userlist_t">{{ $t('member_user_00106') }}</span>
      {{ time }}
    </div>
    <div v-if="$slots.default" class="hr_userlist_cz" @click.stop>
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    name: string
    photo?: string
    job?: string
    time?: string
    stateText?: string
    isBrowse?: number
    invited?: boolean
    downloaded?: boolean
    info?: string[]
  }>(),
  { info: () => [] },
)
defineEmits<{ open: [] }>()
const iconSrc = computed(() => {
  const b = Number(props.isBrowse || 0)
  if (b === 1) return '/legacy/h5/images/tab_new.png'
  if (b === 3) return '/legacy/h5/images/Apply_undetermined.png'
  if (b === 4) return '/legacy/h5/images/Apply_inappropriate.png'
  if (b === 5) return '/legacy/h5/images/Apply_block call.png'
  if (b === 7) return '/legacy/h5/images/Apply_Rz.png'
  return ''
})
</script>
