<template>
    <div class="moduleElHight">
        <div class="admin_datatip"><i class="el-icon-document"></i> {{ lc("admin_data_stats_job_total", [total]) }}
            <span class="admin_datatip_n">{{ lc("admin_failed_count", [wtg]) }}</span>
            <span class="admin_datatip_n">{{ lc("admin_pending_review_count", [dsh]) }} </span>
            <span class="admin_datatip_n">{{ lc("admin_offline_count", [xj]) }}</span>
        </div>
        <div class="moduleElTable" :class="{ 'moduleElTableHig': tableHig }" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="tableData" style="width: 100%" stripe
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" ref="multipleTable" @sort-change="shortChange" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55" fixed></el-table-column>
                <el-table-column prop="name" :label="lc('wap_com_00288')" width="180"></el-table-column>
                <el-table-column prop="snum" :label="lc('admin_user_company_00038')" width="80"></el-table-column>
                <el-table-column prop="jobhits" :label="lc('wap_com_00112')" width="80"></el-table-column>
                <el-table-column prop="jobexpoure" :label="lc('default_00384')" width="80"></el-table-column>
                <el-table-column :label="lc('member_user_00181')" width="120">
                    <template slot-scope="scope">
                        <el-switch :value="scope.row.status == 0" @change="zpstatuschange($event, scope.row.id)"></el-switch>
                        <div>{{ lc('admin_00612') }}</div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('admin_user_company_00040')">
                    <template slot-scope="scope">
                        <div style="margin:5px;">
                            {{ lc('wap_user_00335') }}
                            <el-switch v-model="scope.row.xsdate > curr_time"
                                @change="tgchange($event, scope.row, 1)">
                            </el-switch>
                        </div>
                        <div style="margin:5px;">
                            {{ lc('common.recommended') }}
                            <el-switch v-model="scope.row.isrec" @change="tgchange($event, scope.row, 2)">
                            </el-switch>
                        </div>
                        <div style="margin:5px;">
                            {{ lc('wap_00222') }}
                            <el-switch v-model="scope.row.urgent_time > curr_time"
                                @change="tgchange($event, scope.row, 3)">
                            </el-switch>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="sdate_n" :label="lc('admin_user_weipin_00030')"></el-table-column>
                <el-table-column prop="lastupdate_n_n" :label="lc('admin_user_company_00272')"></el-table-column>
                <el-table-column :label="lc('admin_user_00336')" width="110">
                    <template slot-scope="scope">
                        <div class="admin_state">
                            <div v-if="scope.row.r_status == '2'">
                                <span class="admin_state3">{{ lc('admin_user_00138') }}</span>
                                <div style="display:inline-block">
                                    <el-popover trigger="hover" placement="right">
                                        <p>{{ scope.row.lock_info }}</p>
                                        <div slot="reference" class="name-wrapper">
                                            <i class="el-icon-question el-icon--right"></i>
                                        </div>
                                    </el-popover>
                                </div>
                            </div>
                            <span class="admin_state1" v-else-if="scope.row.state == '1'">{{ lc('wap_user_00165') }}</span>
                            <span class="admin_state1" v-else-if="scope.row.state == '0'">{{ lc('wap_user_00166') }}</span>
                            <div v-else-if="scope.row.state == '3'">
                                <span class="admin_state2">{{ lc('wap_user_00167') }}</span>
                                <div style="display:inline-block">
                                    <el-popover trigger="hover" placement="right">
                                        <p>{{ scope.row.statusbody }}</p>
                                        <div slot="reference" class="name-wrapper">
                                            <i class="el-icon-question el-icon--right"></i>
                                        </div>
                                    </el-popover>
                                </div>
                            </div>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('member_user_00048')" width="110">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini" @click="jobedit(scope.row)">{{ lc('common.edit') }}
                            </el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange"
                               @current-change="handleCurrentChange"
                               :current-page="currentPage" :page-sizes="pageSizes"
                               :page-size="perPage"
                               layout="total, sizes, prev, pager, next, jumper" :total="total" :pager-count="pagerCount">
                </el-pagination>
            </div>
        </div>
        <div class="modluDrawer">
            <el-dialog :title="jobtgtit" :visible.sync="jobtgdrawer" :with-header="true" append-to-body :show-close="true"
                width="400px">
                <div class="wxsettip_small" v-if="jobtgtype == 1">{{ lc('wap_user_00209') }}</div>
                <div class="wxsettip_small" v-else-if="jobtgtype == 2">{{ lc('wap_com_00041') }}</div>
                <div class="wxsettip_small" v-else-if="jobtgtype == 3">{{ lc('wap_com_00043') }}</div>
                <el-input type="number" :placeholder="lc('admin_00614')" v-model="jobtgdays">
                    <template slot="append">{{ lc('common_02067') }}</template>
                </el-input>
                <div class="wxsettip_small" v-if="jobtgetime != ''">{{ lc('admin_00613') }}</div>
                <el-input v-if="jobtgetime != ''" v-model="jobtgetime" disabled>
                </el-input>
                <div style="margin-top:10px;">
                    <i class="el-icon-warning"></i>
                    {{ lc('admin_user_company_00037') }}
                    <span v-if="jobtgtype == 1">{{ lc('wap_com_00238') }}</span>
                    <span v-else-if="jobtgtype == 2">{{ lc('home.recommended_jobs') }}</span>
                    <span v-else-if="jobtgtype == 3">{{ lc('member_com_00326') }}</span>
                    {{ lc('admin_user_company_00039') }}
                    <el-checkbox v-model="qxtgchecked" true-label="1" false-label="0"></el-checkbox>
                    <span>{{ lc('admin_user_company_00036') }}</span>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="jobtgdrawer = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="jobTgSubmit">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <div class="modluDrawer">
            <el-drawer :title="lc('admin_00615')" :visible.sync="drawerEditJob" append-to-body :wrapper-closable="false" size="60%">
                <addjob ref="jobedit" :jid="jobid" :jtypes="job_types" :ctypes="city_types"></addjob>
            </el-drawer>
        </div>
    </div>
