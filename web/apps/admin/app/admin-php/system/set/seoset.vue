<template>
<div id="moduapp" class="tableDome">
        <div class="seosetSubject">
            <div class="tableDome_tip">
                <el-alert :title="lc('admin_system_00576')" type="success" :closable="false">
                </el-alert>
            </div>
            <template>
                <el-tabs :tab-position="tabPosition" v-model="curTab">
                    <el-tab-pane v-for="(item, index) in seomodel" :key="index" :label="item" :name="index">
                        <seotab :ref="index" :action="curTab" v-if="curTab==index"></seotab>
                    </el-tab-pane>
                </el-tabs>
            </template>
        </div>
        <div class="modluDrawer">
            <el-drawer :title="detail.id ? lc('admin_system_00630') : lc('admin_system_00380')" v-model="drawerSeoshezhi" :append-to-body="true" :show-close="true" :with-header="true" size="45%">
                <seoshezhi @child-event="closeSeoshezhi" call="seo" :seoid="detail.id" :detail="detail"></seoshezhi>
            </el-drawer>
        </div>
    </div>
</template>

<script>
import Seotab from './component/seotab.vue'
import Seoshezhi from './component/seoshezhi.vue'

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
        data: function() {
            return {
                tabPosition: 'left',

                curTab: '',
                seomodel: {},

                drawerSeoshezhi: false,
                detail: {},
            }
        },
        components: {
            'seotab': Seotab,
            'seoshezhi': Seoshezhi,
        },
        created: function() {
            this.getSeomodel();


        },
        methods: {
            async getSeomodel() {
                let res = await httpPost('m=system&c=set_seo')
                let data = res.data.data;

                this.seomodel = data.seomodel;

                for (let key in this.seomodel) {
                    this.curTab = key; // 赋值默认tab
                    break;
                }
            },
            openSeoshezhi(data) {
                this.detail = data;

                this.drawerSeoshezhi = true;
            },
            closeSeoshezhi() {
                this.drawerSeoshezhi = false;
            },
            seotabRefresh() {
                let refs = this.$refs[this.curTab];
                if (refs.length > 0) {
                    refs[0].getList();
                }
            },
        },
    }
</script>
