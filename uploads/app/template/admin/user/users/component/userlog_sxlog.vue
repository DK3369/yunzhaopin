<template>
    <!--会员-个人-行为记录：简历刷新记录-->
    <div class="moduleElHight">
        <div class="moduleElSearchInf">
            <div class="moduleElTabInpt" style="flex-wrap: wrap;">
                <div class="moduleInptList" style="margin-bottom: 8px;">
                    <el-input placeholder="{yun:}t key='admin_user_weipin_00003'{/yun}" @keyup.enter.native="handleSearch" size="small" v-model="searchForm.keyword" class="input-with-select" clearable>
                        <el-select v-model="searchForm.type" slot="prepend" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                            <el-option label="个人姓名" value="1"></el-option>
                            <el-option label="期望职位" value="2"></el-option>
                        </el-select>
                    </el-input>
                </div>
                <div class="tableSeachInpt tableSeachInptsmalltwo">
                    <el-date-picker v-model="searchForm.times" type="daterange" align="right" unlink-panels range-separator="至" start-placeholder="{yun:}t key='admin_00525'{/yun}" end-placeholder="{yun:}t key='admin_00526'{/yun}" :picker-options="timeOptions" value-format="yyyy-MM-dd" size="small" @change="handleTimeChange"></el-date-picker>
                </div>
                <div class="tableSeachInpt">
                    <el-button type="primary" icon="el-icon-search" size="mini" @click="handleSearch">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
                </div>
            </div>
        </div>


        <div class="moduleElTable" :class="{ 'moduleElTableHig': tableHig }" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="tableData" style="width: 100%" stripe
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" ref="multipleTable"
                @selection-change="handleSelectionChange" @sort-change="shortChange" :default-sort = "{prop: 'id', order: 'descending'}" v-loading="loading">
                <template slot="empty">
                    <p>{{dataText}}</p>
                </template>
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" label="编号" width="80" sortable="custom"></el-table-column>
                <el-table-column prop="u_name" label="个人姓名" min-width="100" show-overflow-tooltip>
                    <template slot-scope="scope">
                        <el-button type="text" @click="handlePreview(scope)" style="padding: 0">{{ scope.row.u_name }}</el-button>
                    </template>
                </el-table-column>
                <el-table-column prop="r_name" label="期望职位" show-overflow-tooltip>
                    <template slot-scope="scope">
                        <el-button type="text" @click="handlePreview(scope)" style="padding: 0">{{ scope.row.r_name }}</el-button>
                    </template>
                </el-table-column>
                <el-table-column prop="ip" label="IP"></el-table-column>
                <el-table-column prop="port_n" label="端口"></el-table-column>
                <el-table-column prop="r_time_n" label="刷新时间"></el-table-column>
                <el-table-column label="操作" width="80" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button type="danger" size="mini" @click="deleteRow(scope)">{yun:}t key='common.delete'{/yun}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">{yun:}t key='wap_js_00074'{/yun}</el-checkbox>
                <el-button @click="deleteRow(null, true)" size="mini">{yun:}t key='member_com_00055'{/yun}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                    :current-page.sync="searchForm.page" :page-size="searchForm.limit" :page-sizes="pageSizes"
                    layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
        <div class="modluDrawer">
            <el-drawer title="{yun:}t key='member_user_00037'{/yun}" :append-to-body="true" :visible.sync="resumePreviewVisible" :destroy-on-close="true" size="530px">
                <resume_preview :id="info.resume_id" :uid="info.uid"></resume_preview>
            </el-drawer>
        </div>
    </div>
</template>

