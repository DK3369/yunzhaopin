<template>
    <!--会员-企业-行为记录：职位申请记录-->
    <div class="moduleElHight" :class="searchClass == 'drawer' ? 'pad20' : ''">
        <div class="moduleElSearchInf" v-if="cansearch">
            <div class="moduleElTabInpt" style="flex-wrap: wrap;">
                <div class="moduleInptList">
                    <el-input placeholder="输入你要搜索的关键字" @keyup.enter.native="handleSearch" size="small" v-model="searchForm.keyword" class="input-with-select" clearable>
                        <el-select v-model="searchForm.type" slot="prepend" placeholder="职位名称">
                            <el-option label="职位名称" value="1"></el-option>
                            <el-option label="所属企业" value="2"></el-option>
                            <el-option label="姓名" value="3"></el-option>
                        </el-select>
                    </el-input>
                </div>
                <div class="moduleInptList">
                    <el-select v-model="searchForm.browse" size="small" slot="prepend" placeholder="是否查看" clearable @change="handleSearch">
                        <el-option label="未查看" value="1"></el-option>
                        <el-option label="已查看" value="2"></el-option>
                        <el-option label="待通知" value="3"></el-option>
                        <el-option label="不合适" value="4"></el-option>
                        <el-option label="未接通" value="5"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt tableSeachInptsmalltwo">
                    <el-date-picker v-model="searchForm.times" type="daterange" align="right" unlink-panels range-separator="至" start-placeholder="申请开始日期" end-placeholder="申请结束日期" :picker-options="timeOptions" value-format="yyyy-MM-dd" size="small" @change="handleTimeChange"></el-date-picker>
                </div>
                <div class="tableSeachInpt">
                    <el-button type="primary" icon="el-icon-search" size="mini" @click="handleSearch">查询</el-button>
                </div>
            </div>
        </div>
        <div class="moduleElTable"
            style="border: 1px solid #ebeef5; width: calc(100% - 2px); height: calc(100% - 91px);">
            <el-table :data="tableData" style="width: 100%" stripe
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%"
                ref="multipleTable" @selection-change="handleSelectionChange" @sort-change="shortChange" :default-sort="{ prop: 'id', order: 'descending' }" v-loading="loading">
                <template slot="empty">
                    <p>{{dataText}}</p>
                </template>
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" label="编号" sortable="custom" width="80"></el-table-column>
                <el-table-column prop="job_name" label="职位名称" min-width="150" show-overflow-tooltip>
                    <template slot-scope="scope">
                        <el-link :href="scope.row.job_url" target="_blank" type="primary">{{ scope.row.job_name }}
                        </el-link>
                    </template>
                </el-table-column>
                <el-table-column prop="com_name" label="所属企业" min-width="180" show-overflow-tooltip>
                    <template slot-scope="scope">
                        <el-link :href="scope.row.com_url" target="_blank" type="primary">{{ scope.row.com_name }}
                        </el-link>
                    </template>
                </el-table-column>
                <el-table-column prop="username_n" label="姓名">
                    <template slot-scope="scope">
                        <el-button type="text" @click="handlePreview(scope)" style="padding: 0">{{ scope.row.username_n }}
                        </el-button>
                    </template>
                </el-table-column>
                <el-table-column prop="telphone" label="手机号" width="140">
                    <template slot-scope="scope">
                        <el-link @click="jumpToMember(scope.row.uid)" target="_blank" type="primary">{{ scope.row.telphone }}
                        </el-link>
                    </template>
                </el-table-column>
                <el-table-column prop="is_browse" label="是否查看" width="100">
                    <template slot-scope="scope">
                        <div class="admin_state">
                            <span v-if="scope.row.is_browse == 2" class="admin_state1">已查看</span>
                            <span v-else-if="scope.row.is_browse == 3" class="admin_state5">待通知</span>
                            <span v-else-if="scope.row.is_browse == 4" class="admin_state3">不合适</span>
                            <span v-else-if="scope.row.is_browse == 5" class="admin_state2">未接通</span>
                            <span v-else class="admin_state2">未查看</span>
                            <!--<span class="admin_state1">已审核</span>-->
                            <!--<span class="admin_state2">未通过</span>-->
                            <!--<span class="admin_state3">已锁定</span>-->
                            <!--<span class="admin_state4">待审核</span>-->
                            <!--<span class="admin_state5">已暂停</span>-->
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="datetime" sortable="custom" label="申请时间" width="140">
                    <template slot-scope="scope">{{ scope.row.datetime_n_n }}</template>
                </el-table-column>
                <el-table-column prop="isdel_n" label="状态" width="100">
                    <!--<template slot-scope="scope">-->
                    <!--	<div class="admin_state">-->
                    <!--		<span class="admin_state1">正常</span>-->
                    <!--		<span class="admin_state2">异常</span>-->
                    <!--	</div>-->
                    <!--</template>-->
                </el-table-column>
                <el-table-column label="操作" width="80" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button type="danger" size="mini" @click="deleteRow(scope)">删除</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom" style="margin-right: 8px">全选</el-checkbox>
                <el-button @click="deleteRow(null, true)" size="mini">批量删除</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                    :current-page.sync="searchForm.page" :page-size="searchForm.limit" :page-sizes="pageSizes"
                    layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
        <div class="modluDrawer">
            <el-drawer title="简历预览" :append-to-body="true" :visible.sync="resumePreviewVisible" :destroy-on-close="true" size="530px">
                <resume_preview :id="info.eid" :uid="info.uid"></resume_preview>
            </el-drawer>
        </div>
    </div>
