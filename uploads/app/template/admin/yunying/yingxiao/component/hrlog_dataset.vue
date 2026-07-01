<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="180">{{ lc('member_com_00021') }}</th>
                        <th width="560">{{ lc('member_user_00181') }}</th>
                        <th>{{ lc('member_com_00207') }}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_01091') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleForm.sy_datashow_title" placeholder="">
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01091') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_01092') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" v-model="ruleForm.sy_datashowhy_base" placeholder="">
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01092') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_01093') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" v-model="ruleForm.sy_datashowreg_base" placeholder="">
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01093') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_01094') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" v-model="ruleForm.sy_datashowlogin_base" placeholder="">
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01094') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_01095') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" v-model="ruleForm.sy_datashowjob_base" placeholder="">
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01095') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_01096') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="previewUrl" readonly placeholder="">
                                </el-input>
                                <el-button type="primary" style="margin-left: 10px;" @click="dialogPreview = true">{{ lc('wap_00071') }}</el-button>
                                <el-button type="primary" plain @click="copyPreviewUrl">{{ lc('wap_com_00233') }}</el-button>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01096') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_01097') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-radio-group v-model="ruleForm.sy_datashow_city_lev">
                                    <el-radio label="1">{{ lc('admin_yunying_00145') }}</el-radio>
                                    <el-radio label="2">{{ lc('admin_yunying_00147') }}</el-radio>
                                    <el-radio label="3">{{ lc('admin_yunying_00146') }}</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01097') }}</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="save">{{ lc('common.submit') }}</el-button>
        </div>

        <div class="modluDrawer">
            <el-dialog :title="lc('admin_01099')" :visible.sync="dialogPreview" :with-header="true" :modal-append-to-body="false"
                       :show-close="true" width="300px">
                <div class="center">
                    <img :src="previewCode" width="200" height="200">
                </div>
                <div class="code_p">
                    <div class="center">{{ lc('admin_01098') }}</div>
                    <br/>
                </div>
            </el-dialog>
        </div>
    </div>
</template>
    
<script>
module.exports = {
    data: function () {
        return {
            ruleForm: {},

            dialogPreview: false,
            previewUrl: '',
            previewCode: '',

            saveLoading: false,
        }
    },

    mounted() {

    },
    created() {
        this.init();
    },
    methods: {
        init() {
            this.getData();
        },

        getData() {
            let that = this;
            httpPost('m=yunying&c=yingxiao_hrlog&a=datashowset').then(function (response) {
                let res = response.data,
                    data = res.data;

                that.ruleForm = data.config;
                that.previewUrl = data.previewUrl;
                that.previewCode = data.previewCode;
            })
        },

        copyPreviewUrl() {
            // 创建输入框元素
            let oInput = document.createElement('input');
            // 将想要复制的值
            oInput.value = this.previewUrl;
            // 页面底部追加输入框
            document.body.appendChild(oInput);
            // 选中输入框
            oInput.select();
            // 执行浏览器复制命令
            document.execCommand('Copy');
            // 弹出复制成功信息
            message.success(lc('admin_user_company_00368'));
            // 复制后移除输入框
            oInput.remove();
        },

        save() {
            let that = this,
                ruleForm = that.ruleForm;

            if (that.saveLoading) {
                return false;
            }

            that.saveLoading = true;

            httpPost('m=yunying&c=yingxiao_hrlog&a=datashowsetSave', ruleForm).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    message.error(res.msg, function() {
                        that.saveLoading = false;
                    });
                } else {
                    message.success(res.msg, function() {
                        that.saveLoading = false;
                    });
                }
            })
        },
    },
};
</script>
<style scoped>
.center {
    display: flex;
    justify-content: center;
    align-items: center;
}
</style>