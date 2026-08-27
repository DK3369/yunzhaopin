import { ElMessage, ElMessageBox, ElLoading, ElNotification } from 'element-plus'
import { httpPost } from '~/utils/httpPost'
import { lc, loadAliases, loadLangPack, persistLocale, readStoredLocale } from '~/utils/phpLc'

function formatMonth(date: Date) {
  const year = date.getFullYear()
  const month = date.getMonth() + 1
  return `${year}-${month < 10 ? '0' + month : month}`
}
function formatDate(date: Date) {
  const year = date.getFullYear()
  const month = date.getMonth() + 1
  const day = date.getDate()
  return `${year}-${month < 10 ? '0' + month : month}-${day < 10 ? '0' + day : day}`
}
function formatDatetime(date: Date) {
  const hours = date.getHours()
  const minutes = date.getMinutes()
  const seconds = date.getSeconds()
  return (
    formatDate(date) +
    ` ${hours < 10 ? '0' + hours : hours}:${minutes < 10 ? '0' + minutes : minutes}:${seconds < 10 ? '0' + seconds : seconds}`
  )
}
function isEmpty(val: unknown) {
  return val == null || val === ''
}
function isArray(arr: unknown) {
  return Array.isArray(arr)
}
function deepClone<T>(obj: T): T {
  if (obj == null || typeof obj !== 'object') return obj
  return JSON.parse(JSON.stringify(obj)) as T
}
function scrollToTop(container = '.moduleDome') {
  const el = document.querySelector(container)
  if (el) el.scrollTop = 0
  window.scrollTo(0, 0)
}

const message = {
  success(msg: string, closeFun?: () => void) {
    ElMessage.success({ message: lc(msg, null, msg), onClose: closeFun })
  },
  error(msg: string, closeFun?: () => void) {
    ElMessage.error({ message: lc(msg, null, msg), onClose: closeFun })
  },
  warning(msg: string, closeFun?: () => void) {
    ElMessage.warning({ message: lc(msg, null, msg), onClose: closeFun })
  },
  info(msg: string, closeFun?: () => void) {
    ElMessage.info({ message: lc(msg, null, msg), onClose: closeFun })
  },
  open(options: Record<string, unknown>) {
    ElMessage(options as never)
  },
  confirm(
    msg: string,
    confirmFun?: () => void,
    confirmButtonText = '',
    title = '',
    type = 'warning',
    showCancelButton = true,
    cancelButtonText = '',
    cancelFun?: () => void,
  ) {
    ElMessageBox.confirm(lc(msg, null, msg), title || lc('common_01520', null, '提示'), {
      confirmButtonText: confirmButtonText || lc('common_02016', null, '确定'),
      cancelButtonText: cancelButtonText || lc('wap_js_00080', null, '取消'),
      type: type as 'warning',
      showCancelButton,
    })
      .then(() => confirmFun?.())
      .catch(() => cancelFun?.())
  },
  alert(msg: string, confirmFun?: () => void, confirmButtonText = '') {
    ElMessageBox.alert(lc(msg, null, msg), {
      confirmButtonText: confirmButtonText || lc('common_02016', null, '确定'),
    }).then(() => confirmFun?.())
  },
}

function delConfirm(
  _this: { $confirm?: typeof ElMessageBox.confirm },
  params: unknown,
  delFun: (p: unknown) => void,
  msg?: string,
  cancelFun?: () => void,
) {
  const text = msg || lc('admin_00333', null, '确定删除当前项目？')
  ElMessageBox.confirm(text, lc('common_01520', null, '提示'), {
    confirmButtonText: lc('common_02016', null, '确定'),
    cancelButtonText: lc('wap_js_00080', null, '取消'),
    type: 'warning',
  })
    .then(() => delFun(params))
    .catch(() => cancelFun?.())
}

export default defineNuxtPlugin(async (nuxtApp) => {
  const loc = readStoredLocale()
  persistLocale(loc)
  await loadAliases()
  await loadLangPack(loc)
  const i18n = nuxtApp.$i18n as { setLocale?: (c: string) => Promise<void> } | undefined
  try {
    if (i18n?.setLocale) await i18n.setLocale(loc)
  } catch {
    /* pack is already loaded; Element Plus locale follows on next paint */
  }

  if (import.meta.client) {
    window.lc = lc
    window.httpPost = httpPost
    const w = window as unknown as Record<string, unknown>
    w.message = message
    w.delConfirm = delConfirm
    w.formatMonth = formatMonth
    w.formatDate = formatDate
    w.formatDatetime = formatDatetime
    w.deepClone = deepClone
    w.isArray = isArray
    w.isEmpty = isEmpty
    w.scrollToTop = scrollToTop
    w.showFullScreenLoading = () => undefined
    w.tryHideFullScreenLoading = () => undefined
    if (typeof w.$ !== 'function') {
      const jq = w.jQuery as ((sel?: unknown) => unknown) | undefined
      if (typeof jq === 'function') {
        w.$ = jq
      } else {
        const stub = Object.assign(
          (sel: unknown) => {
            if (typeof sel === 'function') {
              sel()
              return
            }
            return {
              length: 0,
              on() {
                return this
              },
            }
          },
          {
            isEmptyObject(o: unknown) {
              return !o || (typeof o === 'object' && Object.keys(o as object).length === 0)
            },
            each(obj: unknown, fn: (k: unknown, v: unknown) => void) {
              if (Array.isArray(obj)) obj.forEach((v, i) => fn(i, v))
              else if (obj && typeof obj === 'object') {
                for (const [k, v] of Object.entries(obj as object)) fn(k, v)
              }
            },
            ajax(opts: { url?: string; data?: unknown; success?: (d: unknown) => void; error?: (e: unknown) => void }) {
              httpPost(String(opts?.url || ''), opts?.data)
                .then((r) => opts?.success?.(r.data))
                .catch((e) => opts?.error?.(e))
            },
            trim: (s: unknown) => String(s ?? '').trim(),
          },
        )
        w.$ = stub
      }
    }
  }

  nuxtApp.vueApp.mixin({
    methods: {
      lc,
      $set(obj: Record<string, unknown>, key: string, val: unknown) {
        if (obj && typeof obj === 'object') obj[key] = val
      },
      $delete(obj: Record<string, unknown>, key: string) {
        if (obj && typeof obj === 'object') delete obj[key]
      },
    },
    created() {
      const inst = this as unknown as Record<string, unknown>
      inst.$message = ElMessage
      inst.$confirm = ElMessageBox.confirm
      inst.$alert = ElMessageBox.alert
      inst.$notify = ElNotification
      inst.$loading = ElLoading.service
    },
  })
})
