<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="180">{yun:}t key='member_com_00021'{/yun}</th>
                        <th>{yun:}t key='member_user_00181'{/yun}</th>
                        <th width="520">{yun:}t key='member_com_00207'{/yun}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00417'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-checkbox-group v-model='code_web_list'>
                                    <el-checkbox label="{yun:}t key='wap_js_00064'{/yun}"></el-checkbox>
                                    <el-checkbox label="{yun:}t key='wap_js_00062'{/yun}"></el-checkbox>
                                    <el-checkbox label="{yun:}t key='wap_js_00130'{/yun}"></el-checkbox>
                                    <el-checkbox label="{yun:}t key='wap_js_00066'{/yun}"></el-checkbox>
                                    <el-checkbox label="{yun:}t key='admin_system_00423'{/yun}"></el-checkbox>
                                    <el-checkbox label="{yun:}t key='wap_00105'{/yun}"></el-checkbox>
                                    <el-checkbox label="{yun:}t key='wap_user_00203'{/yun}"></el-checkbox>
                                    <el-checkbox label="{yun:}t key='wap_js_00123'{/yun}"></el-checkbox>
                                </el-checkbox-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_system_00417'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00422'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio-group v-model="list.code_kind">
                                    <el-radio label="1">{yun:}t key='admin_system_00421'{/yun}</el-radio>
                                    <el-radio label="4">{yun:}t key='admin_system_00416'{/yun}</el-radio>
                                    <el-radio label="5">{yun:}t key='admin_00997'{/yun}</el-radio>
                                    <el-radio label="3">{yun:}t key='admin_system_00418'{/yun}</el-radio>
                                    <el-radio label="6">{yun:}t key='admin_system_00420'{/yun}</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_system_00422'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <template v-if="list.code_kind == 1">
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_00998'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-radio-group v-model="list.code_type">
                                        <el-radio label="1">{yun:}t key='common_01997'{/yun}</el-radio>
                                        <el-radio label="2">{yun:}t key='admin_system_00425'{/yun}</el-radio>
                                        <el-radio label="3">{yun:}t key='admin_00999'{/yun}</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_01000'{/yun}</div>
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
                                <div class="TableTite">{yun:}t key='admin_system_00419'{/yun}</div>
                            </td>
                            <td>
                                <!-- <div class="TableInpt">
                                    <span>宽：</span>
                                    <el-input class="input-text" type="text" v-model='list.code_width' size="10"
                                        maxlength="255" />px&nbsp;&nbsp;
                                </div> -->


                                <div class="TableInpt TableInptCoor" style="justify-content: initial;">
                                    <div class="TableInCooLis">
                                        <el-input v-model='list.code_width' class="input-text" type="text" size="10" maxlength="255" onkeyup="this.value=this.value.replace(/[^0-9.]/g,'')">
                                            <template slot="prepend">宽</template>
                                            <span slot="suffix" class="slotspan">px</span>
                                        </el-input>
                                    </div>
                                    <div class="TableInCooLis">
                                        <el-input v-model="list.code_height" type="text" class="input-text" size="10" maxlength="255" onkeyup="this.value=this.value.replace(/[^0-9.]/g,'')">
                                            <template slot="prepend">{yun:}t key='common_02105'{/yun}</template>
                                            <span slot="suffix" class="slotspan">px</span>
                                        </el-input>
                                    </div>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_system_00419'{/yun}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_01002'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.code_strlength" maxlength="1"
                                        onKeyUp="this.value=this.value.replace(/[^0-9]/g,'')"
                                        placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_01003'{/yun}</span>
                                </div>
                            </td>
                        </tr>
                    </template>
                    <template v-if="list.code_kind == 4">
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_01004'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_dxappid" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_01005'{/yun}<el-link target="_blank" type="info" href="https://www.dingxiang-inc.com">https://www.dingxiang-inc.com</el-link></span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_01006'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_dxappsecret" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_01005'{/yun}<el-link target="_blank" type="info" href="https://www.dingxiang-inc.com">https://www.dingxiang-inc.com</el-link></span>
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
                                    <el-input v-model="list.sy_vaptcha_vid" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_01005'{/yun}<el-link target="_blank" type="info" href="https://www.vaptcha.com">https://www.vaptcha.com</el-link></span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">KEY</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_vaptcha_key" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
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
                                <div class="TableTite">{yun:}t key='admin_01007'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_geetestid" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_01005'{/yun}<el-link target="_blank" type="info" href="http://www.geetest.com">http://www.geetest.com</el-link></span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_01008'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_geetestkey" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_01005'{/yun}<el-link target="_blank" type="info" href="http://www.geetest.com">http://www.geetest.com</el-link></span>
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
                                    <el-input v-model="list.sy_tecentappid" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_01005'{/yun}<el-link target="_blank" type="info" href="https://cloud.tencent.com">https://cloud.tencent.com</el-link><span
                                            style="color:red;padding-left: 5px;">(使用此功能需php版本>=7.0)</span></span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">AppSecretKey：</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_tecentappsecret" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_01005'{/yun} <el-link target="_blank" type="info" href="https://cloud.tencent.com">https://cloud.tencent.com</el-link></span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">TecentSecretId：</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_tecentsecretid" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_01005'{/yun} <el-link target="_blank" type="info" href="https://cloud.tencent.com">https://cloud.tencent.com</el-link></span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">TecentSecretKey：</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_tecentsecretkey" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_01005'{/yun} <el-link target="_blank" type="info" href="https://cloud.tencent.com">https://cloud.tencent.com</el-link></span>
                                </div>
                            </td>
                        </tr>
                    </template>
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