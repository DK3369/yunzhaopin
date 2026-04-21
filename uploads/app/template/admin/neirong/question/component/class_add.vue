<template>
    <div class="drawerModlue">
        <div class="drawerModInfo" style="max-height: calc(100% - 80px); overflow-y: auto; border: none;">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>类别名称</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.name" placeholder="请输入类别名称"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>排序</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.sort" placeholder=""
                              @input="inputIntNumber($event, 'ruleForm', 'sort')"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="越小越在前" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>父类别</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="ruleForm.pid" placeholder="请选择" clearable>
                        <el-option v-for="item in classList" :key="item.id" :label="item.name" :value="item.id">
                        </el-option>
                    </el-select>
                </div>
            </div>
            <div class="drawerModLis" style="align-items: initial;">
                <div class="drawerModTite">
                    <span>简介内容</span>
                </div>
                <div class="drawerModInpt">
                    <div id="editor—wrapper" style="border: 1px solid #ccc;">
                        <div :id="'toolbar-container' + source"><!-- 工具栏 --></div>
                        <div :id="'editor-container' + source" style="height: 300px;"><!-- 编辑器 --></div>
                    </div>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>图片</span>
                </div>
                <div class="drawerModInpt">
                    <el-upload class="avatar-uploader" list-type="picture" action="" :auto-upload="false"
                               :on-change="handleChangePic" :show-file-list="false" :accept="pic_accept">
                        <img v-if="ruleForm.pic_n" :src="ruleForm.pic_n" class="avatar">
                        <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                    </el-upload>
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
    props: {
        refresh: {type: [Number,String], default: ''}, // 刷新参数，此参数用来触发每次打开均加载
        id: {type: [Number,String], default: ''},
        source: {type: String, default: ''}
    },
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),

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
					    selector: '#editor-container' + this.source,
					    html: '',
					    config: editorConfig,
					    mode: 'simple'
					});
					
					let toolbar = createToolbar({
					    editor,
					    selector: '#toolbar-container' + this.source,
					    config: {
					        excludeKeys: ['blockquote', 'header1', 'header2', 'header3', '|', 'through', 'todo', '|', 'insertVideo', 'insertTable', 'codeBlock', '|', 'undo', 'redo', '|',]
					    },
					    mode: 'simple'
					});
				}
			},300);
			
		},
        getInfo() {
            let that = this;

            httpPost('m=neirong&c=question_class&a=add', {id: that.id ? that.id : ''}).then(function (response) {
                let res = response.data,
                    data = res.data,
                    info = data.info;

                that.classList = data.classList;
                if (that.id) {
                    that.ruleForm = {
                        id: info.id,
                        name: info.name,
                        pid: info.pid && info.pid > 0 ? info.pid : '',
                        sort: info.sort,
                        pic_n: info.pic
                    };

                    that.initEditor(info.intro);
                } else {
                    that.ruleForm = {};
                }
            })
        },

        inputIntNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9]/g,'');
        },

        // 上传时触发
        handleChangePic(file, fileList) {
            this.$set(this.ruleForm, 'pic', file.raw);
            this.$set(this.ruleForm, 'pic_n', file.url);
        },

        save() {
            let that = this,
                ruleForm = that.ruleForm,
                content = editor.getHtml(),
                formData = new FormData();

            if (typeof ruleForm.name == 'undefined' || ruleForm.name == '') {
                message.warning('请输入类别名称');
                return;
            }
            if (typeof ruleForm.pic_n == 'undefined' || ruleForm.pic_n == '') {
                message.warning('请上传图片');
                return;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            $.each(ruleForm, function (key, value) {
                if (key != 'pic_n') {
                    formData.append(key, value);
                }
            });

            formData.append('intro', content);

            httpPost('m=neirong&c=question_class&a=save', formData).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg, function () {
                        that.saveLoading = false;
                    });
                } else {
                    if (custoapp.pid > 0 && typeof ruleForm.pid !== 'undefined' && ruleForm.pid > 0) {
                        custoapp.pid = ruleForm.pid;
                    }
                    that.$emit("child-event");
                    if (typeof ruleForm.id === 'undefined') {
                        that.resetData();
                    }
                    message.success(res.msg, function () {
                        that.saveLoading = false;
                    });
                }
            })
        },

        resetData() {
            this.ruleForm = {};
            this.initEditor('');
        },
    },
    watch: {
        refresh: function (val, oldVal) {
            this.resetData();
            this.getInfo();
        },
    }

};
</script>
<style scoped>
    /* 上传样式开始 */
    .avatar-uploader .el-upload {
        border: 1px dashed #d9d9d9;
        border-radius: 6px;
        cursor: pointer;
        position: relative;
        overflow: hidden;
    }

    .avatar-uploader .el-upload:hover {
        border-color: #409EFF;
    }

    .avatar-uploader-icon {
        font-size: 28px;
        color: #8c939d;
        width: 148px;
        height: 148px;
        line-height: 148px;
        text-align: center;
    }

    .avatar {
        width: 148px;
        height: 148px;
        display: block;
    }
    /* 上传样式结束 */
</style>