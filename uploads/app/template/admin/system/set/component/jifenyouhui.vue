<template>
    <div class="moduleElHight">
        <div class="moduleElTable" style="height: calc(100% - 50px);">
            <el-table ref="table" :data="tableData" border style="width: 100%" @selection-change="selectChange"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55">
                </el-table-column>
                <el-table-column prop="id" label="编号" width="220">
                </el-table-column>
                <el-table-column label="积分 (点击修改)" width="300">
                    <template slot-scope="scope">
                        <div class="moduleElTaPax">
                            <template v-if="scope.row.isEditjifen">
                                <el-input v-model="scope.row.integral" type="text" @blur="changeRow(scope, 'jifen')" />
                            </template>
                            <template v-else>
                                <span>{{ scope.row.integral }}</span>
                            </template>
                            <img src="../../../admin/images/bine.png" @click="editRow(scope, 'jifen')">
                        </div>

                    </template>
                </el-table-column>
                <el-table-column label="折扣">
                    <template slot-scope="scope">
                        <div class="moduleElTaPax">
                            <template v-if="scope.row.isEditdiscount">
                                <el-input v-model="scope.row.discount" type="text"
                                    onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"
                                    @blur="changeRow(scope, 'discount')" />
                            </template>
                            <template v-else>
                                <span>{{ scope.row.discount / 10 }}折</span>
                            </template>
                            <img src="../../../admin/images/bine.png" @click="editRow(scope, 'discount')">
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="是否开启" width="220">
                    <template slot-scope="scope">
                        <el-switch v-model="scope.row.status" active-color="#1890FF" inactive-color="#B8BDC9"
                            @change="isOpen(scope)">
                        </el-switch>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" label="操作" width="80">
                    <template slot-scope="scope">
                        <div class="moduleElTaCaoz">
                            <el-button size="mini" type="danger" @click="deljf(scope.row)">删除</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
				<el-checkbox v-model="allchecked" @change="allcheckChange">全选</el-checkbox>
                <el-button size="mini" @click="editDelBatch">删除</el-button>
            </div>
        </div>
    </div>
</template>
    
<script>
module.exports = {
    data: function () {
        return {
            emptytext: '暂无数据',
            loading: false,
            input3: '',
            select: '',
            value: true,
            currentPage4: 4,
            uri: "m=system&c=",
            tableData: [],
            editData: null,

			allchecked: false,
			choosedata: [],
			idsArr:[],

        }
    },
    created() {
        this.list();
    },
    methods: {
        handleSizeChange(val) {
            console.log(`每页 ${val} 条`);
        },
        handleCurrentChange(val) {
            console.log(`当前页: ${val}`);
        },
        list() {
            let _this = this;
            _this.loading = true;
            _this.emptytext = "数据加载中";
            let url = _this.uri + 'set_integral&a=class';
            httpPost(url, {}).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    _this.tableData = res.data;
                    _this.loading = false;
                    if (_this.tableData.length === 0){
                        _this.emptytext = "暂无数据";
                    }
                }
            })
        },
        editRow(scope, fieldName) {
            let index = scope.$index;
            let item = scope.row;
            let isEditFieldName = 'isEdit' + fieldName;
            for (let i in this.tableData) {
                if (index != i) {
                    this.tableData[i][isEditFieldName] = false;
                }
            }
            this.editData = JSON.parse(JSON.stringify(this.tableData[index]))
            this.tableData[index][isEditFieldName] = true;
        },
        changeRow(scope, fieldName) {
			
            let _this = this;
            let index = scope.$index;
            let item = scope.row;
            let isEditFieldName = 'isEdit' + fieldName;
            let sendData = { id: item.id };
			
			if(fieldName=='jifen'){
				
				if(item.integral!=''){
					if (item.integral != this.editData.integral) {
					    sendData.integral = item.integral;
					}else{
						_this.tableData[index][isEditFieldName] = false;return;
					}
				}else{
					message.warning('积分不能为空');
					_this.tableData[index][isEditFieldName] = false;return;
					
				}
			}else if(fieldName=='discount'){
				if(item.discount!=''){
					
					if (item.discount != this.editData.discount) {
					    sendData.discount = item.discount;
					}else{
						_this.tableData[index][isEditFieldName] = false;return;
					}
				}else{
					message.warning('折扣不能为空');
					_this.tableData[index][isEditFieldName] = false;return;
				}
			}
            
            let url = _this.uri + 'set_integral&a=ajax';
            httpPost(url, sendData).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    message.success('操作成功');
                } else {
                    message.error('操作失败');
                }
                _this.tableData[index][isEditFieldName] = false;
                _this.editData = null
                _this.list();
            }).catch(function (error) {
                console.log(error);
            });
        },
        isOpen: function (scope) {
            var status = scope.row.status ? 1 : 0;
            var id = scope.row.id
            let _this = this;
            let url = _this.uri + 'set_integral&a=ajax';
            let sendData = { id: id, rec: status, type: 'state' };
            httpPost(url, sendData).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    message.success('操作成功');
                } else {
                    message.error('操作失败');
                }
                _this.list();
            }).catch(function (error) {
                console.log(error);
            });
        },
        deljf(row){
            delConfirm(this, {delid:row.id}, this.del, '你确定要删除该条数据？');
        },
        del: function (sendData) {
            let _this = this;
            
            let url = this.uri + 'set_integral&a=del';
            httpPost(url, sendData).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    message.success(res.msg, _this.list());
                }
            })
        },
        selectChange: function (val) {
            this.idsArr = [];
            let _this = this;
            if (val.length) {
                val.forEach(item => {
                    _this.idsArr.push(item.id);
                });
            }
			if (this.tableData.length != val.length) {
			    this.allchecked = false;
			} else {
			    this.allchecked = true;
			}
        },
		allcheckChange: function () {
		
		    this.$refs.table.toggleAllSelection();
		
		},
		
        editDelBatch: function () {
            let _this = this;
            if (!_this.idsArr.length) {
                message.error('请选择要删除的数据');
                return;
            }
			
            let url = this.uri + 'set_integral&a=del';

            let sendData = {
                del: _this.idsArr
            };
            _this.$confirm('你确定要删除当前项吗？', '温馨提示', {
                confirmButtonText: '确定',
                cancelButtonText: '取消',
                type: 'warning'
            }).then(() => {
                httpPost(url, sendData).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        message.success(res.msg, _this.list());
                    }
                })
            })

        }

    },
};
</script>
<style scoped></style>