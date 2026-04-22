<template>
    <div class="drawerModlue">
        <div class="drawerModInfo" style="max-height: calc(100% - 80px); overflow-y: auto; border: none;">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>问答标题</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.title" placeholder="请输入问答标题"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>问答类别</span>
                </div>
                <div class="drawerModInpt">
                    <el-cascader v-model="ruleForm.cid" :options="classList" :show-all-levels="false"
                                 :props="{value: 'id', label: 'name', emitPath: false}">
                    </el-cascader>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>访问次数</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.visit" placeholder="请输入访问次数"
                              @input="inputIntNumber($event, 'ruleForm', 'visit')"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>是否推荐</span>
                </div>
                <div class="drawerModInpt">
                    <el-switch v-model="ruleForm.is_recom" active-value="1" inactive-value="0">
                    </el-switch>
                </div>
            </div>
            <div class="drawerModLis" style="align-items: initial;">
                <div class="drawerModTite">
                    <span>问答内容</span>
                </div>
                <div class="drawerModInpt">
                    <div id="editor—wrapper" style="border: 1px solid #ccc;">
                        <div id="toolbar-container"><!-- 工具栏 --></div>
                        <div id="editor-container" style="height: 300px;"><!-- 编辑器 --></div>
                    </div>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border-top: 0px solid #E4E7ED;">
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">提交</el-button>
        </div>
    </div>
</template>

<script>
let editor = null,editorInterval = null;
const { createEditor, createToolbar } = window.wangEditor;
module.exports = {
    props: ['id'],
    data: function () {
        return {
            classList: [],

            ruleForm: {},

            saveLoading: false,
        }
    },
    mounted() {
        
        this.initEditor();
    },
    beforeDestroy() {
        editor = null; 
        editorInterval = null;
    },
    created: function () {
        this.getInfo();
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
                            'toolbarKeys': [
                                "bold",
                                "underline",
                                "italic",
                                "clearStyle",
                                "|",
                                "bulletedList",
                                "numberedList",
                                "justifyLeft",
                                "justifyRight",
                                "justifyCenter"
                            ]
                        },
                        mode: 'simple'
                    });
                }
            },300);
            
        },
        getInfo() {
            let that = this;

            httpPost('m=neirong&c=question&a=add', {id: that.id ? that.id : ''}).then(function (response) {
                let res = response.data,
                    data = res.data,
                    info = data.info;

                that.classList = data.classList;
                if (that.id) {
                    that.ruleForm = {
                        id: info.id,
                        title: info.title,
                        cid: info.cid,
                        visit: info.visit,
                        is_recom: info.is_recom
                    };

                    that.initEditor(info.content);
                } else {
                    that.ruleForm = {};
                }
            })
        },

        inputIntNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9]/g,'');
        },

        save() {
            let that = this,
                params = that.ruleForm,
                content = editor.getHtml();

            if (typeof params.title == 'undefined' || params.title == '') {
                message.warning('请输入问答标题');
                return;
            }
            if (typeof params.cid == 'undefined' || params.cid == '') {
                message.warning('请选择问答类别');
                return;
            }

            // if (content == '' || content == '<p><br></p>') {
            //     message.warning('请输入问答内容');
            //     return false;
            // }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            params.content = content;

            httpPost('m=neirong&c=question&a=save', params).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg, function() {
                        that.saveLoading = false;
                    });
                } else {
                    that.$emit("child-event");
                    message.success(res.msg, function() {
                        that.saveLoading = false;
                    });
                }
            })
        },
    },
    watch: {
        id: function (val, oldVal) {
            this.ruleForm = {};
            this.initEditor('');
            this.getInfo();
        },
    }
};
</script>
<style scoped></style>