</template>

<script>
module.exports = {
    props:{
		from: {
		    type: String,
		    default: ''
		},
        apply_tab: {
            type: Number,
            default: 1
        },
        searchable: {
            type: Boolean,
            default: true
        },
        searchword: {
            type: String,
            default: ''
        },
        searchtype: {
            type: String,
            default: ''
        },
        searchjobid: {
            type: String,
            default: ''
        },
        searchcomid: {
            type: String,
            default: ''
        },
        searchbrowse: {
            type: String,
            default: ''
        },
        searchclass: {
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
            dataText: '数据加载中',
            searchClass: '',
            searchForm: {
                page: 1,
                limit: null,
                type: '1',
                keyword: null,
                browse: null,
                times: null,
                job_id: '',
                com_id: '',
				user_id: ''
            },
            timeOptions: {
                shortcuts: [{
                    text: '昨天',
                    onClick(picker) {
                        const end = new Date();
                        const start = new Date();
                        start.setTime(start.getTime() - 3600 * 1000 * 24);
                        end.setTime(end.getTime() - 3600 * 1000 * 24);
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: '今天',
                    onClick(picker) {
                        const end = new Date();
                        const start = new Date();
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: '本周',
                    onClick(picker) {
                        const start = new Date(new Date().setHours(0, 0, 0) - (new Date().getDay() - 1) * 24 * 60 * 60 * 1000);
                        const end = new Date();
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: '上周',
                    onClick(picker) {
                        const start = new Date(new Date().setHours(0, 0, 0) - (new Date().getDay() + 6) * 24 * 60 * 60 * 1000);
                        const end = new Date(new Date().setHours(0, 0, 0) + (0 - new Date().getDay()) *24 * 60 * 60 *1000);
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: '本月',
                    onClick(picker) {
                        const end = new Date();
                        const start = new Date(new Date(new Date().getFullYear(), new Date().getMonth(), 1).setHours(0, 0, 0));
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: '上月',
                    onClick(picker) {
                        const end = new Date(new Date(new Date().getFullYear(), new Date().getMonth(), 0).setHours(23, 59, 59, 59));
                        const start = new Date(new Date(new Date().getFullYear(), new Date().getMonth() - 1, 1).setHours(0, 0, 0));
                        picker.$emit('pick', [start, end]);
                    }
                }]
            },
            total: 0,
            tableData: [],
            pageSizes: [],
            tableHig: true,
            checked: false,//全选
            isIndeterminate: false,// checkbox 的不确定状态
            selectedItem: [],
            info: {},
            resumePreviewVisible: false,
            cansearch: true,

            prevPage: 0
        }
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
    watch: {
        apply_tab: {
            handler(val){
                if (val > 1){
                    this.getList();
                }
            }
        },
        searchtype: {
            handler(val) {
                this.searchForm.type = val ? val : '1';
            },
            immediate: true,
            deep: true,
        },
        searchword: {
            handler(val) {
                this.searchForm.keyword = val;
            },
            immediate: true,
            deep: true,
        },
        searchable: {
            handler(val) {
                this.cansearch = val;
            },
            immediate: true,
            deep: true,
        },
        searchjobid: {
            handler(val) {
                if (val > 0){
                    this.searchForm.job_id = val;
                }
            },
            immediate: true,
            deep: true,
        },
        searchcomid: {
            handler(val) {
                if (val > 0){
                    this.searchForm.com_id = val;
                }
            },
            immediate: true,
            deep: true,
        },
        searchbrowse: {
            handler(val) {
                if (val > 0){
                    this.searchForm.browse = val;
                }
            },
            immediate: true,
            deep: true,
        },
        searchclass: {
            handler(val){
                this.searchClass = val;
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
				}
			},
			immediate: true,
			deep: true,
		},
    },
    methods: {
        // 跳转到会员中心
        jumpToMember: function (uid) {
            let tmpWin = window.open('', '_blank')
            var params = { uid: uid }
            httpPost('m=user&c=users_member&a=Imitate', params).then(function (result) {
                var res = result.data;
                if (res.error == 0) {
                    tmpWin.location = res.data.url
                } else {
                    message.error(res.msg)
                }
            }).catch(function (e) {
                tmpWin.close()
            })
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
			var url = 'm=user&c=company_comlog&a=index';
            httpPost(url, params,{hideloading: true}).then(function (response) {
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
                        _this.dataText = "暂无数据";
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
                    message.error('请选择要删除的数据');
                    return false;
                }
                let list = [];
                for (let item of this.selectedItem) {
                    list.push(item.id);
                }
                params.del = list;
            } else {
                // let index = scope.$index;
                // this.tableData.splice(index, 1);
                params.del = scope.row.id;
            }

            delConfirm(this, params, this.delete);
        },
        delete(params) {
            let _this = this;
            httpPost('m=user&c=company_comlog&a=deluseridjob', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success('删除成功！');
                    _this.getList();
                } else {
                    message.error('删除失败！');
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        handlePreview(scope) {
            this.info = scope.row;
            this.resumePreviewVisible = true;
        },
        handleTimeChange() {
            this.handleSearch();
        }
    },
    components: {
        'resume_preview': httpVueLoader('../../../component/resume_preview.vue'),
    }
};
</script>
<style scoped>
    .moduleElHight .moduleElTable {padding: 0;margin: 0;height: calc(100% - 110px);width: 100%;}
    .moduleElTableHig {height: calc(100% - 90px) !important;}
    .pad20{padding: 0 20px;}
</style>