<template>
    <div style="margin: 0 20px; overflow: hidden; position: relative; height: 100%;">
        <div class="moduleTable" style="max-height: calc(100% - 80px);">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="200">{yun:}t key='member_com_00021'{/yun}</th>
                    <th width="400">{yun:}t key='member_user_00181'{/yun}</th>
                    <th>{yun:}t key='member_com_00207'{/yun}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_00821'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input placeholder="{yun:}t key='admin_00822'{/yun}" v-model="info.title">
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_00822'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_00283'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="info.sid" placeholder="{yun:}t key='wap_user_00100'{/yun}" @change="cdChange">
                                <el-option v-for="item in spaces" :key="item.id" :label="item.name" :value="item.id">
                                </el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_00283'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_00286'{/yun}</div>
                    </td>
                    <td>
                        <el-cascader v-model="reserved_arr" placeholder="{yun:}t key='admin_00825'{/yun}" :options="zwarr" :props="{ multiple: true }"></el-cascader>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <el-button type="primary" icon="el-icon-document-add" @click="addLocal">{yun:}t key='admin_00823'{/yun}</el-button>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_00126'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="info.did" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                <el-option v-for="(item,index) in dnames" :key="index" :label="item" :value="index"></el-option>
                            </el-select>
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
                        <div class="TableTite">{yun:}t key='wap_user_00087'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-date-picker
                                    v-model="info.starttime"
                                    type="datetime"
                                    style="width: 100%;"
                                    value-format="yyyy-MM-dd HH:mm:ss"
                                    placeholder="{yun:}t key='admin_user_company_00055'{/yun}">
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
                        <div class="TableTite">{yun:}t key='wap_user_00096'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-date-picker
                                    v-model="info.endtime"
                                    type="datetime"
                                    style="width: 100%;"
                                    value-format="yyyy-MM-dd HH:mm:ss"
                                    placeholder="{yun:}t key='admin_user_company_00055'{/yun}">
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
                        <div class="TableTite">{yun:}t key='admin_00282'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input placeholder="{yun:}t key='admin_00826'{/yun}" v-model="info.address">
                            </el-input>
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
                        <div class="TableTite">{yun:}t key='admin_00284'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input type="textarea" placeholder="{yun:}t key='admin_00827'{/yun}" v-model="info.traffic">
                            </el-input>
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
                        <div class="TableTite">{yun:}t key='wap_user_00265'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input placeholder="{yun:}t key='wap_com_00322'{/yun}" v-model="info.phone">
                            </el-input>
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
                        <div class="TableTite">{yun:}t key='admin_00287'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input placeholder="{yun:}t key='admin_00828'{/yun}" v-model="info.organizers">
                            </el-input>
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
                        <div class="TableTite">{yun:}t key='wap_01431'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input placeholder="{yun:}t key='wap_com_00013'{/yun}" v-model="info.user">
                            </el-input>
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
                        <div class="TableTite">{yun:}t key='admin_00285'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-radio v-model="info.is_open" label="1">{yun:}t key='member_com_00287'{/yun}</el-radio>
                            <el-radio v-model="info.is_open" label="0">{yun:}t key='admin_user_00259'{/yun}</el-radio>
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
                        <div class="TableTite">{yun:}t key='admin_00280'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-upload class="avatar-uploader" :action="uploadAction" :show-file-list="false" :on-change="thumbChange"
                                       :accept="pic_accept">
                                <img style="width:200px;" v-if="info.is_themb_n" :src="info.is_themb_n" class="avatar">
                                <el-button v-else type="primary" icon="el-icon-document-add">{yun:}t key='wap_00540'{/yun}</el-button>
                            </el-upload>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span><i class="el-icon-warning"></i>{yun:}t key='admin_00276'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_00824'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-upload class="avatar-uploader" :action="uploadAction" :show-file-list="false" :accept="pic_accept" :on-change="bannerChange">
                                <img style="width:200px;" v-if="info.banner_n" :src="info.banner_n" class="avatar">
                                <el-button v-else type="primary" icon="el-icon-document-add">{yun:}t key='wap_00540'{/yun}</el-button>
                            </el-upload>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span><i class="el-icon-warning"></i>{yun:}t key='admin_00275'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_00279'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-upload class="avatar-uploader" :action="uploadAction" :show-file-list="false" :accept="pic_accept" :on-change="thembwapChange">
                                <img style="width:200px;" v-if="info.is_themb_wap_n" :src="info.is_themb_wap_n" class="avatar">
                                <el-button v-else type="primary" icon="el-icon-document-add">{yun:}t key='wap_00540'{/yun}</el-button>
                            </el-upload>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span><i class="el-icon-warning"></i>{yun:}t key='admin_00277'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_00281'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-upload class="avatar-uploader" :action="uploadAction"
                                       :accept="pic_accept"
                                       :show-file-list="false" :on-change="bannerwapChange">
                                <img style="width:200px;" v-if="info.banner_wap_n" :src="info.banner_wap_n" class="avatar">
                                <el-button v-else type="primary" icon="el-icon-document-add">{yun:}t key='wap_00540'{/yun}</el-button>
                            </el-upload>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span><i class="el-icon-warning"></i>{yun:}t key='admin_00278'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='wap_00566'{/yun}</div>
                    </td>
                    <td colspan="2">
                        <div style="border: 1px solid #ccc;">
                            <div id="toolbar-container-desc"><!-- 工具栏 --></div>
                            <div id="editor-container-desc" style="height: 300px;"><!-- 编辑器 --></div>
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
                        <div class="TableTite">{yun:}t key='wap_00569'{/yun}</div>
                    </td>
                    <td colspan="2">
                        <div style="border: 1px solid #ccc;">
                            <div id="toolbar-container-mt"><!-- 工具栏 --></div>
                            <div id="editor-container-mt" style="height: 300px;"><!-- 编辑器 --></div>
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
                        <div class="TableTite">{yun:}t key='default_00344'{/yun}</div>
                    </td>
                    <td colspan="2">
                        <div style="border: 1px solid #ccc;">
                            <div id="toolbar-container-fw"><!-- 工具栏 --></div>
                            <div id="editor-container-fw" style="height: 300px;"><!-- 编辑器 --></div>
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
                        <div class="TableTite">{yun:}t key='wap_00564'{/yun}</div>
                    </td>
                    <td colspan="2">
                        <div style="border: 1px solid #ccc;">
                            <div id="toolbar-container-sz"><!-- 工具栏 --></div>
                            <div id="editor-container-sz" style="height: 300px;"><!-- 编辑器 --></div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='wap_00568'{/yun}</div>
                    </td>
                    <td colspan="2">
                        <div id="editor—wrapper" style="border: 1px solid #ccc;">
                            <div id="toolbar-container-cy"><!-- 工具栏 --></div>
                            <div id="editor-container-cy" style="height: 300px;"><!-- 编辑器 --></div>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save" :disabled="submitLoading">{yun:}t key='common.submit'{/yun}</el-button>
        </div>
    </div>
