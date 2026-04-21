<template>
    <!--会员-个人-行为记录：简历被下载记录-->
    <div class="moduleElHight">
        <div class="moduleElTable" style="border: 1px solid #ebeef5; width: calc(100% - 2px); height: calc(100% - 50px) !important;">
            <el-table :data="tableData" style="width: 100%" stripe
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" ref="multipleTable" @sort-change="shortChange" v-loading="loading" :empty-text="emptytext">
                <el-table-column label="姓名" width="230">
                    <template slot-scope="scope">
                        <div class=" ">
                            <div class="username">
                                <el-button type="text" @click="handlePreview(scope)" style="padding: 0">{{ scope.row.username }}</el-button>
                            </div>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="手机号">
                    <template slot-scope="scope">
                        <div class=" ">
                            <div class=" ">{{ scope.row.telMob }}</div>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="resume" label="被下载简历">
                    <template slot-scope="scope">
                        <el-button type="text" @click="handlePreview(scope)" style="padding: 0">{{ scope.row.resume }}</el-button>
                    </template>
                </el-table-column>
                <el-table-column prop="downtime" label="下载时间" sortable="custom">
                    <template slot-scope="scope">{{ scope.row.downtime_n }}</template>
                </el-table-column>
                <el-table-column prop="isdel_n" label="状态" width="110"></el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <!--<div>-->
                <!--<el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">全选</el-checkbox>-->
                <!--<el-button @click="deleteRow(null, true)">批量删除</el-button>-->
            <!--</div>-->
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange" :page-sizes="pageSizes"
                    :page-size="searchForm.limit" :current-page.sync="searchForm.page" layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
        <div class="modluDrawer">
            <el-drawer title="简历预览" :append-to-body="true" :visible.sync="resumePreviewVisible" :destroy-on-close="true" size="530px">
                <resume_preview :id="info.resume_id" :uid="info.uid"></resume_preview>
            </el-drawer>
        </div>
    </div>
</template>

<script>
module.exports = {
    props:{
        searchword: String,
		from: {
		    type: String,
		    default: ''
		},
        searchcomid: {
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
            emptytext: '暂无数据',
            searchForm: {
                page: 1,
                limit: null,
                type: "2",
                keyword: null,
                time: null,
				comid:''
            },
            total: 0,
            tableData: [],
            tableHig: true,
            checked: false,//全选
            isIndeterminate: false,// checkbox 的不确定状态
            selectedItem: [],
            info: {},
            resumePreviewVisible: false,
            resize: false,
            pageSizes:[],
            prevPage: 0
        }
    },
    created() {
        this.getList();
    },
    watch: {
        searchword: {
            handler(val) {
                this.searchForm.keyword = val;
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
            _this.emptytext = "数据加载中";
			var url = 'm=user&c=users_userlog&a=down';
            httpPost(url, params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    _this.tableData = res.data.list;
                    _this.total = res.data.total;
                    _this.searchForm.limit = res.data.perPage
                    _this.pageSizes = res.data.pageSizes
                    _this.loading = false;
                    if(_this.prevPage != _this.searchForm.page){
                        _this.prevPage = _this.searchForm.page;
                        _this.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    }
                    if (_this.tableData.length === 0){
                        _this.emptytext = "暂无数据";
                    }
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        handlePreview(scope) {
            this.info = scope.row;
            this.resumePreviewVisible = true;
        },
    },
    components: {
        'resume_preview': httpVueLoader('../../../component/resume_preview.vue'),
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