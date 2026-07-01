<template>
    <div class="tableDome">
        <div class="moduleTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="200">{{ lc('member_com_00021') }}</th>
                    <th width="460">{{ lc('member_user_00181') }}</th>
                    <th>{{ lc('member_com_00207') }}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('member_com_00343') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input :placeholder="lc('wap_user_00076')" v-model="ruleForm.title">
                                <!-- <span slot="suffix" class="slotspan">天</span> -->
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('member_com_00343') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_yunying_00122') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">

                            <el-input :placeholder="lc('wap_user_00076')" v-model="ruleForm.limit" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_01205') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_01206') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-checkbox-group v-model="ruleForm.rating">
                                <el-checkbox v-for="(item) in qyData" :label="item.id" :key="item.id" size="mini" border>{{ item.name }}</el-checkbox>
                            </el-checkbox-group>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_01207') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect">
                            <el-select v-model="ruleForm.tpl" :placeholder="lc('wap_user_00100')">
                                <el-option v-for="(item, index) in fileData" :key="index" :label="item"
                                    :value="item">
                                </el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_01208') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_system_00263') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.display" label="1">{{ lc('member_com_00287') }}</el-radio>
                            <el-radio v-model="ruleForm.display" label="0">{{ lc('common.close') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_01209') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.com_bm" label="1">{{ lc('member_com_00287') }}</el-radio>
                            <el-radio v-model="ruleForm.com_bm" label="0">{{ lc('common.close') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_01210') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">

                            <el-input :placeholder="lc('wap_user_00076')" v-model="ruleForm.integral" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_01210') }}</span>
                        </div>
                    </td>
                </tr>

                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_01211') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-date-picker v-model="ruleForm.etime" type="date" value-format="yyyy-MM-dd" :placeholder="lc('admin_00346')">
                            </el-date-picker>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>

                <tr>
                    <td>
                        <div class="TableTite">{{ lc('member_com_00022') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">

                            <el-input :placeholder="lc('wap_user_00076')" v-model="ruleForm.sort" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_01212') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_01213') }}</div>
                    </td>
                    <td>
                        <div class="TableUpload" style="display: flex;align-items: center;">
                            <el-upload :accept="pic_accept" :action="uploadAction" :on-change="uploadChangePic"
                                :show-file-list="false">
                                <el-button size="small" type="primary">{{ lc('wap_00540') }}</el-button>
                            </el-upload>
                            <div class="up_sy_logo_div" style="margin-left: 15px;">
                                <el-image v-if="ruleForm.pic" style="width:100px;" :src="ruleForm.pic" :preview-src-list="ruleForm.pic ? [ruleForm.pic] : []"></el-image>
                            </div>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_01214') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_01215') }}</div>
                    </td>
                    <td>
                        <div class="TableUpload" style="display: flex;align-items: center;">
                            <el-upload :accept="pic_accept" :action="uploadAction" :on-change="uploadChangeBackground"
                                :show-file-list="false">
                                <el-button size="small" type="primary">{{ lc('wap_00540') }}</el-button>
                            </el-upload>
                            <div class="up_sy_logo_div" style="margin-left: 15px;">
                                <el-image v-if="ruleForm.background" style="width:100px;" :src="ruleForm.background" :preview-src-list="ruleForm.background ? [ruleForm.background] : []"></el-image>
                            </div>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_01216') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_01217') }}</div>
                    </td>
                    <td>
                        <div class="TableUpload" style="display: flex;align-items: center;">
                            <el-upload :accept="pic_accept" :action="uploadAction" :on-change="uploadChangeWappic"
                                :show-file-list="false">
                                <el-button size="small" type="primary">{{ lc('wap_00540') }}</el-button>
                            </el-upload>
                            <div class="up_sy_logo_div" style="margin-left: 15px;">
                                <el-image v-if="ruleForm.wappic" style="width:100px;" :src="ruleForm.wappic" :preview-src-list="ruleForm.wappic ? [ruleForm.wappic] : []"></el-image>
                            </div>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_01218') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_01219') }}</div>
                    </td>
                    <td>
                        <div class="TableUpload" style="display: flex;align-items: center;">
                            <el-upload :accept="pic_accept" :action="uploadAction" :on-change="uploadChangeWapback"
                                :show-file-list="false">
                                <el-button size="small" type="primary">{{ lc('wap_00540') }}</el-button>
                            </el-upload>
                            <div class="up_sy_logo_div" style="margin-left: 15px;">
                                <el-image v-if="ruleForm.wapback" style="width:100px;" :src="ruleForm.wapback" :preview-src-list="ruleForm.wapback ? [ruleForm.wapback] : []"></el-image>
                            </div>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_01220') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('default_00328') }}</div>
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
                            <span></span>
                        </div>
                    </td>
                </tr>

                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">{{ lc('common.submit') }}</el-button>
        </div>
    </div>
