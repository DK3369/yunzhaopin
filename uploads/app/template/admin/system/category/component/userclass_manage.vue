<template>
    <div class="moduleElenAlCategorySub">
        <div class="moduleSeachs categorySub">
            <div></div>
            <div class="categoryTopBtn">
                <el-button class="" type="primary" icon="el-icon-document-add" size="mini" @click="addVisible = true">添加分类</el-button>
            </div>
        </div>
        <div class="moduleElTable moduleElTableCategoreSub">
            <el-table :data="tableData" border style="width: 100%" ref="multipleTable" @selection-change="handleSelectionChange"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" label="分类编号" width="100"></el-table-column>
                <el-table-column label="分类名称(点击修改)" property="name">
                    <template slot-scope="scope">
                        <el-input v-if="scope.row[scope.column.property + 'isShow']" :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                            v-model="scope.row.name" @blur="alterData(scope)"></el-input>
                        <span v-else>
                            <template v-if="scope.$index == 0">
                                一级分类：{{ scope.row.name }}
                            </template>
                            <template v-else>
                                &emsp;&emsp;┗{{ scope.row.name }}<img @click="editData(scope)" class="editIcon" src="../../../admin/images/bine.png" alt="" style="margin-left: 4px;" width="14" height="14">
                            </template>
                        </span>
                    </template>
                </el-table-column>
                <el-table-column label="排序(越小越在前)" property="sort">
                    <template slot-scope="scope">
                        <el-input v-if="scope.row[scope.column.property + 'isShow']" :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                            v-model="scope.row.sort" @blur="alterData(scope, 'int')" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"></el-input>
                        <span v-else>
                            <template v-if="scope.$index == 0"></template>
                            <template v-else>{{ scope.row.sort }}<img @click="editData(scope)" class="editIcon" src="../../../admin/images/bine.png" alt="" style="margin-left: 4px;" width="14" height="14"></template>
                        </span>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" label="操作" width="70">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button type="danger" size="mini" @click="deleteRow(scope)">删除</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="">
                <div class="">
                    <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">全选</el-checkbox>
                    <el-button size="mini" @click="deleteRow(null, true)">批量删除</el-button>
                </div>
            </div>
        </div>
        <el-dialog title="个人会员分类" width="40%" :visible.sync="addVisible" :modal-append-to-body="false" :append-to-body="true">
            <userclass_add :position="position" @child-event-getlist="handleList"></userclass_add>
        </el-dialog>
    </div>
</template>

<script setup>
module.exports = {
    props: {
        id: {type: [Number, String], default: 0},
    },
    data: function () {
        return {
            emptytext: '暂无数据',
            tableData: [], //表格数据
            checked: false,
            isIndeterminate: false,// checkbox 的不确定状态
            selectedItem: [],
            position: [],//一级分类列表
            addVisible: false,
            oldData: null,
			loading: true
        }
    },
    created() {
        console.log('comclass_manage created id', this.id);
        this.getList();
    },
    methods: {
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
        alterData(scope, type) {
            if (this.oldData == null) {
                return false;
            }
            let index = scope.$index;
            let row = scope.row;
            let column = scope.column;
            if (type === 'int') {
                row[column.property] = row[column.property].replace(/[^0-9]/g, '');
            }
            let copyRow = JSON.parse(JSON.stringify(row));
            copyRow[column.property + "isShow"] = false;
            this.$set(this.tableData, index, copyRow);
            if (row[column.property] === this.oldData[column.property]) {
                return false;
            }
            let _this = this;
            let sendData = {id: row.id};
            sendData[column.property] = row[column.property];
            httpPost('m=system&c=category_userclass&a=ajax', sendData, {hideloading: true}).then(function (response) {
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
            let params = {id: this.id};
			_this.loading = true;
            _this.emptytext = "数据加载中";
            httpPost('m=system&c=category_userclass&a=up', params).then(function (response) {
                let res = response.data;
                let list = [];
                if (res.data.class1) {
                    list.push(res.data.class1);
                }
                for (let item of res.data.class2) {
                    list.push(item);
                }
                _this.tableData = list;
                _this.position = res.data.position;
				_this.loading = false;
                if (_this.tableData.length === 0){
                    _this.emptytext = "暂无数据";
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        handleList(ctype) {
            if (ctype == 1) {
                this.addVisible = false;
                this.$emit("child-event-getlist");
            } else {
                this.getList();
            }
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
    },
    components: {
        'userclass_add': httpVueLoader('./userclass_add.vue'),
    }
}
</script>

<style scoped>
	.el-table .el-table__cell { padding: 12px 0; }
</style>