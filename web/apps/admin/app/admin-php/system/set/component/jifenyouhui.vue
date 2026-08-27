<template>
    <div class="moduleElHight">
        <div class="moduleElTable" style="height: calc(100% - 50px);">
            <el-table ref="table" :data="tableData" border style="width: 100%" @selection-change="selectChange"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55">
                </el-table-column>
                <el-table-column prop="id" :label="lc('member_com_00345')" width="220">
                </el-table-column>
                <el-table-column :label="lc('admin_00922')" width="300">
                    <template #default="scope">
                        <div class="moduleElTaPax">
                            <template v-if="scope.row.isEditjifen">
                                <el-input v-model="scope.row.integral" type="text" @blur="changeRow(scope, 'jifen')" />
                            </template>
                            <template v-else>
                                <span>{{ scope.row.integral }}</span>
                            </template>
                            <img src="/admin/php-admin/images/bine.png" @click="editRow(scope, 'jifen')">
                        </div>

                    </template>
                </el-table-column>
                <el-table-column :label="lc('admin_system_00333')">
                    <template #default="scope">
                        <div class="moduleElTaPax">
                            <template v-if="scope.row.isEditdiscount">
                                <el-input v-model="scope.row.discount" type="text"
                                    onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"
                                    @blur="changeRow(scope, 'discount')" />
                            </template>
                            <template v-else>
                                <span>{{ lc('admin_00921') }}</span>
                            </template>
                            <img src="/admin/php-admin/images/bine.png" @click="editRow(scope, 'discount')">
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('admin_system_00332')" width="220">
                    <template #default="scope">
                        <el-switch v-model="scope.row.status" active-color="#1890FF" inactive-color="#B8BDC9"
                            @change="isOpen(scope)">
                        </el-switch>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" :label="lc('member_user_00048')" width="80">
                    <template #default="scope">
                        <div class="moduleElTaCaoz">
                            <el-button size="small" type="danger" @click="deljf(scope.row)">{{ lc('common.delete') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
				<el-checkbox v-model="allchecked" @change="allcheckChange">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button size="small" @click="editDelBatch">{{ lc('common.delete') }}</el-button>
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
            emptytext: lc('wap_js_00113'),
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
            console.log(`Page size: ${val}`);
        },
        handleCurrentChange(val) {
            console.log(`Current page: ${val}`);
        },
        list() {
            let _this = this;
            _this.loading = true;
            _this.emptytext = lc('admin_user_weipin_00026');
            let url = _this.uri + 'set_integral&a=class';
            httpPost(url, {}).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    _this.tableData = res.data;
                    _this.loading = false;
                    if (_this.tableData.length === 0){
                        _this.emptytext = lc('wap_js_00113');
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
					message.warning(lc('admin_vue_00053'));
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
					message.warning(lc('admin_vue_00054'));
					_this.tableData[index][isEditFieldName] = false;return;
				}
			}
            
            let url = _this.uri + 'set_integral&a=ajax';
            httpPost(url, sendData).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    message.success(lc('wap_user_00264'));
                } else {
                    message.error(lc('wap_js_00141'));
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
                    message.success(lc('wap_user_00264'));
                } else {
                    message.error(lc('wap_js_00141'));
                }
                _this.list();
            }).catch(function (error) {
                console.log(error);
            });
        },
        deljf(row){
            delConfirm(this, {delid:row.id}, this.del, lc('admin_vue_00055'));
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
                message.error(lc('admin_user_weipin_00005'));
                return;
            }
			
            let url = this.uri + 'set_integral&a=del';

            let sendData = {
                del: _this.idsArr
            };
            _this.$confirm(lc('admin_00333'), lc('wap_user_00205'), {
                confirmButtonText: lc('common.confirm'),
                cancelButtonText: lc('common.cancel'),
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