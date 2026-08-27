<template>
<div id="yemanapp" class="tableDome">
    <div class="tableDome_tip">
        <el-alert :title="lc('admin_system_00582')" type="success" :closable="false"></el-alert>
    </div>
    <div class="moduleTable">
        <table class="tableVue">
            <thead>
            <tr align="left">
                <th width="200">{{ lc('member_com_00021') }}</th>
                <th width="500">{{ lc('member_user_00181') }}</th>
                <th>{{ lc('member_com_00207') }}</th>
            </tr>
            </thead>
            <tbody>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00643') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-radio-group v-model="list.sy_seo_rewrite">
                            <el-radio label="1">{{ lc('member_com_00287') }}</el-radio>
                            <el-radio label="0">{{ lc('resume_00030') }}</el-radio>
                        </el-radio-group>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00594') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00637') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-radio-group v-model="list.sy_header_fix">
                            <el-radio label="1">{{ lc('member_com_00287') }}</el-radio>
                            <el-radio label="0">{{ lc('resume_00030') }}</el-radio>
                        </el-radio-group>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00596') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00629') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-radio-group v-model="list.sy_footer_fix">
                            <el-radio label="1">{{ lc('member_com_00287') }}</el-radio>
                            <el-radio label="0">{{ lc('resume_00030') }}</el-radio>
                        </el-radio-group>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00605') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00636') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-radio-group v-model="list.sy_linksq">
                            <el-radio label="1">{{ lc('member_com_00287') }}</el-radio>
                            <el-radio label="0">{{ lc('resume_00030') }}</el-radio>
                        </el-radio-group>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00615') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00617') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-radio-group v-model="list.sy_wap_jump">
                            <el-radio label="1">{{ lc('member_com_00287') }}</el-radio>
                            <el-radio label="0">{{ lc('resume_00030') }}</el-radio>
                        </el-radio-group>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00543') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00613') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-radio-group v-model="list.sy_pc_jump_wap">
                            <el-radio label="1">{{ lc('member_com_00287') }}</el-radio>
                            <el-radio label="0">{{ lc('resume_00030') }}</el-radio>
                        </el-radio-group>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00612') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_01027') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-radio-group v-model="list.sy_h5_share">
                            <el-radio label="1">{{ lc('member_com_00287') }}</el-radio>
                            <el-radio label="0">{{ lc('resume_00030') }}</el-radio>
                        </el-radio-group>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00627') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00626') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-radio-group v-model="list.sy_advice_mobilecode">
                            <el-radio label="1">{{ lc('member_com_00287') }}</el-radio>
                            <el-radio label="0">{{ lc('resume_00030') }}</el-radio>
                        </el-radio-group>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00599') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00625') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-radio-group v-model="list.sy_wxwap_list">
                            <el-radio label="1">{{ lc('wap_js_00076') }}</el-radio>
                            <el-radio label="2">{{ lc('admin_system_00647') }}</el-radio>
                        </el-radio-group>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00616') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00614') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-radio-group v-model="list.sy_wap_comtpl">
                            <el-radio label="0">{{ lc('admin_system_00654') }}</el-radio>
                            <el-radio label="1">{{ lc('admin_system_00655') }}</el-radio>
                        </el-radio-group>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00604') }}</span>
                    </div>
                </td>
            </tr>
            
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00638') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-radio-group v-model="list.sy_news_rewrite">
                            <el-radio label="1">{{ lc('admin_system_00653') }}</el-radio>
                            <el-radio label="2">{{ lc('admin_system_00656') }}</el-radio>
                        </el-radio-group>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00593') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00628') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-select v-model="list.sy_ewm_type" :placeholder="lc('admin_system_00631')">
                            <el-option :label="lc('wap_js_00098')" value="wap"></el-option>
                            <el-option :label="lc('admin_system_00610')" value="weixin"></el-option>
                        </el-select>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00584') }}</span><br>
                        <span>{{ lc('admin_system_00602') }}</span><br>
                        <span>{{ lc('admin_system_00595') }}</span><br>
                    </div>
                </td>
            </tr>

            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00618') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-radio-group v-model="list.sy_default_userclass">
                            <el-radio label="1">{{ lc('common_02085') }}</el-radio>
                            <el-radio label="2">{{ lc('common_02063') }}</el-radio>
                        </el-radio-group>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00600') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00619') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-radio-group v-model="list.sy_default_comclass">
                            <el-radio label="1">{{ lc('common_02085') }}</el-radio>
                            <el-radio label="2">{{ lc('common_02063') }}</el-radio>
                        </el-radio-group>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00601') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00651') }}</div>
                </td>
                <td>
                    <div class="TableSelect">
                        <el-select v-model="list.resume_salarytype" :placeholder="lc('wap_user_00100')">
                            <el-option v-for="item in salary" :key="item.value" :label="item.label" :value="item.value"></el-option>
                        </el-select>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00598') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00621') }}</div>
                </td>
                <td>
                    <div class="TableInpt">
                        <el-input :placeholder="lc('wap_user_00076')" type="number" v-model="list.sy_indexpage" autocomplete="off" min="0"></el-input>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00645') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00639') }}</div>
                </td>
                <td>
                    <div class="TableInpt">
                        <el-input :placeholder="lc('wap_user_00076')" v-model="list.sy_datacycle" type="number" autocomplete="off" min="0">
                            <template #suffix><span class="slotspan">{{ lc('common_02067') }}</span></template>
                        </el-input>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00588') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00640') }}</div>
                </td>
                <td>
                    <div class="TableInpt">
                        <el-input :placeholder="lc('wap_user_00076')" v-model="list.sy_datacycle_job" type="number" autocomplete="off" min="0">
                            <template #suffix><span class="slotspan">{{ lc('common_02067') }}</span></template>
                        </el-input>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00589') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00634') }}</div>
                </td>
                <td>
                    <div class="TableInpt">
                        <el-input :placeholder="lc('wap_user_00076')" v-model="list.sy_datacycle_com" autocomplete="off" type="number" min="0">
                            <template #suffix><span class="slotspan">{{ lc('common_02067') }}</span></template>
                        </el-input>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00587') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00650') }}</div>
                </td>
                <td>
                    <div class="TableInpt">
                        <el-input :placeholder="lc('wap_user_00076')" v-model="list.sy_logintime" autocomplete="off" type="number" min="0">
                            <template #suffix><span class="slotspan">{{ lc('common_02067') }}</span></template>
                        </el-input>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00608') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00641') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-radio-group v-model="list.sy_login_type">
                            <el-radio label="1">{{ lc('wap_00308') }}</el-radio>
                            <el-radio label="2">{{ lc('wap_00648') }}</el-radio>
                        </el-radio-group>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00583') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00611') }}</div>
                </td>
                <td>
                    <div class="TableInpt">

                        <el-input :placeholder="lc('wap_user_00076')" v-model="list.sy_resume_visitors" autocomplete="off" type="number" min="0">
                            <template #suffix><span class="slotspan">{{ lc('common_02088') }}</span></template>
                        </el-input>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00645') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00609') }}</div>
                </td>
                <td>
                    <div class="TableInpt">

                        <el-input :placeholder="lc('wap_user_00076')" v-model="list.sy_adclick" autocomplete="off" type="number" min="0">
                            <template #suffix><span class="slotspan">{{ lc('wap_js_00128') }}</span></template>
                        </el-input>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00597') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00607') }}</div>
                </td>
                <td>
                    <div class="TableInpt">

                        <el-input :placeholder="lc('wap_user_00076')" v-model="list.sy_recommend_day_num" autocomplete="off" type="number" min="0">
                            <template #suffix><span class="slotspan">{{ lc('common_02089') }}</span></template>
                        </el-input>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00646') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00606') }}</div>
                </td>
                <td>
                    <div class="TableInpt">

                        <el-input :placeholder="lc('wap_user_00076')" v-model="list.sy_recommend_interval" type="number" min="0">
                            <template #suffix><span class="slotspan">{{ lc('common_02094') }}</span></template>
                        </el-input>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00645') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00622') }}</div>
                </td>
                <td>
                    <div class="TableInpt">

                        <el-input :placeholder="lc('wap_user_00076')" autocomplete="off" v-model="list.sy_resumeout_day_num" type="number" min="0">
                            <template #suffix><span class="slotspan">{{ lc('common_02089') }}</span></template>
                        </el-input>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00646') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00623') }}</div>
                </td>
                <td>
                    <div class="TableInpt">

                        <el-input :placeholder="lc('wap_user_00076')" v-model="list.sy_resumeout_interval" autocomplete="off" type="number" min="0">
                            <template #suffix><span class="slotspan">{{ lc('common_02094') }}</span></template>
                        </el-input>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00645') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00630') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-radio-group v-model="list.sy_zhanzhang_baidu">
                            <el-radio label="1">{{ lc('member_com_00287') }}</el-radio>
                            <el-radio label="2">{{ lc('resume_00030') }}</el-radio>
                        </el-radio-group>
                    </div>
                </td>
                <td>
                    <div class="TableShuom"></div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00648') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-radio-group v-model="list.sy_outlinks">
                            <el-radio label="1">{{ lc('member_com_00287') }}</el-radio>
                            <el-radio label="2">{{ lc('resume_00030') }}</el-radio>
                        </el-radio-group>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00586') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00649') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-input type="textarea" :rows="2" :placeholder="lc('wap_user_00076')" v-model="list.sy_shenming"></el-input>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00591') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00632') }}</div>
                </td>
                <td>
                    <div class="jianlidian">
                        <div class="TableInpt">
                            <el-input :placeholder="lc('admin_user_00342')" value="1" disabled></el-input>
                        </div>
                        <div class="hxian"></div>
                        <div class="TableInpt">
                            <el-input :placeholder="lc('admin_user_00342')" v-model="list.sy_job_hits" type="number" max="100"></el-input>
                        </div>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00590') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00624') }}</div>
                </td>
                <td>
                    <div class="morenset">
                        <div class="TableSelect">
                            <el-select v-model="list.sy_web_city_one" :placeholder="lc('wap_user_00100')" @change="cityChange" clearable>
                                <el-option v-for="item in province" :key="item.value" :label="item.label" :value="item.value"></el-option>
                            </el-select>
                        </div>
                        <div class="TableSelect">
                            <el-select v-model="list.sy_web_city_two" :placeholder="lc('wap_user_00100')" clearable>
                                <el-option v-for="item in city_type" :key="item.value" :label="item.label" :value="item.value"></el-option>
                            </el-select>
                        </div>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00592') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00635') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-radio-group v-model="list.sy_sxsjgs">
                            <el-radio label="1">{{ lc('admin_system_00633') }}</el-radio>
                            <el-radio label="2">{{ lc('admin_system_00642') }}</el-radio>
                        </el-radio-group>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00585') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00620') }}</div>
                </td>
                <td>
                    <div class="TableInpt">
                        <el-input :placeholder="lc('wap_user_00076')" v-model="list.sy_closeOrder" type="number" min="0">
                            <template #suffix><span class="slotspan">{{ lc('common_02067') }}</span></template>
                        </el-input>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00603') }}</span>
                    </div>
                </td>
            </tr>
            </tbody>
        </table>
    </div>
    <div class="setBasicButn" style="border: none;">
        <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">{{ lc('wap_user_00176') }}</el-button>
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
                salary: [
                    {value: '1', label: lc('common_02056')},
                    {value: '2', label: lc('admin_01028')},
                    {value: '3', label: 'K'},
                    {value: '4', label: 'k'},
                ],
                province: [],
                value: '',
                uri: "m=system&c=",
                list: {},
                cache: {},
                city_index: [],
                city_type: [],

                saveLoading: false
            }
        },
        created() {
            this.getList();
        },
        methods: {
            cityChange: function () {
                this.list.sy_web_city_two = '';
                this.city(this.list.sy_web_city_one);
            },
            city: async function (cityId) {
                let _this = this;
                let url = _this.uri + 'set_web_config&a=city';
                let sendData = {};
                if (cityId) {
                    sendData.city_id = cityId;
                }
                let res = await httpPost(url, sendData, {hideloading: true})
                if (res.data.error == 0) {
                    _this.city_type = res.data.data.city
                }
            },
            getList: function () {
                let _this = this;
                let url = _this.uri + 'set_web_config&a=index';
                httpPost(url, {}, {hideloading: true}).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        _this.province = res.data.province;
                        _this.list = res.data.config;
                        if (_this.list.sy_web_city_one) {
                            _this.city(_this.list.sy_web_city_one)
                        } else {
                            _this.city('');
                        }
                    }
                })
            },
            save: function () {
                let _this = this;
                let url = this.uri + "set_web_config&a=save";
                let ruleForm = {};
                ruleForm.config = 1;
                ruleForm.sy_seo_rewrite = this.list.sy_seo_rewrite == '1' ? 1 : 0;
                ruleForm.sy_header_fix = this.list.sy_header_fix == "1" ? 1 : 0;
                ruleForm.sy_footer_fix = this.list.sy_footer_fix == "1" ? 1 : 0;
                ruleForm.sy_wap_comtpl = this.list.sy_wap_comtpl;
                ruleForm.sy_uni_comtpl = this.list.sy_uni_comtpl;
                ruleForm.sy_news_rewrite = this.list.sy_news_rewrite;
                ruleForm.sy_linksq = this.list.sy_linksq == "1" ? 1 : 0;
                ruleForm.sy_wap_jump = this.list.sy_wap_jump == "1" ? 1 : 0;
                ruleForm.sy_pc_jump_wap = this.list.sy_pc_jump_wap == "1" ? 1 : 0;
                ruleForm.sy_default_comclass = this.list.sy_default_comclass;
                ruleForm.sy_zhanzhang_baidu = this.list.sy_zhanzhang_baidu;
                ruleForm.sy_default_userclass = this.list.sy_default_userclass;
                ruleForm.sy_ewm_type = this.list.sy_ewm_type;
                ruleForm.sy_job_hits = parseInt(this.list.sy_job_hits);
                ruleForm.sy_indexpage = parseInt(this.list.sy_indexpage);
                ruleForm.sy_resume_visitors = parseInt(this.list.sy_resume_visitors);
                ruleForm.sy_datacycle = parseInt(this.list.sy_datacycle) > 0 ? parseInt(this.list.sy_datacycle) : 0;
                ruleForm.sy_datacycle_job = parseInt(this.list.sy_datacycle_job) > 0? parseInt(this.list.sy_datacycle_job) : 0;
                ruleForm.sy_datacycle_com = parseInt(this.list.sy_datacycle_com) > 0 ? parseInt(this.list.sy_datacycle_com) : 0;
                ruleForm.sy_logintime = parseInt(this.list.sy_logintime) > 0 ? parseInt(this.list.sy_logintime) : 1;
                ruleForm.sy_login_type = this.list.sy_login_type;
                ruleForm.sy_closeOrder = parseInt(this.list.sy_closeOrder) > 0 ? parseInt(this.list.sy_closeOrder) : 15;
                ruleForm.sy_h5_share = this.list.sy_h5_share == "1" ? 1 : 0;
                ruleForm.sy_advice_mobilecode = this.list.sy_advice_mobilecode == "1" ? 1 : 0;
                ruleForm.sy_job_lookfx = this.list.sy_job_lookfx == "1" ? 1 : 0;
                ruleForm.sy_wxwap_list = this.list.sy_wxwap_list;
                ruleForm.sy_adclick = parseInt(this.list.sy_adclick);
                ruleForm.sy_outlinks = this.list.sy_outlinks;
                ruleForm.sy_shenming = this.list.sy_shenming;
                ruleForm.sy_autoref = this.list.sy_autoref;
                ruleForm.sy_autorefrand = this.list.sy_autorefrand;
                ruleForm.sy_recommend_day_num = parseInt(this.list.sy_recommend_day_num);
                ruleForm.sy_recommend_interval = parseInt(this.list.sy_recommend_interval);
                ruleForm.sy_resumeout_day_num = parseInt(this.list.sy_resumeout_day_num);
                ruleForm.sy_resumeout_interval = parseInt(this.list.sy_resumeout_interval);
                ruleForm.sy_web_city_one = this.list.sy_web_city_one;
                ruleForm.sy_web_city_two = this.list.sy_web_city_two;
                ruleForm.resume_salarytype = this.list.resume_salarytype;
                ruleForm.sy_sxsjgs = this.list.sy_sxsjgs;
                _this.saveLoading = true;
                httpPost(url, ruleForm, {hideloading: true}).then(function (response) {
                    var res = response.data;
                    if (res.error == 0) {
                        message.success(lc('wap_user_00264'), _this.getList());
                    } else {
                        message.error(res.msg);
                    }
                }).finally(function () {
                    setTimeout(function () {
                        _this.saveLoading = false;
                    }, 2000);
                });
            }
        }
    }
</script>
