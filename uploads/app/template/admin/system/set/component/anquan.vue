<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="180">名称</th>
                        <th width="260">状态</th>
                        <th>说明</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">系统安全码</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="list.sy_safekey" placeholder="请输入内容">
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>系统加密串，请自定义修改，如：986jhgyutw.*x</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">CSRF防御</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-switch v-model="list.sy_iscsrf_status">
                                </el-switch>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>CSRF防御</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">后台修改模板</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-switch v-model="list.sy_istemplate_status">
                                </el-switch>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>后台修改模板</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">过滤关键词</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="list.sy_fkeyword" placeholder="请输入内容"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>如：台湾,台独</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">替换过滤关键词</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="list.sy_fkeyword_all" placeholder="请输入内容"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>将敏感关键词替换</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">USER_AGENT过滤</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="2" autosize v-model="list.sy_useragent" placeholder="请输入内容"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>一行一条，屏蔽无意义的HTTP_USER_AGENT，可以防止恶意爬虫伪造蜘蛛造成的CC攻击</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">禁止IP访问</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="2" autosize v-model="list.sy_bannedip" placeholder="请输入内容"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>例：127.0.0.1|192.168.1.1</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">禁止IP访问提示</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input  type="textarea" :rows="2" autosize v-model="list.sy_bannedip_alert" placeholder="请输入内容"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>禁止访问提示</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">数据列表登录访问</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-switch v-model="list.sy_list_login" active-value="1" inactive-value="2">
                                </el-switch>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>开启登录访问，职位、企业、校园招聘、培训等列表需要登录用户才可以访问（仅针对PC/WAP）</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">数据列表访问白名单</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="2" autosize v-model="list.sy_list_agent" placeholder=""></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>一行一条，针对部分HTTP_USER_AGENT，可以跳过必须登录才可以访问数据列表的开关</span>
                            </div>
                        </td>
                    </tr>
                    
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">提交</el-button>
        </div>
    </div>
</template>
    
<script>
module.exports = {
    props: {
        list: Object
    },
    data: function () {
        return {
            value1: true,
            value: '',
            textarea: '',
            radio: '1',
            input: '',
            uri:"m=system&c=",
            saveLoading: false
        }
    },

    mounted() {

    },
    methods: {
        save:function (){
            let _this = this;
            let url = this.uri+"set_config&a=save";
            let ruleForm = {
                sy_safekey: _this.list.sy_safekey,
                sy_istemplate: _this.list.sy_istemplate_status ? 1 : 2,
                sy_iscsrf: _this.list.sy_iscsrf_status ? 1 : 2,
                sy_bannedip: _this.list.sy_bannedip,
                sy_fkeyword_all: _this.list.sy_fkeyword_all,
                sy_useragent	: _this.list.sy_useragent,
                sy_bannedip_alert: _this.list.sy_bannedip_alert,
                sy_fkeyword: _this.list.sy_fkeyword,
                sy_list_login: _this.list.sy_list_login,
                sy_list_agent: _this.list.sy_list_agent
            };
            _this.saveLoading = true;
            httpPost(url, ruleForm).then(function (response) {
                var res = response.data;
                if (res.error == 0) {
                    message.success('操作成功');
                    _this.$emit('get-list', true)
                } else {
                    message.error(res.msg);
                }
            }).finally(function () {
                setTimeout(function () {
                    _this.saveLoading = false;
                }, 2000);
            });
        }
    },
};
</script>
<style scoped></style>