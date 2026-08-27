<template>
<div id="hyfl" class="moduleElenAl">
        <div class="moduleSeachs">
            <div class="">{{ lc('admin_00322') }}</div>
            <div class="nrtopbtn">
                <el-button type="primary" icon="el-icon-document-add" @click="dialogVisible = true" size="small">{{ lc('admin_00326') }}</el-button>
            </div>
        </div>
        <div class="moduleElTable">
            <el-table :data="tableData" stripe border style="width: 100%;height: 100%;"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"> </el-table-column>
                <el-table-column prop="id" :label="lc('member_com_00345')" width="180">
                </el-table-column>
                <el-table-column :label="lc('admin_00323')" property="name">
                    <template #default="scope">
                        <span>{{ scope.row.name }}<img src="/admin/php-admin/images/bine.png" alt=""
                                style="margin-left: 4px;" width="14" height="14"></span>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('admin_00324')" property="px">
                    <template #default="scope">
                        <span>{{ scope.row.px }}<img src="/admin/php-admin/images/bine.png" alt=""
                                style="margin-left: 4px;" width="14" height="14"></span>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" :label="lc('member_user_00048')" width="120">
                    <div class="TableLink">
                        <el-link type="primary">{{ lc('wap_00071') }}</el-link>
                        <el-link type="primary">{{ lc('wap_js_00073') }}</el-link>
                    </div>
                    <div class="TableLink">
                        <el-link type="primary">{{ lc('admin_00306') }}</el-link>
                        <el-link type="primary" @click="deldialog = true">{{ lc('wap_js_00077') }}</el-link>
                    </div>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="">
                <div class="">
                    <el-checkbox v-model="checked" style="margin-right: 8px">{{ lc('wap_js_00074') }}</el-checkbox>
                    <el-button>{{ lc('member_com_00055') }}</el-button>
                </div>
            </div>
        </div>
        <el-dialog :title="lc('admin_00222')" width="30%" v-model="dialogVisible" :modal-append-to-body="false">
            <div class="hydialog_item">
                <span>{{ lc('admin_00260') }}</span>
                <el-input type="textarea" v-model="lbname" style="flex: 1;"></el-input>
            </div>
            <template #footer><div class="dialog-footer">
                <el-button type="primary" @click="dialogVisible = false">{{ lc('wap_js_00091') }}</el-button>
            </div></template>
        </el-dialog>
        <!-- 删除弹窗 -->
        <el-dialog :title="lc('admin_00308')" v-model="deldialog" width="20%" :modal-append-to-body="false">
            <span>{{ lc('wap_user_00001') }}</span>
            <template #footer><span class="dialog-footer">
                <el-button @click="deldialog = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                <el-button type="primary" @click="deldialog = false">{{ lc('wap_com_00019') }}</el-button>
            </span></template>
        </el-dialog>
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
                tableData: [
                    {
                        id: "1", //分类编号
                        name: lc('admin_neirong_zph_00001'), //分类名称
                        px: "1",
                    },
                    {
                        id: "1", //分类编号
                        name: lc('admin_neirong_zph_00001'), //分类名称
                        px: "2",
                    },
                    {
                        id: "1", //分类编号
                        name: lc('admin_neirong_zph_00001'), //分类名称
                        px: "4",
                    },
                    {
                        id: "1", //分类编号
                        name: lc('admin_neirong_zph_00001'), //分类名称
                        px: "6",
                    },
                    {
                        id: "1", //分类编号
                        name: lc('admin_neirong_zph_00001'), //分类名称
                        px: "9",
                    },
                    {
                        id: "1", //分类编号
                        name: lc('admin_neirong_zph_00001'), //分类名称
                        px: "7",
                    },
                ], //表格数据
                randomKey: Math.random(),
                checked: false,
                zmselect: '',
                options: [{
                    value: lc('admin_demo_00001'),
                    label: lc('admin_00327')
                }, {
                    value: lc('admin_demo_00002'),
                    label: lc('admin_00328')
                }, {
                    value: lc('admin_demo_00003'),
                    label: lc('admin_00329')
                }, {
                    value: lc('admin_demo_00004'),
                    label: lc('admin_00330')
                }, {
                    value: lc('admin_demo_00005'),
                    label: lc('admin_00331')
                }],
                sfoptions: [
                    {
                        value: lc('admin_demo_00001'),
                        label: lc('common_02085')
                    }, {
                        value: lc('admin_demo_00002'),
                        label: lc('common_02063')
                    }],
                dialogVisible: false,
                lbname: '',
                deldialog: false,
            }
        },
        methods: {

        }
    }
</script>
