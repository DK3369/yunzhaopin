export default defineNitroPlugin((nitroApp) => {
  nitroApp.hooks.hook('render:html', (html) => {
    const patch = (s: string) =>
      s.replace(/<script(?![^>]*\bdata-cfasync=)/gi, '<script data-cfasync="false"')
    html.head = html.head.map(patch)
    html.bodyPrepend = html.bodyPrepend.map(patch)
    html.body = html.body.map(patch)
    html.bodyAppend = html.bodyAppend.map(patch)
  })
})
