<template>
    <div class="moduleElHight">
        <div class="moduleElTabSeach">
            <div class="moduleElTabInpt">
                <div class="moduleInptList">
                    <el-input size="small" v-model="searchForm.keyword" :placeholder="lc('admin_yunying_00157')" clearable></el-input>
                </div>
                <div class="moduleInptList">
                    <el-button type="primary" icon="el-icon-search" size="mini" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
                </div>
            </div>
            <div class="moduleSeachButn">
                <el-button type="primary" icon="el-icon-document-add" size="mini" @click="openSet">{{ lc('wap_com_00307') }}</el-button>
            </div>
        </div>
        <div class="moduleElTable" style="height: calc(100% - 109px)">
            <el-table :data="list" border style="width: 100%" ref="multipleTable"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" v-loading="loading">
                <template slot="empty">
                    <p>{{dataText}}</p>
                </template>
                <el-table-column prop="uid" :label="lc('admin_user_00130')" width="100">
                </el-table-column>
                <el-table-column prop="com_name" :label="lc('wap_com_00157')" min-width="240">
                </el-table-column>
                <el-table-column :label="lc('wap_user_00154')" width="180">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <span>{{ lc('admin_01100') }}</span>
                            <span>{{ lc('admin_yunying_00151') }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('wap_com_00428')" width="160">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <span>{{ lc('admin_yunying_00149') }}</span>
                            <span>{{ lc('admin_01101') }}</span>
                            <span>{{ lc('admin_01102') }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('wap_js_00145')" width="130">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <span>{{ lc('admin_01103') }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('admin_yunying_00168')" min-width="220">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <span>{{ lc('admin_yunying_00150') }}</span>
                            <span v-if="scope.row.lastwork">{{ lc('admin_yunying_00148') }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" :label="lc('member_user_00048')" width="150">
                    <template slot-scope="scope">
                        <div class="moduleElTaCaoz">
                            <el-button size="mini" @click="updateData(scope.row.id)">{{ lc('wap_00225') }}</el-button>
                            <el-button size="mini" @click="openEdit(scope.row)">{{ lc('wap_js_00073') }}</el-button>
                            <el-button size="mini" @click="openHb(scope.row)">{{ lc('member_com_00270') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div></div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                    :current-page="page" :page-sizes="pageSizes" :page-size="limit"
                    layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>

        <div class="modluDrawer">
            <el-drawer :title="lc('admin_01105')" :visible.sync="drawerEdit" :direction="direction" :modal-append-to-body="false"
                size="500px">
                <div class="moduleSchools">
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('wap_com_00157') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="detail.com_name" :disabled="true" placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_yunying_00165') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormEdit.job" @input="inputIntNumber($event, 'ruleFormEdit', 'job')"
                                placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_yunying_00164') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormEdit.lookjob"
                                @input="inputIntNumber($event, 'ruleFormEdit', 'lookjob')" placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_01104') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormEdit.login" @input="inputIntNumber($event, 'ruleFormEdit', 'login')"
                                placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_yunying_00163') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormEdit.nightwork"
                                @input="inputIntNumber($event, 'ruleFormEdit', 'nightwork')" placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('member_com_00037') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormEdit.yq" @input="inputIntNumber($event, 'ruleFormEdit', 'yq')"
                                placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_yunying_00166') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormEdit.sqjob" @input="inputIntNumber($event, 'ruleFormEdit', 'sqjob')"
                                placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_yunying_00161') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-date-picker v-model="ruleFormEdit.lastwork" type="datetime" :placeholder="lc('admin_00346')">
                            </el-date-picker>
                        </div>
                    </div>
                    <div class="setBasicButn" style="border: none;">
                        <el-button type="primary" size="medium" @click="saveEdit">{{ lc('common.submit') }}</el-button>
                    </div>
                </div>
            </el-drawer>
        </div>
        <div class="modluDrawer">
            <el-drawer :title="lc('wap_com_00307')" :visible.sync="drawerSet" :direction="direction" :modal-append-to-body="false" size="45%">
                <div class="moduleSchools">
                    <div class="drawerModTishi">
                        <p><b>{{ lc('member_user_00449') }}</b><span>{{ lc('admin_yunying_00153') }}</span></p>
                        <p><b>{{ lc('admin_01065') }}</b><span>{{ lc('admin_yunying_00152') }}</span></p>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_system_00332') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-switch v-model="ruleFormSet.sy_yearreport_isopen" active-color="#13ce66"
                                inactive-color="#ff4949" active-value="1" inactive-value="0">
                            </el-switch>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_yunying_00160') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-radio v-model="ruleFormSet.sy_yearreport_ewmtype" label="wap">{{ lc('wap_js_00098') }}</el-radio>
                            <el-radio v-model="ruleFormSet.sy_yearreport_ewmtype" label="weixin">{{ lc('admin_yunying_00158') }}</el-radio>
                        </div>
                        <div class="wenzirushi">
                            <el-alert :title="lc('admin_yunying_00156')" type="info" show-icon :closable="false"></el-alert>
                            <el-alert :title="lc('admin_yunying_00154')" type="info" show-icon :closable="false"></el-alert>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_yunying_00162') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-upload class="avatar-uploader" list-type="picture" :accept="pic_accept" action="" :auto-upload="false"
                                :on-change="handleChangePic" :show-file-list="false">
                                <img v-if="ruleFormSet.sy_yearreport_pic_n" :src="ruleFormSet.sy_yearreport_pic_n"
                                    class="avatar">
                                <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                            </el-upload>
                        </div>
                        <div class="wenzirushi">
                            <el-alert :title="lc('admin_yunying_00143')" type="info" show-icon :closable="false"></el-alert>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{{ lc('admin_yunying_00159') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-upload class="avatar-uploader" list-type="picture" :accept="pic_accept" action="" :auto-upload="false"
                                :on-change="handleChangeTip" :show-file-list="false">
                                <img v-if="ruleFormSet.sy_yearreport_tip_n" :src="ruleFormSet.sy_yearreport_tip_n"
                                    class="avatar">
                                <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                            </el-upload>
                        </div>
                        <div class="wenzirushi">
                            <el-alert :title="lc('admin_yunying_00155')" type="info" show-icon :closable="false"></el-alert>
                        </div>
                    </div>
                    <div class="setBasicButn" style="border: none;">
                        <el-button type="primary" size="medium" @click="saveSet" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
                    </div>
                </div>
            </el-drawer>

            <el-dialog :title="lc('member_com_00270')" class="abow_dialog" :visible.sync="dialogHb" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="400px">
                <div class="center">
                    <img :src="hbUrl" width="360" height="640">
                </div>
                <div class="center">
                    <el-button type="primary" size="mini" @click="downloadHb">{{ lc('common_02219') }}</el-button>
                </div>
            </el-dialog>
        </div>
    </div>
</template>
    
<script>
module.exports = {
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            loading: false,
            dataText: lc('admin_user_weipin_00026'),
            direction: 'rtl',

            // 搜索筛选项
            searchForm: {},

            // list
            page: 1,
            limit: 0,
            list: [],
            total: 0,
            pageSizes: [],

            saveLoading: false,

            detail: {},

            // set up
            drawerSet: false,
            ruleFormSet: {},

            // Update
            drawerEdit: false,
            ruleFormEdit: {},

            // poster
            dialogHb: false,
            hbUid: '',
            hbUrl: '',
            prevPage:0
        }
    },

    mounted() {

    },
    created() {
        this.init();
    },
    methods: {
        init() {
            this.resetSearch();
            this.search();
        },

        resetSearch() {
            this.searchForm = {};
            this.limit = 0;
        },

        handleSizeChange(val) {
            this.limit = val;
            this.getList();
        },
        handleCurrentChange(val) {
            this.page = val;
            this.getList();
        },
        search() {
            this.page = 1;
            this.getList();
        },
        getList() {
            let that = this,
                searchForm = that.searchForm,
                params = {
                    page: that.page,
                    limit: that.limit,
                };
            that.loading = true;
            httpPost('m=yunying&c=yingxiao_hrlog', { ...params, ...searchForm }, {hideloading: true}).then(function (response) {
                let res = response.data,
                    data = res.data;

                that.list = data.list;
                that.total = parseInt(data.total);
                that.pageSizes = data.page_sizes;
                if (that.limit === 0) {
                    that.limit = parseInt(data.limit); // 取系统配置默认数量
                }
                if (that.page > data.page) {
                    that.page = parseInt(data.page); // 最后一页被删除后，取最新的页数
                }
                if(that.prevPage != that.page){
                    that.prevPage = that.page;
                    that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                }
                that.loading = false;
                if (that.list.length === 0) {
                    that.dataText = lc('wap_js_00113');
                }
            })
        },

        openSet() {
            let that = this;

            httpPost('m=yunying&c=yingxiao_hrlog&a=set').then(function (response) {
                let res = response.data,
                    data = res.data;

                that.ruleFormSet = data.set;
                that.drawerSet = true;
            })
        },

        // 上传时触发
        handleChangeTip(file, fileList) {
            this.$set(this.ruleFormSet, 'sy_yearreport_tip', file.raw);
            this.$set(this.ruleFormSet, 'sy_yearreport_tip_n', file.url);
        },

        // 上传时触发
        handleChangePic(file, fileList) {
            this.$set(this.ruleFormSet, 'sy_yearreport_pic', file.raw);
            this.$set(this.ruleFormSet, 'sy_yearreport_pic_n', file.url);
        },

        saveSet() {
            let that = this,
                ruleFormSet = that.ruleFormSet,
                formData = new FormData();

            if (that.saveLoading) {
                return false;
            }

            that.saveLoading = true;

            $.each(ruleFormSet, function (key, value) {
                if (key != 'sy_yearreport_tip_n' && key != 'sy_yearreport_pic_n') {
                    formData.append(key, value);
                }
            });

            httpPost('m=yunying&c=yingxiao_hrlog&a=setSave', formData).then(function (res) {
                if (res.data.error > 0) {
                    that.saveLoading = false;
                    message.error(res.data.msg);
                } else {
                    that.drawerSet = false;
                    message.success(res.data.msg, function () {
                        that.saveLoading = false;
                    });
                }
            })
        },

        inputIntNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9]/g, '');
        },
        updateData: function(id) {
            var _this = this;
        
            var params = {
                id: id
            };
            delConfirm(_this, params, this.updatePost,lc('admin_vue_00099'))
        },
        async updatePost(params) {
        
            let that = this;
        
            httpPost('m=yunying&c=yingxiao_hrlog&a=rehrlog', params).then(function(result) {
        
                var res = result.data;
                if (res.error == 0) {
                    message.success(res.msg, function() { that.getList() });
                    return;
                } else {
                    message.error(res.msg);
                    return;
                }
            }).catch(function(e) {
                console.log(e)
            })
        },
        openEdit(data) {
            this.detail = data;
            this.ruleFormEdit = {
                id: data.id,
                job: data.job,
                lastwork: data.lastwork && data.lastwork !== '0' ? new Date(data.lastwork_n) : '',
                login: data.login,
                lookjob: data.lookjob,
                lookresume: data.lookresume,
                nightwork: data.nightwork,
                sqjob: data.sqjob,
                uid: data.uid,
                yq: data.yq,
            }
            this.drawerEdit = true;
        },

        saveEdit() {
            let that = this,
                params = that.ruleFormEdit;

            if (that.saveLoading) {
                return false;
            }

            params.lastwork = params.lastwork ? formatDatetime(params.lastwork) : '';

            that.saveLoading = true;

            httpPost('m=yunying&c=yingxiao_hrlog&a=editsave', params).then(function (res) {
                if (res.data.error > 0) {
                    message.error(res.data.msg, function () {
                        that.saveLoading = false;
                    });
                } else {
                    message.success(res.data.msg, function () {
                        that.getList();
                        that.drawerEdit = false;
                        that.saveLoading = false;
                    });
                }
            })
        },

        openHb(data) {
            let that = this;

            if (that.hbUid == data.uid) {
                that.dialogHb = true;
                return false;
            }

            httpPost('m=yunying&c=yingxiao_hrlog&a=getHb', { uid: data.uid }).then(function (res) {
                that.hbUid = data.uid;
                that.hbUrl = res.data.data.hbUrl;
                that.dialogHb = true;
            })
        },

        downloadHb() {
            var a = document.createElement("a");
            a.href = this.hbUrl;
            a.download = lc('admin_yunying_00167');
            a.click();
        },
    },
};
</script>
<style>

.el-dialog{
    margin-top: 2vh !important;
    width: 400px;
    position: relative;
    background: #fff;
    border-radius: 2px;
    box-shadow: 0 1px 30px rgb(0 0 0 / 10%);
    box-sizing: border-box;
}

.drawerModTite {
    width: 130px;
}

.drawerModInpt {
    width: calc(100% - 140px);
}

.wenzirushi {
    overflow: hidden;
    position: relative;
    width: calc(100% - 140px);
    padding-left: 140px;
    padding-top: 5px;
}

.wenzirushi .el-alert {
    padding: 3px 0;
    background: none;
}

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
.center{
    overflow: hidden;
    position: relative;
}
.center .el-button{
    width: 100%;
    margin-bottom: 15px;
    margin-top: 5px;
}
</style>