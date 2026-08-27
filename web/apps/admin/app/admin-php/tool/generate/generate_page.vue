<template>
<div id="daohaapp" class="moduleElenAl">
    <div class="tableDome">
        <div class="tableDome_tip">
            <template>
                <el-alert
                    :title="lc('admin_tool_00405')"
                    type="success"
                    :description="lc('admin_html_00001')" :closable="false">
                </el-alert>
            </template>
        </div>
        <div class="tool_boxall">
            <div class="tool_box">
                <el-card class="box-card">
                    <div class="tool_name"><i class="el-icon-folder"></i> {{ lc('admin_tool_00409') }}</div>
                    <div class="tool_tip">{{ lc('admin_tool_00388') }}</div>
                    <div class="tool_but">
                        <el-button type="primary" size=" " icon="el-icon-folder" @click="indexVisible = true">{{ lc('admin_tool_00396') }}</el-button>
                    </div>
                </el-card>
            </div>
            <div class="tool_box">
                <el-card class="box-card">
                    <div class="tool_name"><i class="el-icon-folder-opened"></i> {{ lc('admin_00195') }}</div>
                    <div class="tool_tip">{{ lc('admin_tool_00388') }}</div>
                    <div class="tool_but">
                        <el-button type="primary" size=" " icon="el-icon-folder-opened" @click="newsVisible = true">{{ lc('admin_tool_00392') }}</el-button>
                    </div>
                </el-card>
            </div>
            <div class="tool_box">
                <el-card class="box-card">
                    <div class="tool_name"><i class="el-icon-reading"></i> {{ lc('admin_tool_00399') }}</div>
                    <div class="tool_tip">{{ lc('admin_tool_00388') }}</div>
                    <div class="tool_but">
                        <el-button type="primary" size=" " icon="el-icon-reading" @click="archiveVisible = true">{{ lc('admin_tool_00390') }}</el-button>
                    </div>
                </el-card>
            </div>
            <div class="tool_box">
                <el-card class="box-card">
                    <div class="tool_name"><i class="el-icon-tickets"></i> {{ lc('admin_tool_00398') }}</div>
                    <div class="tool_tip">{{ lc('admin_tool_00388') }}</div>
                    <div class="tool_but">
                        <el-button type="primary" size=" " icon="el-icon-tickets" @click="onceVisible = true">{{ lc('admin_tool_00393') }}</el-button>
                    </div>
                </el-card>
            </div>
        </div>
        <div class="tool_boxall">
            <div class="tool_box">
                <el-card class="box-card">
                    <div class="tool_name"><i class="el-icon-tickets"></i> {{ lc('admin_00170') }}</div>
                    <div class="tool_tip">{{ lc('admin_tool_00388') }}</div>
                    <div class="tool_but">
                        <el-button type="primary" size=" " icon="el-icon-tickets" @click="newsclassVisible = true">{{ lc('admin_tool_00391') }}</el-button>
                    </div>
                </el-card>
            </div>
            <div class="tool_box">
                <el-card class="box-card">
                    <div class="tool_name"><i class="el-icon-tickets"></i> {{ lc('admin_tool_00400') }}</div>
                    <div class="tool_tip">{{ lc('admin_tool_00388') }}</div>
                    <div class="tool_but">
                        <el-button type="primary" size=" " icon="el-icon-tickets" @click="allVisible = true">{{ lc('admin_tool_00400') }}</el-button>
                    </div>
                </el-card>
            </div>
            <div class="tool_box">
            </div>
            <div class="tool_box">
            </div>
        </div>
    </div>
    <!--生成首页-->
    <div class="modluDrawer">
        <el-dialog :title="lc('admin_tool_00410')" v-model="indexVisible" :with-header="true" :modal-append-to-body="false"
            :show-close="true" width="500px">
            <div class="yunyinDialog">
                <div class="tableDoAlert">
                    {{ lc('admin_tool_00387') }}
                    <br>
                    {{ lc("admin_nav_link_tip", [sy_weburl]) }}
                </div>
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>{{ lc('admin_tool_00411') }}</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-input v-model="ruleFormIndex.make_index_url" :placeholder="lc('admin_tool_00412')"></el-input>
                    </div>
                </div>
            </div>
            <template #footer><span class="dialog-footer">
                <el-button @click="indexVisible = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                <el-button type="primary" @click="submitFormIndex">{{ lc('wap_com_00019') }}</el-button>
            </span></template>
        </el-dialog>
    </div>
    <!--生成新闻首页-->
    <div class="modluDrawer">
        <el-dialog :title="lc('admin_tool_00415')" v-model="newsVisible" :with-header="true" :modal-append-to-body="false"
            :show-close="true" width="500px">
            <div class="yunyinDialog">
                <div class="tableDoAlert">
                    {{ lc('admin_tool_00387') }}
                    <br>
                    {{ lc("admin_nav_link_tip", [sy_weburl]) }}
                </div>
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>{{ lc('admin_tool_00413') }}</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-input v-model="ruleFormNews.make_new_url" :placeholder="lc('admin_tool_00414')"></el-input>
                    </div>
                </div>
            </div>
            <template #footer><span class="dialog-footer">
                <el-button @click="newsVisible = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                <el-button type="primary" @click="submitFormNews">{{ lc('wap_com_00019') }}</el-button>
            </span></template>
        </el-dialog>
    </div>
    <!--生成新闻详细页-->
    <div class="modluDrawer">
        <el-drawer :title="lc('admin_tool_00394')" v-model="archiveVisible" :modal-append-to-body="false" :show-close="true"
            :destroy-on-close="false" size="700px">
            <div class="drawerModlue">
                <div class="tableDome_tip tableDoAlert">
                    <div class="shiTopAllTips">
                        <p>{{ lc('admin_tool_00387') }}</p>
                        <p>{{ lc("admin_nav_link_tip", [sy_weburl]) }}</p>
                    </div>
                </div>
                <div class="moduleTable" style="margin-top:10px;">
                    <table class="tableVue">
                        <thead>
                        <tr align="left">
                            <th width="100">{{ lc('member_com_00021') }}</th>
                            <th width="300">{{ lc('wap_user_00102') }}</th>
                            <th width=" ">{{ lc('member_com_00207') }}</th>
                        </tr>
                        </thead>
                        <tbody>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_tool_00404') }}</div>
                            </td>
                            <td>
                                <div class="TableSelect">
                                    <el-select v-model="ruleFormArchive.group" placeholder="">
                                        <el-option :label="lc('wap_js_00075')" value="0"></el-option>
                                        <el-option
                                            v-for="item in news_group_list"
                                            :key="item.id"
                                            :label="item.name"
                                            :value="item.id">
                                        </el-option>
                                    </el-select>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>  </span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_00751') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-date-picker v-model="ruleFormArchive.time" type="daterange" :range-separator="lc('admin_vue_00136')" :start-placeholder="lc('admin_00343')"
                                        :end-placeholder="lc('admin_00344')" value-format="YYYY-MM-dd">
                                    </el-date-picker>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span></span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_tool_00401') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormArchive.startid" @input="inputIntNumber($event, 'ruleFormArchive', 'startid')" :placeholder="lc('admin_user_00342')"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_tool_00397') }}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_tool_00403') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormArchive.endid" @input="inputIntNumber($event, 'ruleFormArchive', 'endid')" :placeholder="lc('admin_user_00342')"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_tool_00395') }}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_tool_00402') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="ruleFormArchive.limit" @input="inputIntNumber($event, 'ruleFormArchive', 'limit')" :placeholder="lc('admin_user_00342')"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_tool_00389') }}</span>
                                </div>
                            </td>
                        </tr>
                        </tbody>
                    </table>
                </div>
                <div class="setBasicButn" style="border: none;">
                    <el-button type="primary" size="medium" @click="submitFormArchive" :disabled="submitLoadingArchive">{{ lc('wap_user_00176') }}</el-button>
                </div>
            </div>
        </el-drawer>
    </div>
    <!--生成单页面-->
    <div class="modluDrawer">
        <el-dialog :title="lc('admin_tool_00398')" v-model="onceVisible" :with-header="true" :modal-append-to-body="false"
            :show-close="true" width="500px">
            <div class="yunyinDialog">
                <div class="tableDoAlert">
                    {{ lc('admin_tool_00387') }}
                    <br>
                    {{ lc("admin_nav_link_tip", [sy_weburl]) }}
                </div>
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>{{ lc('admin_tool_00404') }}</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-select v-model="ruleFormOnce.group" placeholder="">
                            <el-option :label="lc('wap_js_00075')" value="0"></el-option>
                            <el-option
                                v-for="item in description_list"
                                :key="item.id"
                                :label="item.name"
                                :value="item.id">
                            </el-option>
                        </el-select>
                    </div>
                </div>
            </div>
            <template #footer><span class="dialog-footer">
                <el-button @click="onceVisible = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                <el-button type="primary" @click="submitFormOnce">{{ lc('wap_com_00019') }}</el-button>
            </span></template>
        </el-dialog>
    </div>
    <!--生成新闻类别-->
    <div class="modluDrawer">
        <el-dialog :title="lc('admin_tool_00416')" v-model="newsclassVisible" :with-header="true" :modal-append-to-body="false"
            :show-close="true" width="500px">
            <div class="yunyinDialog">
                <div class="tableDoAlert">
                    {{ lc('admin_tool_00387') }}
                    <br>
                    {{ lc("admin_nav_link_tip", [sy_weburl]) }}
                </div>
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>{{ lc('admin_tool_00404') }}</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-select v-model="ruleFormNewsclass.group" :placeholder="lc('wap_user_00100')">
                            <el-option
                                v-for="item in news_group_list"
                                :key="item.id"
                                :label="item.name"
                                :value="item.id">
                            </el-option>
                        </el-select>
                    </div>
                </div>
            </div>
            <template #footer><span class="dialog-footer">
                <el-button @click="newsclassVisible = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                <el-button type="primary" @click="submitFormNewsclass">{{ lc('wap_com_00019') }}</el-button>
            </span></template>
        </el-dialog>
    </div>
    <!--  一键更新-->
    <div class="modluDrawer">
        <el-dialog :title="lc('admin_tool_00400')" v-model="allVisible" :with-header="true" :modal-append-to-body="false"
            :show-close="true" width="500px">
            <div class="yunyinDialog">
                <div class="tableDoAlert">
                    {{ lc('admin_tool_00387') }}
                    <br>
                    {{ lc("admin_nav_link_tip", [sy_weburl]) }}
                </div>
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>{{ lc('admin_tool_00411') }}</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-input v-model="ruleFormAll.make_index_url" :placeholder="lc('admin_tool_00412')"></el-input>
                    </div>
                </div>
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>{{ lc('admin_tool_00413') }}</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-input v-model="ruleFormAll.make_new_url" :placeholder="lc('admin_tool_00414')"></el-input>
                    </div>
                </div>
            </div>
            <template #footer><span class="dialog-footer">
                <el-button @click="allVisible = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                <el-button type="primary" @click="submitFormAll">{{ lc('wap_com_00019') }}</el-button>
            </span></template>
        </el-dialog>
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
                submitLoading: false,
                sy_weburl: localStorage.getItem("sy_weburl"),
                config: {},
                news_group_list: [],
                description_list: [],
                //生成首页
                indexVisible: false,
                ruleFormIndex: {
                    make_index_url: '',
                    madeall: '1',
                },
                //生成新闻首页
                newsVisible: false,
                ruleFormNews: {
                    make_new_url: '',
                    madeall: '1',
                },
                //生成新闻详细页
                archiveVisible: false,
                ruleFormArchive: {
                    action: 'makearchive',
                    group: '0',
                    time: null,
                    stime: '',
                    etime: '',
                    startid: '0',
                    endid: '0',
                    limit: '20',
                },
                submitLoadingArchive: false,
                //生成单页面
                onceVisible: false,
                ruleFormOnce: {
                    make: 1,
                    group: '0',
                },
                //生成新闻类别
                newsclassVisible: false,
                ruleFormNewsclass: {
                    action: 'makeclass',
                    group: null,
                },
                submitLoadingNewsclass: false,
                //一键更新
                allVisible: false,
                ruleFormAll: {
                    action: 'makeall',
                    make_index_url: '',
                    make_new_url: '',
                    type: 'cache',
                    value: 0,
                },
            }
        },
        created(){
            this.getBaseData();
        },
        methods: {
            getBaseData() {
                let _this = this;
                httpPost('m=tool&c=generate_page&a=baseData', {}).then(function (response) {
                    let res = response.data;
                    if (res.error === 0) {
                        _this.config = res.data.config;
                        _this.ruleFormIndex.make_index_url = _this.config.make_index_url;
                        _this.ruleFormAll.make_index_url = _this.config.make_index_url;
                        _this.ruleFormNews.make_new_url = _this.config.make_new_url;
                        _this.ruleFormAll.make_new_url = _this.config.make_new_url;
                        _this.news_group_list = res.data.news_group_list;
                        _this.description_list = res.data.description_list;
                    }
                }).catch(function (error) {
                    console.log(error);
                });
            },
            inputIntNumber(val, form, key) {
                this.$data[form][key] = val.replace(/[^0-9]/g,'');
                // console.log(this[form])
                // this.$set(this[form], key, val.replace(/[^0-9]/g,''))
            },
            submitFormIndex() {
                let _this = this;
                let params = JSON.parse(JSON.stringify(this.ruleFormIndex));
                if (_this.submitLoading) {
                    return false;
                }
                _this.submitLoading = true;
                httpPost('m=tool&c=generate_page&a=index', params).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        message.success(res.msg);
                        _this.indexVisible = false;
                    } else {
                        message.error(res.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                }).finally(function () {
                    _this.submitLoading = false;
                });
            },
            submitFormNews() {
                let _this = this;
                let params = JSON.parse(JSON.stringify(this.ruleFormNews));
                if (_this.submitLoading) {
                    return false;
                }
                _this.submitLoading = true;
                httpPost('m=tool&c=generate_page&a=news', params).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        message.success(res.msg);
                        _this.newsVisible = false;
                    } else {
                        message.error(res.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                }).finally(function () {
                    _this.submitLoading = false;
                });
            },
            submitFormArchive() {
                let _this = this;
                let params = JSON.parse(JSON.stringify(this.ruleFormArchive));
                if (Array.isArray(params.time)) {
                    params.stime = params.time[0];
                    params.etime = params.time[1];
                }
                delete params.time;

                this.submitLoadingArchive = true;
                message.success(lc('admin_tool_00406'));
                this.makearchive(params, "archive", 0, lc('admin_tool_00406'));
            },
            /**
             * @param params  JSON: action, group, stime, etime, startid, endid, limit
             * @param type
             * @param value
             * @param msg
             */
            makearchive(params, type, value, msg) {
                if (type != 'ok') {
                    let _this = this;
                    params.type = type;
                    params.value = value
                    httpPost('m=tool&c=generate_page&a=archive', params).then(function (response) {
                        let res = response.data;
                        if (res.error == 0) {
                            message.success(res.msg);
                            _this.makearchive(params, res.data.type, res.data.value, res.msg);
                        } else {
                            message.error(res.msg);
                        }
                    }).catch(function (error) {
                        console.log(error);
                        message.error(error.msg);
                    })
                } else {
                    this.newsVisible = false;
                    this.submitLoadingArchive = false;
                }
            },
            submitFormOnce() {
                let _this = this;
                let params = JSON.parse(JSON.stringify(this.ruleFormOnce));
                if (_this.submitLoading) {
                    return false;
                }
                _this.submitLoading = true;
                httpPost('m=tool&c=generate_page&a=once', params).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        message.success(res.msg);
                        _this.onceVisible = false;
                    } else {
                        message.error(res.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                }).finally(function () {
                    _this.submitLoading = false;
                });
            },
            submitFormNewsclass() {
                let _this = this;
                let params = JSON.parse(JSON.stringify(this.ruleFormNewsclass));
                if (!params.group) {
                    params.group = 'all'
                }
                _this.submitLoadingNewsclass = true;
                message.success(lc('admin_tool_00407'));
                this.makenewsclass(params, "class", 0, lc('admin_tool_00407'));
            },
            /**
             * @param params JSON action, group
             * @param type
             * @param value
             * @param msg
             */
            makenewsclass(params, type, value, msg) {
                if(type != 'ok'){
                    let _this = this;
                    params.type = type;
                    params.value = value
                    httpPost('m=tool&c=generate_page&a=newsclass', params).then(function (response) {
                        let res = response.data;
                        if (res.error == 0) {
                            message.success(res.msg);
                            _this.makenewsclass(params, res.data.type, res.data.value, res.msg);
                        } else {
                            message.error(res.msg);
                        }
                    }).catch(function (error) {
                        console.log(error);
                        message.error(error.msg);
                    })
                } else {
                    this.newsclassVisible = false;
                    this.submitLoadingNewsclass = false;
                }
            },
            submitFormAll() {
                let _this = this;
                let params = JSON.parse(JSON.stringify(this.ruleFormAll));

                _this.submitLoading = true;
                this.make_all(params, "cache", 0, lc('admin_tool_00408'));
            },
            make_all(params, type, value, msg) {
                if(type != 'ok'){
                    let _this = this;
                    params.type = type;
                    params.value = value
                    httpPost('m=tool&c=generate_page&a=all', params).then(function (response) {
                        let res = response.data;
                        if (res.error == 0) {
                            message.success(res.msg);
                            _this.make_all(params, res.data.type, res.data.value, res.msg);
                        } else {
                            res.msg ? message.error(res.msg) : message.error(lc('admin_00334'));
                        }
                    }).catch(function (error) {
                        console.log(error);
                        message.error(error.msg);
                    })
                } else {
                    this.allVisible = false;
                    this.submitLoading = false;
                }
            },
        }
    }
</script>
