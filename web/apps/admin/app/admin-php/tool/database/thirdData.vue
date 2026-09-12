<template>
<div id="thirdDataApp" class="moduleElenAl">
    <div class="moduleSeachs">
        <div class="moduleSeachInpt">{{ lc('admin_tool_00730') }}</div>
        <div class="">
            <el-button type="primary" icon="el-icon-document-add" size="small" @click="openAdd">{{ lc('admin_tool_00707') }}</el-button>
        </div>
    </div>
    <div class="tableDome_tip" style="margin: 0 12px 8px;">
        <el-alert :title="lc('admin_tool_00733')" type="success" :closable="false"></el-alert>
    </div>
    <div class="moduleElTable">
        <el-table :data="tableData" border style="width: 100%" :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="100%" @selection-change="handleSelectionChange" ref="dataTable" v-loading="loading" :empty-text="emptytext">
            <el-table-column type="selection" width="55"></el-table-column>
            <el-table-column prop="sort" :label="lc('admin_vue_00044')" width="80" align="center"></el-table-column>
            <el-table-column prop="name" :label="lc('admin_system_00292')" min-width="160"></el-table-column>
            <el-table-column :label="lc('admin_00101')" min-width="280">
                <template #default="scope">
                    <a :href="scope.row.url" target="_blank" rel="noopener">{{ scope.row.url }}</a>
                </template>
            </el-table-column>
            <el-table-column prop="provider" :label="lc('admin_tool_00731')" width="120" align="center"></el-table-column>
            <el-table-column :label="lc('member_user_00181')" width="100" align="center">
                <template #default="scope">
                    <el-tag type="success" size="small" v-if="scope.row.enabled==1">{{ lc('member_com_00287') }}</el-tag>
                    <el-tag type="info" size="small" v-else>{{ lc('common.close') }}</el-tag>
                </template>
            </el-table-column>
            <el-table-column fixed="right" :label="lc('member_user_00048')" width="200" align="center">
                <template #default="scope">
                    <div class="cz_button">
                        <el-button size="small" @click="openEdit(scope.row)">{{ lc('wap_js_00073') }}</el-button>
                        <el-button type="danger" size="small" @click="delRow(scope.row)">{{ lc('wap_js_00077') }}</el-button>
                    </div>
                </template>
            </el-table-column>
        </el-table>
    </div>
    <div class="modulePaging">
        <div class="modulecz">
            <el-checkbox :indeterminate="isIndeterminate" v-model="checkAll" @change="handleCheckAllChange">{{ lc('wap_js_00074') }}</el-checkbox>
            <el-button size="small" @click="delSel">{{ lc('member_com_00055') }}</el-button>
        </div>
        <div class="modulePagNum">
            <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange" v-model:current-page="currentPage" v-model:page-size="pageSize" :page-sizes="pageSizes" layout="total, sizes, prev, pager, next, jumper" :total="total"></el-pagination>
        </div>
    </div>
    <el-dialog :title="form.id ? lc('wap_js_00073') : lc('admin_tool_00707')" v-model="dialogShow" :modal-append-to-body="false" width="560px">
        <el-form :model="form" label-width="110px">
            <el-form-item :label="lc('admin_system_00292')">
                <el-input v-model="form.name"></el-input>
            </el-form-item>
            <el-form-item :label="lc('admin_00101')">
                <el-input v-model="form.url"></el-input>
            </el-form-item>
            <el-form-item :label="lc('admin_tool_00732')">
                <el-input v-model="form.api_url"></el-input>
            </el-form-item>
            <el-form-item :label="lc('admin_tool_00731')">
                <el-select v-model="form.provider" style="width: 100%;">
                    <el-option label="greenhouse" value="greenhouse"></el-option>
                    <el-option label="ashby" value="ashby"></el-option>
                    <el-option label="lever" value="lever"></el-option>
                    <el-option label="workday" value="workday"></el-option>
                    <el-option label="other" value="other"></el-option>
                </el-select>
            </el-form-item>
            <el-form-item :label="lc('admin_vue_00044')">
                <el-input v-model="form.sort"></el-input>
            </el-form-item>
            <el-form-item :label="lc('member_user_00181')">
                <el-radio-group v-model="form.enabled">
                    <el-radio label="1">{{ lc('member_com_00287') }}</el-radio>
                    <el-radio label="0">{{ lc('common.close') }}</el-radio>
                </el-radio-group>
            </el-form-item>
        </el-form>
        <template #footer>
            <el-button @click="dialogShow = false">{{ lc('admin_user_weipin_00043') }}</el-button>
            <el-button type="primary" :loading="saveLoading" @click="save">{{ lc('wap_com_00019') }}</el-button>
        </template>
    </el-dialog>
</div>
</template>

