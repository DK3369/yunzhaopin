import { h } from 'vue'
import { ElMessage, ElMessageBox, ElLoading, ElNotification, ElSwitch as ElSwitchBase } from 'element-plus'
import { httpPost } from '~/utils/httpPost'
import { lc, persistLocale, readStoredLocale } from '~/utils/phpLc'

function coerceSwitchValue(val: unknown, active: unknown, inactive: unknown) {
  if (Object.is(val, active) || Object.is(val, inactive)) return val
  if (val === true || val === 1 || val === '1') return active
  if (val === false || val === 0 || val === '0' || val == null || val === '') return inactive
  if (String(val) === String(active)) return active
  if (String(val) === String(inactive)) return inactive
  return inactive
}

function attrOf(attrs: Record<string, unknown>, camel: string, kebab: string) {
  return attrs[camel] !== undefined ? attrs[camel] : attrs[kebab]
}

function phpSwitchVNode(attrs: Record<string, unknown>, slots: Record<string, unknown>) {
  const active = attrOf(attrs, 'activeValue', 'active-value') ?? true
  const inactive = attrOf(attrs, 'inactiveValue', 'inactive-value') ?? false
  const model = attrOf(attrs, 'modelValue', 'model-value')
  return h(
    ElSwitchBase,
    {
      ...attrs,
      modelValue: coerceSwitchValue(model, active, inactive),
    },
    slots,
  )
}

function unwrapMaybeRef(v: unknown): unknown {
  if (v && typeof v === 'object' && 'value' in (v as object) && !Array.isArray(v)) {
    const inner = (v as { value: unknown }).value
    if (inner == null || typeof inner === 'string' || typeof inner === 'number' || typeof inner === 'boolean') {
      return inner
    }
  }
  return v
}

function phpTabName(tab: unknown): string {
  if (tab == null) return ''
  if (typeof tab === 'string' || typeof tab === 'number') return String(tab)
  if (typeof tab !== 'object') return ''
  const t = tab as Record<string, unknown>
  const props = t.props as Record<string, unknown> | undefined
  const raw = unwrapMaybeRef(t.paneName) ?? unwrapMaybeRef(props?.name) ?? unwrapMaybeRef(t.name)
  if (raw == null || typeof raw === 'object') return ''
  return String(raw)
}

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

/** Element Plus table has no Vue2 `bodyWrapper`; PHP pages still assign scrollTop. */
function ensureBodyWrapper(t: { bodyWrapper?: unknown; $el?: HTMLElement } | null | undefined) {
  if (!t || typeof t !== 'object') return
  if (t.bodyWrapper) return
  const dummy = { scrollTop: 0 }
  try {
    Object.defineProperty(t, 'bodyWrapper', {
      configurable: true,
      enumerable: false,
      get() {
        return t.$el?.querySelector?.('.el-table__body-wrapper') || dummy
      },
      set() {},
    })
  } catch {
    t.bodyWrapper = dummy
  }
}

function looksLikeTable(v: unknown): v is { bodyWrapper?: unknown; $el?: HTMLElement } {
  if (!v || typeof v !== 'object') return false
  const el = (v as { $el?: HTMLElement }).$el
  if (!el || typeof el.querySelector !== 'function') return false
  return el.classList?.contains('el-table') || !!el.querySelector('.el-table__body-wrapper')
}

