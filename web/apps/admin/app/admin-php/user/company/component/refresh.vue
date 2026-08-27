<template>
    <div class="moduleElHight">
        <div class="moduleSeachbig">
            <div class="tableSeachInpt tableSeachInptsmall tableSeacFromer" style="padding: 2px 0;">
                <el-input v-model="search_params.keyword" @keyup.enter="search" :placeholder="lc('admin_00340')" size="small" clearable>
                	<template #prepend><el-select v-model="search_params.type" size="small" :placeholder="lc('admin_user_00140')">
                	    <el-option :label="lc('wap_user_00080')" value="1"></el-option>
                	    <el-option :label="lc('wap_com_00288')" value="2"></el-option>
                	</el-select></template>
                </el-input>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" icon="el-icon-search" size="small" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
            </div>
        </div>
        <!--<div class="admin_datatip"><i class="el-icon-document"></i> Data stats: 400 total<span class="admin_datatip_n">Unreviewed: 32-->
			<!--</span><span class="admin_datatip_n">Rejected: 3</span> <span class="admin_datatip_n">Locked: 1</span> <span-->
                <!--class="admin_datatip_n">Search results: 400</span>-->
        <!--</div>-->
        <div class="moduleElTable"
             style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="tableData" style="width: 100%" stripe @sort-change='sortChange'
                      :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
                      :default-sort="{ prop: 'id', order: 'descending' }"
                      @selection-change="handleSelectionChange" ref="multipleTable" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" :label="lc('admin_00492')" width="110" sortable="custom"></el-table-column>
                <el-table-column prop="jobname" :label="lc('wap_user_00154')">
                    <template #default="props">
                        <div class="moduleProps">
                            <div class=" ">
                                <el-link :href="props.row.joburl" target="_blank" type="primary">{{ props.row.name }}</el-link>
                            </div>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="name" :label="lc('wap_user_00153')">
                    <template #default="props">
                        <div class="moduleProps">
                            <div class=" ">
                                <el-link :href="props.row.comurl" target="_blank">{{ props.row.com_name }}</el-link>
                            </div>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="sxtime" :label="lc('admin_00777')">
                    <template #default="props">
                        {{ lc("admin_reserve_interval_minutes", [props.row.reserve_interval]) }}
                    </template>
                </el-table-column>
                <el-table-column prop="reserve_start" :label="lc('admin_company_00005')"></el-table-column>
                <el-table-column prop="reserve_end" :label="lc('admin_company_00006')"></el-table-column>
                <el-table-column prop="sx_time_n" :label="lc('admin_user_company_00397')"></el-table-column>
                <el-table-column :label="lc('member_user_00048')" width="200" fixed="right">
                    <template #default="scope">
                        <div class="cz_button">

                            <el-button size="small" plain @click="tz(scope.row)">{{ lc('admin_user_company_00401') }}</el-button>
                            <el-button size="small" plain @click="closeReserve(scope.row.id, 1)">{{ lc('common.close') }}</el-button>
                            <el-button size="small" type="danger"  @click="delrow(scope.row.id)">{{ lc('common.delete') }}</el-button>

                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button @click="delAllBottom" size="small">{{ lc('member_com_00055') }}</el-button>
                <el-button @click="closeReserve('', 2)" size="small">{{ lc('admin_user_company_00399') }}</el-button>
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
        <!-- Batch category transfer -->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_00755')" v-model="drawertz" :with-header="true" append-to-body :show-close="true"
                       width="400px">
                <div v-if="curr_data">
                    <div class="wxsettip_small">{{ lc('wap_00850') }}</div>
                    <div class="TableInpt">
                        <el-radio v-model="curr_data.reserve_status" label="1">{{ lc('member_com_00287') }}</el-radio>
                        <el-radio v-model="curr_data.reserve_status" label="2">{{ lc('common.close') }}</el-radio>
                    </div>
                    <div class="wxsettip_small">{{ lc('wap_com_00227') }}</div>
                    <div class="TableSelect">
                        <el-select v-model="curr_data.reserve_interval" :placeholder="lc('wap_user_00100')">
                            <el-option v-for="(item, index) in jg_data" :key="index" :label="item.label" :value="item.value">
                            </el-option>
                        </el-select>
                    </div>
                    <div v-if="curr_data.reserve_interval == 1" class="wxsettip_small">{{ lc('admin_user_company_00361') }}</div>
                    <div class="TableInpt" v-if="curr_data.reserve_interval == 1">
                        <el-input v-model="userinterval" :placeholder="lc('admin_00756')" size="small">
                            <template #append>{{ lc('wap_com_00247') }}</template>
                        </el-input>
                    </div>
                    <div class="wxsettip_small">{{ lc('wap_com_00234') }}</div>
                    <div class="TableInpt">
                        <el-date-picker v-model="curr_data.reserve_end" value-format="YYYY-MM-dd" type="date" :placeholder="lc('admin_00346')" :picker-options="pickerOptions">
                        </el-date-picker>
                    </div>
                    <div class="wxsettip_small">{{ lc('wap_com_00220') }}</div>
                    <div class="TableInpt">
                        <el-time-picker v-model="curr_data.s_time" value-format="HH:mm">
                        </el-time-picker>
                        <div class="TableInptline">-</div>
                        <el-time-picker v-model="curr_data.e_time" value-format="HH:mm">
                        </el-time-picker>
                    </div>
                </div>
                <template #footer><span class="dialog-footer">
					<el-button @click="drawertz = false">{{ lc('admin_user_weipin_00043') }}</el-button>
					<el-button type="primary" @click="submitTz" :loading="saveLoading">{{ lc('wap_com_00019') }}</el-button>
				</span></template>
            </el-dialog>
        </div>
    </div>
