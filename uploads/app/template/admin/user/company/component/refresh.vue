<template>
    <div class="moduleElHight">
        <div class="moduleSeachbig">
            <div class="tableSeachInpt tableSeachInptsmall tableSeacFromer" style="padding: 2px 0;">
                <el-input v-model="search_params.keyword" @keyup.enter.native="search" placeholder="{yun:}t key='admin_00340'{/yun}" size="small" clearable>
                	<el-select v-model="search_params.type" size="small" slot="prepend" placeholder="{yun:}t key='admin_user_00140'{/yun}">
                	    <el-option label="公司名称" value="1"></el-option>
                	    <el-option label="职位名称" value="2"></el-option>
                	</el-select>
                </el-input>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" icon="el-icon-search" size="mini" @click="search">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
            </div>
        </div>
        <!--<div class="admin_datatip"><i class="el-icon-document"></i> 数据统计：共 400 条<span class="admin_datatip_n">未审核：32 条-->
			<!--</span><span class="admin_datatip_n">未通过：3 条</span> <span class="admin_datatip_n">已锁定：1 条</span> <span-->
                <!--class="admin_datatip_n">搜索结果： 400 条</span>-->
        <!--</div>-->
        <div class="moduleElTable"
             style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="tableData" style="width: 100%" stripe @sort-change='sortChange'
                      :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
                      :default-sort="{ prop: 'id', order: 'descending' }"
                      @selection-change="handleSelectionChange" ref="multipleTable" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" label="职位编号" width="110" sortable="custom"></el-table-column>
                <el-table-column prop="jobname" label="职位">
                    <template slot-scope="props">
                        <div class="moduleProps">
                            <div class=" ">
                                <el-link :href="props.row.joburl" target="_blank" type="primary">{{ props.row.name }}</el-link>
                            </div>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="name" label="企业">
                    <template slot-scope="props">
                        <div class="moduleProps">
                            <div class=" ">
                                <el-link :href="props.row.comurl" target="_blank">{{ props.row.com_name }}</el-link>
                            </div>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="sxtime" label="时间间隔">
                    <template slot-scope="props">
                        {{ lc("admin_reserve_interval_minutes", [props.row.reserve_interval]) }}
                    </template>
                </el-table-column>
                <el-table-column prop="reserve_start" label="开始时间"></el-table-column>
                <el-table-column prop="reserve_end" label="结束时间"></el-table-column>
                <el-table-column prop="sx_time_n" label="刷新周期"></el-table-column>
                <el-table-column label="操作" width="200" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">

                            <el-button size="mini" plain @click="tz(scope.row)">{yun:}t key='admin_user_company_00401'{/yun}</el-button>
                            <el-button size="mini" plain @click="closeReserve(scope.row.id, 1)">{yun:}t key='common.close'{/yun}</el-button>
                            <el-button size="mini" type="danger"  @click="delrow(scope.row.id)">{yun:}t key='common.delete'{/yun}</el-button>

                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" @change="selectAllBottom">{yun:}t key='wap_js_00074'{/yun}</el-checkbox>
                <el-button @click="delAllBottom" size="mini">{yun:}t key='member_com_00055'{/yun}</el-button>
                <el-button @click="closeReserve('', 2)" size="mini">{yun:}t key='admin_user_company_00399'{/yun}</el-button>
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
        <!--批量转移类别-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_00755'{/yun}" :visible.sync="drawertz" :with-header="true" append-to-body :show-close="true"
                       width="400px">
                <div v-if="curr_data">
                    <div class="wxsettip_small">{yun:}t key='wap_00850'{/yun}</div>
                    <div class="TableInpt">
                        <el-radio v-model="curr_data.reserve_status" label="1">{yun:}t key='member_com_00287'{/yun}</el-radio>
                        <el-radio v-model="curr_data.reserve_status" label="2">{yun:}t key='common.close'{/yun}</el-radio>
                    </div>
                    <div class="wxsettip_small">{yun:}t key='wap_com_00227'{/yun}</div>
                    <div class="TableSelect">
                        <el-select v-model="curr_data.reserve_interval" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                            <el-option v-for="(item, index) in jg_data" :key="index" :label="item.label" :value="item.value">
                            </el-option>
                        </el-select>
                    </div>
                    <div v-if="curr_data.reserve_interval == 1" class="wxsettip_small">{yun:}t key='admin_user_company_00361'{/yun}</div>
                    <div class="TableInpt" v-if="curr_data.reserve_interval == 1">
                        <el-input v-model="userinterval" placeholder="{yun:}t key='admin_00756'{/yun}" size="small">
                            <template slot="append">{yun:}t key='wap_com_00247'{/yun}</template>
                        </el-input>
                    </div>
                    <div class="wxsettip_small">{yun:}t key='wap_com_00234'{/yun}</div>
                    <div class="TableInpt">
                        <el-date-picker v-model="curr_data.reserve_end" value-format="yyyy-MM-dd" type="date" placeholder="{yun:}t key='admin_00346'{/yun}" :picker-options="pickerOptions">
                        </el-date-picker>
                    </div>
                    <div class="wxsettip_small">{yun:}t key='wap_com_00220'{/yun}</div>
                    <div class="TableInpt">
                        <el-time-picker v-model="curr_data.s_time" value-format="HH:mm">
                        </el-time-picker>
                        <div class="TableInptline">-</div>
                        <el-time-picker v-model="curr_data.e_time" value-format="HH:mm">
                        </el-time-picker>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
					<el-button @click="drawertz = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
					<el-button type="primary" @click="submitTz" :loading="saveLoading">{yun:}t key='wap_com_00019'{/yun}</el-button>
				</span>
            </el-dialog>
        </div>
    </div>
