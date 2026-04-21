<template>
    <div>
        <div>
            <div class="hydialog_item" style="display: flex; align-items: center;">
                <span class="item_span" style="margin-top: -3px;">类别选择：</span>
                <el-radio-group v-model="ruleForm.ctype" @change="handleChange" style="flex: 1;">
                    <el-radio label="1">一级分类</el-radio>
                    <el-radio label="2">二级分类</el-radio>
                </el-radio-group>
            </div>
            <el-tabs v-model="ruleForm.ctype">
                <el-tab-pane label="1" name="1">
                    <div class="dialog_item">
                        <span class="item_span">类别名称：</span>
                        <el-input type="textarea" v-model="ruleForm.name" style="flex: 1;"></el-input>
                    </div>
                    <div class="dialog_item">
                        <span class="item_span">变量名称：</span>
                        <el-input type="textarea" v-model="ruleForm.str" style="flex: 1;"></el-input>
                    </div>
                </el-tab-pane>
                <el-tab-pane label="2" name="2">
                    <div class="dialog_item">
                        <span class="item_span">父类：</span>
                        <el-select v-model="ruleForm.nid" placeholder="请选择" style="flex: 1;">
                            <el-option v-for="item in position" :key="item.id" :label="item.name" :value="item.id"></el-option>
                        </el-select>
                    </div>
                    <div class="dialog_item">
                        <span class="item_span">类别名称：</span>
                        <el-input type="textarea" v-model="ruleForm.name" style="flex: 1;"></el-input>
                    </div>
                </el-tab-pane>
            </el-tabs>
            <i class="el-icon-info" style="margin-top: 10px;">说明：可以添加多条分类（请按回车Enter键换行，一行一个)</i>
        </div>
        <div slot="footer" class="dialog-footer">
            <el-button type="primary" @click="submitForm('ruleForm')" :disabled="submitLoading">添加</el-button>
        </div>
    </div>
</template>

<script setup>
module.exports = {
    props: {
        position: Array,
    },
    data: function () {
        return {
            ruleForm: {
                ctype: '1',
                nid: null,
                name: '',
                str: ''
            },
            submitLoading: false,
        }
    },
    created() {
    },
    methods: {
        handleChange(val) {
            this.ruleForm.ctype = val;
        },
        submitForm(formName) {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.ruleForm));
            params.name = params.name.split("\n").join("-");
            params.str = params.str.split("\n").join("-");

            if (params.ctype == '' || params.ctype == null) {
                message.error('请选择类型！');
                return;
            }
            if (params.name == '') {
                message.error('类别名称不能为空！');
                return;
            }
            if (params.ctype == '1' && $.trim(params.str) == '') {
                message.error('调用变量名不能为空！');
                return;
            }
            _this.submitLoading = true;
            httpPost('m=system&c=category_partclass&a=save', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                    _this.clearForm();
                    _this.addVisible = false;
                } else {
                    message.error(res.msg);
                }
                _this.$emit("child-event-getlist", params.ctype);
            }).catch(function (error) {
                console.log(error);
            }).finally(function () {
                _this.submitLoading = false;
            });
        },
        clearForm() {
            this.ruleForm.nid = null;
            this.ruleForm.name = '';
            this.ruleForm.str = '';
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
    width: 75px;
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