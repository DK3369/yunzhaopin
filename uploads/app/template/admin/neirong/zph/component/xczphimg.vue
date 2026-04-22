<template>
    <div class="moduleElHight" style="padding: 0 20px;">
        <!-- <div class="tableDome_tip">
            <el-alert title="可添加招聘会时的参会图片" type="success" :closable="false">
            </el-alert>
        </div> -->
        <div class="modulemoreSeach" style="width: 100%;">
            <div class="moduleSeachleft">
            </div>
            <div class="moduleHeadrButn" style="margin-bottom: 12px;margin-left: 10px;">
                <el-button type="primary" icon="el-icon-document-add" size="mini" @click="addPic">添加图片</el-button>
            </div>
        </div>
        <div class="moduleElTable">
            <el-table :data="tableData" height="100%" style="border: 1px solid #eee;" stripe
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" v-loading="loading"
                :empty-text="emptytext">
                <el-table-column prop="id" label="ID"></el-table-column>
                <el-table-column prop="wenjian" label="图片" width="150">
                    <template slot-scope="scope">
                        <div class="demo-image__preview">
                            <el-image width="100%" height="60px" :src="scope.row.pic_n" :preview-src-list="srcList">
                            </el-image>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="title" label="图片名称">
                </el-table-column>
                <el-table-column prop="sort" label="排序">
                </el-table-column>
                <el-table-column fixed="right" label="操作" width="250">
                    <template slot-scope="scope">
                        <div class="moduleElTaCaoz">
                            <el-button size="mini" @click="editPic(scope.row)">修改</el-button>
                            <el-button type="danger" size="mini" @click="delrow(scope.row.id)">删除</el-button>
                            <el-button v-if="scope.row.is_themb == '0'" size="mini"
                                @click="thumb(scope.row.id)">设为缩略图</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modluDrawer">
            <el-drawer :title="info.id ? '修改图片' : '添加图片'" :visible.sync="editBox" append-to-body
                :modal-append-to-body="false" :show-close="true" :with-header="true" size="45%">
                <div class="drawerModlue">
                    <div class="drawerModInfo">
                        <div class="drawerModLis">
                            <div class="drawerModTite">
                                <span>图片名称</span>
                            </div>
                            <div class="drawerModInpt">
                                <el-input v-model="info.title"></el-input>
                            </div>
                        </div>
                        <div class="drawerModLis">
                            <div class="drawerModTite">
                                <span>招聘会图片</span>
                            </div>
                            <div class="drawerModInpt">
                                <el-upload :accept="pic_accept" class="avatar-uploader" :action="uploadAction"
                                    :show-file-list="false" :on-change="uploadChange">
                                    <img style="width:200px;" v-if="info.pic_n" :src="info.pic_n" class="avatar">
                                    <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                                </el-upload>
                            </div>
                        </div>
                        <div class="drawerModLis">
                            <div class="drawerModTite">
                                <span>排序</span>
                            </div>
                            <div class="drawerModInpt">
                                <el-input v-model="info.sort"
                                    onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"></el-input>
                            </div>
                        </div>

                    </div>
                    <div class="setBasicButn" style="border: none;">
                        <el-button type="primary" size="medium" @click="save" :disabled="submitLoading">提交</el-button>
                    </div>
                </div>
            </el-drawer>
        </div>
        <div class="modluDrawer">
            <el-dialog title="信息" :visible.sync="delTplBox" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="30%">
                <span>确定要删除？</span>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="delTplBox = false">取 消</el-button>
                    <el-button type="primary" @click="delTplSubmit">确 定</el-button>
                </span>
            </el-dialog>
        </div>
    </div>