</template>
<script>
let editor = null,editorInterval = null;
const {createEditor, createToolbar} = window.wangEditor;
module.exports = {
    props: {
        id: {type: Number, default: 0},
        fileData: {type: Array, default: []},//专题模板
        qyData: {type: Array, default: []},//企业等级
    },
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            ruleForm: {
                id: 0,
                title: '',//{{ lc('member_com_00343') }}
                limit: "",//{{ lc('admin_yunying_00122') }}
                rating: [],//{{ lc('admin_01206') }}
                tpl: "",//{{ lc('admin_01207') }}
                display: "1",//{{ lc('admin_system_00263') }}
                com_bm: "1",//{{ lc('admin_01209') }}
                integral: '',//integral
                etime: '',//{{ lc('admin_01211') }}
                sort: "",//{{ lc('member_com_00022') }}
                pic: "",//PC{{ lc('admin_00119') }}
                background: '',//PC背景图
                wappic: '',//{{ lc('admin_01217') }}
                wapback: "",//{{ lc('admin_01219') }}
                intro: "",//{{ lc('default_00328') }}
            },
            filePic: {},
            fileBackground: {},
            fileWappic: {},
            fileWapback: {},
            submitLoading: false,
			uploadAction: baseUrl + 'm=common&c=common_upload'
        }
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
        uploadChangePic(file) {
            if (file.status !== 'success') return;
            this.ruleForm.pic = URL.createObjectURL(file.raw);
            // 复刻文件信息
            this.filePic = file;
        },
        uploadChangeBackground(file) {
            if (file.status !== 'success') return;
            this.ruleForm.background = URL.createObjectURL(file.raw);
            // 复刻文件信息
            this.fileBackground = file;
        },
        uploadChangeWappic(file) {
            if (file.status !== 'success') return;
            this.ruleForm.wappic = URL.createObjectURL(file.raw);
            // 复刻文件信息
            this.fileWappic = file;
        },
        uploadChangeWapback(file) {
            if (file.status !== 'success') return;
            this.ruleForm.wapback = URL.createObjectURL(file.raw);
            // 复刻文件信息
            this.fileWapback = file;
        },
        getInfo() {
            let _this = this;
            let params = {id: this.id,add:1};
            httpPost('m=yunying&c=special_special&a=add', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    let info = res.data;
                    for (let index in _this.ruleForm) {
                        if (info.hasOwnProperty(index)) {
                            _this.ruleForm[index] = info[index];
                        }
                    }
                    _this.initEditor(_this.ruleForm.intro);
                } else {
                    message.error(lc('wap_js_00113'));
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        submitForm(formName) {
            let _this = this;
            this.ruleForm.intro = editor.getHtml();
            let params = JSON.parse(JSON.stringify(this.ruleForm));
            if (params.title == '') {
                message.error(lc('admin_01439'));
                return false;
            }
            if (params.tpl == '') {
                message.error(lc('admin_01440'));
                return false;
            }
            delete params.pic;
            delete params.background;
            delete params.wappic;
            delete params.wapback;
            let formData = new FormData();
            Object.keys(params).forEach((key) => {
                if (Array.isArray(params[key])) {
                    params[key].forEach((v) => {
                        formData.append(key + '[]', v);
                    });
                } else {
                    formData.append(key, params[key]);
                }
            });
            if (this.filePic.hasOwnProperty('raw')) {
                formData.append('sl', this.filePic.raw);
            }
            if (this.fileBackground.hasOwnProperty('raw')) {
                formData.append('bg', this.fileBackground.raw);
            }
            if (this.fileWappic.hasOwnProperty('raw')) {
                formData.append('wapsl', this.fileWappic.raw);
            }
            if (this.fileWapback.hasOwnProperty('raw')) {
                formData.append('wapbg', this.fileWapback.raw);
            }
            _this.submitLoading = true;
            httpPost('m=yunying&c=special_special&a=add', formData).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                    _this.$emit("child-event");
                } else {
                    message.error(res.msg);
                }
            }).catch(function (error) {
                console.log(error);
            }).finally(function () {
                _this.submitLoading = false;
            });
        }
    },
    watch: {
        id: {
            handler: function (newValue, oldValue) {
                if (newValue) {
                    this.getInfo();
                }
                this.filePic = {};
                this.fileBackground = {};
                this.fileWappic = {};
                this.fileWapback = {};
            },
            deep: true,
            immediate: true
        },
    }
};
</script>
<style scoped>
.moduleTable {
    max-height: calc(100% - (60px + 20px));
}
</style>