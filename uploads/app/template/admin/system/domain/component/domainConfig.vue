<template>
    <div class="tableDome" style="top: 40px;">
        <div class="tableDome_tip">
            <el-alert title="管理员可以通过当前页面进行分站配置：开启分站、自动跳转等；" type="info" :closable="false"></el-alert>
        </div>
        <div class="moduleTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="200">名称</th>
                        <th width="500">状态</th>
                        <th>说明</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">分站状态</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio-group v-model="domainConfig.sy_web_site">
                                    <el-radio :label="1">开启</el-radio>
                                    <el-radio :label="2">关闭</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>开启多城市并且绑定域名不支持2级目录，本地测试如 http://localhost/phpyun 请解析测试域名</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">自动跳转</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio-group v-model="domainConfig.sy_gotocity">
                                    <el-radio :label="1">开启</el-radio>
                                    <el-radio :label="2">关闭</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>PC端根据IP地址跳转，手机端根据坐标定位提示跳转</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">设定默认城市</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="请输入内容" v-model="domainConfig.sy_indexcity"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>开启分站后默认城市 如：全国、总站</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">默认域名</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="请输入内容" v-model="domainConfig.sy_indexdomain"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>默认城市对应的域名 如全站对应域名 http://www.hr135.com 而不是 beijing.hr135.com</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">一级域名</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="请输入内容" v-model="domainConfig.sy_onedomain"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>如果默认域名为二级域名，则请填写一级域名</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>

        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="setDomainConfig" :disabled="saveLoading">提交</el-button>
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