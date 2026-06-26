<template>
    <div class="moduleElHight" style="margin-left:10px;">
        <div class="moduleSeachbig">
            <div class="tableSeachInpt">
                <el-input placeholder="{yun:}t key='admin_user_00158'{/yun}" size="small" v-model="search_params.keyword"
                          clearable prefix-icon="el-icon-search">
                </el-input>
            </div>
            <div class="tableSeachInpt">
                <div class="block">
                    <el-cascader  v-model="sel_jobtype" :options="joboptions" :props="{ checkStrictly: true }"
                                 clearable placeholder="{yun:}t key='admin_user_company_00377'{/yun}" size="small" filterable></el-cascader>
                </div>
            </div>
            <div class=" tableSeachInpt">
                <div class="block">
                    <el-cascader  v-model="sel_city" :options="cityoptions" :props="{ checkStrictly: true }"
                                 clearable placeholder="{yun:}t key='member_user_00198'{/yun}" size="small" filterable></el-cascader>
                </div>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" icon="el-icon-search" size="mini" @click="search">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
            </div>
        </div>
        <div class="moduleElTable"
            style="border: 1px solid #ebeef5; width: calc(100% - 2px); height: calc(100% - 110px);">
            <el-table :data="tableData" style="width: 100%" stripe
                      :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
                      @selection-change="handleSelectionChange" ref="multipleTable" height="100%" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" label="{yun:}t key='member_com_00012'{/yun}" width="90"></el-table-column>
                <el-table-column label="{yun:}t key='admin_00766'{/yun}" width="210">
                    <template slot-scope="scope">
                        <div class=" ">
                            <div class="username">
                                <el-link type="primary" :underline="false" @click="toMember(scope.row)">{{ scope.row.uname
                                }}</el-link>
                            </div>
                            <div class=" ">
                                {{ scope.row.moblie }}<span class="telgsd" v-if="scope.row.moblie_address">{{
                                    scope.row.moblie_address }}</span>
                            </div>
                            <span class="gsd">{{ scope.row.username }}</span>
                            <!--<a href="index.php?m=user_member&c=Imitate&uid={yun:}$v.uid{/yun}" target="_blank" class="admin_com_name" >{yun:}$v.username{/yun}</a>-->
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='wap_00456'{/yun}" >
                    <template slot-scope="scope">
                        <div class=" ">
                            <div class="user_resumejob">
                                <span @click="openPreview(scope.row)">{{ scope.row.name }}</span>
                                <span v-if="scope.row.defaults == 1" class="user_resumrmr">{yun:}t key='wap_js_00098'{/yun}</span>
                            </div>
                            <div class="">
                                {yun:}t key='admin_00496'{/yun}
                                <span v-if="scope.row.edu_n">{yun:}t key='admin_00497'{/yun}</span>
                                <span v-if="scope.row.exp_n">{yun:}t key='admin_00498'{/yun}</span>
                            </div>
                            <div class="">
                                <span class="gsd">
                                    <el-tooltip effect="dark" :disabled="scope.row.citynum <= 1"
                                        :content="scope.row.cityall" placement="top" v-if="scope.row.city_n">
                                        <span>{{ scope.row.city_n }}.</span>
                                    </el-tooltip>
                                    <span>{{ scope.row.salary }}.</span>
                                    <span>{{ scope.row.report_n }}.</span>
                                    <span>{{ scope.row.type_n }}</span>
                                </span>
                            </div>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='admin_00509'{/yun}" width="130" align="center">
                    <template slot-scope="scope">
                        <div class=" ">
                            <el-progress type="circle" :percentage="parseInt(scope.row.integrity)" :width="45"></el-progress>
                        </div>
                        <div v-if="scope.row.status == 1" class="jlzt">
                            <i class="el-icon-unlock"></i> {yun:}t key='wap_js_00005'{/yun}
                        </div>
                        <div v-else-if="scope.row.status == 3" class="jlzt">
                            <i class="el-icon-unlock"></i> {yun:}t key='admin_user_00249'{/yun}
                        </div>
                        <div v-else class="jlwgk jlzt">
                            <i class="el-icon-lock"></i> {yun:}t key='admin_user_00253'{/yun}
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='admin_00510'{/yun}" width="80" align="center">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            {{ scope.row.sq_num ? scope.row.sq_num : 0 }}
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="logintime" label="{yun:}t key='admin_00767'{/yun}" width="150">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <span>{{ scope.row.ctime_n }}</span>
                            <span class="gsd">{{ scope.row.lastupdate_n }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="140" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button type="small  " size=" " plain @click="sendOrToudi(scope.row.id,scope.row.uid,'0', 1)">{yun:}t key='admin_user_company_00379'{/yun}</el-button>
                            <el-button type="small  " size=" " plain @click="sendOrToudi(scope.row.id,scope.row.uid,'0', 2)">{yun:}t key='admin_user_company_00378'{/yun}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" @change="selectAllBottom">{yun:}t key='wap_js_00074'{/yun}</el-checkbox>
                <el-button @click="directs(1)" size="mini">{yun:}t key='admin_user_company_00376'{/yun}</el-button>
                <el-button  @click="directs(2)" size="mini">{yun:}t key='admin_user_company_00375'{/yun}</el-button>
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
        <!-- 批量推荐进度弹窗 -->
        <div class="tck_setbox">
            <el-dialog :title="cztype+'{yun:}t key=\'admin_company_00042\'{/yun}'" :visible.sync="showprogress" :with-header="true" append-to-body
                       :modal-append-to-body="false" :show-close="true" width="200px">
                <div class="code_img" style="display:flex;justify-content: center;margin-bottom: 20px;">
                    <span style="margin-bottom: 10px;">{yun:}t key='admin_00765'{/yun}</span>
                </div>
            </el-dialog>
        </div>
    </div>
</template>

<script>
module.exports = {
    props: {
        job: {
            type: Object,
            default: {}
        },
        jobtypes: {
            type: Array,
            default: []
        },
        citytypes: {
            type: Array,
            default: []
        },
    },
    data: function () {
        return {
            loading: false,
            emptytext: "{yun:}t key='wap_js_00113'{/yun}",
            checkedAllIndeterminate: false,
            seachbutn: true,
            tableHig: true,
            jobinfo: null,
            search_params:{
                keyword: '',
            },
            joboptions: [], // 职位选项
            cityoptions: [], // 城市选项
            sel_city: [],
            sel_jobtype: [],
            checkedAll: false,
            selectedItem: [],
            tableData: [],
            currentPage: 1,
            perPage: 10,
            pageSizes: [],
            total: 0,
            job_types: [],
            sendnum: 0,
            showprogress: false,
            cztype: '",
            islook: false,

            prevPage: 0
        }
    },
    mounted() {

    },
    watch: {
        job: {
            handler(val) {
                this.jobinfo = val;
                if (val.job1 > 0) {
                    this.sel_jobtype.push(val.job1)
                }
                if (val.job1_son > 0) {
                    this.sel_jobtype.push(val.job1_son)
                }
                if (val.job_post > 0) {
                    this.sel_jobtype.push(val.job_post)
                }
                if (val.provinceid > 0) {
                    this.sel_city.push(val.provinceid)
                }
                if (val.cityid > 0) {
                    this.sel_city.push(val.cityid)
                }
                if (val.three_cityid > 0) {
                    this.sel_city.push(val.three_cityid)
                }
            },
            immediate: true,
            deep: true,
        },
        jobtypes: {
            handler(val) {
                this.joboptions = val;
            },
            immediate: true,
            deep: true,
        },
        citytypes: {
            handler(val) {
                this.cityoptions = val;
            },
            immediate: true,
            deep: true,
        }
    },
    created() {

    },
    methods: {
        // {yun:}t key='admin_user_company_00376'{/yun}、{yun:}t key='admin_user_company_00378'{/yun}
        directs:function (cz_type) {
            var that= this
            if (cz_type == 1) {
                that.cztype = "{yun:}t key='admin_user_company_00379'{/yun}"
            } else if (cz_type == 2) {
                that.cztype = "{yun:}t key='admin_user_company_00378'{/yun}"
            }
            if (!this.selectedItem.length) {
                message.error(lc("admin_company_00043", [that.cztype]));
                return false;
            }
            delConfirm(this, {}, function (params) {
                that.showprogress = true
                that.sendnum = 0
                that.tableData.forEach(function(item){
                    if (that.selectedItem.includes(item.id)) {
                        that.sendOrToudi(item.id, item.uid, that.selectedItem.length, cz_type)
                    }
                })
            }, "{yun:}t key='common.confirm'{/yun}" + that.cztype + "{yun:}t key='member_com_00286'{/yun}")
        },
        async sendOrToudi(eid, uid, type, cz_type) {//type=0单条发送，其他数组为批量发送总数量
            var that = this
            var params = {
                id: that.jobinfo.id,
                comid: that.jobinfo.uid,
                eid: eid,
                uid: uid
            }
            var url = '', msgtype = '"
            if (cz_type == 1) {// {yun:}t key='admin_user_company_00379'{/yun}
                url = "m=user&c=company&a=directrecom'
                msgtype = "{yun:}t key='admin_user_company_00379'{/yun}"
            } else if (cz_type == 2) {// 投递
                url = 'm=user&c=company_job&a=applyJob'
                msgtype = "{yun:}t key='admin_user_company_00378'{/yun}"
            }
            if (url == '') {
                message.error("{yun:}t key='model_00001'{/yun}")
                return false
            }
            if (type == 0) {
                httpPost(url, params).then(function (result) {
                    var res = result.data
                    if (res.error == 0) {
                        message.success(res.msg, function(){
                            that.getList()
                        })
                    } else {
                        message.error(res.msg)
                    }
                }).catch(function (e) {
                    console.log(e)
                })
            } else if (type > 0) {
                await httpPost(url, params).then(function (result) {
                    var res = result.data
                    if (res.error == 0) {
                        that.sendnum++
                        if (that.sendnum == that.selectedItem.length) {
                            message.success(msgtype + "{yun:}t key='wap_js_00104'{/yun}", function(){
                                that.getList()
                                that.showprogress = false
                            })
                        }
                    }
                }).catch(function (e) {
                    console.log(e)
                })
            }
        },
        handleSelectionChange(val) {
            this.selectedItem = [];
            let _this = this;
            if (val.length) {
                val.forEach(item => {
                    _this.selectedItem.push(item.id);
                });
            }
            if (_this.selectedItem.length == 0) {
                _this.checkedAll = false;
            } else {
                if (_this.selectedItem.length == _this.tableData.length) {
                    _this.checkedAll = true;
                } else {
                    _this.checkedAll = false;
                }
            }
        },
        selectAllBottom(value) {
            value ? this.$refs.multipleTable.toggleAllSelection() : this.$refs.multipleTable.clearSelection();
        },
        handleSizeChange(val) {
            this.perPage = val;
            this.getList()
        },
        handleCurrentChange(val) {
            this.currentPage = val;
            this.getList()
        },
        search() {
            this.currentPage = 1;
            this.getList();
        },
        async getList() {
            let that = this;
            let params = {
                page: that.currentPage,
                pageSize: that.perPage
            }
            if (that.search_params.keyword) {
                params.keyword = that.search_params.keyword
            }
            if (that.sel_jobtype[2]) {
                params.job_class = that.sel_jobtype[2]
            } else if (that.sel_jobtype[1]) {
                params.job_class = that.sel_jobtype[1]
            } else if (that.sel_jobtype[0]) {
                params.job_class = that.sel_jobtype[0]
            }
            if (that.sel_city[2]) {
                params.city_class = that.sel_city[2]
            } else if (that.sel_city[1]) {
                params.city_class = that.sel_city[1]
            } else if (that.sel_city[0]) {
                params.city_class = that.sel_city[0]
            }
            if (that.jobinfo.id > 0) {
                params.id = that.jobinfo.id
            }
            that.loading = true;
            that.emptytext = "{yun:}t key='admin_user_weipin_00026'{/yun}";
            httpPost('m=user&c=company_job&a=matching', params, {hideloading: true}).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    that.tableData = res.data.list
                    that.perPage = parseInt(res.data.perPage)
                    that.pageSizes = res.data.pageSizes
                    that.total = parseInt(res.data.total)
                    if(that.prevPage != that.currentPage){
                        that.prevPage = that.currentPage;
                        that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    }
                    that.loading = false;
                    if (that.tableData.length === 0){
                        that.emptytext = "{yun:}t key='wap_js_00113'{/yun}";
                    }
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
    },
};
</script>
<style>
.jlzt .el-button--text {
    font-size: 12px;
}

.jlwgk .el-button--text {
    color: #666;
    font-size: 12px;
}

.el-tag {
    margin-right: 10px;
    margin-bottom: 10px;
}

.button-new-tag {
    margin-left: 10px;
    height: 32px;
    line-height: 30px;
    padding-top: 0;
    padding-bottom: 0;
}

.input-new-tag {
    width: 90px;
    margin-left: 10px;
    vertical-align: bottom;
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
    width: 100px;
    height: 100px;
    line-height: 100px;
    text-align: center;
}

.avatar {
    width: 100px;
    height: 100px;
    display: block;
}

.fenpeizhand .toolClasList {
    flex-wrap: wrap;
}

.toolClasTipse {
    overflow: hidden;
    position: relative;
    padding-left: 75px;
    width: calc(100% - 75px);
}

.toolClasTipse .el-alert {
    overflow: hidden;
    position: relative;
    padding: 6px 0;
    background: none;
}

/* 上传样式结束 */
</style>
