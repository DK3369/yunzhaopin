<template>
    <div class="moduleElenAlCategorySub">
        <div class="moduleElTable moduleElTableCategoreSub">
            <el-table :data="tableData" stripe border style="width: 100%;" height="100%"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" ref="multipleTable"
                @selection-change="handleSelectionChange" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" :label="lc('member_com_00345')" width="80"></el-table-column>
                <el-table-column :label="lc('admin_system_00109')" property="name">
                    <template slot-scope="scope">
                        <el-input v-if="scope.row[scope.column.property + 'isShow']"
                            :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                            v-model="scope.row.name" @blur="alterData(scope)"></el-input>
                        <span v-else>
                            <template>
                                <template v-if="scope.row.level == 'one'">{{ lc('admin_system_00111') }}</template>
                                <template v-else-if="scope.row.level == 'two'">&emsp;{{ lc('admin_00291') }}：┗</template>
                                <template v-else-if="scope.row.level == 'three'">&emsp;&emsp;┗</template>
                                {{ scope.row.name }}<img @click="editData(scope)" class="editIcon"
                                src="../../../admin/images/bine.png" alt="" style="margin-left: 4px;" width="14"
                                height="14">
                            </template>
                        </span>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('admin_system_00108')" property="e_name">
                    <template slot-scope="scope">
                        <el-input v-if="scope.row[scope.column.property + 'isShow']"
                            :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                            v-model="scope.row.e_name" @blur="alterData(scope)"></el-input>
                        <span v-else>
                            {{ scope.row.e_name }}<img @click="editData(scope)" class="editIcon"
                            src="../../../admin/images/bine.png" alt="" style="margin-left: 4px;" width="14" height="14">
                        </span>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('admin_system_00110')" property="s_name">
                    <template slot-scope="scope">
                        <el-input v-if="scope.row[scope.column.property + 'isShow']"
                            :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                            v-model="scope.row.s_name" @blur="alterData(scope)"></el-input>
                        <span v-else>
                            {{ scope.row.s_name }}<img @click="editData(scope)" class="editIcon"
                            src="../../../admin/images/bine.png" alt="" style="margin-left: 4px;" width="14"
                            height="14">
                        </span>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('admin_system_00114')" property="rec" width="80">
                    <template slot-scope="scope">
                        <el-switch v-model="scope.row.rec_n" @change="changeRec(scope)"></el-switch>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('admin_system_00113')" property="sort" width="80">
                    <template slot-scope="scope">
                        <el-input v-if="scope.row[scope.column.property + 'isShow']"
                            :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                            v-model="scope.row.sort" @blur="alterData(scope, 'int')"
                            onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"></el-input>
                        <span v-else>
                            {{ scope.row.sort }}<img @click="editData(scope)" class="editIcon"
                            src="../../../admin/images/bine.png" alt="" style="margin-left: 4px;" width="14" height="14">
                        </span>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('admin_system_00118')" width="110">
                    <template slot-scope="scope">
                        <el-link v-if="scope.row.level != 'one'" :underline="false" type="primary" @click="editRow(scope)">{{ lc('admin_system_00119') }}</el-link>
                    </template>
                </el-table-column>
                <el-table-column header-align="center" align="right" :label="lc('member_user_00048')" :width="210">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button v-if="scope.row.level == 'two'" size="mini" @click="moveRow(1, scope)">{{ lc('admin_system_00115') }}</el-button>
                            <el-button v-if="scope.row.level == 'three'" size="mini" @click="moveRow(2, scope)">{{ lc('admin_system_00115') }}</el-button>
                            <el-button v-if="scope.row.level == 'two' || scope.row.level == 'three'" size="mini" @click="editRow(scope)">{{ lc('wap_js_00073') }}</el-button>
                            <el-button size="mini" @click="deleteRow(scope)" type="danger">{{ lc('common.delete') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="">
                <div class="">
                    <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
                    <el-button size="mini" @click="deleteRow(null, true)">{{ lc('member_com_00055') }}</el-button>
                </div>
            </div>
        </div>
        <div class="modluDrawer">
            <el-drawer :title="titleAddEdit" :visible.sync="addVisible" :modal-append-to-body="false" :append-to-body="true" :show-close="true"
                :destroy-on-close="true" size="80%">
                <job_class_edit :tid="tid" :id="threeid" @child-event-getlist="getList"></job_class_edit>
            </el-drawer>
        </div>
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_system_00117')" :visible.sync="moveVisible" :with-header="true" :modal-append-to-body="false" :append-to-body="true"
                :show-close="true" width="30%">
                <div class="dialog_item">
                    <div class="item_span">
                        <span>{{ lc('admin_system_00111') }}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-select v-model="moveForm.nid" :placeholder="lc('wap_user_00100')" style="flex: 1;"
                            @change="getClass(moveForm.nid)" clearable>
                            <el-option v-for="item in position" :key="item.id" :label="item.name"
                                :value="item.id"></el-option>
                        </el-select>
                    </div>
                </div>
                <template v-if="moveForm.type == 2">
                    <div class="dialog_item">
                        <div class="item_span">
                            <span>{{ lc('admin_system_00112') }}</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-select v-model="moveForm.keyid" :placeholder="lc('wap_user_00100')" style="flex: 1;" clearable>
                                <el-option v-for="item in positionTwo" :key="item.id" :label="item.name"
                                    :value="item.id"></el-option>
                            </el-select>
                        </div>
                    </div>
                </template>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="moveVisible = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="moveSubmit">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
    </div>
</template>
<script>
module.exports = {
    props: {
        id: {type: [Number, String], default: 0},
    },
    data: function () {
        return {
            tableData: [], //表格数据
            checked: false,
            isIndeterminate: false,// checkbox 的不确定状态
            selectedItem: [],
            oldData: null,
            position: [],
            positionTwo: [],//第二级分类
            moveVisible: false,
            moveForm: {
                pid: null, //自身id
                type: 0,
                nid: null, //父类id
                keyid: null //第二级分类id
            },
            tid: 0,
            threeid: 0,
            addVisible: false,
            titleAddEdit: window.yunAdminT(lc('admin_00222')),
            loading: false,
            emptytext: window.yunAdminT(lc('wap_js_00113')),
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
        getList() {
            this.addVisible = false;
            let _this = this;
            //清空列表
            let newlist = [];
            _this.loading = true;
            _this.emptytext = window.yunAdminT(lc('admin_user_weipin_00026'));
            httpPost('m=system&c=category_job_class&a=up", {id: this.id}).then(function (response) {
                let res = response.data;
                _this.position = res.data.position;
                //{{ lc('admin_yunying_00145') }}
                res.data.onejob.level = "one";
                newlist.push(res.data.onejob);
                if (Array.isArray(res.data.twojob)) {
                    for (let twoitem of res.data.twojob) {
                        //{{ lc('admin_yunying_00147') }}
                        twoitem.level = "two";
                        newlist.push(twoitem);
                        //{{ lc('admin_yunying_00146') }}
                        let tow_class_id = twoitem.id;
                        let threeList = res.data.threejob[tow_class_id];
                        if (Array.isArray(threeList)) {
                            for (let threeitem of threeList) {
                                threeitem.level = "three';
                                newlist.push(threeitem);
                            }
                        }
                    }
                }
                newlist.forEach((item) => {
                    item.rec_n = item.rec > 0 ? true : false;
                });
                _this.tableData = newlist;
                _this.loading = false;
                if (_this.tableData.length === 0){
                    _this.emptytext = window.yunAdminT(lc('wap_js_00113'));
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        deleteRow(scope, isMore) {
            let params = {};
            if (isMore) {
                if (!this.selectedItem.length) {
                    message.error(window.yunAdminT(lc('admin_user_weipin_00005')));
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
            httpPost('m=system&c=category_job_class&a=del', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(window.yunAdminT(lc('admin_user_00187')));
                    _this.getList();
                } else {
                    message.error(window.yunAdminT(lc('admin_user_00186')));
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
            httpPost('m=system&c=category_job_class&a=ajax', sendData, {hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(window.yunAdminT(lc('admin_user_company_00208')));
                } else {
                    message.error(window.yunAdminT(lc('admin_00187')));
                }
                _this.oldData = null;
                _this.getList();
            }).catch(function (error) {
                console.log(error);
            });
        },
        changeRec(scope) {
            let recVal = scope.row.rec_n ? 1 : 0;
            let recBefore = scope.row.rec;
            if (recBefore == recVal) {
                return;
            }
            let _this = this;
            let params = {rec: recVal, id: scope.row.id};
            httpPost('m=system&c=category_job_class&a=setrec', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    _this.getList();
                    message.success(window.yunAdminT(lc('admin_user_company_00208')));
                } else {
                    message.error(window.yunAdminT(lc('admin_00187')));
                }
            }).catch(function (error) {
                console.log(error);
            })
        },
        getClass(nid) {
            let _this = this;
            if (this.moveForm.type == 1 || nid <= 0) {
                _this.moveForm.keyid = null;
                _this.positionTwo = [];
                return false;
            }
            this.moveForm.keyid = null;
            httpPost('m=system&c=category_job_class&a=get_class", {nid: nid}).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    _this.positionTwo = res.data;
                } else {
                    _this.positionTwo = [];
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        /**
         * {{ lc('admin_system_00115') }}
         * @param type 1 获取一级分类，2 获取二级分类
         * @param scope
         */
        moveRow(type, scope) {
            this.moveForm.pid = scope.row.id;
            this.moveForm.type = type;
            if (type == 2) {
                this.getClass(this.moveForm.nid);
            }
            this.moveVisible = true;
        },
        moveSubmit() {
            this.moveVisible = false;
            let _this = this;
            delConfirm(this, this.moveForm, function (params) {
                if (params.type == 1) {
                    params.keyid = null;
                }
                httpPost("m=system&c=category_job_class&a=move', params).then(function (response) {
                    let res = response.data;
                    if (res.error === 0) {
                        message.success(window.yunAdminT(lc('admin_system_00122')))
                        _this.getList();
                    } else {
                        message.error(window.yunAdminT(lc('admin_system_00121')))
                    }
                }).catch(function (error) {
                    console.log(error);
                });
            }, window.yunAdminT(lc('admin_system_00120')));
        },
        editRow(scope) {
            let item = scope.row;
            // let v = Date.now();
            if (item.level === 'two') {
                // window.location.href = './job_class_add.html?v=' + v + '&tid=' + item.id;
                this.tid = item.id;
                this.threeid = 0;
            } else if (item.level === 'three') {
                // window.location.href = './job_class_add.html?v=' + v + '&id=' + item.id;
                this.tid = 0;
                this.threeid = item.id;
            }
            this.titleAddEdit = window.yunAdminT(lc('admin_00221'));
            this.addVisible = true;
        }
    },
    components: {
        'job_class_edit': httpVueLoader('./job_class_edit.vue'),
    }
}
</script>
<style scoped>
.moduleElTableCategoreSub {
    height: calc(100% - 60px);
}.el-table .el-table__cell {
    padding: 12px 0;
}
</style>