<script setup lang="ts">
import { errKey } from '~/utils/site'

const props = withDefaults(defineProps<{ kind?: 'user' | 'com' }>(), { kind: 'user' })
const api = useApi()
const route = useRoute()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const peer = computed(() => Number(route.query.peer || 0) || 0)
const draft = ref('')
const msg = ref('')
const { data: conv, refresh: refreshConv } = await useAsyncData(
  () => `chat-conv-${props.kind}-${page.value}`,
  () =>
    api
      .post<{ list?: Array<{ peer_uid: number; peer_username?: string; last_body?: string; last_at_n?: string; unread?: number }> }>(
        '/v1/mcenter/chat/conversations',
        { page: page.value, page_size: pageSize },
      )
      .catch(() => ({ list: [] })),
)
const { data: thread, refresh: refreshThread } = await useAsyncData(
  () => `chat-with-${peer.value}`,
  () =>
    peer.value
      ? api
          .post<{ list?: Array<{ id: number; body: string; mine?: boolean; created_at_n?: string }> }>(
            '/v1/mcenter/chat/with',
            { peer: peer.value, limit: 50 },
          )
          .catch(() => ({ list: [] }))
      : Promise.resolve({ list: [] }),
)
const list = computed(() => conv.value?.list || [])
const msgs = computed(() => thread.value?.list || [])
const total = computed(() => inferTotal(conv.value, list.value))

async function openPeer(uid: number) {
  await navigateTo({ query: { ...route.query, peer: String(uid) } })
}

async function send() {
  msg.value = ''
  const body = draft.value.trim()
  if (!peer.value || !body) return
  try {
    await api.post('/v1/mcenter/chat/send', { peer_uid: peer.value, body })
    draft.value = ''
    await api.post('/v1/mcenter/chat/with/read', { peer: peer.value }).catch(() => null)
    await Promise.all([refreshThread(), refreshConv()])
  } catch (e: unknown) {
    if (errKey(e) === 'chat_need_vip') {
      await navigateTo(props.kind === 'com' ? '/com/member-right' : '/user/member-right')
      return
    }
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

let timer: ReturnType<typeof setInterval> | null = null
onMounted(() => {
  timer = setInterval(() => {
    if (peer.value) refreshThread()
    refreshConv()
  }, 8000)
})
onBeforeUnmount(() => {
  if (timer) clearInterval(timer)
})
</script>

<template>
  <div>
    <div class="site-h5 chatnewcardbg">
      <div class="chatnewcardheader">{{ $t('wap_user_00363') }}</div>
      <div class="chatnewcard">
        <ul>
          <li v-for="row in list" :key="row.peer_uid" @click="openPeer(row.peer_uid)">
            <i class="card_word">{{ row.peer_username || row.peer_uid }}</i>
            <div v-if="row.unread" class="card_logo_circle">{{ row.unread }}</div>
          </li>
        </ul>
      </div>
    </div>
    <div v-if="list.length" class="sysynews_tit site-pc">
      <div class="sysynews_span sysynews_name">{{ $t('wap_user_00363') }}</div>
      <div class="sysynews_span sysynews_time">{{ $t('member_user_00104') }}</div>
    </div>
    <div v-for="row in list" :key="row.peer_uid" class="sysynews_list site-pc" @click="openPeer(row.peer_uid)">
      <div class="sysynews_span sysynews_name">
        {{ row.peer_username || row.peer_uid }}
        <span v-if="row.unread" class="sysynews_span_nolook">{{ row.unread }}</span>
      </div>
      <div class="sysynews_span sysynews_time">{{ row.last_at_n }}</div>
      <div class="sysynews_span">{{ row.last_body }}</div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    <div v-if="peer" class="site-pc com_release_box">
      <div v-for="m in msgs" :key="m.id" :class="{ muted: !m.mine }">
        <span>{{ m.created_at_n }}</span>
        {{ m.body }}
      </div>
      <form @submit.prevent="send">
        <textarea v-model="draft" rows="3" maxlength="2000" />
        <button type="submit" class="btn_01">{{ $t('common.submit') }}</button>
      </form>
    </div>
    <div v-if="peer" class="site-h5 issue_post_body">
      <div v-for="m in msgs" :key="'h5-' + m.id" class="com_cardlist">
        <div class="com_cardlist_tit">{{ m.mine ? $t('common.yes') : $t('common.no') }}</div>
        <div class="com_cardlist_p">{{ m.body }}</div>
        <div class="com_cardlist_p">{{ m.created_at_n }}</div>
      </div>
      <form class="yun_createbox" @submit.prevent="send">
        <textarea v-model="draft" rows="3" maxlength="2000" />
        <button type="submit" class="issue_post_body_btn">{{ $t('common.submit') }}</button>
      </form>
    </div>
    <p v-if="msg" class="muted">{{ msg }}</p>
  </div>
</template>
