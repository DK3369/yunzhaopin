<template>
    <div>
        <div class="moduleElHight">
            <div class="tableDome_tip">
                <el-alert title="可添加订单合同图片" type="success" :closable="false">
                </el-alert>
            </div>
            <div class="moduleHeadrButn" style=" margin-bottom: 12px;">
                <el-button type="primary" icon="el-icon-document-add" @click="addPic">添加图片</el-button>
            </div>
            <div class="moduleElTable">
                <el-table :data="tableData" border style="width: 100%;" ref="multipleTable"
                          :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" v-loading="loading" :empty-text="emptytext">
                    <el-table-column prop="id" label="用户ID" width="150">
                    </el-table-column>
                    <el-table-column prop="wenjian" label="图片">
                        <template slot-scope="scope">
                            <div class="demo-image__preview">
                                <el-image style="width: 100px; height: 60px" :src="scope.row.pic_n"
                                          :preview-src-list="srcList">
                                </el-image>
                            </div>
                        </template>
                    </el-table-column>
                    <el-table-column fixed="right" label="操作" width="120">
                        <template slot-scope="scope">
                            <div class="moduleElTaCaoz">
                                <el-button type="text" size="small" @click="delrow(scope.row.id)">删除</el-button>
                            </div>
                        </template>
                    </el-table-column>
                </el-table>
            </div>
            <div class="modulePaging">
                <div>
                    <!--<el-checkbox v-model="checkedAll" @change="selectAllBottom">全选</el-checkbox>-->
                    <!--<el-button @click="deleteRow(null, true)">批量删除</el-button>-->
                </div>
                <div class="modulePagNum">
                    <el-pagination background @size-change="handleSizeChange"
                                   @current-change="handleCurrentChange"
                                   :current-page="currentPage" :page-sizes="pageSizes"
                                   :page-size="perPage"
                                   layout="total, sizes, prev, pager, next, jumper" :total="total">
                    </el-pagination>
                </div>
            </div>
            <div class="modluDrawer">
                <el-drawer title="添加图片" :visible.sync="editBox" append-to-body :modal-append-to-body="false"
                           :show-close="true" :with-header="true" size="45%">
                    <div class="drawerModlue">
                        <div class="drawerModInfo">
                            <div class="drawerModLis">
                                <div class="drawerModTite">
                                    <span>合同图片</span>
                                </div>
                                <div class="drawerModInpt">
                                    <el-upload :action='uploadAction' multiple :limit="3" list-type="picture-card"
                                               :accept="pic_accept"
                                               :on-success="handleAvatarSuccess" ref='files'
                                               :on-exceed="exceedFun"
                                               :before-upload="onBeforeUpload">
                                        <i slot="default" class="el-icon-plus"></i>
                                        <div slot="file" slot-scope="{file}">
                                            <img class="el-upload-list__item-thumbnail" :src="file.url" alt="">
                                            <span class="el-upload-list__item-actions">
                                                <span class="el-upload-list__item-delete" @click="handleRemove(file)">
                                                    <i class="el-icon-delete"></i>
                                                </span>
                                            </span>
                                        </div>
                                    </el-upload>
                                    <div style="font-size: 12px;color: #8c939d">
                                        <i class="el-icon-warning-outline"></i>
                                        <span>只能上传jpg/png文件，且不超过500kb</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="setBasicButn" style="border: none;">
                            <el-button type="primary" size="medium" @click="saveImg">提交</el-button>
                        </div>
                    </div>
                </el-drawer>
            </div>
        </div>
    </div>
</template>
<script>
    module.exports = {
        props: {
            oid: {
                type: String,
                default: ''
            },
        },
        data: function () {
            return {
                pic_accept: localStorage.getItem("pic_accept"),
                loading: false,
                emptytext: '暂无数据',
                order_id: '',
                tableData: [],
                srcList: [],
                picurl: [], // 保存图片地址
                editBox: false,
                currentPage: 1,
                perPage: 0,
                pageSizes: [],
                total: 0,
                uploadAction: baseUrl + 'm=user&c=company_order&a=multiupload',

                prevPage: 0
            }
        },
        watch: {
            oid: {
                handler(val, oldVal) {
                    this.order_id = val;
                },
                immediate: true,
                deep: true,
            }
        },
        created() {
            this.getList();
        },
        methods: {
            handleSizeChange(val) {
                this.perPage = val;
                this.getList()
            },
            handleCurrentChange(val) {
                this.currentPage = val;
                this.getList()
            },
            addPic() {
                this.picurl = [];
                this.editBox = true;
            },
            delrow(id) {
                delConfirm(this, id, this.delete);
            },
            async delete(id) {
                let that = this;
                let params = {
                    delid: id
                };
                httpPost('m=user&c=company_order&a=htpic_del', params).then(function (response) {
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
            onBeforeUpload: function (file) {
                const isJPG =
                    file.type === 'image/jpg' || file.type === 'image/png' || file.type === 'image/jpeg' || file.type === 'image/gif';
                const isLt2M = file.size / 1024 / 1024 < 5;
                if (!isJPG) {
                    this.$message.error('上传图片只能是 JPG, PNG, JPEG, GIF 格式!');
                }
                if (!isLt2M) {
                    this.$message.error('上传图片大小不能超过 2MB!');
                }
                return isJPG && isLt2M;
            },
            handleAvatarSuccess(res, file) {
                if (res.error == 0) {
                    this.picurl.push(res.picurl);
                }
            },
            exceedFun(files, fileList){
                this.$message.error('超过上传图片最大限制!');
            },
            saveImg: function () {
                let that = this;
                if (that.picurl.length == 0) {
                    message.error('请上传合同图片');
                    return false;
                }
                let sendData = {
                    add: 1,
                    order_id: that.order_id,
                    picurl: this.picurl
                }
                httpPost('m=user&c=company_order&a=uploadsave', sendData).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        message.success(res.msg, function(){
                            that.editBox = false;
                            that.getList()
                        })
                    }else {
                        message.error(res.msg);
                    }
                })
            },
            async getList() {
                let that = this;
                let param = {id: that.order_id};
                that.loading = true;
                that.emptytext = "数据加载中";
                httpPost('m=user&c=company_order&a=upload', param).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        that.tableData = res.data.list;
                        that.srcList = res.data.pics;
                        that.perPage = parseInt(res.data.perPage)
                        that.pageSizes = res.data.pageSizes
                        that.total = parseInt(res.data.total);
                        that.loading = false;
                        if(that.prevPage != that.currentPage){
                            that.prevPage = that.currentPage;
                            that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                        }
                        if (that.tableData.length === 0){
                            that.emptytext = "暂无数据";
                        }
                    }
                }).catch(function (error) {
                    console.log(error)
                })
            },
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