<template>
    <div class="tableDome" style="top: 40px;">
        <div class="tableDome_tip">
            <el-alert :title="lc('admin_01051')" type="info" :closable="false"></el-alert>
        </div>
        <div class="moduleTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="200">{{ lc('member_com_00021') }}</th>
                        <th width="500">{{ lc('member_user_00181') }}</th>
                        <th>{{ lc('member_com_00207') }}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_01045') }}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio-group v-model="domainConfig.sy_web_site">
                                    <el-radio :label="1">{{ lc('member_com_00287') }}</el-radio>
                                    <el-radio :label="2">{{ lc('common.close') }}</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_system_00175') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_01046') }}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio-group v-model="domainConfig.sy_gotocity">
                                    <el-radio :label="1">{{ lc('member_com_00287') }}</el-radio>
                                    <el-radio :label="2">{{ lc('common.close') }}</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01047') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_01048') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input :placeholder="lc('wap_user_00076')" v-model="domainConfig.sy_indexcity"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01049') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00178') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input :placeholder="lc('wap_user_00076')" v-model="domainConfig.sy_indexdomain"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01050') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00177') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input :placeholder="lc('wap_user_00076')" v-model="domainConfig.sy_onedomain"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_system_00176') }}</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>

        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="setDomainConfig" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
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