function patchTableBodyWrapper(inst: Record<string, unknown>) {
  const refs = inst.$refs as Record<string, unknown> | undefined
  if (!refs) return
  for (const v of Object.values(refs)) {
    const list = Array.isArray(v) ? v : [v]
    for (const item of list) {
      if (looksLikeTable(item)) ensureBodyWrapper(item)
    }
  }
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
  const switchCompat = {
    name: 'PhpElSwitch',
    inheritAttrs: false,
    setup(_props: unknown, { attrs, slots }: { attrs: Record<string, unknown>; slots: Record<string, unknown> }) {
      return () => phpSwitchVNode(attrs, slots)
    },
  }
  nuxtApp.vueApp.component('ElSwitch', switchCompat)
  nuxtApp.vueApp.component('PhpElSwitch', switchCompat)

  const loc = readStoredLocale()
  persistLocale(loc)
  const i18n = nuxtApp.$i18n as { setLocale?: (c: string) => Promise<void> } | undefined
  try {
    if (i18n?.setLocale) await i18n.setLocale(loc)
  } catch {
    /* Element Plus locale follows useI18n on next paint */
  }

  if (import.meta.client) {
    window.lc = lc
    window.httpPost = httpPost
    const w = window as unknown as Record<string, unknown>
    // PHP api.js: yunAdminT wraps already-translated lc() text. Nuxt lc() already i18n's.
    w.yunAdminT = (text: unknown) => String(text ?? '')
    w.yunAdminTransText = (text: unknown) => String(text ?? '')
    w.yunAdminTranslateDOM = () => undefined
    // PHP pages load wangEditor in HTML; Nuxt must still have a fallback if the
    // script tag races Vue chunks. Real editor comes from public/php-admin/js/wangeditor.
    if (!w.wangEditor || typeof (w.wangEditor as { createEditor?: unknown }).createEditor !== 'function') {
      const stubEditor = () => ({
        getHtml: () => '',
        getText: () => '',
        setHtml: () => undefined,
        destroy: () => undefined,
        on: () => undefined,
      })
      w.wangEditor = {
        createEditor: stubEditor,
        createToolbar: () => ({ destroy: () => undefined }),
      }
    }
    // Remaining PHP Vue2 pages still call UE.getEditor (UEditor). Stub so they
    // don't throw ReferenceError; content is read/written on the textarea by id.
    if (!w.UE || typeof (w.UE as { getEditor?: unknown }).getEditor !== 'function') {
      const ueById = new Map<string, Record<string, unknown>>()
      w.UE = {
        getEditor(id: string) {
          const key = String(id || '')
          const hit = ueById.get(key)
          if (hit) return hit
          const read = () => {
            const el = document.getElementById(key) as HTMLTextAreaElement | HTMLElement | null
            if (!el) return ''
            return 'value' in el ? String((el as HTMLTextAreaElement).value || '') : el.innerHTML
          }
          const write = (html: string) => {
            const el = document.getElementById(key) as HTMLTextAreaElement | HTMLElement | null
            if (!el) return
            if ('value' in el) (el as HTMLTextAreaElement).value = html
            else el.innerHTML = html
          }
          const inst: Record<string, unknown> = {
            ready(fn?: () => void) {
              fn?.()
            },
            getContent: read,
            getPlainTxt: read,
            setContent: write,
            setDisabled() {},
            setEnabled() {},
            destroy() {
              ueById.delete(key)
            },
          }
          ueById.set(key, inst)
          return inst
        },
      }
    }
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
    let loadingInst: { close: () => void } | null = null
    w.startLoading = () => {
      loadingInst = ElLoading.service({
        lock: true,
        text: 'Loading',
        background: 'rgba(57, 61, 73, 0.5)',
      })
    }
    w.endLoading = () => {
      loadingInst?.close()
      loadingInst = null
    }
    w.baseUrl = '/admin/api/php-admin?'
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
      const inst = this as unknown as Record<string, unknown> & {
        handleClick?: (tab: unknown, event?: unknown) => unknown
        __epTabPatched?: boolean
      }
      inst.$message = ElMessage
      inst.$confirm = ElMessageBox.confirm
      inst.$alert = ElMessageBox.alert
      inst.$notify = ElNotification
      inst.$loading = ElLoading.service
      // Element Plus tab-click gives paneName / props.name; PHP still reads tab.name.
      const origClick = inst.handleClick
      if (typeof origClick === 'function' && !inst.__epTabPatched) {
        inst.__epTabPatched = true
        inst.handleClick = function (tab: unknown, event?: unknown) {
          const name = phpTabName(tab)
          const phpTab =
            tab && typeof tab === 'object'
              ? new Proxy(tab as object, {
                  get(target, prop, recv) {
                    if (prop === 'name' || prop === 'paneName') return name
                    return Reflect.get(target, prop, recv)
                  },
                })
              : { name, paneName: name }
          try {
            return origClick.call(this, phpTab, event)
          } catch {
            const refs = (this as { $refs?: Record<string, { getList?: () => void; doLayout?: () => void; init?: () => void }> }).$refs
            const hit = name && refs ? refs[name] : undefined
            hit?.getList?.()
            hit?.doLayout?.()
            hit?.init?.()
          }
        }
      }
    },
    mounted() {
      patchTableBodyWrapper(this as unknown as Record<string, unknown>)
    },
    updated() {
      patchTableBodyWrapper(this as unknown as Record<string, unknown>)
    },
  })
})
