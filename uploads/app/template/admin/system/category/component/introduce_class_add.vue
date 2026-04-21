<template>
    <div class="drawerModlue" v-loading="addloading" style="display: table;">
        <div class="drawerModInfo">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>介绍名称：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.name" placeholder="请输入内容"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>介绍内容：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input type="textarea" :rows="10" placeholder="请输入内容" v-model="ruleForm.content"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>排序：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.sort" placeholder="请输入内容" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"></el-input>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="submitForm" :disabled="submitLoading">提交</el-button>
        </div>
    </div>
</template>

<script>
const {createEditor, createToolbar} = window.wangEditor
module.exports = {
    props: {
        id: Number,
    },
    data: function () {
        return {
            ruleForm: {
                name: '',
                sort: '',
                content: '',
            },
            addloading: false,
            submitLoading: false
        }
    },
    mounted() {
    },
    methods: {
        getInfo() {
            if (!this.id) {
                return false;
            }
            this.addloading = true;
            let _this = this;
            httpPost('m=system&c=category_introduce_class&a=classadd', {id: this.id}).then(function (response) {
                _this.addloading = false;
                let res = response.data;
                if (res.error === 0) {
                    _this.ruleForm.name = res.data.name;
                    _this.ruleForm.sort = res.data.sort;
                    _this.ruleForm.content = res.data.content;
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        submitForm(formName) {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.ruleForm));

            if (params.name == '') {
                message.error('类别名称不能为空！');
                return;
            }
            params.id = this.id;
            _this.submitLoading = true;
            httpPost('m=system&c=category_introduce_class&a=save', params).then(function (response) {
                _this.submitLoading = false;
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                    _this.clearForm();
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
        clearForm() {
            this.ruleForm.name = '';
            this.ruleForm.sort = '';
            this.ruleForm.content = '';
        },
    },
    watch: {
        id: {
            handler(val, oldVal) {
                if (val) {
                    this.getInfo();
                }
            },
            deep: true,
            immediate: true
        }
    }
}
</script>

<style scoped>
</style>