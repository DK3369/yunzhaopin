<template>
    <div class="drawerModlue"  v-loading="addloading" style="display: table;">
        <div class="drawerModInfo">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>模板页面：</span>
                </div>
                <div class="drawerModInpt">
                    {{tpl_n}}
                </div>
            </div>

            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>标题：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input placeholder="请输入标题" v-model="info.title" maxlength="60" show-word-limit></el-input>
                </div>
            </div>

            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>内容：</span>
                </div>
                <div class="drawerModInpt">
                    <div id="editor—wrapper" style="border: 1px solid #ccc;">
                        <div id="toolbar-container"><!-- 工具栏 --></div>
                        <div id="editor-container" style="height: 300px;"><!-- 编辑器 --></div>
                    </div>
				</div>
            </div>

        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="saveinfo" :loading="saveloading">提交</el-button>
        </div>
        <div>
            <table width="100%" class="table_form">
                <tr>
                    <th colspan="2" class="admin_bold_box">
                        <div class="admin_bold">调用说明</div>
                    </th>
                </tr>
                <tr v-for="(item,index) in tpl_temp" :key="index">
                    <th width="150" height="36">{{item}}</th>
                    <td>代码：{{index}}</td>
                </tr>

            </table>
        </div>
    </div>
</template>

<script>
let editor = null,editorInterval = null;
const { createEditor, createToolbar } = window.wangEditor;
module.exports = {
    props: {
        tpl: {
            type: String,
            default: ''
        },
    },
    data: function () {
        return {
            tpl_n:'',
            info:{},
            tpl_temp:[],
            addloading:false,
            saveloading:false,
        }
    },
    created:function(){
        this.getInfo();
    },
	mounted() {
        this.initEditor();
    },
    beforeDestroy() {
        editor = null; 
        editorInterval = null;
    },
    methods: {
        initEditor: function (content=null) {
            clearInterval(editorInterval);
            editorInterval = setInterval(()=>{
                if (editor !== null){
                    clearInterval(editorInterval);
                    if(content!==null){
                        editor.setHtml(content);
                    }
                }else{
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
                }
            },50);
            
        },
        async getInfo() {
            let that = this;
            let params = {
                name:that.tpl
            }

            this.addloading = true;

            httpPost('m=tool&c=emailset&a=gettpl', params).then((result)=>{

                this.addloading = false;

                var res = result.data;
                if (res.error == 0) {
                    that.info = res.data.info;
                    that.initEditor(that.info.content);
                    that.tpl_temp = res.data.tpl_temp;
                    that.tpl_n = res.data.tpl_n;
                }
            }).catch(function(e){
                console.log(e)
            })
        },
        saveinfo: function () {
            var that = this;

            var param = {
                name:that.tpl,
                content:editor.getHtml(),
                title:that.info.title
            };

            this.saveloading = true;

            httpPost('m=tool&c=emailset&a=savetpl', param).then((res)=>{
                if (res.data.error == 0) {
                    message.success(res.data.msg,()=>{
                        this.$emit("close-update");
                    });
                } else {
                    message.error(res.data.msg);
                }
            }).finally(function () {
                setTimeout(function () {
                    that.saveloading = false;
                }, 2000);
            });
        }
    },
};
</script>
<style scoped>
.drawerModInfo::-webkit-scrollbar {
    display: none;
}

</style>