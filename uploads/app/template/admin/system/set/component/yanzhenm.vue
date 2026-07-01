<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="180">{{ lc('member_com_00021') }}</th>
                        <th>{{ lc('member_user_00181') }}</th>
                        <th width="520">{{ lc('member_com_00207') }}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00417') }}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-checkbox-group v-model='code_web_list'>
                                    <el-checkbox :label="lc('wap_js_00064')"></el-checkbox>
                                    <el-checkbox :label="lc('wap_js_00062')"></el-checkbox>
                                    <el-checkbox :label="lc('wap_js_00130')"></el-checkbox>
                                    <el-checkbox :label="lc('wap_js_00066')"></el-checkbox>
                                    <el-checkbox :label="lc('admin_system_00423')"></el-checkbox>
                                    <el-checkbox :label="lc('wap_00105')"></el-checkbox>
                                    <el-checkbox :label="lc('wap_user_00203')"></el-checkbox>
                                    <el-checkbox :label="lc('wap_js_00123')"></el-checkbox>
                                </el-checkbox-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_system_00417') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00422') }}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio-group v-model="list.code_kind">
                                    <el-radio label="1">{{ lc('admin_system_00421') }}</el-radio>
                                    <el-radio label="4">{{ lc('admin_system_00416') }}</el-radio>
                                    <el-radio label="5">{{ lc('admin_00997') }}</el-radio>
                                    <el-radio label="3">{{ lc('admin_system_00418') }}</el-radio>
                                    <el-radio label="6">{{ lc('admin_system_00420') }}</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_system_00422') }}</span>
                            </div>
                        </td>
                    </tr>
                    <template v-if="list.code_kind == 1">
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_00998') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-radio-group v-model="list.code_type">
                                        <el-radio label="1">{{ lc('common_01997') }}</el-radio>
                                        <el-radio label="2">{{ lc('admin_system_00425') }}</el-radio>
                                        <el-radio label="3">{{ lc('admin_00999') }}</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_01000') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-radio-group v-model="list.code_filetype">
                                        <el-radio label="jpg">JPG</el-radio>
                                        <el-radio label="png">PNG</el-radio>
                                        <el-radio label="gif">GIF</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                            <td></td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_system_00419') }}</div>
                            </td>
                            <td>
                                <!-- <div class="TableInpt">
                                    <span>{{ lc('admin_vue_00135') }}：</span>
                                    <el-input class="input-text" type="text" v-model='list.code_width' size="10"
                                        maxlength="255" />px&nbsp;&nbsp;
                                </div> -->


                                <div class="TableInpt TableInptCoor" style="justify-content: initial;">
                                    <div class="TableInCooLis">
                                        <el-input v-model='list.code_width' class="input-text" type="text" size="10" maxlength="255" onkeyup="this.value=this.value.replace(/[^0-9.]/g,'')">
                                            <template slot="prepend">{{ lc('admin_vue_00135') }}</template>
                                            <span slot="suffix" class="slotspan">px</span>
                                        </el-input>
                                    </div>
                                    <div class="TableInCooLis">
                                        <el-input v-model="list.code_height" type="text" class="input-text" size="10" maxlength="255" onkeyup="this.value=this.value.replace(/[^0-9.]/g,'')">
                                            <template slot="prepend">{{ lc('common_02105') }}</template>
                                            <span slot="suffix" class="slotspan">px</span>
                                        </el-input>
                                    </div>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_system_00419') }}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_01002') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.code_strlength" maxlength="1"
                                        onKeyUp="this.value=this.value.replace(/[^0-9]/g,'')"
                                        :placeholder="lc('wap_user_00076')"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_01003') }}</span>
                                </div>
                            </td>
                        </tr>
                    </template>
                    <template v-if="list.code_kind == 4">
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_01004') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_dxappid" :placeholder="lc('wap_user_00076')"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_01005') }}<el-link target="_blank" type="info" href="https://www.dingxiang-inc.com">https://www.dingxiang-inc.com</el-link></span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_01006') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_dxappsecret" :placeholder="lc('wap_user_00076')"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_01005') }}<el-link target="_blank" type="info" href="https://www.dingxiang-inc.com">https://www.dingxiang-inc.com</el-link></span>
                                </div>
                            </td>
                        </tr>
                    </template>
                    <template v-if="list.code_kind == 5">
                        <tr>
                            <td>
                                <div class="TableTite">VID</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_vaptcha_vid" :placeholder="lc('wap_user_00076')"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_01005') }}<el-link target="_blank" type="info" href="https://www.vaptcha.com">https://www.vaptcha.com</el-link></span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">KEY</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_vaptcha_key" :placeholder="lc('wap_user_00076')"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span></span>
                                </div>
                            </td>
                        </tr>
                    </template>
                    <template v-if="list.code_kind == 3">
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_01007') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_geetestid" :placeholder="lc('wap_user_00076')"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_01005') }}<el-link target="_blank" type="info" href="http://www.geetest.com">http://www.geetest.com</el-link></span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_01008') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_geetestkey" :placeholder="lc('wap_user_00076')"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_01005') }}<el-link target="_blank" type="info" href="http://www.geetest.com">http://www.geetest.com</el-link></span>
                                </div>
                            </td>
                        </tr>
                    </template>
                    <template v-if="list.code_kind == 6">
                        <tr>
                            <td>
                                <div class="TableTite">CaptchaAppId：</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_tecentappid" :placeholder="lc('wap_user_00076')"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_01005') }}<el-link target="_blank" type="info" href="https://cloud.tencent.com">https://cloud.tencent.com</el-link><span
                                            style="color:red;padding-left: 5px;">{{ lc('admin_vue_00134') }}</span></span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">AppSecretKey：</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_tecentappsecret" :placeholder="lc('wap_user_00076')"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_01005') }} <el-link target="_blank" type="info" href="https://cloud.tencent.com">https://cloud.tencent.com</el-link></span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">TecentSecretId：</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_tecentsecretid" :placeholder="lc('wap_user_00076')"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_01005') }} <el-link target="_blank" type="info" href="https://cloud.tencent.com">https://cloud.tencent.com</el-link></span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">TecentSecretKey：</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_tecentsecretkey" :placeholder="lc('wap_user_00076')"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_01005') }} <el-link target="_blank" type="info" href="https://cloud.tencent.com">https://cloud.tencent.com</el-link></span>
                                </div>
                            </td>
                        </tr>
                    </template>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
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
            uri: "m=system&c=",
            code_web_list: [],
            saveLoading: false,
        }
    },
    methods: {
        save: function () {
            let _this = this;
            let url = this.uri + "set_config&a=save";
            let codewebarr = this.code_web_list
            codewebarr = codewebarr.toString();
            let ruleForm = {
                code_web: codewebarr,
                code_kind: this.list.code_kind,
                code_type: this.list.code_type,
                code_filetype: this.list.code_filetype,
                code_width: this.list.code_width,
                code_height: this.list.code_height,
                sy_geetestid: this.list.sy_geetestid,
                sy_geetestkey: this.list.sy_geetestkey,
                sy_dxappid: this.list.sy_dxappid,
                sy_dxappsecret: this.list.sy_dxappsecret,
                sy_geetestmid: this.list.sy_geetestmid,
                sy_geetestmkey: this.list.sy_geetestmkey,
                sy_vaptcha_vid: this.list.sy_vaptcha_vid,
                sy_vaptcha_key: this.list.sy_vaptcha_key,
                sy_tecentappid: this.list.sy_tecentappid,
                sy_tecentappsecret: this.list.sy_tecentappsecret,
                sy_tecentsecretid: this.list.sy_tecentsecretid,
                sy_tecentsecretkey: this.list.sy_tecentsecretkey,
                code_strlength: this.list.code_strlength
            };
            _this.saveLoading = true;
            httpPost(url, ruleForm).then(function (response) {
                var res = response.data;
                if (res.error == 0) {
                    message.success(lc('wap_user_00264'));
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
    watch: {
        list: {
            handler(newValue, oldValue) {
                if (newValue.code_web) {
                    this.code_web_list = newValue.code_web.split(',')
                }
            },
            deep: true,
            immediate: true
        }
    }
};
</script>
<style scoped></style>