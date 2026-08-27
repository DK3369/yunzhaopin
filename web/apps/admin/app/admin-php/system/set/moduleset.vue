<template>
<div id="moduapp" class="tableDome">
        <div class="tableDome_tip">
            <el-alert :title="lc('admin_system_00459')" type="success" show-icon>
            </el-alert>
        </div>
        <div class="moduleTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="100">{{ lc('admin_system_00466') }}</th>
                        <th width="100">{{ lc('member_user_00181') }}</th>
                        <th>{{ lc('admin_system_00460') }}</th>
                        <th width="220">{{ lc('admin_system_00465') }}</th>
                        <th width="160">{{ lc('admin_system_00467') }}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="(item, index) in list" :key="index">
                        <td>
                            <div class="TableTite">{{item.value}}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-checkbox v-model="item.web" true-label="1" false-label="2">{{item.web=='1'?lc('member_com_00287'):lc('resume_00030')}}</el-checkbox>
                            </div>
                        </td>
                        <td>
                            <div class="TableDoma">
                                <el-input placeholder="" v-model="item.domain" class="input-with-select">
                                    <template #prepend><el-select v-model="item.ssl">
                                        <el-option label="https://" value="1"></el-option>
                                        <el-option label="http://" value="0"></el-option>
                                    </el-select></template>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="item.dir" disabled :placeholder="lc('admin_system_00469')"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableLink seolinksGai" v-if="index != 'error'">
                                <el-link type="primary" @click="openNavset(index, item.value)">{{ lc('admin_system_00464') }}</el-link>
                                <el-link type="primary" @click="openSeoshezhi(index, item.value)">{{ lc('admin_system_00463') }}</el-link>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">{{ lc('wap_user_00101') }}</el-button>
        </div>
        <!-- 抽屉弹窗 -->
        <div class="modluDrawer">
            <el-drawer :title="lc('admin_00201')" v-model="drawer1" :modal-append-to-body="false" :show-close="true" :with-header="true" size="35%">
                <navset @child-event="closeNavset" :config="detail.config" :name="detail.name"></navset>
            </el-drawer>
        </div>
        <div class="modluDrawer">
            <el-drawer :title="lc('admin_system_00463')" v-model="drawer2" :append-to-body="true" :show-close="true" :with-header="true" size="50%">
                <seoshezhi @child-event="closeSeoshezhi" call="module" :config="detail.config"></seoshezhi>
            </el-drawer>
        </div>
    </div>
</template>

<script>
import Jibenset from './component/jibenset.vue'
import Navset from './component/navset.vue'
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
                list: [],

                drawer1: false,
                drawer2: false,

                detail: {},
                saveLoading: false
            }
        },
        components: {
            'jibenset': Jibenset,
            'navset': Navset,
            'seoshezhi': Seoshezhi,
        },
        created: function() {
            this.getModule();


        },
        methods: {
            async getModule() {
                let res = await httpPost('m=system&c=set_module',{},{hideloading: true})
                let data = res.data.data;

                this.list = data.module;
            },
            save() {
                let that = this;
                that.saveLoading = true;
                httpPost('m=system&c=set_module&a=save', that.list,{hideloading: true}).then(function(res) {
                    if (res.data.error > 0) {
                        message.error(res.data.msg);
                    } else {
                        message.success(res.data.msg);
                    }
                }).finally(function () {
                    setTimeout(function () {
                        that.saveLoading = false;
                    }, 2000);
                });
            },
            openNavset(config, name) {
                this.detail = {
                    config: config,
                    name: name
                };

                this.drawer1 = true;
            },
            closeNavset() {
                this.drawer1 = false;
            },
            openSeoshezhi(config, name) {
                this.detail = {
                    config: config,
                    name: name
                };

                this.drawer2 = true;
            },
            closeSeoshezhi() {
                this.drawer2 = false;
            },
        }
    }
</script>