<script>
module.exports = {
    data: function () {
        return {
            loading: false,
            dataText: "{yun:}t key='admin_user_weipin_00026'{/yun}",
            searchForm: {
                page: 1,
                limit: null,
                type: "1",
                keyword: null,
                status: null,
                times: null,
            },
            timeOptions: {
                shortcuts: [{
                    text: "{yun:}t key='common_02000'{/yun}",
                    onClick(picker) {
                        const end = new Date();
                        const start = new Date();
                        start.setTime(start.getTime() - 3600 * 1000 * 24);
                        end.setTime(end.getTime() - 3600 * 1000 * 24);
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: "{yun:}t key='common_01940'{/yun}",
                    onClick(picker) {
                        const end = new Date();
                        const start = new Date();
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: "{yun:}t key='admin_user_00146'{/yun}",
                    onClick(picker) {
                        const start = new Date(new Date().setHours(0, 0, 0) - (new Date().getDay() - 1) * 24 * 60 * 60 * 1000);
                        const end = new Date();
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: "{yun:}t key='admin_user_00142'{/yun}",
                    onClick(picker) {
                        const start = new Date(new Date().setHours(0, 0, 0) - (new Date().getDay() + 6) * 24 * 60 * 60 * 1000);
                        const end = new Date(new Date().setHours(0, 0, 0) + (0 - new Date().getDay()) *24 * 60 * 60 *1000);
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: "{yun:}t key='admin_user_00147'{/yun}",
                    onClick(picker) {
                        const end = new Date();
                        const start = new Date(new Date(new Date().getFullYear(), new Date().getMonth(), 1).setHours(0, 0, 0));
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: "{yun:}t key='admin_user_00143'{/yun}",
                    onClick(picker) {
                        const end = new Date(new Date(new Date().getFullYear(), new Date().getMonth(), 0).setHours(23, 59, 59, 59));
                        const start = new Date(new Date(new Date().getFullYear(), new Date().getMonth() - 1, 1).setHours(0, 0, 0));
                        picker.$emit('pick", [start, end]);
                    }
                }]
            },
            total: 0,
            tableData: [],
            pageSizes: [],
            tableHig: true,
            checked: false,//{yun:}t key='wap_js_00074'{/yun}
            isIndeterminate: false,// checkbox 的不确定状态
            selectedItem: [],

            info: {},
            resumePreviewVisible: false,

            prevPage: 0
        }
    },
    components: {
        "resume_preview': httpVueLoader('../../../component/resume_preview.vue'),
    },
    created() {
		var that = this;
		let params = window.parent.homeapp.$route.params;
		let query = window.parent.homeapp.$route.query;
		
		if (!$.isEmptyObject(query.params)) {
			params = {...params,...query.params};
		}
		
		if (!$.isEmptyObject(params)) {
			delete params.activeName;
			this.getParams(params);
		}
        this.getList();
    },
    methods: {
        handlePreview(scope) {
            this.info = scope.row;
            this.resumePreviewVisible = true;
        },
		getParams:function(params={},search=false){
			var that = this;
			for(let i in params){
				if(typeof that.searchForm[i]!='undefined'){
					that.searchForm[i] = params[i];
				}
			}
			if(search){
				this.handleSearch();
			}
		},
        handleSelectionChange(val) {
            this.selectedItem = val;
            if (this.selectedItem.length == 0) {
                this.isIndeterminate = false;
                this.checked = false;
            } else {
                if (this.selectedItem.length == this.tableData.length) {
                    this.isIndeterminate = false;
                    this.checked = true;
                } else {
                    this.isIndeterminate = true;
                    this.checked = false;
                }
            }
        },
        selectAllBottom(value) {
            value ? this.$refs.multipleTable.toggleAllSelection() : this.$refs.multipleTable.clearSelection();
        },
        shortChange(e) {
            let orderMap = {ascending: 'asc', descending: 'desc'}
            this.searchForm.t = e.order ? e.prop : null;
            this.searchForm.order = orderMap[e.order];
            this.searchForm.page = 1;
            this.getList();
        },
        handleSizeChange(val) {
            this.searchForm.limit = val;
            this.getList();
        },
        handleCurrentChange(val) {
            this.searchForm.page = val;
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
            _this.loading = true;
            httpPost('m=user&c=users_userlog&a=sxLog', params,{hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    _this.tableData = res.data.list;
                    _this.total = res.data.total;
                    _this.searchForm.limit = res.data.perPage;
                    _this.pageSizes = res.data.pageSizes;
                    _this.loading = false;
                    if(_this.prevPage != _this.searchForm.page){
                        _this.prevPage = _this.searchForm.page;
                        _this.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    }
                    if (_this.tableData.length === 0) {
                        _this.dataText = "{yun:}t key='wap_js_00113'{/yun}";
                    }
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        deleteRow(scope, isMore) {
            let params = {};
            if (isMore) {
                if (!this.selectedItem.length) {
                    message.error("{yun:}t key='admin_user_weipin_00005'{/yun}");
                    return false;
                }
                let list = [];
                for (let item of this.selectedItem) {
                    list.push(item.id);
                }
                params.del = list.join(',');
            } else {
                // let index = scope.$index;
                // this.tableData.splice(index, 1);
                params.id = scope.row.id;
            }
            delConfirm(this, params, this.delete);
        },
        delete(params) {
            let _this = this;
            httpPost('m=user&c=users_userlog&a=delSxLog', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success("{yun:}t key='admin_user_00187'{/yun}");
                    _this.getList();
                } else {
                    message.error("{yun:}t key='admin_user_00186'{/yun}");
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        handleTimeChange() {
            this.handleSearch();
        }
    },
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