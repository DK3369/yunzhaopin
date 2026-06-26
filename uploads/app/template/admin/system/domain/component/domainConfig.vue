<template>
    <div class="tableDome" style="top: 40px;">
        <div class="tableDome_tip">
            <el-alert title="{yun:}t key='admin_01051'{/yun}" type="info" :closable="false"></el-alert>
        </div>
        <div class="moduleTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="200">{yun:}t key='member_com_00021'{/yun}</th>
                        <th width="500">{yun:}t key='member_user_00181'{/yun}</th>
                        <th>{yun:}t key='member_com_00207'{/yun}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_01045'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio-group v-model="domainConfig.sy_web_site">
                                    <el-radio :label="1">{yun:}t key='member_com_00287'{/yun}</el-radio>
                                    <el-radio :label="2">{yun:}t key='common.close'{/yun}</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_system_00175'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_01046'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio-group v-model="domainConfig.sy_gotocity">
                                    <el-radio :label="1">{yun:}t key='member_com_00287'{/yun}</el-radio>
                                    <el-radio :label="2">{yun:}t key='common.close'{/yun}</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_01047'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_01048'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="domainConfig.sy_indexcity"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_01049'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00178'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="domainConfig.sy_indexdomain"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_01050'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00177'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="domainConfig.sy_onedomain"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_system_00176'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>

        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="setDomainConfig" :disabled="saveLoading">{yun:}t key='common.submit'{/yun}</el-button>
        </div>
    </div>
</template>
<!-- script -->
<script>
module.exports = {
    data: function () {
        return {
            domainConfig: {
                sy_web_site: '',
                sy_gotocity: '',
                sy_indexcity: '',
                sy_indexdomain: '',
                sy_onedomain: ''
            },
			saveLoading: false
        }
    },
    created() {
        this.getDomainConfig();
    },
    methods: {
        async getDomainConfig() {
            let res = await httpPost('m=system&c=domain_list&a=config');
            if (res.data.error == 0) {

                this.domainConfig = res.data.data;
                this.domainConfig.sy_web_site = this.domainConfig.sy_web_site == '1' ? 1 : 2;
                this.domainConfig.sy_gotocity = this.domainConfig.sy_gotocity == '1' ? 1 : 2;
            }
        },
        setDomainConfig: function () {
            let that = this;
            let params = {
                domainConfig: 1,
                sy_web_site: that.domainConfig.sy_web_site,
                sy_gotocity: that.domainConfig.sy_gotocity,
                sy_indexcity: that.domainConfig.sy_indexcity,
                sy_indexdomain: that.domainConfig.sy_indexdomain,
                sy_onedomain: that.domainConfig.sy_onedomain,
            };
            that.saveLoading = true;
            httpPost('m=system&c=domain_list&a=configSave', params).then(function (res) {
                if (res.data.error == 0) {
                    message.success(res.data.msg, function () {
                        that.$emit("child-event");
                    });
                } else {
                    message.error(res.data.msg);
                }
            }).finally(function () {
                setTimeout(function () {
                    that.saveLoading = false;
                }, 2000);
            });
        },
    },
};
</script>