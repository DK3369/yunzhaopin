<template>
    <!-- Member > Company > Behavior Records: Job application records -->
    <div class="moduleElHight" :class="searchClass == 'drawer' ? 'pad20' : ''">
        <div class="moduleElSearchInf" v-if="cansearch">
            <div class="moduleElTabInpt" style="flex-wrap: wrap;">
                <div class="moduleInptList">
                    <el-input :placeholder="lc('admin_user_weipin_00003')" @keyup.enter="handleSearch" size="small" v-model="searchForm.keyword" class="input-with-select" clearable>
                        <template #prepend><el-select v-model="searchForm.type" :placeholder="lc('wap_com_00288')">
                            <el-option :label="lc('wap_com_00288')" value="1"></el-option>
                            <el-option :label="lc('admin_user_00247')" value="2"></el-option>
                            <el-option :label="lc('wap_00529')" value="3"></el-option>
                        </el-select></template>
                    </el-input>
                </div>
                <div class="moduleInptList">
                    <el-select v-model="searchForm.browse" size="small" :placeholder="lc('admin_user_00250')" clearable @change="handleSearch">
                        <el-option :label="lc('wap_user_00260')" value="1"></el-option>
                        <el-option :label="lc('wap_user_00258')" value="2"></el-option>
                        <el-option :label="lc('admin_user_00252')" value="3"></el-option>
                        <el-option :label="lc('wap_user_00354')" value="4"></el-option>
                        <el-option :label="lc('member_com_00108')" value="5"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt tableSeachInptsmalltwo">
                    <el-date-picker v-model="searchForm.times" type="daterange" align="right" unlink-panels :range-separator="lc('admin_company_00019')" :start-placeholder="lc('admin_00588')" :end-placeholder="lc('admin_00589')" :picker-options="timeOptions" value-format="YYYY-MM-dd" size="small" @change="handleTimeChange"></el-date-picker>
                </div>
                <div class="tableSeachInpt">
                    <el-button type="primary" icon="el-icon-search" size="small" @click="handleSearch">{{ lc('admin_user_weipin_00049') }}</el-button>
                </div>
            </div>
        </div>
        <div class="moduleElTable"
            style="border: 1px solid #ebeef5; width: calc(100% - 2px); height: calc(100% - 91px);">
            <el-table :data="tableData" style="width: 100%" stripe
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%"
                ref="multipleTable" @selection-change="handleSelectionChange" @sort-change="shortChange" :default-sort="{ prop: 'id', order: 'descending' }" v-loading="loading">
                <template #empty>
                    <p>{{dataText}}</p>
                </template>
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" :label="lc('member_com_00345')" sortable="custom" width="80"></el-table-column>
                <el-table-column prop="job_name" :label="lc('wap_com_00288')" min-width="150" show-overflow-tooltip>
                    <template #default="scope">
                        <el-link :href="scope.row.job_url" target="_blank" type="primary">{{ scope.row.job_name }}
                        </el-link>
                    </template>
                </el-table-column>
                <el-table-column prop="com_name" :label="lc('admin_user_00247')" min-width="180" show-overflow-tooltip>
                    <template #default="scope">
                        <el-link :href="scope.row.com_url" target="_blank" type="primary">{{ scope.row.com_name }}
                        </el-link>
                    </template>
                </el-table-column>
                <el-table-column prop="username_n" :label="lc('wap_00529')">
                    <template #default="scope">
                        <el-button type="text" @click="handlePreview(scope)" style="padding: 0">{{ scope.row.username_n }}
                        </el-button>
                    </template>
                </el-table-column>
                <el-table-column prop="telphone" :label="lc('wap_user_00180')" width="140">
                    <template #default="scope">
                        <el-link @click="jumpToMember(scope.row.uid)" target="_blank" type="primary">{{ scope.row.telphone }}
                        </el-link>
                    </template>
                </el-table-column>
                <el-table-column prop="is_browse" :label="lc('admin_user_00250')" width="100">
                    <template #default="scope">
                        <div class="admin_state">
                            <span v-if="scope.row.is_browse == 2" class="admin_state1">{{ lc('wap_user_00258') }}</span>
                            <span v-else-if="scope.row.is_browse == 3" class="admin_state5">{{ lc('admin_user_00252') }}</span>
                            <span v-else-if="scope.row.is_browse == 4" class="admin_state3">{{ lc('wap_user_00354') }}</span>
                            <span v-else-if="scope.row.is_browse == 5" class="admin_state2">{{ lc('member_com_00108') }}</span>
                            <span v-else class="admin_state2">{{ lc('wap_user_00260') }}</span>
                            <!--<span class="admin_state1">Reviewed</span>-->
                            <!--<span class="admin_state2">Rejected</span>-->
                            <!--<span class="admin_state3">Locked</span>-->
                            <!--<span class="admin_state4">Pending review</span>-->
                            <!--<span class="admin_state5">Suspended</span>-->
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="datetime" sortable="custom" :label="lc('wap_com_00342')" width="140">
                    <template #default="scope">{{ scope.row.datetime_n_n }}</template>
                </el-table-column>
                <el-table-column prop="isdel_n" :label="lc('member_user_00181')" width="100">
                    <!--<template #default="scope">-->
                    <!--	<div class="admin_state">-->
                    <!--		<span class="admin_state1">Normal</span>-->
                    <!--		<span class="admin_state2">Abnormal</span>-->
                    <!--	</div>-->
                    <!--</template>-->
                </el-table-column>
                <el-table-column :label="lc('member_user_00048')" width="80" fixed="right">
                    <template #default="scope">
                        <div class="cz_button">
                            <el-button type="danger" size="small" @click="deleteRow(scope)">{{ lc('common.delete') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom" style="margin-right: 8px">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button @click="deleteRow(null, true)" size="small">{{ lc('member_com_00055') }}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                    v-model:current-page="searchForm.page" :page-size="searchForm.limit" :page-sizes="pageSizes"
                    layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
        <div class="modluDrawer">
            <el-drawer :title="lc('member_user_00037')" :append-to-body="true" v-model="resumePreviewVisible" :destroy-on-close="true" size="530px">
                <resume_preview :id="info.eid" :uid="info.uid"></resume_preview>
            </el-drawer>
        </div>
    </div>
</template>

<script>
import ResumePreview from '../../../component/resume_preview.vue'

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
            dataText: lc('admin_user_weipin_00026'),
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
                    text: lc('common_02000'),
                    onClick(picker) {
                        const end = new Date();
                        const start = new Date();
                        start.setTime(start.getTime() - 3600 * 1000 * 24);
                        end.setTime(end.getTime() - 3600 * 1000 * 24);
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: lc('common_01940'),
                    onClick(picker) {
                        const end = new Date();
                        const start = new Date();
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: lc('admin_user_00146'),
                    onClick(picker) {
                        const start = new Date(new Date().setHours(0, 0, 0) - (new Date().getDay() - 1) * 24 * 60 * 60 * 1000);
                        const end = new Date();
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: lc('admin_user_00142'),
                    onClick(picker) {
                        const start = new Date(new Date().setHours(0, 0, 0) - (new Date().getDay() + 6) * 24 * 60 * 60 * 1000);
                        const end = new Date(new Date().setHours(0, 0, 0) + (0 - new Date().getDay()) *24 * 60 * 60 *1000);
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: lc('admin_user_00147'),
                    onClick(picker) {
                        const end = new Date();
                        const start = new Date(new Date(new Date().getFullYear(), new Date().getMonth(), 1).setHours(0, 0, 0));
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: lc('admin_user_00143'),
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
            checked: false,//{{ lc('wap_js_00074') }}
            isIndeterminate: false,// Checkbox indeterminate state
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
        // Jump to member center
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
                        _this.dataText = lc('wap_js_00113');
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
                    message.error(lc('admin_user_weipin_00005'));
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
                    message.success(lc('admin_user_00187'));
                    _this.getList();
                } else {
                    message.error(lc('admin_user_00186'));
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
        'resume_preview': ResumePreview,
    }
};
</script>
<style scoped>
    .moduleElHight .moduleElTable {padding: 0;margin: 0;height: calc(100% - 110px);width: 100%;}
    .moduleElTableHig {height: calc(100% - 90px) !important;}
    .pad20{padding: 0 20px;}
</style>