<script>
const httpPost = (...a) => window.httpPost(...a)
const lc = (...a) => window.lc(...a)
const message = typeof window !== 'undefined' && window.message ? window.message : { success(){}, error(){}, warning(){}, confirm(){}, alert(){}, open(){} }
const delConfirm = (...a) => window.delConfirm(...a)

export default {
    data: function () {
        return {
            loading: false,
            emptytext: window.lc('wap_js_00113'),
            tableData: [],
            total: 0,
            currentPage: 1,
            prevPage: 0,
            pageSize: 20,
            pageSizes: [10, 20, 50, 100],
            checkAll: false,
            isIndeterminate: false,
            selectedItem: [],
            dialogShow: false,
            saveLoading: false,
            form: {
                id: 0,
                name: '',
                url: '',
                api_url: '',
                provider: 'other',
                sort: 0,
                enabled: '1',
            },
        }
    },
    created() {
        this.getDataList()
    },
    methods: {
        getDataList() {
            const that = this
            that.loading = true
            that.emptytext = window.lc('admin_user_weipin_00026')
            httpPost('m=tool&c=thirdData', {
                page: that.currentPage,
                pageSize: that.pageSize,
            }, { hideloading: true }).then(function (res) {
                const data = res.data.data || {}
                that.tableData = Array.isArray(data.list) ? data.list : []
                that.total = data.total || 0
                if (data.pageSize) that.pageSize = parseInt(data.pageSize)
                if (data.pageSizes) that.pageSizes = data.pageSizes
                that.loading = false
                if (that.prevPage != that.currentPage) {
                    that.prevPage = that.currentPage
                    if (that.$refs.dataTable && that.$refs.dataTable.bodyWrapper) {
                        that.$refs.dataTable.bodyWrapper.scrollTop = 0
                    }
                }
                if (that.tableData.length === 0) {
                    that.emptytext = window.lc('wap_js_00113')
                }
            }).catch(function () {
                that.loading = false
            })
        },
        openAdd() {
            this.form = { id: 0, name: '', url: '', api_url: '', provider: 'other', sort: 0, enabled: '1' }
            this.dialogShow = true
        },
        openEdit(row) {
            this.form = {
                id: row.id,
                name: row.name || '',
                url: row.url || '',
                api_url: row.api_url || '',
                provider: row.provider || 'other',
                sort: row.sort || 0,
                enabled: Number(row.enabled) === 0 ? '0' : '1',
            }
            this.dialogShow = true
        },
        save() {
            const that = this
            if (!that.form.name || !that.form.url) {
                message.error(window.lc('admin_tool_00316'))
                return
            }
            that.saveLoading = true
            const params = {
                name: that.form.name,
                url: that.form.url,
                api_url: that.form.api_url,
                provider: that.form.provider,
                sort: Number(that.form.sort) || 0,
                enabled: that.form.enabled === '0' || that.form.enabled === 0 ? 0 : 1,
            }
            if (that.form.id) params.id = that.form.id
            httpPost('m=tool&c=thirdData&a=save', params).then(function (res) {
                if (res.data.error == 0) {
                    message.success(res.data.msg, function () {
                        that.dialogShow = false
                        that.getDataList()
                    })
                } else {
                    message.error(res.data.msg)
                }
            }).finally(function () {
                that.saveLoading = false
            })
        },
        delRow(row) {
            delConfirm(this, { id: row.id }, this.delete, window.lc('admin_vue_00137', [row.name]))
        },
        delSel() {
            if (!this.selectedItem.length) {
                message.error(window.lc('admin_user_weipin_00005'))
                return
            }
            const ids = this.selectedItem.map((r) => r.id)
            const name = this.selectedItem.map((r) => r.name).join(', ')
            delConfirm(this, { id: ids }, this.delete, window.lc('admin_vue_00137', [name]))
        },
        delete(params) {
            const self = this
            httpPost('m=tool&c=thirdData&a=del', params).then(function (res) {
                if (res.data.error == 0) {
                    message.success(res.data.msg, function () {
                        self.getDataList()
                    })
                } else {
                    message.error(res.data.msg)
                }
            })
        },
        handleSelectionChange(val) {
            this.selectedItem = val
            if (this.selectedItem.length == 0) {
                this.isIndeterminate = false
                this.checkAll = false
            } else if (this.selectedItem.length == this.tableData.length) {
                this.isIndeterminate = false
                this.checkAll = true
            } else {
                this.isIndeterminate = true
                this.checkAll = false
            }
        },
        handleCheckAllChange(val) {
            val ? this.$refs.dataTable.toggleAllSelection() : this.$refs.dataTable.clearSelection()
        },
        handleSizeChange(val) {
            this.pageSize = val
            this.getDataList()
        },
        handleCurrentChange(val) {
            this.currentPage = val
            this.getDataList()
        },
    },
}
</script>