</template>
<script>
    let editor_desc = null, editor_mt = null, editor_fw = null, editor_sz = null, editor_cy = null,
        toolbar_desc = null, toolbar_mt = null, toolbar_fw = null, toolbar_sz = null, toolbar_cy = null,
        editorInterval_desc = null,editorInterval_mt = null,editorInterval_fw = null,editorInterval_sz = null,editorInterval_cy = null;
    const {createEditor, createToolbar} = window.wangEditor;
    module.exports = {
        props: {
            cdata: {
                type: Object,
                default: ''
            },
            spacearr: {
                type: Array,
                default: []
            },
            dnamearr: {
                type: Object,
                default: {}
            }
        },
        data: function () {
            return {
                pic_accept: localStorage.getItem("pic_accept"),
                info: {
                    is_open:'1'
                },
                spaces: [],
                dnames: {},
                thumblist: [],
                bannerlist: [],
                wapthumblist: [],
                wapbannerlist: [],
                zwarr: [],
                reserved_arr: [],
                submitLoading:false,
                uploadAction: baseUrl + 'm=common&c=common_upload'
            }
        },
        watch: {
            cdata: {
                handler(val, oldVal) {
                    this.info = val;
                    if (val.id==''){
                        this.info.is_open = '1';
                    }
                },
                immediate: true,
                deep: true,
            },
            spacearr: {
                handler(val, oldVal) {
                    this.spaces = val;
                },
                immediate: true,
                deep: true,
            },
            dnamearr: {
                handler(val, oldVal) {
                    this.dnames = val;
                },
                immediate: true,
                deep: true,
            }
        },
        beforeDestroy() {
            editor_desc = null; editor_mt = null; editor_fw = null; editor_sz = null; editor_cy = null;
            toolbar_desc = null; toolbar_mt = null; toolbar_fw = null; toolbar_sz = null; toolbar_cy = null;
            editorInterval_desc = null;editorInterval_mt = null;editorInterval_fw = null;editorInterval_sz = null;editorInterval_cy = null;
        },
        methods: {
            cdChange(val){
                let that = this;
                let params = {
                    sid: val,
                    zid: that.info.id
                };
                httpPost('m=neirong&c=zhaopinhui&a=getzhanwei", params).then(function (response) {
                    if (response.data.error == 0) {
                        that.zwarr = response.data.data.space;
                        that.reserved_arr = response.data.data.reserved_arr;
                    } else {
                        message.error(response.data.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                })
            },
            // pc{yun:}t key='admin_00119'{/yun}
            thumbChange(file) {
                var tmp = deepClone(this.info)
                // 预览文件处理
                tmp.is_themb_n = URL.createObjectURL(file.raw);
                // 复刻文件信息
                this.thumblist[0] = file.raw;
                this.info = tmp
            },
            //PC{yun:}t key='member_com_00077'{/yun}
            bannerChange(file){
                var tmp = deepClone(this.info)
                // 预览文件处理
                tmp.banner_n = URL.createObjectURL(file.raw);
                // 复刻文件信息
                this.bannerlist[0] = file.raw;
                this.info = tmp
            },
            // wap{yun:}t key='admin_00119'{/yun}
            thembwapChange(file) {
                var tmp = deepClone(this.info)
                // 预览文件处理
                tmp.is_themb_wap_n = URL.createObjectURL(file.raw);
                // 复刻文件信息
                this.wapthumblist[0] = file.raw;
                this.info = tmp
            },
            // wap{yun:}t key='member_com_00077'{/yun}
            bannerwapChange(file){
                var tmp = deepClone(this.info)
                // 预览文件处理
                tmp.banner_wap_n = URL.createObjectURL(file.raw);
                // 复刻文件信息
                this.wapbannerlist[0] = file.raw;
                this.info = tmp
            },
            initEditor() {
                
                var that = this
                
                let editorConfig = {
                    MENU_CONF: {
                        uploadImage: {
                            server: baseUrl + "m=index&c=uploadfile',
                            fieldName: 'file'
                        }
                    }
                };
                clearInterval(editorInterval_desc);
                editorInterval_desc = setInterval(()=>{
                    if (editor_desc !== null){
                        clearInterval(editorInterval_desc);
                        if (that.info.body) {
                            
                            editor_desc.setHtml(that.info.body);
                        } else {
                            editor_desc.setHtml('");
                        }
                    }else{
                        // {yun:}t key='wap_00566'{/yun}
                        if (!editor_desc) {
                            editor_desc = createEditor({
                                selector: "#editor-container-desc',
                                html: '',
                                config: editorConfig,
                                mode: 'simple'
                            });
                        }
                        if (!toolbar_desc) {
                            toolbar_desc = createToolbar({
                                editor: editor_desc,
                                selector: '#toolbar-container-desc',
                                config: {
                                    excludeKeys: ['blockquote', 'header1', 'header2', 'header3', '|', 'through', 'todo', '|', 'insertVideo', 'insertTable', 'codeBlock', '|', 'undo', 'redo', '|',]
                                },
                                mode: 'simple"
                            });
                        }
                    }
                },300);
                
                
                // {yun:}t key='wap_00569'{/yun}
                clearInterval(editorInterval_mt);
                editorInterval_mt = setInterval(()=>{
                    if (editor_mt !== null){
                        clearInterval(editorInterval_mt);
                        if (that.info.media) {
                            editor_mt.setHtml(that.info.media);
                        } else {
                            editor_mt.setHtml("');
                        }
                    }else{
                        if (!editor_mt) {
                            editor_mt = createEditor({
                                selector: '#editor-container-mt',
                                html: '',
                                config: editorConfig,
                                mode: 'simple'
                            });
                        }
                        if (!toolbar_mt) {
                            toolbar_mt = createToolbar({
                                editor: editor_mt,
                                selector: '#toolbar-container-mt',
                                config: {
                                    excludeKeys: ['blockquote', 'header1', 'header2', 'header3', '|', 'through', 'todo', '|', 'insertVideo', 'insertTable', 'codeBlock', '|', 'undo', 'redo', '|',]
                                },
                                mode: 'simple"
                            });
                        }
                        
                    }
                },300);
                
                
                // {yun:}t key='default_00344'{/yun}
                clearInterval(editorInterval_fw);
                editorInterval_fw = setInterval(()=>{
                    if (editor_fw !== null){
                        clearInterval(editorInterval_fw);
                        if (that.info.packages) {
                            editor_fw.setHtml(that.info.packages);
                        } else {
                            editor_fw.setHtml("');
                        }
                    }else{
                        if (!editor_fw) {
                            editor_fw = createEditor({
                                selector: '#editor-container-fw',
                                html: '',
                                config: editorConfig,
                                mode: 'simple'
                            });
                        }
                        if (!toolbar_fw) {
                            toolbar_fw = createToolbar({
                                editor: editor_fw,
                                selector: '#toolbar-container-fw',
                                config: {
                                    excludeKeys: ['blockquote', 'header1', 'header2', 'header3', '|', 'through', 'todo', '|', 'insertVideo', 'insertTable', 'codeBlock', '|', 'undo', 'redo', '|',]
                                },
                                mode: 'simple"
                            });
                        }
                        
                    }
                },300);
                
                // {yun:}t key='wap_00564'{/yun}
                clearInterval(editorInterval_sz);
                editorInterval_sz = setInterval(()=>{
                    if (editor_sz !== null){
                        clearInterval(editorInterval_sz);
                        if (that.info.booth) {
                            editor_sz.setHtml(that.info.booth);
                        } else {
                            editor_sz.setHtml("');
                        }
                    }else{
                        if (!editor_sz) {
                            editor_sz = createEditor({
                                selector: '#editor-container-sz',
                                html: '',
                                config: editorConfig,
                                mode: 'simple'
                            });
                        }
                        if (!toolbar_sz) {
                            toolbar_sz = createToolbar({
                                editor: editor_sz,
                                selector: '#toolbar-container-sz',
                                config: {
                                    excludeKeys: ['blockquote', 'header1', 'header2', 'header3', '|', 'through', 'todo', '|', 'insertVideo', 'insertTable', 'codeBlock', '|', 'undo', 'redo', '|',]
                                },
                                mode: 'simple"
                            });
                        }
                        
                    }
                },300);
                
                
                // {yun:}t key='wap_00568'{/yun}
                clearInterval(editorInterval_cy);
                editorInterval_cy = setInterval(()=>{
                    if (editor_cy !== null){
                        clearInterval(editorInterval_cy);
                        if (that.info.participate) {
                            editor_cy.setHtml(that.info.participate);
                        } else {
                            editor_cy.setHtml("');
                        }
                    }else{
                        if (!editor_cy) {
                            editor_cy = createEditor({
                                selector: '#editor-container-cy',
                                html: '',
                                config: editorConfig,
                                mode: 'simple'
                            });
                        }
                        if (!toolbar_cy) {
                            toolbar_cy = createToolbar({
                                editor: editor_cy,
                                selector: '#toolbar-container-cy',
                                config: {
                                    excludeKeys: ['blockquote', 'header1', 'header2', 'header3', '|', 'through', 'todo', '|', 'insertVideo', 'insertTable', 'codeBlock', '|', 'undo', 'redo', '|',]
                                },
                                mode: 'simple'
                            });
                        }
                        
                    }
                },300);
                
            },
            async save() {
                let that = this;
                if (that.info.title == '') {
                    message.error('请填写招聘会名称');
                    return false;
                }
                that.info.body = editor_desc.getHtml() != '<p><br></p>' ? editor_desc.getHtml() : '';
                that.info.media = editor_mt.getHtml()  != '<p><br></p>' ? editor_mt.getHtml() : '';
                that.info.packages = editor_fw.getHtml() != '<p><br></p>' ? editor_fw.getHtml() : '';
                that.info.booth = editor_sz.getHtml() != '<p><br></p>' ? editor_sz.getHtml() : '';
                that.info.participate = editor_cy.getHtml() != '<p><br></p>' ? editor_cy.getHtml() : '';

                if (that.thumblist.length) {
                    that.info.sl = this.thumblist[0];
                }
                if (that.bannerlist.length) {
                    that.info.hf = this.bannerlist[0];
                }
                if (that.wapthumblist.length) {
                    that.info.wapsl = this.wapthumblist[0];
                }
                if (that.wapbannerlist.length) {
                    that.info.waphf = this.wapbannerlist[0];
                }
                that.info.reserved_arr = JSON.stringify(that.reserved_arr);
                that.info.submit = true;
                let params = that.info;
                var formData = new FormData();
                Object.keys(params).forEach((key) => {
                    if (Array.isArray(params[key])) {
                        params[key].forEach((v) => {
                            formData.append(key + '[]', v);
                        });
                    } else {
                        formData.append(key, params[key]);
                    }
                });

                this.submitLoading = true;
                httpPost('m=neirong&c=zhaopinhui&a=add', formData).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        message.success(res.msg);
                        that.thumblist = [];
                        that.bannerlist = [];
                        that.wapthumblist = [];
                        that.wapbannerlist = [];
                        that.$emit("child-event-list");
                    } else {
                        message.error(res.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                }).finally(function () {
                    that.submitLoading = false;
                });
            },
            addLocal:function () {
                let sendData = {type:'cd'};

                this.$emit('change',sendData);

            }
        },
    };
</script>
<style scoped>
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
</style>