<template>
    <div class="moduleElHight">
        <div class="moduleElTabSeach">
            <div class="moduleElTabInpt">
                <div class="moduleInptList">
                    <el-input size="small" v-model="searchForm.keyword" placeholder="{yun:}t key='admin_yunying_00157'{/yun}" clearable></el-input>
                </div>
                <div class="moduleInptList">
                    <el-button type="primary" icon="el-icon-search" size="mini" @click="search">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
                </div>
            </div>
            <div class="moduleSeachButn">
                <el-button type="primary" icon="el-icon-document-add" size="mini" @click="openSet">{yun:}t key='wap_com_00307'{/yun}</el-button>
            </div>
        </div>
        <div class="moduleElTable" style="height: calc(100% - 109px)">
            <el-table :data="list" border style="width: 100%" ref="multipleTable"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" v-loading="loading">
                <template slot="empty">
                    <p>{{dataText}}</p>
                </template>
                <el-table-column prop="uid" label="{yun:}t key='admin_user_00130'{/yun}" width="100">
                </el-table-column>
                <el-table-column prop="com_name" label="{yun:}t key='wap_com_00157'{/yun}" min-width="240">
                </el-table-column>
                <el-table-column label="{yun:}t key='wap_user_00154'{/yun}" width="180">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <span>{yun:}t key='admin_01100'{/yun}</span>
                            <span>{yun:}t key='admin_yunying_00151'{/yun}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='wap_com_00428'{/yun}" width="160">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <span>{yun:}t key='admin_yunying_00149'{/yun}</span>
                            <span>{yun:}t key='admin_01101'{/yun}</span>
                            <span>{yun:}t key='admin_01102'{/yun}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='wap_js_00145'{/yun}" width="130">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <span>{yun:}t key='admin_01103'{/yun}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='admin_yunying_00168'{/yun}" min-width="220">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <span>{yun:}t key='admin_yunying_00150'{/yun}</span>
                            <span v-if="scope.row.lastwork">{yun:}t key='admin_yunying_00148'{/yun}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" label="{yun:}t key='member_user_00048'{/yun}" width="150">
                    <template slot-scope="scope">
                        <div class="moduleElTaCaoz">
                            <el-button size="mini" @click="updateData(scope.row.id)">{yun:}t key='wap_00225'{/yun}</el-button>
                            <el-button size="mini" @click="openEdit(scope.row)">{yun:}t key='wap_js_00073'{/yun}</el-button>
                            <el-button size="mini" @click="openHb(scope.row)">{yun:}t key='member_com_00270'{/yun}</el-button>
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
            <el-drawer title="{yun:}t key='admin_01105'{/yun}" :visible.sync="drawerEdit" :direction="direction" :modal-append-to-body="false"
                size="500px">
                <div class="moduleSchools">
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='wap_com_00157'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="detail.com_name" :disabled="true" placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='admin_yunying_00165'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormEdit.job" @input="inputIntNumber($event, 'ruleFormEdit', 'job')"
                                placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='admin_yunying_00164'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormEdit.lookjob"
                                @input="inputIntNumber($event, 'ruleFormEdit', 'lookjob')" placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='admin_01104'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormEdit.login" @input="inputIntNumber($event, 'ruleFormEdit', 'login')"
                                placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='admin_yunying_00163'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormEdit.nightwork"
                                @input="inputIntNumber($event, 'ruleFormEdit', 'nightwork')" placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='member_com_00037'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormEdit.yq" @input="inputIntNumber($event, 'ruleFormEdit', 'yq')"
                                placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='admin_yunying_00166'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleFormEdit.sqjob" @input="inputIntNumber($event, 'ruleFormEdit', 'sqjob')"
                                placeholder=""></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='admin_yunying_00161'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-date-picker v-model="ruleFormEdit.lastwork" type="datetime" placeholder="{yun:}t key='admin_00346'{/yun}">
                            </el-date-picker>
                        </div>
                    </div>
                    <div class="setBasicButn" style="border: none;">
                        <el-button type="primary" size="medium" @click="saveEdit">{yun:}t key='common.submit'{/yun}</el-button>
                    </div>
                </div>
            </el-drawer>
        </div>
        <div class="modluDrawer">
            <el-drawer title="{yun:}t key='wap_com_00307'{/yun}" :visible.sync="drawerSet" :direction="direction" :modal-append-to-body="false" size="45%">
                <div class="moduleSchools">
                    <div class="drawerModTishi">
                        <p><b>{yun:}t key='member_user_00449'{/yun}</b><span>{yun:}t key='admin_yunying_00153'{/yun}</span></p>
                        <p><b>{yun:}t key='admin_01065'{/yun}</b><span>{yun:}t key='admin_yunying_00152'{/yun}</span></p>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='admin_system_00332'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-switch v-model="ruleFormSet.sy_yearreport_isopen" active-color="#13ce66"
                                inactive-color="#ff4949" active-value="1" inactive-value="0">
                            </el-switch>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='admin_yunying_00160'{/yun}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-radio v-model="ruleFormSet.sy_yearreport_ewmtype" label="wap">{yun:}t key='wap_js_00098'{/yun}</el-radio>
                            <el-radio v-model="ruleFormSet.sy_yearreport_ewmtype" label="weixin">{yun:}t key='admin_yunying_00158'{/yun}</el-radio>
                        </div>
                        <div class="wenzirushi">
                            <el-alert title="{yun:}t key='admin_yunying_00156'{/yun}" type="info" show-icon :closable="false"></el-alert>
                            <el-alert title="{yun:}t key='admin_yunying_00154'{/yun}" type="info" show-icon :closable="false"></el-alert>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='admin_yunying_00162'{/yun}</span>
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
                            <el-alert title="{yun:}t key='admin_yunying_00143'{/yun}" type="info" show-icon :closable="false"></el-alert>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>{yun:}t key='admin_yunying_00159'{/yun}</span>
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
                            <el-alert title="{yun:}t key='admin_yunying_00155'{/yun}" type="info" show-icon :closable="false"></el-alert>
                        </div>
                    </div>
                    <div class="setBasicButn" style="border: none;">
                        <el-button type="primary" size="medium" @click="saveSet" :disabled="saveLoading">{yun:}t key='common.submit'{/yun}</el-button>
                    </div>
                </div>
            </el-drawer>

            <el-dialog title="{yun:}t key='member_com_00270'{/yun}" class="abow_dialog" :visible.sync="dialogHb" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="400px">
                <div class="center">
                    <img :src="hbUrl" width="360" height="640">
                </div>
                <div class="center">
                    <el-button type="primary" size="mini" @click="downloadHb">{yun:}t key='common_02219'{/yun}</el-button>
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
            dataText: "{yun:}t key='admin_user_weipin_00026'{/yun}",
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
                    that.dataText = "{yun:}t key='wap_js_00113'{/yun}";
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
            a.download = "{yun:}t key='admin_yunying_00167'{/yun}";
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