<template>
    <div class="drawerModlue">
        <div class="drawerModInfo" style="max-height: calc(100% - 80px); overflow-y: auto;">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_00966'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="ruleForm.nid" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                        <el-option v-for="item in type" :key="item.id" :label="item.typename" :value="item.id">
                        </el-option>
                    </el-select>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_00967'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.name" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='member_user_00299'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.sort" @input="inputIntNumber($event, 'ruleForm', 'sort')" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_00968'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.eject" label="1">{yun:}t key='admin_00205'{/yun}</el-radio>
                    <el-radio v-model="ruleForm.eject" label="0">{yun:}t key='admin_00203'{/yun}</el-radio>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_00969'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.model" label="hot">{yun:}t key='common_02091'{/yun}</el-radio>
                    <el-radio v-model="ruleForm.model" label="new">{yun:}t key='common_02081'{/yun}</el-radio>
                    <el-radio v-model="ruleForm.model" label="">{yun:}t key='common_02082'{/yun}</el-radio>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_00970'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.bold" label="1">{yun:}t key='common.yes'{/yun}</el-radio>
                    <el-radio v-model="ruleForm.bold" label="0">{yun:}t key='common.no'{/yun}</el-radio>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00087'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.display" label="1">{yun:}t key='common.yes'{/yun}</el-radio>
                    <el-radio v-model="ruleForm.display" label="0">{yun:}t key='common.no'{/yun}</el-radio>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">{yun:}t key='common.save'{/yun}</el-button>
        </div>
    </div>
</template>
    
<script>
module.exports = {
    props:['config', 'name'],
    data: function () {
        return {
            type: [],
            ruleForm: {},
            saveLoading: false,
        }
    },

    mounted() {

    },
    created: function () {
        this.getInfo();
    },
    methods: {
        inputIntNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9]/g,'');
        },
        getInfo() {
            let that = this;
            httpPost('m=system&c=set_module&a=navset', {config: that.config, name: that.name},{hideloading: true}).then(function (response) {
                let data = response.data.data;

                that.type = data.type;

                that.ruleForm = data.nav;
                if (!data.nav.id) {
                    that.ruleForm.eject = '0';
                    that.ruleForm.model = '';
                    that.ruleForm.bold = '0';
                    that.ruleForm.display = '0';
                }
            })
        },
        save(){
            let that = this;
            that.saveLoading = true;
            httpPost('m=system&c=set_module&a=navsetSave', that.ruleForm,{hideloading: true}).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    message.success(res.msg, function() {
                        if (res.data) {
                            that.ruleForm.id = res.data.id;
                        }
                        that.$emit("child-event");
                    })
                }
            }).finally(function () {
                setTimeout(function () {
                    that.saveLoading = false;
                }, 2000);
            });
        },
    },
    watch: {
        config: function (val, oldVal) {
            this.ruleForm = {};
            this.ruleForm.eject = '0';
            this.ruleForm.model = '';
            this.ruleForm.bold = '0';
            this.ruleForm.display = '0';

            this.getInfo();
        }
    }
};
</script>
<style scoped></style>