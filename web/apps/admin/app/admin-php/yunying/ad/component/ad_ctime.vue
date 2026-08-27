<template>
    <div>
        <!--运营-广告-广告管理 批量延期-->
        <div>
            <div class="hydialog_item" style="display: flex; align-items: center;">
                <span class="item_span">{{ lc('admin_user_weipin_00025') }}</span>
                <el-input v-model="ruleForm.endtime" :placeholder="lc('admin_00614')" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')">
                    <template #append>{{ lc('common_02067') }}</template>
                </el-input>
            </div>
        </div>
        <div class="dialog-footer">
            <el-button type="primary" @click="submitForm('ruleForm')" :disabled="submitLoading">{{ lc('wap_js_00091') }}</el-button>
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
        jobid: String,
    },
    data: function () {
        return {
            ruleForm: {
                endtime: '',
            },
            submitLoading: false,
        }
    },
    mounted() {
        console.log('ad_ctime mounted')
    },
    methods: {
        submitForm(formName) {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.ruleForm));
            params.jobid = this.jobid;

            if (params.endtime == '' || params.endtime < 1) {
                message.error(lc('admin_vue_00092'));
                return;
            }
            _this.submitLoading = true;
            httpPost('m=yunying&c=ad&a=ctime', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                    _this.$emit("child-event");
                } else {
                    message.error(res.msg);
                }
            }).catch(function (error) {
                console.log(error);
            }).finally(function () {
                _this.submitLoading = false;
            });
        },
    }
}
</script>

<style scoped>
.dialog_item {
    margin-top: 25px;
    display: flex;
}

.item_span {
    width: 90px;
    text-align: right;
    display: block;
}

.dialog-footer {
    padding: 30px 0 0;
    text-align: right;
    -webkit-box-sizing: border-box;
    box-sizing: border-box;
}
</style>