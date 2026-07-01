<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="180">{{ lc('member_com_00021') }}</th>
                        <th width="500">{{ lc('member_user_00181') }}</th>
                        <th>{{ lc('member_com_00207') }}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_01106') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-radio-group v-model="utype" @change="utypeChange">
                                    <el-radio label="1">{{ lc('admin_user_00122') }}</el-radio>
                                    <el-radio label="2">{{ lc('admin_user_00124') }}</el-radio>
                                    <el-radio label="5">{{ lc('admin_system_00206') }}</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01107') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-if="utype == 5">
                        <td>
                            <div class="TableTite">{{ lc('admin_00673') }}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-input v-model="email_user" :placeholder="lc('admin_01109')"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01108') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_yunying_00172') }}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-input v-model="email_title" :placeholder="lc('admin_yunying_00171')"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_yunying_00172') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_00668') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <div id="editor—wrapper" style="border: 1px solid #ccc;">
                                    <div id="toolbar-container"><!-- 工具栏 --></div>
                                    <div id="editor-container" style="height: 300px;"><!-- 编辑器 --></div>
                                </div>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_00668') }}</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="send">{{ lc('resume_00033') }}</el-button>
        </div>
    </div>
</template>

<script>
let editor = null,editorInterval = null;
const { createEditor, createToolbar } = window.wangEditor;
module.exports = {
    props: {},
    data: function () {
        return {
            utype: '',
            email_user: '',
            email_title: '',

            sendLoading: null,
        }
    },

    mounted() {
        this.initEditor();
        clearInterval(editorInterval);
        editorInterval = setInterval(()=>{
            if (editor !== null){
                clearInterval(editorInterval);
            }else{
                this.initEditor();
            }
        },50);
    },
    beforeDestroy() {
        editor = null; 
        editorInterval = null;
    },
    methods: {
        initEditor: function () {
            let editorConfig = {
                MENU_CONF: {
                    uploadImage: {
                        server: baseUrl + 'm=index&c=uploadfile',
                        fieldName: 'file'
                    }
                }
            };
            editor = createEditor({
                selector: '#editor-container',
                html: '',
                config: editorConfig,
                mode: 'simple'
            });
            
            let toolbar = createToolbar({
                editor,
                selector: '#toolbar-container',
                config: {
                    excludeKeys: ['blockquote', 'header1', 'header2', 'header3', '|', 'through', 'todo', '|', 'insertVideo', 'insertTable', 'codeBlock', '|', 'undo', 'redo', '|',]
                },
                mode: 'simple'
            });
        },
        init() {
        },

        utypeChange(val) {
            if (val != 5) {
                this.email_user = '';
            }
        },

        resetData() {
            this.utype = '';
            this.email_user = '';
            this.email_title = '';
            this.sendLoading = null;
            editor.setHtml('');
        },

        send() {
            let that = this,
                utype = that.utype,
                email = that.email_user,
                title = that.email_title,
                content = editor.getHtml();

            if (!utype) {
                message.error(lc('admin_vue_00100'));
                return false;
            }
            if (utype == 5) {
                if (email == '') {
                    message.error(lc('admin_vue_00101'));
                    return false;
                }
            }
            if (title == '') {
                message.error(lc('admin_yunying_00171'));
                return false;
            }
            if (content == '' || content == '<p><br></p>') {
                message.error(lc('admin_vue_00102'));
                return false;
            }
            that.sendDivEmail(utype, title, content, email, 3, 20, 0, 0, 0, lc('admin_yunying_00170'));
        },

        sendDivEmail(utype, title, content, email, status, pagelimit, value, sendok, sendno, msg) {
            let that = this;
            if (status == "3") {
                var pagelimit = 20;

                if (!this.sendLoading) {
                    this.sendLoading = this.$loading({
                        lock: true,
                        text: msg,
                        spinner: 'el-icon-loading',
                        background: 'rgba(0, 0, 0, 0.6)'
                    })
                }

                httpPost('m=yunying&c=yingxiao_tuiguang&a=send', {
                    utype: utype,
                    email_title: title,
                    content: content,
                    email_user: email,
                    pagelimit: pagelimit,
                    value: value,
                    sendok: sendok,
                    sendno: sendno
                }, {hideloading: true}).then(function (response) {
                    let res = response.data,
                        data = res.data;

                    if (res.error == 3) {
                        that.sendDivEmail(utype, title, content, email, res.error, pagelimit, data.value, data.sendok, data.sendno, res.msg);
                    } else if (res.error > 0) {
                        that.sendLoading.close();
                        message.error(res.msg, function () {
                            that.sendLoading = null;
                        });
                    } else {
                        that.sendLoading.close();
                        message.confirm(res.msg, function () {
                            that.resetData();
                        }, '', '', '', false);
                    }
                })
            } else {
                that.sendLoading.close();
                message.confirm(msg, function () {
                    that.resetData();
                }, '', '', '', false);
            }
        },
    },
};
</script>
<style scoped></style>