</template>
<script>
module.exports = {
    props: {
        zphid: {
            type: String,
            default: ''
        },
    },
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            emptytext: '暂无数据',
            loading: false,
            submitLoading: false,
            zph_id: '',
            tableData: [],
            srcList: [],
            info: {
                title: '',
                pic_n: '',
                sort: '',
                id: ''
            },
            files: [],
            editBox: false,
            tplid: '',
            delTplBox: false,
            uploadAction: baseUrl + 'm=common&c=common_upload'
        }
    },
    watch: {
        zphid: {
            handler(val, oldVal) {
                this.zph_id = val;
            },
            immediate: true,
            deep: true,
        }
    },
    created() {
        this.getList();
    },
    methods: {
        addPic() {
            this.info.title = '';
            this.info.pic_n = '';
            this.info.sort = '';
            this.info.id = '';
            this.editBox = true;
        },
        delrow(id) {
            delConfirm(this, id, this.delete);
        },
        async delete(id) {
            let that = this;
            let params = {
                del: id
            };
            httpPost('m=neirong&c=zhaopinhui&a=delpic', params).then(function (response) {
                if (response.data.error == 0) {
                    message.success(response.data.msg);
                    that.getList();
                } else {
                    message.error(response.data.msg);
                }
            }).catch(function (error) {
                console.log(error);
            })
        },
        async thumb(id) {
            let that = this;
            httpPost('m=neirong&c=zhaopinhui&a=setthemb', { id: id }).then(function (response) {
                if (response.data.error == 0) {
                    message.success(response.data.msg, function () {
                        that.getList();
                    });
                } else {
                    message.error(response.data.msg);
                }
            }).catch(function (error) {
                console.log(error);
            })
        },
        async delTplSubmit() {
            let that = this;
            if (that.tplid == '') {
                message.error('请选择要删除的模板');
                return false;
            }
            httpPost('m=system&c=set_tplset&a=comtpldel', { id: that.tplid }).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    that.delTplBox = false;
                    message.success(res.msg, function () {
                        that.getList();
                    });
                } else {
                    message.error(res.msg);
                }
            }).catch(function (error) {
                console.log(error)
            })
        },
        async save() {
            let that = this;
            let formData = new FormData();
            if (that.info.title == '') {
                message.error('请填写图片名称');
                return false;
            }
            if (that.info.pic_n == '') {
                message.error('请上传图片');
                return false;
            }
            formData.append('title', that.info.title);
            formData.append('sort', that.info.sort);
            if (that.files.length !== 0) {
                formData.append('file', that.files);
            }
            formData.append('id', that.info.id);
            formData.append('zph_id', that.zph_id)
            this.submitLoading = true;
            httpPost('m=neirong&c=zhaopinhui&a=uploadsave', formData).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    that.editBox = false;
                    message.success(res.msg, function () {
                        that.getList();
                    });
                } else {
                    message.error(res.msg);
                }
            }).catch(function (error) {
                console.log(error);
            }).finally(function () {
                that.submitLoading = false;
            });
        },
        editPic(row) {
            this.info.title = row.title;
            this.info.pic_n = row.pic_n;
            this.info.sort = row.sort;
            this.info.id = row.id;
            this.editBox = true;
        },
        async getList() {
            let that = this;
            let param = { id: that.zph_id };
            that.loading = true;
            that.emptytext = "数据加载中";
            httpPost('m=neirong&c=zhaopinhui&a=upload', param).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    that.tableData = res.data.list;
                    that.srcList = res.data.pics;
                    that.loading = false;
                    if (that.tableData.length === 0) {
                        that.emptytext = "暂无数据";
                    }
                }
            }).catch(function (error) {
                console.log(error)
            })
        },
        uploadChange(file) {
            var tmp = deepClone(this.info)
            tmp.pic_n = URL.createObjectURL(file.raw)
            this.info = tmp
            // 复刻文件信息
            this.files = file.raw;
        },
    },
};
</script>
<style>
.avatar-uploader .el-upload {
    border: 1px dashed #d9d9d9;
    border-radius: 6px;
    cursor: pointer;
    position: relative;
    overflow: hidden;
}

.avatar-uploader-icon {
    font-size: 28px;
    color: #8c939d;
    width: 148px;
    height: 148px;
    line-height: 148px;
    text-align: center;
    border: 1px dashed #d9d9d9;
}

.avatar {
    width: 148px;
    height: 148px;
    display: block;
}

.moduleElHight .moduleElTable {
    height: calc(100% - 66px);
}
</style>