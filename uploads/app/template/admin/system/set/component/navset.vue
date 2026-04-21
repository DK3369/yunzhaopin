<template>
    <div class="drawerModlue">
        <div class="drawerModInfo" style="max-height: calc(100% - 80px); overflow-y: auto;">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>导航类别：</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="ruleForm.nid" placeholder="请选择">
                        <el-option v-for="item in type" :key="item.id" :label="item.typename" :value="item.id">
                        </el-option>
                    </el-select>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>导航名称：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.name" placeholder="请输入内容"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>排　　序：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.sort" @input="inputIntNumber($event, 'ruleForm', 'sort')" placeholder="请输入内容"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>弹出窗口：</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.eject" label="1">新窗口</el-radio>
                    <el-radio v-model="ruleForm.eject" label="0">原窗口</el-radio>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>状　　态：</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.model" label="hot">热</el-radio>
                    <el-radio v-model="ruleForm.model" label="new">新</el-radio>
                    <el-radio v-model="ruleForm.model" label="">无</el-radio>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>加　　粗：</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.bold" label="1">是</el-radio>
                    <el-radio v-model="ruleForm.bold" label="0">否</el-radio>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>显　　示：</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.display" label="1">是</el-radio>
                    <el-radio v-model="ruleForm.display" label="0">否</el-radio>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">保存</el-button>
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