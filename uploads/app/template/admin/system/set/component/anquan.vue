<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="180">{yun:}t key='member_com_00021'{/yun}</th>
                        <th width="260">{yun:}t key='member_user_00181'{/yun}</th>
                        <th>{yun:}t key='member_com_00207'{/yun}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00873'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="list.sy_safekey" placeholder="{yun:}t key='wap_user_00076'{/yun}">
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00874'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00875'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-switch v-model="list.sy_iscsrf_status">
                                </el-switch>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00875'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00876'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-switch v-model="list.sy_istemplate_status">
                                </el-switch>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00876'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00877'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="list.sy_fkeyword" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00878'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00879'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="list.sy_fkeyword_all" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00880'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00881'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="2" autosize v-model="list.sy_useragent" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00882'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00883'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="2" autosize v-model="list.sy_bannedip" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00884'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00885'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input  type="textarea" :rows="2" autosize v-model="list.sy_bannedip_alert" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00886'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00887'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-switch v-model="list.sy_list_login" active-value="1" inactive-value="2">
                                </el-switch>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00888'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00889'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="2" autosize v-model="list.sy_list_agent" placeholder=""></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00890'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">{yun:}t key='common.submit'{/yun}</el-button>
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
                    message.success("{yun:}t key='wap_user_00264'{/yun}");
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