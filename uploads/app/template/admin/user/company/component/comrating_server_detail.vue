<template>
    <!--会员-企业-套餐服务：增值服务-详情-->
    <div class="jobspecial_box">
        <div class="moduleSeachbig">
        </div>
        <div class="admin_datatip">
            <i class="el-icon-document"></i>
            该页面您设置的增值服务详细信息，可对增值服务进行选择查看，编辑，删除等操作
        </div>
        <div class="moduleElenAlRight">
            <div class="moduleElTable" style="height: calc(100% - 70px);">
                <el-table :data="tableData" style="width: 100%" stripe
                    :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%"
                    ref="multipleTable" @selection-change="handleSelectionChange" @sort-change="shortChange" v-loading="loading" :empty-text="emptytext">
                    <el-table-column type="selection" width="55"></el-table-column>
                    <el-table-column prop="id" label="编号" width="80"></el-table-column>
                    <el-table-column prop="service_price" label="服务价格">
                        <template slot-scope="scope">{{scope.row.service_price}}元</template>
                    </el-table-column>
                    <el-table-column prop="resume" label="下载简历">
                        <template slot-scope="scope">{{scope.row.resume}}份</template>
                    </el-table-column>
                    <el-table-column prop="interview" label="邀请面试">
                        <template slot-scope="scope">{{scope.row.interview}}份</template>
                    </el-table-column>
                    <el-table-column prop="job_num" label="上架职位">
                        <template slot-scope="scope">{{scope.row.job_num}}份</template>
                    </el-table-column>
                    <el-table-column prop="breakjob_num" label="刷新职位">
                        <template slot-scope="scope">{{scope.row.breakjob_num}}份</template>
                    </el-table-column>
                    <el-table-column prop="top_num" label="职位置顶">
                        <template slot-scope="scope">{{scope.row.top_num}}天</template>
                    </el-table-column>
                    <el-table-column prop="rec_num" label="职位推荐">
                        <template slot-scope="scope">{{scope.row.rec_num}}天</template>
                    </el-table-column>
                    <el-table-column prop="urgent_num" label="职位紧急">
                        <template slot-scope="scope">{{scope.row.urgent_num}}天</template>
                    </el-table-column>
                    <el-table-column prop="zph_num" label="报名招聘会" width="100">
                        <template slot-scope="scope">{{scope.row.zph_num}}次</template>
                    </el-table-column>
                    <el-table-column prop="sort" label="排序" width="80"></el-table-column>
                    <el-table-column label="操作" width="140" fixed="right">
                        <template slot-scope="scope">
                            <div class="cz_button">
                                <el-button size="small" plain @click="editRow(scope)">修改</el-button>
                                <el-button type="danger" size="small" @click="deleteRow(scope)">删除</el-button>
                            </div>
                        </template>
                    </el-table-column>
                </el-table>
            </div>
            <div class="modulePaging">
                <div class="modulecz modulePagButn">
                    <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">全选</el-checkbox>
                    <el-button size="mini" @click="deleteRow('',1)">批量删除</el-button>
                </div>
                <div class="modulePagNum">
                </div>
            </div>
        </div>
        <!--修改-->
        <el-drawer title="设置增值包" :visible.sync="addVisible" :destroy-on-close="true" :modal-append-to-body="false" :append-to-body="true" :wrapper-closable="false" size="770px">
            <comrating_server_detail_edit :tid="info?info.id:0" @child-event-list="handleEditClose"></comrating_server_detail_edit>
        </el-drawer>
    </div>
</template>

<script>
module.exports = {
    props: {
        id: {type: [Number, String], default: 0},
    },
    data: function () {
        return {
            loading: false,
            emptytext: '暂无数据',
            searchForm: {
                page: 1,
                limit: null,
            },
            total: 0,
            tableData: [],
            tableHig: true,
            checked: false,//全选
            isIndeterminate: false,// checkbox 的不确定状态
            selectedItem: [],
            addVisible: false,
            config: {},
            info: {},
        }
    },
    created() {
        this.getList();
    },
    methods: {
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
        getList() {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.searchForm));
            for (let index in params) {
                (params[index] === '') && (params[index] = null);
            }
            params.id = this.id;
            _this.loading = true;
            _this.emptytext = "数据加载中";
            httpPost('m=user&c=company_comrating&a=list', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    _this.tableData = res.data.list;
                    _this.config = res.data.config;
                    _this.loading = false;
                    if (_this.tableData.length === 0){
                        _this.emptytext = "暂无数据";
                    }
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        handleEditClose() {
            this.info = {};
            this.addVisible = false;
            this.getList();
        },
        editRow(scope) {
            // this.titleAddEdit = '修改增值套餐';
            this.info = scope.row;
            this.addVisible = true;
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
                params.id = scope.row.id;
            }

            delConfirm(this, params, this.delete);
        },
        delete(params) {
            let _this = this;
            httpPost('m=user&c=company_comrating&a=del', params).then(function (response) {
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
    },
    components: {
        'comrating_server_detail_edit': httpVueLoader('./comrating_server_detail_edit.vue'),
    }
};
</script>
<style scoped>
.jobspecial_box{
    overflow: hidden;
    position: relative;
    padding: 0 20px;
    height: 100%;
}
.moduleElenAlRight{
    overflow: hidden;
    position: relative;
    width: 100%;
    height: calc(100% - 30px);
}
.moduleElenAlRight .moduleElTable{
    overflow: hidden;
    position: relative;
    width: 100%;
    height: calc(100% - 30px);
    padding: 0;
}
</style>