</template>

<script>
module.exports = {
    props:{
        searchword: String,
        searchuid: Number,
		from: {
			type: String,
			default: ''
		},
		param:{
			type: Object,
			default: function(){
				return {};
			}
		}
    },
    data: function () {
        return {
            loading: false,
            emptytext: lc('wap_js_00113'),
            searchForm: {
                page: 1,
                limit: null,
                type: "1",
                keyword: null,
                time: null,
				uid:'',
				state:'',
				status:''
            },
            total: 0,
            tableData: [],
            tableHig: true,
            checked: false,//{{ lc('wap_js_00074') }}
            isIndeterminate: false,// checkbox 的不确定状态
            selectedItem: [],
            info: {},
            resumePreviewVisible: false,
            curr_time: 0,
            dsh: 0,
            wtg: 0,
            xj: 0,
            job_types: [],
            city_types: [],
            jobcomdatacache: [],
            jobcomclassnamecache: [],
            jionly: 0,
            jobcache: null,
            currentPage: 1,
            perPage: 0,
            pageSizes: [],
            pagerCount: 5,
            curr_job: null,
            jobtgtype: '',
            jobtgtit: '',
            jobtgdrawer: false,
            jobtgdays: '',
            jobtgetime: '',
            qxtgchecked: '0',
            tgjid: '',

            jobid:'',
            drawerEditJob: false,

            prevPage: 0
        }
    },
    mounted() {
        var that = this
        setTimeout(function () {
            that.getCacheFun();
        }, 200)
    },
    watch: {
        searchword: {
            handler(val) {
                if (typeof val != 'undefined'){
                    this.searchForm.keyword = val;
                    this.getList();
                }
            },
            immediate: true,
            deep: true,
        },
        searchuid: {
            handler(val) {
                if (typeof val != 'undefined'){
                    this.searchForm.uid = val;
                    this.getList();
                }
            },
            immediate: true,
            deep: true,
        },
		param: {
		    handler(obj) {
		    	if (!$.isEmptyObject(obj)) {
		    		for(let i in obj){
		    		    if(typeof this.searchForm[i]!='undefined'){
		    		        this.searchForm[i] = obj[i];
		    		    }
		    		}
		    		debugger
			        this.getList();
                }
		    },
		    immediate: true,
		    deep: true,
		},
    },
	methods: {
        getCacheFun:function(){
            let that = this;
            httpPost('m=user&c=company_job&a=getCacheData', {},{hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    that.curr_rime = res.data.curr_time;
                    that.job_types = res.data.job_types;
                    that.city_types = res.data.city_types;
                    that.jionly = res.data.jionly;
                    that.jobcomdatacache = res.data.comdata;
                    that.jobcomclassnamecache = res.data.comclass_name;
                    that.jobcache = res.data.cache;
                }
            })
        },
        // 职位招聘状态修改
        zpstatuschange: function (val, id) {
            var that = this
            httpPost('m=user&c=company_job&a=checkstate', { id: id, state: val ? 2 : 1 }).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    that.getList()
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        jobedit: function (row) {
            var that = this
            that.jobid = row.id
            that.drawerEditJob = true
            that.$nextTick(function () {
                that.$refs.jobedit.edit()
            })
        },
        // 职位推广设置
        tgchange: function (val, data, type) {
            this.jobtgtype = type
            this.curr_job = data
            this.tgjid = data.id
            if (type == 1) { // {{ lc('wap_user_00335') }}
                this.curr_job.istop = !this.curr_job.istop // 防止switch状态直接改变
                this.jobtgetime = data.top_time_n ? data.top_time_n : ''
                this.jobtgtit = lc('wap_com_00238')
            } else if (type == 2) { // 推荐
                this.curr_job.isrec = !this.curr_job.isrec // 防止switch状态直接改变
                this.jobtgetime = data.rec_time_n != undefined ? data.rec_time_n : ''
                this.jobtgtit = lc('wap_com_00237')
            } else if (type == 3) { // 紧急
                this.curr_job.isurgent = !this.curr_job.isurgent // 防止switch状态直接改变
                this.jobtgetime = data.urgent_time_n ? data.urgent_time_n : ''
                this.jobtgtit = lc('member_com_00613')
            }
            this.jobtgdrawer = true
        },
        // 职位推广提交
        jobTgSubmit: function () {
            var that = this
            var url = 'm=user&c=company_job&a='
            if (that.jobtgtype == 1) {
                url += 'xuanshang'
                if (that.qxtgchecked == 0 && that.jobtgdays == '') {
                    message.error(lc('common_06281'))
                    return false
                }
            } else if (that.jobtgtype == 2) {
                url += 'recommend'
                if (that.qxtgchecked == 0 && that.jobtgdays == '') {
                    message.error(lc('common_06282'))
                    return false
                }
            } else if (that.jobtgtype == 3) {
                url += 'urgent'
                if (that.qxtgchecked == 0 && that.jobtgdays == '') {
                    message.error(lc('admin_company_00027'))
                    return false
                }
            }
            var params = {
                pid: that.tgjid,
                days: that.jobtgdays,
                s: that.qxtgchecked
            }
            httpPost(url, params).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.getList()
                        that.jobtgdrawer = false
                        that.jobtgdays = ''
                        that.qxtgchecked = false
                    })
                } else {
                    message.error(res.msg)
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        shortChange(e) {
            let orderMap = {ascending: 'asc', descending: 'desc'}
            this.searchForm.t = e.order ? e.prop : null;
            this.searchForm.order = orderMap[e.order];
            this.searchForm.page = 1;
            this.getList();
        },
        handleSearch() {
            this.searchForm.page = 1
            this.getList()
        },
        getList() {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.searchForm));
            for (let index in params) {
                (params[index] === '') && (params[index] = null);
            }
            params.page = _this.currentPage;
            params.pageSize = _this.perPage;
            _this.loading = true;
            _this.emptytext = lc('admin_user_weipin_00026');

			var url = 'm=user&c=company_job&a=index';
            httpPost(url, params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    _this.tableData = res.data.list;
                    _this.total = res.data.total;
                    let dsh = 0,
                        wtg = 0,
                        xj = 0;
                    _this.tableData.forEach(function (e) {
                        if (e.state == '0') {
                            dsh++
                        } else if (e.state == '3') {
                            wtg++
                        } else if (e.status == '1') {
                            xj++
                        }
                    })
                    _this.dsh = dsh;
                    _this.wtg = wtg;
                    _this.xj = xj;
                    _this.perPage = parseInt(res.data.perPage)
                    _this.pageSizes = res.data.pageSizes

                    _this.loading = false;
                    if(_this.prevPage != _this.currentPage){
                        _this.prevPage = _this.currentPage;
                        _this.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    }
                    if (_this.tableData.length === 0){
                        _this.emptytext = lc('wap_js_00113');
                    }
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        handleSizeChange(val) {
            this.perPage = val;
            this.getList()
        },
        handleCurrentChange(val) {
            this.currentPage = val;
            this.getList()
        },
    },
    components: {
        'addjob': httpVueLoader('./addjob.vue'),
    }
};
</script>
<style scoped>

.moduleElHight .moduleElTable {
    padding: 0;
    margin: 0;
    height: calc(100% - 110px);
    width: 100%;
}

.moduleElTableHig {
    height: calc(100% - 90px) !important;
}
</style>