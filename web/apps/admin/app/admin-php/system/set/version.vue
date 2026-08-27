<template>
<div id="adminnav" class="moduleElenAl">
        <div class="moduleElTable">
            <el-table :data="list" border style="width: 100%" row-key="id" :default-expand-all="false" lazy 
                ref="tableref" :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="100%"
                :tree-props="{children: 'children', hasChildren: 'hasChildren'}" v-loading="loading" :empty-text="emptytext">
                <el-table-column prop="version" :label="lc('admin_system_00579')" width="200">
                </el-table-column>
                <el-table-column prop="ctime_n" :label="lc('admin_system_00578')" width="200">
                </el-table-column>
            </el-table>
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
                   
                    list: [],
                    
                    emptytext: lc('admin_user_weipin_00026'),
                    loading: true,
                }
            },
            created: function () {
                this.getList();
            },
            methods: {
                async getList() {
                    let that = this;
                    let res = await httpPost('m=system&c=admin_nav&a=version', { });
                    if (res.data.error == 0) {
                        that.list = res.data.data.list;
                        that.loading = false;
                        if (that.list.length === 0){
                            that.emptytext = lc('wap_js_00113');
                        }
                    }

                },
            }
        }
</script>
