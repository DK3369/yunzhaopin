<template>
    <div class="drawerModlue">
        <div class="drawerModInfo">
            <div v-if="keyid_name" class="drawerModLis">
                <div class="drawerModTite">
                    <span>上　　级：</span>
                </div>
                <div class="drawerModInpt">
                    <div class="spandiv">{{ keyid_name }}</div>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>城市名称：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.name" placeholder="请输入内容"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>英文名称/拼音：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.e_name"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>字　　母：</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="ruleForm.letter" placeholder="请选择" filterable>
                        <el-option v-for="item in letterOptions" :key="item" :label="item" :value="item">
                        </el-option>
                    </el-select>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>显　　示：</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="ruleForm.display" placeholder="请选择">
                        <el-option v-for="item in displayOptions" :key="item.value" :label="item.label" :value="item.value">
                        </el-option>
                    </el-select>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>区域编号：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.code" placeholder=""></el-input>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">保存</el-button>
        </div>
    </div>
</template>

<script setup>
module.exports = {
    props: {
        keyid: {type: [Number, String], default: 0},
        keyid_name: {type: String, default: ''},
        level: {type: [Number, String], default: 0},
        letterOptions: {type: Array, default: []},
        displayOptions: {type: Array, default: []}
    },
    data: function () {
        return {
            ruleForm: {
                keyid: 0,
                name: '',
                letter: '',
                display: "1",
                sort: 0,
                e_name: '',
                code: null,
            },
            submitLoading: false,
        }
    },
    mounted() {
        console.log('city_add mounted')
    },
    methods: {
        handleChange(val) {
            this.ruleForm.ctype = val;
        },
        submitForm(formName) {
            let _this = this;
            this.ruleForm.keyid = this.keyid;
            let params = JSON.parse(JSON.stringify(this.ruleForm));
            if (params.name == '') {
                message.error('城市名称不能为空！');
                return;
            }
            _this.submitLoading = true;
            httpPost('m=system&c=category_city&a=add_single', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                    _this.clearForm();
                } else {
                    message.error(res.msg);
                }
                _this.$emit("child-event-getlist", _this.keyid);
            }).catch(function (error) {
                console.log(error);
            }).finally(function () {
                _this.submitLoading = false;
            });
        },
        clearForm() {
            this.ruleForm.name = '';
            this.ruleForm.letter = '';
            this.ruleForm.display = "1";
            this.ruleForm.sort = 0;
            this.ruleForm.e_name = '';
            this.ruleForm.code = null;
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

.drawerModTite {
    width: 110px;
}

.drawerModInpt {
    width: calc(100% - 120px);
}
</style>