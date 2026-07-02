const utilFile = {
    // Create an anchor tag and trigger download
    downloadFile: function (blob, fileName) {
        const link = document.createElement('a')
        link.href = window.URL.createObjectURL(blob)
        link.download = fileName
        // This approach is compatible with Firefox
        document.body.appendChild(link)
        const evt = document.createEvent('MouseEvents')
        evt.initEvent('click', false, false)
        link.dispatchEvent(evt)
        document.body.removeChild(link)
    },
    // Convert a Base64 file to Blob
    buildBlobByByte: function (data) {
        const raw = window.atob(data)
        const rawLength = raw.length
        const uInt8Array = new Uint8Array(rawLength)
        for (let i = 0; i < rawLength; ++i) {
            uInt8Array[i] = raw.charCodeAt(i)
        }
        return new Blob([uInt8Array])
    },
    // Generate a file from a byte array
    downloadFileByByte: function (data, fileName) {
        const blob = this.buildBlobByByte(data)
        this.downloadFile(blob, fileName)
    }
}
