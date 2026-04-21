<template>
    <div>
        <!--运营-广告-广告管理 批量延期-->
        <div>
            <div class="hydialog_item" style="display: flex; align-items: center;">
                <span class="item_span">延长时间：</span>
                <el-input v-model="ruleForm.endtime" placeholder="请输入天数" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')">
                    <template slot="append">天</template>
                </el-input>
            </div>
        </div>
        <div slot="footer" class="dialog-footer">
            <el-button type="primary" @click="submitForm('ruleForm')" :disabled="submitLoading">添加</el-button>
        </div>
    </div>
</template>

<script setup>
module.exports = {
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
                message.error('请选输入天数！');
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