<template>
<div id="moduapp" class="tableDome">
        <div class="seosetSubject">
            <div class="tableDome_tip">
                <el-alert :title="lc('admin_system_00576')" type="success" :closable="false">
                </el-alert>
            </div>
            <template>
                <el-tabs :tab-position="tabPosition" v-model="curTab">
                    <el-tab-pane v-for="pane in seomodelTabs" :key="pane.key" :label="pane.label" :name="pane.key">
                        <seotab ref="seotabInst" :action="curTab" v-if="curTab===pane.key"></seotab>
                    </el-tab-pane>
                </el-tabs>
            </template>
        </div>
        <div class="modluDrawer">
            <el-drawer :title="detail.id ? lc('admin_system_00630') : lc('admin_system_00380')" v-model="drawerSeoshezhi" :append-to-body="true" :show-close="true" :with-header="true" size="45%">
                <seoshezhi v-if="drawerSeoshezhi" @child-event="closeSeoshezhi" call="seo" :seoid="detail.id" :detail="detail"></seoshezhi>
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

                curTab: 'index',
                seomodel: {},

                drawerSeoshezhi: false,
                detail: {},
            }
        },
        components: {
            'seotab': Seotab,
            'seoshezhi': Seoshezhi,
        },
        computed: {
            seomodelTabs() {
                const m = this.seomodel
                if (!m || typeof m !== 'object' || Array.isArray(m)) return []
                return Object.keys(m).map((key) => ({ key, label: m[key] }))
            },
        },
        created: function() {
            if (typeof window !== 'undefined') window.custoapp = this
            this.getSeomodel();
        },
        beforeUnmount() {
            if (typeof window !== 'undefined' && window.custoapp === this) window.custoapp = undefined
        },
        methods: {
            async getSeomodel() {
                try {
                    let res = await httpPost('m=system&c=set_seo')
                    let data = (res && res.data && res.data.data) || {}
                    const model = data.seomodel
                    this.seomodel = model && typeof model === 'object' && !Array.isArray(model) ? model : {}
                    const keys = Object.keys(this.seomodel)
                    if (keys.length) {
                        this.curTab = this.seomodel[this.curTab] ? this.curTab : keys[0]
                    }
                } catch (e) {
                    this.seomodel = {}
                }
            },
            openSeoshezhi(data) {
                this.detail = data || {};

                this.drawerSeoshezhi = true;
            },
            closeSeoshezhi() {
                this.drawerSeoshezhi = false;
            },
            seotabRefresh() {
                const refs = this.$refs.seotabInst
                const inst = Array.isArray(refs) ? refs[0] : refs
                if (inst && typeof inst.getList === 'function') inst.getList()
            },
        },
    }
</script>
<style>
.seosetSubject {
    min-height: calc(100vh - 180px);
}
</style>
