<template>
    <div class="tableDome" style="top: 40px;">
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
                        <div class="TableTite">{{ lc('admin_00219') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input :placeholder="lc('wap_user_00076')" v-model="ruleForm.position">
                                <!-- <template #suffix><span class="slotspan">天</span></template> -->
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_00219') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_system_00104') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input :placeholder="lc('wap_user_00076')" v-model="ruleForm.e_name">
                                <!-- <template #suffix><span class="slotspan">天</span></template> -->
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_system_00104') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_00290') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="ruleForm.nid" :placeholder="lc('wap_user_00100')" @change="getClass(ruleForm.nid)" clearable>
                                <el-option v-for="item in position" :key="item.id" :label="item.name" :value="item.id"></el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_00290') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_00291') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="ruleForm.keyid" :placeholder="lc('wap_user_00100')" clearable>
                                <el-option v-for="item in positionTwo" :key="item.id" :label="item.name"
                                    :value="item.id">
                                </el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_00291') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_system_00106') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input type="textarea" :placeholder="lc('wap_user_00076')" v-model="ruleForm.content">
                                <!-- <template #suffix><span class="slotspan">天</span></template> -->
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_system_00106') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_system_00105') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.sort" :placeholder="lc('admin_user_00342')" @input="inputIntNumber($event, 'ruleForm', 'sort')"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_00218') }}</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">{{ lc('common.submit') }}</el-button>
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
    props: {
        tid: {type: [Number, String], default: 0},//第二级
        id: {type: [Number, String], default: 0},//第三级
    },
    data: function () {
        return {
            ruleForm: {
                id: 0,//提交时的id
                position: '',//{{ lc('admin_00219') }}
                e_name: "",//英文名称/拼音
                nid: null,//{{ lc('admin_00290') }}
                keyid: null,//{{ lc('admin_00291') }}
                content: null,//{{ lc('admin_system_00106') }}
                sort: "",//{{ lc('member_com_00022') }}
            },
            position: [],//{{ lc('admin_00290') }}
            positionTwo: [],//第二级分类
            submitLoading: false,
        }
    },
    mounted() {
        if (this.tid > 0 || this.id > 0) {
            // Update
            this.ruleForm.id = this.tid > 0 ? (this.tid) : (this.id > 0 ? this.id : 0);
            this.getInfo();
        } else {
            // Add
            this.ruleForm.id = 0;
            this.getPosition();
        }
    },
    methods: {
        getInfo() {
            let _this = this;
            let params = {};
            if (this.id) {
                params.id = this.id;
            }
            if (this.tid) {
                params.tid = this.tid;
            }
            httpPost('m=system&c=category_job_class&a=classadd', params).then(function (response) {
                let res = response.data;
                if (res.data.info) {
                    _this.ruleForm.position = res.data.info.name;
                    _this.ruleForm.e_name = res.data.info.e_name;
                    _this.ruleForm.content = res.data.info.content;
                    _this.ruleForm.sort = res.data.info.sort;
                    if (res.data.type === 'two') {
                        _this.ruleForm.nid = res.data.info.keyid;
                    } else if (res.data.type === 'three') {
                        _this.ruleForm.nid = res.data.job.keyid;
                        _this.ruleForm.keyid = res.data.info.keyid;
                        _this.positionTwo = res.data.class2;
                    }
                }
                _this.position = res.data.position;
            }).catch(function (error) {
                console.log(error);
            });
        },
        getPosition() {
            let _this = this;
            httpPost('m=system&c=category_job_class&a=getJobClass', {}, {hideloading: true}).then(function (response) {
                let res = response.data;
                _this.position = res.data;
            }).catch(function (error) {
                console.log(error);
            });
        },
        getClass(nid) {
            let _this = this;
            if (nid <= 0) {
                _this.ruleForm.keyid = null;
                _this.positionTwo = [];
                return false;
            }
            this.ruleForm.keyid = null;
            httpPost('m=system&c=category_job_class&a=get_class', {nid: nid}).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    _this.positionTwo = res.data;
                } else {
                    _this.positionTwo = [];
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        submitForm(formName) {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.ruleForm));
            params.submit = 'submit';
            if (params.position == '') {
                message.error(window.yunAdminT(lc('admin_00208')));
                return;
            }
            _this.submitLoading = true;
            httpPost('m=system&c=category_job_class&a=save', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                    _this.$emit("child-event-getlist");
                } else {
                    message.error(res.msg);
                }
            }).catch(function (error) {
                console.log(error);
            }).finally(function () {
                _this.submitLoading = false;
            });
        },
        inputIntNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9]/g, '');
        },
    },
};
</script>
<style scoped>
.moduleTable {
    max-height: calc(100% - (60px + 20px));
}
</style>