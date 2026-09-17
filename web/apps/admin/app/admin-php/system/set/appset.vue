<template>
<div id="appsetApp" class="moduleElenAl">
    <div class="moduleSeachs">
        <div class="moduleSeachleft">
            <div class="tableSeachInptsmall newsinput">
                <el-select v-model="platform" size="small" :clearable="true" :placeholder="lc('admin_tool_00744')" @change="search">
                    <el-option label="iOS" value="ios"></el-option>
                    <el-option label="Android" value="android"></el-option>
                </el-select>
            </div>
            <div style="overflow: hidden;position: relative;display: flex;flex-wrap: wrap;align-items: center;">
                <el-button type="primary" icon="el-icon-search" size="small" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
            </div>
        </div>
        <div class="moduleSeachButn" style="display: flex;align-items: center;">
            <el-button type="primary" icon="el-icon-document-add" size="small" @click="openAdd">{{ lc('admin_tool_00707') }}</el-button>
        </div>
    </div>
    <div class="moduleElTable">
        <el-table :data="tableData" border style="width: 100%" :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="100%" @selection-change="handleSelectionChange" ref="dataTable" v-loading="loading" :empty-text="emptytext">
            <el-table-column type="selection" width="55"></el-table-column>
            <el-table-column prop="platform" :label="lc('admin_tool_00744')" width="100" align="center"></el-table-column>
            <el-table-column prop="version" :label="lc('admin_system_00579')" width="120" align="center"></el-table-column>
            <el-table-column prop="version_code" :label="lc('admin_tool_00741')" width="110" align="center"></el-table-column>
            <el-table-column :label="lc('admin_00101')" min-width="240">
                <template #default="scope">
                    <a v-if="scope.row.download_url" :href="scope.row.download_url" target="_blank" rel="noopener">{{ scope.row.download_url }}</a>
                </template>
            </el-table-column>
            <el-table-column :label="lc('admin_tool_00742')" width="110" align="center">
                <template #default="scope">
                    <el-tag type="danger" size="small" v-if="scope.row.is_force==1">{{ lc('member_com_00287') }}</el-tag>
                    <el-tag type="info" size="small" v-else>{{ lc('common.close') }}</el-tag>
                </template>
            </el-table-column>
            <el-table-column prop="changelog" :label="lc('admin_tool_00743')" min-width="160" show-overflow-tooltip></el-table-column>
            <el-table-column prop="released_at_n" :label="lc('admin_user_weipin_00030')" width="170" align="center"></el-table-column>
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
            <div class="modulePagNum" style="margin: 0 auto;">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange" v-model:current-page="currentPage" v-model:page-size="pageSize" :page-sizes="pageSizes" layout="total, sizes, prev, pager, next, jumper" :total="total"></el-pagination>
            </div>
        </div>
    </div>
    <div class="appsetFooterSpace"></div>
    <el-dialog :title="form.id ? lc('wap_js_00073') : lc('admin_tool_00707')" v-model="dialogShow" :modal-append-to-body="false" width="560px">
        <el-form :model="form" label-width="110px">
            <el-form-item :label="lc('admin_tool_00744')">
                <el-select v-model="form.platform" style="width: 100%;">
                    <el-option label="iOS" value="ios"></el-option>
                    <el-option label="Android" value="android"></el-option>
                </el-select>
            </el-form-item>
            <el-form-item :label="lc('admin_system_00579')">
                <el-input v-model="form.version"></el-input>
            </el-form-item>
            <el-form-item :label="lc('admin_tool_00741')">
                <el-input v-model="form.version_code"></el-input>
            </el-form-item>
            <el-form-item :label="lc('admin_00101')">
                <el-input v-model="form.download_url"></el-input>
            </el-form-item>
            <el-form-item :label="lc('admin_tool_00742')">
                <el-radio-group v-model="form.is_force">
                    <el-radio label="1">{{ lc('member_com_00287') }}</el-radio>
                    <el-radio label="0">{{ lc('common.close') }}</el-radio>
                </el-radio-group>
            </el-form-item>
            <el-form-item :label="lc('admin_tool_00743')">
                <el-input type="textarea" :rows="4" v-model="form.changelog"></el-input>
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

function emptyForm() {
    return {
        id: 0,
        platform: 'android',
        version: '',
        version_code: '',
        download_url: '',
        is_force: '0',
        changelog: '',
    }
}

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
            platform: '',
            form: emptyForm(),
        }
    },
    created() {
        this.getDataList()
    },
    methods: {
        search() {
            this.currentPage = 1
            this.getDataList()
        },
        getDataList() {
            const that = this
            that.loading = true
            that.emptytext = window.lc('admin_user_weipin_00026')
            const params = {
                page: that.currentPage,
                pageSize: that.pageSize,
            }
            if (that.platform) params.platform = that.platform
            httpPost('m=system&c=appset', params, { hideloading: true }).then(function (res) {
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
            this.form = emptyForm()
            this.dialogShow = true
        },
        openEdit(row) {
            this.form = {
                id: row.id,
                platform: row.platform || 'android',
                version: row.version || '',
                version_code: row.version_code == null ? '' : String(row.version_code),
                download_url: row.download_url || '',
                is_force: Number(row.is_force) === 1 ? '1' : '0',
                changelog: row.changelog || '',
            }
            this.dialogShow = true
        },
        save() {
            const that = this
            if (!that.form.platform || !that.form.version || !that.form.download_url) {
                message.error(window.lc('admin_tool_00745'))
                return
            }
            that.saveLoading = true
            const params = {
                platform: that.form.platform,
                version: that.form.version,
                version_code: Number(that.form.version_code) || 0,
                download_url: that.form.download_url,
                is_force: that.form.is_force === '1' || that.form.is_force === 1,
                changelog: that.form.changelog || '',
            }
            const action = that.form.id ? 'm=system&c=appset&a=update' : 'm=system&c=appset&a=save'
            if (that.form.id) params.id = that.form.id
            httpPost(action, params).then(function (res) {
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
            delConfirm(this, { id: row.id }, this.delete, window.lc('admin_vue_00137', [row.version || row.platform]))
        },
        delSel() {
            if (!this.selectedItem.length) {
                message.error(window.lc('admin_user_weipin_00005'))
                return
            }
            const ids = this.selectedItem.map((r) => r.id)
            const name = this.selectedItem.map((r) => r.version || r.platform).join(', ')
            delConfirm(this, { id: ids }, this.delete, window.lc('admin_vue_00137', [name]))
        },
        delete(params) {
            const self = this
            const raw = params.id
            const ids = Array.isArray(raw) ? raw : [raw]
            const run = function (i) {
                if (i >= ids.length) {
                    message.success(window.lc('wap_js_00077'), function () {
                        self.getDataList()
                    })
                    return
                }
                httpPost('m=system&c=appset&a=del', { id: ids[i] }).then(function (res) {
                    if (res.data.error == 0) {
                        run(i + 1)
                    } else {
                        message.error(res.data.msg)
                    }
                })
            }
            run(0)
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
<style>
#appsetApp .moduleElTable {
    height: calc(100% - (60px + 50px + 12px + 12px + 52px));
}
.appsetFooterSpace {
    height: 40px;
}
</style>