</template>
<script>
const httpPost = (...a) => window.httpPost(...a)
const lc = (...a) => window.lc(...a)
const message = typeof window !== 'undefined' && window.message ? window.message : { success(){}, error(){}, warning(){}, confirm(){}, alert(){}, open(){} }
const delConfirm = (...a) => window.delConfirm(...a)
const formatDate = (...a) => window.formatDate(...a)
const formatMonth = (...a) => window.formatMonth(...a)
const formatDatetime = (...a) => window.formatDatetime(...a)
const deepClone = (...a) => window.deepClone(...a)
const scrollToTop = (...a) => window.scrollToTop(...a)
const isEmpty = (...a) => window.isEmpty(...a)
const isArray = (...a) => window.isArray(...a)
const $ = typeof window !== 'undefined' && window.$ ? window.$ : Object.assign(function(){ return { length: 0 } }, {})
const echarts = typeof window !== 'undefined' && window.echarts ? window.echarts : { init(){ return { setOption(){}, resize(){} } }, graphic: { LinearGradient: function(){} } }

    export default {
        data: function () {
            return {
                loading: false,
                emptytext: lc('wap_js_00113'),
                search_params: {
                    type: '1',
                    keyword: '',
                },
                checkedAll: false,
                selectedItem: [],
                tableData: [],
                currentPage: 1,
                perPage: 0,
                pageSizes: [],
                total: 0,
                sort_type: '',
                sort_col: '',
                tableHig: true,
                drawertz: false,
                curr_data: null,
                pickerOptions: {// el-date-picker date limits
                    disabledDate(time) {
                        // Today and earlier dates
                        // return time.getTime() > Date.now();
                        // Today and later dates
                        return time.getTime() < Date.now() - 8.64e7;
                    }
                },
                jg_data: [
                    {label: lc('admin_00757'), value: '60'},
                    {label: lc('admin_00758'), value: '120'},
                    {label: lc('admin_00759'), value: '180'},
                    {label: lc('admin_00760'), value: '240'},
                    {label: lc('admin_00761'), value: '300'},
                    {label: lc('admin_00762'), value: '360'},
                    {label: lc('admin_00763'), value: '420'},
                    {label: lc('admin_00764'), value: '480'},
                    {label: lc('wap_00852'), value: '1'},
                ],
                userinterval: '',
                islook: false,
                saveLoading: false,

                prevPage: 0
            }
        },
        mounted() {
            var that = this
            setTimeout(function(){
                that.ajaxCloseReserve()
            }, 200)
        },
        created() {
            this.getList();
        },
        methods: {
            getParams:function(params={}){
                var that = this;
                for(let i in params){
                    if(i!='page' && typeof that.search_params[i]!='undefined'){
                        that.search_params[i] = params[i];
                    }
                }
            },
            tz: function(row){
                let date = new Date();
                this.curr_data = row;
                this.curr_data.reserve_end = this.curr_data.reserve_end == lc('common_01936')? date:this.curr_data.reserve_end;

                var intervalArr = ['60', '120', '180', '240', '300', '360', '420', '480'];
                if (intervalArr.indexOf(this.curr_data.reserve_interval) < 0){
                    this.userinterval = this.curr_data.reserve_interval
                    this.curr_data.reserve_interval = '1'
                }
                this.drawertz = true
            },
            submitTz: function(){
                var that = this
                if (that.curr_data.reserve_status == '' || that.curr_data.reserve_status == 0 || that.curr_data.reserve_status == undefined) {
                    message.error(lc('member_com_00279'));
                    return false;
                } else if (that.curr_data.reserve_status == 1) {
                    if (that.curr_data.reserve_interval <= 0) {
                        message.error(lc('wap_00851'));
                        return false;
                    }
                    if (that.curr_data.reserve_interval == 1 && that.userinterval == '') {
                        message.error(lc('admin_company_00018'));
                        return false;
                    }
                    if (that.curr_data.s_time.length > 0 && that.curr_data.e_time.length > 0) {
                        var stime = that.curr_data.s_time.split(':');
                        var etime = that.curr_data.e_time.split(':');
                        if (parseInt(stime[0]) > parseInt(etime[0]) || (parseInt(stime[0]) == parseInt(etime[0]) && parseInt(stime[1]) >= parseInt(etime[1]))) {
                            message.error(lc('wap_com_00213'));
                            return false;
                        }
                    }
                }
                that.saveLoading= true;
                httpPost('m=user&c=company_job&a=upReserveJob', {
                    job_id: that.curr_data.id,
                    end_time: that.curr_data.reserve_end,
                    interval: that.curr_data.reserve_interval == 1 ? that.userinterval : that.curr_data.reserve_interval,
                    status: that.curr_data.reserve_status,
                    s_time: that.curr_data.s_time,
                    e_time: that.curr_data.e_time,
                    uid: that.curr_data.uid
                }).then(function (response) {
                    if (response.data.error == 0) {
                        message.success(response.data.msg, function(){
                            that.getList();
                            that.drawertz = false
                        });
                    } else {
                        message.error(response.data.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                }).finally(function() {
                    setTimeout(function() {
                        that.saveLoading = false;
                    }, 2000);
                });
            },
            ajaxCloseReserve: function(){
                httpPost('m=user&c=company_job&a=ajaxCloseReserve', {},{hideloading: true}).then(function (response) {

                }).catch(function (error) {
                    console.log(error);
                })
            },
            // Close reservation
            closeReserve: function (ids, type) {
                var that= this
                var params = {ids: ids}
                if (type == 1) {// Single operation
                    params.ids = ids
                } else {// Batch operation
                    if (that.selectedItem.length == 0) {
                        message.error(lc('admin_company_00008'));
                        return false;
                    } else {
                        params.ids = that.selectedItem.join(',')
                    }
                }
                httpPost('m=user&c=company_job&a=closeReserve', params).then(function (response) {
                    if (response.data.error == 0) {
                        message.success(lc('admin_company_00017'), function(){
                            that.$refs.multipleTable.clearSelection();
                            that.getList();
                        });
                    } else {
                        message.error(response.data.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                })
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
                scrollToTop()
                this.getList()
            },
            handleCurrentChange(val) {
                this.currentPage = val;
                this.getList()
            },
            sortChange: function (column) {
                if (column.order == 'descending') {
                    this.sort_type = 'desc';
                } else if (column.order == 'ascending') {
                    this.sort_type = 'asc';
                } else {
                    this.sort_type = '';
                }
                this.sort_col = column.prop
                this.search();
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
                if (that.search_params.type) {
                    params.type = that.search_params.type
                }
                if (that.search_params.keyword) {
                    params.keyword = that.search_params.keyword
                }
                if (that.sort_type && that.sort_col) {
                    params.order = that.sort_type
                    params.t = that.sort_col
                }
                that.loading = true;
                that.emptytext = lc('admin_user_weipin_00026');
                httpPost('m=user&c=company_job&a=reserveJob', params, {hideloading: true}).then(function (result) {
                    var res = result.data
                    if (res.error == 0) {
                        that.tableData = res.data.list
                        that.perPage = parseInt(res.data.perPage)
                        that.pageSizes = res.data.pageSizes
                        that.total = parseInt(res.data.total)
                        that.loading = false;
                        if(that.prevPage != that.currentPage){
                            that.prevPage = that.currentPage;
                            that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                            scrollToTop()
                        }
                        if (that.tableData.length === 0){
                            that.emptytext = lc('wap_js_00113');
                        }
                    }
                }).catch(function (e) {
                    console.log(e)
                })
            },
            delrow(id) {
                delConfirm(this, id, this.delete);
            },
            delAllBottom() {
                if (!this.selectedItem.length) {
                    message.error(lc('admin_user_weipin_00005'));
                    return false;
                }
                delConfirm(this, this.selectedItem, this.delete);
            },
            async delete(id) {
                let that = this;
                let params = {
                    del: id
                };
                httpPost('m=user&c=company_job&a=del', params).then(function (response) {
                    if (response.data.error == 0) {
                        message.success(lc('wap_user_00264'), function(){
                            that.$refs.multipleTable.clearSelection();
                            that.getList();
                        });
                    } else {
                        message.error(response.data.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                })
            },
        },
    };
</script>
<style scoped>
.tableSeacFromer{
    margin-right: 8px;
}
.tableSeacFromer .el-input-group__prepend{
    padding: 0;
    background: none;
}
.tableSeacFromer .el-select{
    margin-right: 0;
    width: 160px;
}
.tableSeacFromer .el-input{
    margin-right: 0;
}

</style>
