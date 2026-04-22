<template>
    <div style="overflow: hidden; position: relative; height: 100%;">
        <div class="moduleTable" style="max-height: calc(100% - 80px);">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="200">名称</th>
                        <th width="500">状态</th>
                        <th width="150">说明</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">商品名称</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="请输入内容" v-model="info.name">
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>商品名称</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">商品类别</div>
                        </td>
                        <td>
                            <div class="TableSelect" style="display: flex;align-items: center;">
                                <el-select v-model="info.nid" placeholder="请选择" style="width: 50%;" @change="classchange">
                                    <el-option v-for="item in positionOne" :key="item.id" :label="item.name"
                                        :value="item.id">
                                    </el-option>
                                </el-select>
                                <el-select v-model="info.tnid" placeholder="请选择" style="width: 50%;">
                                    <el-option v-for="item in positionTwo" :key="item.id" :label="item.name" :value="item.id">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>商品类别</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">商品图片</div>
                        </td>
                        <td>
                            <div class="TableUpload">
                                <el-upload class="upload-demo" :auto-upload="false" action="" :show-file-list="false"
                                    :on-change="uploadChange" :accept="pic_accept">
                                    <el-button size="small" type="primary">点击上传</el-button>
                                    <img v-if="info.pic" :src="info.pic" class="avatar" width="70" height="70">
                                </el-upload>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>商品图片</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">兑换{{ integral_pricename }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="请输入内容" v-model="info.integral">
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>兑换{{ integral_pricename }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">限购数量</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="请输入内容" v-model="info.restriction">
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>0为不限</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">库存数量</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="请输入内容" v-model="info.stock">
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>库存数量</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">简介内容</div>
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
                                <span>简介内容</span>
                            </div>
                        </td>
                    </tr>

                    <tr>
                        <td>
                            <div class="TableTite">排序</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="请输入内容" v-model="info.sort" type="number">
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>越小越在前</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">状态</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio v-model="info.status" label="1">上架</el-radio>
                                <el-radio v-model="info.status" label="2">下架</el-radio>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>状态</span>
                            </div>
                        </td>
                    </tr>

                </tbody>
            </table>
        </div>

        <div class="setBasicButn">
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">提交</el-button>
        </div>
    </div>
</template>
    <!-- script -->
<script>
let editor = null,editorInterval = null;
const { createEditor, createToolbar } = window.wangEditor;
module.exports = {
    props: {
		id_p:{
			type: String,
			default: ''
		}
	},
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
			id: '',
            info: { status: '1',nid:'',tnid:'',pic:'' },
            positionOne: [],
            positionTwo: [],
            integral_pricename: '',

            files: [],
            loading: false,
            saveLoading: false,
        }
    },
	mounted() {
        this.initEditor();
        
        if(this.id_p!=''){
            this.id = this.id_p;
        }
        this.getInfo();
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
        save() {
            let that = this;
            let formData = new FormData();
            if (that.info.name == '' || that.info.name == undefined) {
                message.error('请填写商品名称！');
                return false;
            }
            if (that.info.nid == '' || that.info.nid == undefined) {
                message.error('请选择商品类别！');
                return false;
            }
            if (that.info.pic == '' || that.info.pic == undefined) {
                message.error('请上传商品图片！');
                return false;
            }
            if (that.info.integral == '' || that.info.integral == undefined) {
                message.error('请填写兑换' + that.integral_pricename);
                return false;
            }
            if (that.info.restriction == '' || that.info.restriction == undefined) {
                message.error('请填写限购数量！');
                return false;
            }
            if (that.info.stock == '' || that.info.stock == undefined) {
                message.error('请填写库存数量！');
                return false;
            }

            formData.append('name', that.info.name);
            formData.append('nid', that.info.nid);
            formData.append('tnid', that.info.tnid);
            formData.append('integral', that.info.integral);
            formData.append('restriction', that.info.restriction);
            formData.append('stock', that.info.stock);
            formData.append('sort', that.info.sort);
            formData.append('status', that.info.status);
            let content = editor.getHtml();
            formData.append('content', content);
            if (that.files.length !== 0) {
                formData.append('file', that.files);
            }
            if (that.id > 0) {
                formData.append('id', that.id);
            }
            if (that.loading == true) {
                return;
            }
            that.loading = true;
            that.saveLoading = true;
            httpPost('m=yunying&c=shop_reward&a=add', formData).then(function (response) {
                let res = response.data;
                that.saveLoading = false;

                if (res.error > 0) {
                    that.loading = false;
                    message.error(res.msg);
                } else {
                    message.success(res.msg, function () {
                        that.loading = false;
						that.resetForm();
                        that.$emit("child-event");

                    });
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        getInfo() {
            let that = this;
            let params = {};
            if (that.id != '') {
                params.id = that.id;
            }
            params.add=1;
            httpPost('m=yunying&c=shop_reward&a=add', params).then(function (response) {
                let res = response.data;
				if(!$.isEmptyObject(res.data.info)){
                    that.info = res.data.info;
                    if (that.info.content_n !== '') {
                        that.initEditor(that.info.content_n);
                    }
                    if (that.info.nid) {
                        that.getclass(that.info.nid)
                    }
                    if (that.info.status == '0'){
                        that.info.status = '1';
                    }

                }
				that.positionOne = res.data.class;
                that.integral_pricename = res.data.integral_pricename;
            }).catch(function (error) {
                console.log(error);
            });
        },
        classchange() {
            this.info.tnid = '';
            this.getclass(this.info.nid);
        },
        getclass: function (nid) {
            let that = this;
            let params = {};
            if (nid) {
                params.nid = nid;
            }

            httpPost('m=yunying&c=shop_reward&a=getclass', params,{hideloading: true}).then(function (response) {
                let res = response.data,
                    data = res.data;
                that.positionTwo = data.class;
            }).catch(function (error) {
                console.log(error);
            });
        },
        handleClick(tab, event) {
            console.log(tab, event);
        },
        goBack() {
            history.go(-1);
        },
        uploadChange(file,fileList) {

            this.info.pic = URL.createObjectURL(file.raw);
            // 复刻文件信息
            this.files = file.raw;
        },
        resetForm() {
            this.info = { status: '', pic: '' };
            this.initEditor('');
        },
    }
};
</script>
<style scoped>
.TableUpload .el-upload{
    overflow: hidden;
    position: relative;
    display: flex;
    align-items: center;
}
.TableUpload .el-upload .el-button{
    margin-right: 12px;
}
</style>