<template>
    <div class="moduleElHight">
		<div class="tableDome_tip">
			<el-alert :title="lc('admin_tool_00587')" type="success" :closable="false">
			</el-alert>
		</div>
        <div class="moduleSeachs">
            <div class="moduleSeachleft">
                
                <div class="tableSeachInpt" style="margin-bottom: 0px;;">
                    <el-input :placeholder="lc('admin_tool_00591')" v-model="keyword" size="small" prefix-icon="el-icon-search" clearable>
                    </el-input>
                </div>

                <div class="tableSeachInpt" style="margin-bottom: 0px;;">
                    <el-button type="primary" icon="el-icon-search" size="small" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
                </div>
            </div>
            
        </div>

        <div class="moduleElTable">
            <el-table  ref="table" :data="tableData" v-loading="list_loading" @selection-change="selectionChange" border style="width: 100%"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" :empty-text="emptytext">
                <el-table-column type="selection" width="55">
                </el-table-column>
                <el-table-column prop="uid" label="	uid" width="80">
                </el-table-column>
                <el-table-column prop="username" :label="lc('admin_vue_00042')"  >
                </el-table-column>
                <el-table-column prop="wxid" :label="lc('admin_tool_00588')" >
                </el-table-column>
				 
                
                <el-table-column prop="wxbindtime_n" :label="lc('admin_vue_00043')" width="180">
                </el-table-column>
              
            </el-table>
        </div>


        <div class="modulePaging">
            <div class="modulecz">
                <el-checkbox v-model="allchecked" @change="allcheckChange">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button  size="small" @click="deleteAll">{{ lc('member_user_00054') }}</el-button>
            </div>
            <div class="modulePagNum"  >
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                    :current-page="currentPage" :page-size="limit" :page-sizes="page_sizes" :total="total"
                    layout="total, sizes, prev, pager, next, jumper">
                </el-pagination>
            </div>
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
            emptytext: window.yunAdminT(lc('wap_js_00113')),
            tableData: [],
            total: 0,
            limit: 0,
            currentPage: 1,
			prevPage:0,
            page_sizes: [],

            list_loading: false,
            
            allchecked: false,
            choosedata: [],

            keyword:'',
            type:'1',
            status:'',
            time:'',
            daterange:[],
            sort_t:'',
            order:'',
        }
    },

    mounted() {
        this.getList();
    },
    methods: {
        async getList() {
            let that = this;
            let params = {
                page: that.currentPage,
                limit: that.limit,
                t:that.sort_t,
                order:that.order,
                keyword:that.keyword,
            }
            
            this.list_loading = true;
            that.emptytext = window.yunAdminT(lc('admin_user_weipin_00026'));
            httpPost('m=tool&c=weixinrecord&a=userbd', params, {hideloading: true}).then((result) => {
                this.list_loading = false;
                var res = result.data;

                if (res.error == 0) {
                    that.tableData = res.data.list
                    that.total = parseInt(res.data.total)
                    that.page_sizes = res.data.page_sizes;
					that.limit = res.data.page_size;
					
					if(that.prevPage != that.currentPage){
						that.prevPage = that.currentPage;
						that.$refs.table.bodyWrapper.scrollTop = 0;
					}
                    if (that.tableData.length === 0){
                        that.emptytext = window.yunAdminT(lc('wap_js_00113'));
                    }
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        search: function () {
            this.currentPage = 1;
            this.getList();
        },
        
        handleCurrentChange(val) {
            this.currentPage = val;
            this.getList()
        },
        handleSizeChange(val) {
            this.currentPage = 1
            this.limit = val
            this.getList()
        },
        allcheckChange: function () {

            this.$refs.table.toggleAllSelection();

        },
        selectionChange: function (e) {
            if (this.tableData.length != e.length) {
                this.allchecked = false;
            } else {
                this.allchecked = true;
            }
            this.choosedata = e;
        },
        
        deleteAll: function () {
            var _this = this;
            var idarr = [];
            if (this.choosedata.length > 0) {
                for (let i in this.choosedata) {
                    idarr.push(this.choosedata[i].uid);
                }
            } else {
                message.error(window.yunAdminT(lc('admin_tool_00592'))); return;
            }
            var params = {
                del: idarr
            };

            delConfirm(_this, params, this.deletePost,window.yunAdminT(lc('admin_tool_00593')))
        },
        async deletePost(params) {

            let that = this;

            httpPost('m=tool&c=weixinrecord&a=deluser', params).then(function (result) {

                var res = result.data;
                if (res.error == 0) {
                    message.success(res.msg, function () { that.getList(); }); return;
                } else {
                    message.error(res.msg); return;
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        doLayout(){
            if (this.$refs.table) {
                this.$nextTick(() => {
                    this.$refs.table.doLayout();
                })
            }
        }
    },
};
</script>
<style scoped>
.moduleSeachmore {
    padding: 0px;
}

.moduleSeachs {
    padding: 0px 0px 12px 0px;
    width: 100%;
}

 .moduleElTable {
    padding: 0;
    margin: 0;
  height: calc(100% - 136px);
    width: 100%;
}

.tableSeachInptsmalltwo {
    margin-bottom: 0px;
    margin-right: 12px;
}

.tableSeachInptsmalltwo .el-input__inner {
    height: 32px;
    line-height: 32px;
    width: 260px;
    padding: 0px 5px;
    ;
} 
.el-dialog__body{padding:0px 20px;}
</style>