</template>
<script>
    module.exports = {
        data: function () {
            return {
                loading: false,
                emptytext: "{yun:}t key='wap_js_00113'{/yun}",
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
                pickerOptions: {//el-date-picker 时间限定
                    disabledDate(time) {
                        // 今天及今天之前的日期
                        // return time.getTime() > Date.now();
                        // 今天及今天之后的日期
                        return time.getTime() < Date.now() - 8.64e7;
                    }
                },
                jg_data: [
                    {label: '每隔1小时', value: '60'},
                    {label: "{yun:}t key='admin_00758'{/yun}", value: '120'},
                    {label: "{yun:}t key='admin_00759'{/yun}", value: '180'},
                    {label: "{yun:}t key='admin_00760'{/yun}", value: '240'},
                    {label: "{yun:}t key='admin_00761'{/yun}", value: '300'},
                    {label: "{yun:}t key='admin_00762'{/yun}", value: '360'},
                    {label: "{yun:}t key='admin_00763'{/yun}", value: '420'},
                    {label: "{yun:}t key='admin_00764'{/yun}", value: '480'},
                    {label: "{yun:}t key='wap_00852'{/yun}", value: '1'},
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
                this.curr_data.reserve_end = this.curr_data.reserve_end == "{yun:}t key='common_01936'{/yun}"? date:this.curr_data.reserve_end;

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
                    message.error("{yun:}t key='member_com_00279'{/yun}");
                    return false;
                } else if (that.curr_data.reserve_status == 1) {
                    if (that.curr_data.reserve_interval <= 0) {
                        message.error("{yun:}t key='wap_00851'{/yun}");
                        return false;
                    }
                    if (that.curr_data.reserve_interval == 1 && that.userinterval == '') {
                        message.error('请填写自定义刷新间隔');
                        return false;
                    }
                    if (that.curr_data.s_time.length > 0 && that.curr_data.e_time.length > 0) {
                        var stime = that.curr_data.s_time.split(':');
                        var etime = that.curr_data.e_time.split(':');
                        if (parseInt(stime[0]) > parseInt(etime[0]) || (parseInt(stime[0]) == parseInt(etime[0]) && parseInt(stime[1]) >= parseInt(etime[1]))) {
                            message.error("{yun:}t key='wap_com_00213'{/yun}");
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
            // 关闭预约
            closeReserve: function (ids, type) {
                var that= this
                var params = {ids: ids}
                if (type == 1) {// 单个操作
                    params.ids = ids
                } else {// 批量操作
                    if (that.selectedItem.length == 0) {
                        message.error("请选择要关闭的职位！");
                        return false;
                    } else {
                        params.ids = that.selectedItem.join(',')
                    }
                }
                httpPost('m=user&c=company_job&a=closeReserve', params).then(function (response) {
                    if (response.data.error == 0) {
                        message.success('关闭成功', function(){
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
                that.emptytext = "{yun:}t key='admin_user_weipin_00026'{/yun}";
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
                            that.emptytext = "{yun:}t key='wap_js_00113'{/yun}";
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
                    message.error("{yun:}t key='admin_user_weipin_00005'{/yun}");
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
                        message.success("{yun:}t key='wap_user_00264'{/yun}", function(){
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
