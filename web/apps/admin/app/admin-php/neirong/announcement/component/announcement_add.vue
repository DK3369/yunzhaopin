<template>
    <div class="drawerModlue">
        <div class="moduleTable" style="max-height: calc(100% - 80px);">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="100">{{ lc('admin_00103') }}</th>
                    <th>{{ lc('wap_user_00102') }}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_00102') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt w_400">
                            <el-input :placeholder="lc('admin_00869')" v-model="ruleForm.title"></el-input>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_00126') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect w_400" style="display: flex;align-items: center;">
                            <el-select v-model="ruleForm.did" :placeholder="lc('wap_user_00100')">
                                <el-option v-for="(item, key) in domainList" :key="key" :label="item" :value="key"></el-option>
                            </el-select>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_00149') }}</div>
                    </td>
                    <td style="display: flex;">
                        <div class="w_400" style="display: flex;align-items: center;">
                            <el-input :placeholder="lc('wap_00510')" v-model="ruleForm.keyword"></el-input>
                        </div>
                        <div class="TableShuom" style="padding: 6px;">
                            <span><i class="el-icon-warning"></i>{{ lc('admin_00135') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('wap_user_00087') }}</div>
                    </td>
                    <td style="display: flex;">
                        <div class="TableInpt w_400">
                            <el-date-picker v-model="ruleForm.startime" type="date" :picker-options="pickerOptions" style="width: 100%;" :placeholder="lc('wap_com_00323')"></el-date-picker>
                        </div>
                        <div class="TableShuom" style="padding: 6px;">
                            <span><i class="el-icon-warning"></i>{{ lc('admin_00140') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('wap_user_00096') }}</div>
                    </td>
                    <td style="display: flex;">
                        <div class="TableInpt w_400">
                            <el-date-picker v-model="ruleForm.endtime" type="date" :picker-options="pickerOptions" style="width: 100%;" :placeholder="lc('wap_com_00324')"></el-date-picker>
                        </div>
                        <div class="TableShuom" style="padding: 6px;">
                            <span><i class="el-icon-warning"></i>{{ lc('admin_00138') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_00231') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt w_400">
                            <el-input type="textarea" :placeholder="lc('wap_00936')" v-model="ruleForm.description"></el-input>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_00868') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <textarea type="textarea" id="projectBasis" class="editor" name="projectBasis" cols="150" rows="30" style="width: 80%">
                            </textarea>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
        </div>
    </div>
</template>
<!-- script -->
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
        props: ['id'],
        data: function () {
            return {
                pickerOptions: {
                    disabledDate(time) {
                        // 禁止选择过去日期（不能选择今天）
                        return time.getTime() < Date.now();
                    },
                },
                ruleForm: {},
                domainList: [],
                saveLoading: false,
                ue:''
            }
        },
        mounted() {
            var ue = UE.getEditor('projectBasis', {
                wordCount: false,           // 关闭字数统计
                elementPathEnabled: false,  //{{ lc('common.close') }}elementPath {{ lc('common_05704') }}
                autoHeightEnabled: false,   //关闭自适应高度，超出部分以滚动条形式展示
                initialFrameHeight: 480,    //默认的编辑区域高度
                initialFrameWidth: 600      //初始化编辑器宽度,{{ lc('wap_js_00098') }}1000
            });

        },
        created: function () {
            this.getInfo();
        },
        methods: {
            getInfo() {
                let that = this;
                httpPost('m=neirong&c=announcement&a=add', {id: that.id ? that.id : ''}).then(function (response) {
                    let res = response.data,
                        data = res.data,
                        info = data.info;

                    that.domainList = data.domainList;
                    if (that.id) {
                        that.ruleForm = {
                            id: info.id,
                            title: info.title,
                            did: info.did,
                            keyword: info.keyword,
                            startime: info.startime > 0 ? new Date(info.startime_n) : new Date(),
                            endtime: info.endtime > 0 ? new Date(info.endtime_n) : '',
                            description: info.description,
                        };
                        if (info.content){
                            UE.getEditor('projectBasis').setContent(info.content);
                        }
                        // editor.setHtml(info.content);
                    } else {
                        that.ruleForm = {
                            did: '-1',
                            startime: new Date()
                        };
                    }
                })
            },

            save() {
                let that = this,
                    params = that.ruleForm;

                if (typeof params.title == 'undefined' || params.title == '') {
                    message.warning(lc('admin_00869'));
                    return;
                }
                if (typeof params.keyword == 'undefined' || params.keyword == '') {
                    message.warning(lc('admin_vue_00066'));
                    return;
                }

                if (params.startime && params.endtime && params.startime > params.endtime) {
                    message.warning(lc('admin_vue_00067'));
                    return;
                }

                if (typeof params.description == 'undefined' || params.description == '') {
                    message.warning(lc('admin_vue_00068'));
                    return;
                }


                if (that.saveLoading) {
                    return false;
                }
                that.saveLoading = true;

                params.content = UE.getEditor('projectBasis').getContent();

                params.submit = true;
                httpPost('m=neirong&c=announcement&a=add', params).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        message.error(res.msg, function () {
                            that.saveLoading = false;
                        });
                    } else {
                        that.$emit("child-event");
                        message.success(res.msg, function () {
                            that.saveLoading = false;
                        });
                    }
                })
            },
        },
        watch: {
            id: function (val, oldVal) {
                this.ruleForm = {};
                UE.getEditor('projectBasis').setContent('');
                this.getInfo();
            },
        }
    };
</script>
<style>
    .w_400{ width: 400px;}
    .tableVue .TableSelect .el-select{ width: 400px;}
</style>