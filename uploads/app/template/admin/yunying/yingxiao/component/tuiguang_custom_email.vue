<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="180">名称</th>
                        <th width="500">状态</th>
                        <th>说明</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">选择用户</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-radio-group v-model="utype" @change="utypeChange">
                                    <el-radio label="1">个人用户</el-radio>
                                    <el-radio label="2">企业用户</el-radio>
                                    <el-radio label="5">自定义用户</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>注：全部用户发送，时间较长，建议分批发送</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-if="utype == 5">
                        <td>
                            <div class="TableTite">用户邮箱</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-input v-model="email_user" placeholder="请输入用户邮箱"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>多个邮箱请用,(半角)隔开</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">邮件主题</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-input v-model="email_title" placeholder="请输入邮件主题"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>邮件主题</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">邮件内容</div>
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
                                <span>邮件内容</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="send">发送</el-button>
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
                message.error('请选择用户类型');
                return false;
            }
            if (utype == 5) {
                if (email == '') {
                    message.error('请输入自定义邮箱');
                    return false;
                }
            }
            if (title == '') {
                message.error('请输入邮件主题');
                return false;
            }
            if (content == '' || content == '<p><br></p>') {
                message.error('请输入邮件内容');
                return false;
            }
            that.sendDivEmail(utype, title, content, email, 3, 20, 0, 0, 0, "正在发送，请稍候。。。");
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