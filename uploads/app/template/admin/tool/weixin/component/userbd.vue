<template>
    <div class="moduleElHight">
		<div class="tableDome_tip">
			<el-alert title="用户绑定是指微信用户通过“微信开放平台（微信登录）”方式后，进行绑定帐号，系统会自动记录用户绑定记录" type="success" :closable="false">
			</el-alert>
		</div>
        <div class="moduleSeachs">
            <div class="moduleSeachleft">
                
                <div class="tableSeachInpt" style="margin-bottom: 0px;;">
                    <el-input placeholder="请输入绑定用户" v-model="keyword" size="small" prefix-icon="el-icon-search" clearable>
                    </el-input>
                </div>

                <div class="tableSeachInpt" style="margin-bottom: 0px;;">
                    <el-button type="primary" icon="el-icon-search" size="mini" @click="search">查询</el-button>
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
                <el-table-column prop="username" label="	绑定用户"  >
                </el-table-column>
                <el-table-column prop="wxid" label="绑定ID（OpenId）" >
                </el-table-column>
				 
                
                <el-table-column prop="wxbindtime_n" label="	绑定时间" width="180">
                </el-table-column>
              
            </el-table>
        </div>


        <div class="modulePaging">
            <div class="modulecz">
                <el-checkbox v-model="allchecked" @change="allcheckChange">全选</el-checkbox>
                <el-button  size="mini" @click="deleteAll">取消绑定</el-button>
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
module.exports = {
    data: function () {
        return {
            emptytext: '暂无数据',
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
            that.emptytext = "数据加载中";
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
                        that.emptytext = "暂无数据";
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
                message.error('请选择要取消绑定的数据'); return;
            }
            var params = {
                del: idarr
            };

            delConfirm(_this, params, this.deletePost,'确定取消绑定？')
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