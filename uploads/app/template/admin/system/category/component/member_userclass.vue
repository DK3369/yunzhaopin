<template>
    <!--系统-类别-会员分类：个人会员-->
    <div class="fl_cont">
        <div class="fl_cont_categoryHeader">
            <div></div>
            <div>
                <el-button type="primary" @click="addVisible = true" icon="el-icon-document-add" size="mini">添加分类</el-button>
            </div>
        </div>
        <div class="fl_cont_table">
            <el-table :data="tableData" stripe border style="width: 100%"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" ref="multipleTable"
                @selection-change="handleSelectionChange" height="calc(100% - 60px)" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" label="分类编号" width="100"></el-table-column>
                <el-table-column label="分类名称(点击修改)" property="name">
                    <template slot-scope="scope">
                        <el-input v-if="scope.row[scope.column.property + 'isShow']" :ref="scope.column.property + scope.$index"
                            :id="scope.column.property + scope.$index" v-model="scope.row.name" @blur="alterData(scope)"></el-input>
                        <span v-else>
                            一级分类：{{ scope.row.name }}<img @click="editData(scope)" class="editIcon"
                            src="../../../admin/images/bine.png" alt="" style="margin-left: 4px;" width="14" height="14">
                        </span>
                    </template>
                </el-table-column>
                <el-table-column prop="variable" label="分类变量名"></el-table-column>
                <el-table-column label="操作" width="140">
                    <template slot-scope="scope">
                        <div class="moduleElTaCaoz">
                            <el-button size="mini" @click="openManage(scope)">管理</el-button>
                            <el-button type="danger" size="mini" @click="deleteRow(scope)">删除</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
            <div class="pageFenyAlls">
                <div class="daochuButton">
                    <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">全选</el-checkbox>
                    <el-button size="mini" @click="deleteRow(null, true)">批量删除</el-button>
                </div>
            </div>
        </div>
        <el-dialog title="个人会员分类" width="580px" :visible.sync="addVisible" :modal-append-to-body="false">
            <userclass_add :position="tableData" @child-event-getlist="getList"></userclass_add>
        </el-dialog>
        <div class="modluDrawer">
            <el-drawer title="管理" :visible.sync="manageVisible" @close="getList" :append-to-body="true"
                :modal-append-to-body="false" :show-close="true" :destroy-on-close="true" size="800px">
                <userclass_manage :id="info.id" @child-event-getlist="getList"></userclass_manage>
            </el-drawer>
        </div>
    </div>
</template>

<script>
module.exports = {
    data: function () {
        return {
            tableData: [], //表格数据
            checked: false,
            isIndeterminate: false,// checkbox 的不确定状态
            selectedItem: [],
            addVisible: false,
            oldData: null,
            info: {},
            manageVisible: false,
			loading: false,
            emptytext: '暂无数据',
        };
    },
    mounted() {
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
        getList() {
            this.addVisible = false;
            let _this = this;
			_this.loading = true;
            _this.emptytext = "数据加载中";
            httpPost('m=system&c=category_userclass&a=index').then(function (response) {
                let res = response.data;
                _this.tableData = res.data;
				_this.loading = false;
                if (_this.tableData.length === 0){
                    _this.emptytext = "暂无数据";
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
                params.delType = 'more';
                params.del = list;
            } else {
                // let index = scope.$index;
                // this.tableData.splice(index, 1);
                params.delType = 'single';
                params.delid = scope.row.id;
            }

            delConfirm(this, params, this.delete);
        },
        delete(params) {
            let _this = this;
            httpPost('m=system&c=category_userclass&a=del', params).then(function (response) {
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
        editData(scope) {
            let index = scope.$index;
            let row = scope.row;
            let column = scope.column;
            this.oldData = JSON.parse(JSON.stringify(row));
            let copyRow = JSON.parse(JSON.stringify(row));
            copyRow[column.property + "isShow"] = true;
            this.$set(this.tableData, index, copyRow);
            this.$nextTick(() => {
                let ref = column.property + index;
                $("#" + ref).focus();
            });
        },
        alterData(scope) {
            if (this.oldData == null) {
                return false;
            }
            let index = scope.$index;
            let row = scope.row;
            let column = scope.column;
            let copyRow = JSON.parse(JSON.stringify(row));
            copyRow[column.property + "isShow"] = false;
            this.$set(this.tableData, index, copyRow);
            if (row[column.property] === this.oldData[column.property]) {
                return false;
            }
            let _this = this;
            let sendData = {id: row.id};
            sendData[column.property] = row[column.property];
            httpPost('m=system&c=category_userclass&a=ajax', sendData).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success('修改成功');
                } else {
                    message.error('修改失败');
                }
                _this.oldData = null;
                _this.getList();
            }).catch(function (error) {
                console.log(error);
            });
        },
        openManage(scope) {
            this.info = scope.row;
            this.manageVisible = true;
        }
    },
    components: {
        'userclass_add': httpVueLoader('./userclass_add.vue'),
        'userclass_manage': httpVueLoader('./userclass_manage.vue'),
    }
};
</script>
<style scoped>
.cell span {
    display: flex;
    align-items: center;
}

.el-tabs__header {
    display: none !important;
}

.el-tabs__content {
    padding: 0;
}

.dialog_item {
    margin-top: 25px;
    display: flex;
}

.item_span {
    width: 75px;
    text-align: right;
    display: block;
}

.editIcon {
    padding-left: 5px;
    cursor: pointer;
}

.fl_cont {
    overflow: hidden;
    position: relative;
    width: 100%;
    height: 100%;
}

.fl_cont_categoryHeader {
    margin-bottom: 10px;
    display: flex;
    justify-content: space-between;
}

.fl_cont_table {
    overflow: hidden;
    position: relative;
    width: 100%;
    height: calc(100% - 38px);
}

.fl_cont_table .el-table {
    overflow: hidden;
    position: relative;
    width: 100%;
}
		.el-table .el-table__cell {
	    padding: 12px 0;
	} 
</style>