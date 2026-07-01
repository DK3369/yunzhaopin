<template>
    <div class="drawerModlue">
        <div class="drawerModInfo" style="max-height: calc(100% - 80px); overflow-y: auto;">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_00966') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="ruleForm.nid" :placeholder="lc('wap_user_00100')">
                        <el-option v-for="item in type" :key="item.id" :label="item.typename" :value="item.id">
                        </el-option>
                    </el-select>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_00967') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.name" :placeholder="lc('wap_user_00076')"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('member_user_00299') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.sort" @input="inputIntNumber($event, 'ruleForm', 'sort')" :placeholder="lc('wap_user_00076')"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_00968') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.eject" label="1">{{ lc('admin_00205') }}</el-radio>
                    <el-radio v-model="ruleForm.eject" label="0">{{ lc('admin_00203') }}</el-radio>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_00969') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.model" label="hot">{{ lc('common_02091') }}</el-radio>
                    <el-radio v-model="ruleForm.model" label="new">{{ lc('common_02081') }}</el-radio>
                    <el-radio v-model="ruleForm.model" label="">{{ lc('common_02082') }}</el-radio>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_00970') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.bold" label="1">{{ lc('common.yes') }}</el-radio>
                    <el-radio v-model="ruleForm.bold" label="0">{{ lc('common.no') }}</el-radio>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_system_00087') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.display" label="1">{{ lc('common.yes') }}</el-radio>
                    <el-radio v-model="ruleForm.display" label="0">{{ lc('common.no') }}</el-radio>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">{{ lc('common.save') }}</el-button>
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