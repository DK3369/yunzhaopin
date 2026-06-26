<template>
    <div class="tableDome" style="top: 40px;">
        <div class="tableDome_tip">
            <el-alert title="{yun:}t key='admin_system_00144'{/yun}" type="info" :closable="false"></el-alert>
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
                            <div class="TableTite">{yun:}t key='admin_system_00166'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="domainInfo.title"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_01040'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00161'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="domainInfo.mode" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                    <el-option v-for="(item,key) in modeOptionS" :key="key" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_system_00161'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-show="domainInfo.mode == 1">
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00168'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="domainInfo.domain"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_01041'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-show="domainInfo.mode == 2">
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00162'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="domainInfo.indexdir"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_01042'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00163'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="domainInfo.fz_type" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                    <el-option v-for="(item,key) in typeOptionS" :key="key" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_system_00163'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-show="domainInfo.fz_type == 1">
                        <td>
                            <div class="TableTite">{yun:}t key='wap_com_00015'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableSelect" style="display: flex;align-items: center;">
                                <el-select v-model="domainInfo.province" placeholder="{yun:}t key='wap_user_00100'{/yun}" @change="handelProOption">
                                    <el-option v-for="(item,key) in provinceOptionS" :key="key" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                                <el-select v-model="domainInfo.cityid" placeholder="{yun:}t key='wap_user_00100'{/yun}" style="margin: 0 6px;"
                                    @change="handelCityOption">
                                    <el-option v-for="(item,key) in cityOptionS" :key="key" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                                <el-select v-model="domainInfo.three_cityid" placeholder="{yun:}t key='wap_user_00100'{/yun}" @change="handelCountyOption">
                                    <el-option v-for="(item,key) in countyOptionS" :key="key" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_system_00160'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-show="domainInfo.fz_type == 2">
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00167'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="domainInfo.hy" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                    <el-option v-for="(item,key) in hyOptionS" :key="key" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_system_00164'{/yun}</span>
                            </div>
                        </td>
                    </tr>

                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00169'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="domainInfo.style" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                    <el-option v-for="(item,key) in styleOptionS" :key="key" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_system_00146'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00170'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="domainInfo.tpl"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_system_00145'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00153'{/yun}</div>
                        </td>
                        <td>
                            <div class="setBasicIput">
                                <el-switch v-model="isType" active-text="开启"></el-switch>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_system_00147'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00157'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="2" placeholder="{yun:}t key='wap_user_00076'{/yun}"
                                    v-model="domainInfo.webtitle"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_system_00157'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00152'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="2" placeholder="{yun:}t key='wap_user_00076'{/yun}"
                                    v-model="domainInfo.webkeyword"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_system_00152'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00156'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="2" placeholder="{yun:}t key='wap_user_00076'{/yun}"
                                    v-model="domainInfo.webmeta"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_system_00156'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00154'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableUpload">
                                <el-upload class="upload-demo" :accept="pic_accept" :action="upLogoUrl" :auto-upload="false"
                                    :show-file-list="false" :on-change="upLogoChange">
                                    <el-button slot="trigger" size="small" type="primary">{yun:}t key='admin_system_00158'{/yun}</el-button>
                                    <img class="el-upload-list__item-thumbnail" v-if="domainInfo.weblogo"
                                        :src="domainInfo.weblogo" alt="{yun:}t key='admin_system_00155'{/yun}" />
                                </el-upload>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_01043'{/yun}<br>{yun:}t key='admin_01044'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="saveDomain" :disabled="saveLoading">{yun:}t key='common.submit'{/yun}</el-button>
        </div>
    </div>
</template>
<script>

module.exports = {
    props: {
        domain_id: Number
    },
    watch: {
        domain_id: {
            handler(newValue, oldValue) {
                this.getCache();
                if (newValue != 0) {
                    this.getDomainInfo();
                } else {
                    this.resetDomainInfo();
                }
            },
            deep: true,
            immediate: true
        }
    },
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            domainInfo: {
                title: '',
                mode: 1,
                domain: '',
                indexdir: '',
                fz_type: 1,
                province: '',
                cityid: '',
                three_cityid: '',
                hy: '',
                style: '',
                tpl: '',
                type: 1,
                webtitle: '',
                webkeyword: '',
                webmeta: '',
                weblogo: ''
            },
            upLogoUrl: '',
            files: [],
            isType: true,
            modeOptionS: [{
                value: 1,
                label: "{yun:}t key='admin_system_00159'{/yun}"
            }, {
                value: 2,
                label: "{yun:}t key='admin_system_00171'{/yun}"
            }],
            typeOptionS: [{
                value: 1,
                label: "{yun:}t key='admin_system_00165'{/yun}"
            }, {
                value: 2,
                label: "{yun:}t key='wap_00507'{/yun}"
            }],
            styleOptionS: [],

            City: [],
            provinceOptionS: [],
            cityOptionS: [],
            countyOptionS: [],

            hyOptionS: [],

            picMaxSize: '5',
            picType: '',
            saveLoading: false
		}
    },
    methods: {
        async getCache() {
            this.provinceOptionS = [];
            this.hyOptionS = [];
            this.styleOptionS = [];
            let res = await httpPost('m=system&c=domain_list&a=getDomainCache');
            if (res.data.error == 0) {
                let data = res.data.data;
                this.City = data.cityArr;
                var provinceArr = data.provinceArr;
                provinceArr.forEach((item) => {
                    this.provinceOptionS.push({ value: item.id, label: item.name })
                });
                var industryArr = data.industryArr;
                industryArr.forEach((item) => {
                    this.hyOptionS.push({ value: item.id, label: item.name })
                });
                var styleList = data.styleList;
                styleList.forEach((item) => {
                    this.styleOptionS.push({ value: item.dir, label: item.dir });
                });

                this.picMaxSize = data.picMaxSize;
                this.picType = data.picType;
            }
		},
        getDomainInfo: function () {
            var self = this;
            httpPost('m=system&c=domain_list&a=domainInfo', { id: this.domain_id }).then(function (res) {
                if (res.data.error == 0) {
                    var cityId = res.data.data.cityid,
                        threeCityId = res.data.data.three_cityid;

                    self.domainInfo = res.data.data;
                    self.domainInfo.mode = parseInt(res.data.data.mode);
                    self.domainInfo.fz_type = parseInt(res.data.data.fz_type);
                    self.isType = self.domainInfo.type == '1' ? true : false;
                    self.domainInfo.hy = parseInt(res.data.data.hy);

                    if (parseInt(self.domainInfo.province) > 0) {
                        self.handelProOption(self.domainInfo.province);
                        if (parseInt(cityId) > 0) {
                            setTimeout(function () {
                                self.handelCityOption(cityId);
                            }, 500)
                        }
                        if (parseInt(threeCityId) > 0) {
                            setTimeout(function () {
                                self.handelCountyOption(threeCityId);
                            }, 1000)
                        }
                    }
                }
            });
        },
        resetDomainInfo: function () {
            var self = this;
            self.domainInfo = {
                title: '',
                mode: 1,
                domain: '',
                indexdir: '',
                fz_type: 1,
                province: '',
                cityid: '',
                three_cityid: '',
                hy: '',
                style: '',
                tpl: '',
                type: 1,
                webtitle: '',
                webkeyword: '',
                webmeta: '',
                weblogo: ''
            };
        },
        handelProOption: function (val) {
            this.cityOptionS = [];
            this.countyOptionS = [];
            this.domainInfo.cityid = '';
            this.domainInfo.three_cityid = '';
            this.City.forEach((item, index) => {
                if (item.pid == val) {

                    this.cityOptionS.push({ value: item.id, label: item.name });
                }
            });
        },
        handelCityOption: function (val) {
            this.countyOptionS = [];
            this.domainInfo.cityid = val;
            this.domainInfo.three_cityid = '';
            this.City.forEach((item, index) => {
                if (item.pid == val) {

                    this.countyOptionS.push({ value: item.id, label: item.name });
                }
            });
        },
        handelCountyOption: function (val) {
            this.domainInfo.three_cityid = val;
        },
        upLogoChange(file) {
            var that = this;
            //  判断图片类型
            let picTypeArr = this.picType.split(',');
            let isImage = false;
            picTypeArr.forEach(item => {
                if (file.raw.type === 'image/' + item) {
                    isImage = true;
                }
            });
            if (!isImage) {
                message.error('上传头像图片只能是 （' + this.picType + "） {yun:}t key='common_02005'{/yun}!");
                return false;
            }

            //  判断图片大小
            let isLtNumM = file.size / 1024 / 1024 < this.picMaxSize;
            if (!isLtNumM) {
                message.error('上传头像图片大小不能超过 ' + this.picMaxSize + 'MB!');
                return false;
            }

            // 预览文件处理
            this.domainInfo.weblogo = URL.createObjectURL(file.raw);
            console.log('LOGO预览地址：' + that.domainInfo.weblogo);

            // 复刻文件信息
            this.files = file.raw;
            console.log(this.files);
        },
        saveDomain: function () {
            var self = this;
            let formData = new FormData();
            if (self.domainInfo.title == '') {
                message.error('请填写域名备注');
                return false;
            } else {
                formData.append('title', self.domainInfo.title);
            }

            formData.append('mode', self.domainInfo.mode);
            if (self.domainInfo.mode == 1) {
                if (self.domainInfo.domain == '') {
                    message.error("{yun:}t key='admin_system_00148'{/yun}");
                    return false;
                } else {
                    formData.append('domain', self.domainInfo.domain);
                }
            }
            if (self.domainInfo.mode == 2) {
                if (self.domainInfo.indexdir == '') {
                    message.error("{yun:}t key='admin_system_00149'{/yun}");
                    return false;
                } else {
                    formData.append('indexdir', self.domainInfo.indexdir);
                }
            }

            formData.append('fz_type', self.domainInfo.fz_type);
            if (self.domainInfo.fz_type == 1) {
                if (self.domainInfo.province == '') {

                    message.error("{yun:}t key='admin_system_00150'{/yun}");
                    return false;
                } else {

                    formData.append('province', self.domainInfo.province);
                    formData.append('cityid', self.domainInfo.cityid);
                    formData.append('three_cityid', self.domainInfo.three_cityid);
                }
            }
            if (self.domainInfo.fz_type == 2) {
                if (self.domainInfo.hy == '') {

                    message.error("{yun:}t key='admin_system_00151'{/yun}");
                    return false;
                } else {

                    formData.append('hy', self.domainInfo.hy);
                }
            }

            formData.append('style', self.domainInfo.style);
            formData.append('tpl', self.domainInfo.tpl);
            formData.append('type', self.isType ? '1' : '2');
            formData.append('webtitle', self.domainInfo.webtitle);
            formData.append('webkeyword', self.domainInfo.webkeyword);
            formData.append('webmeta', self.domainInfo.webmeta);

            if (this.files.length !== 0) {

                formData.append('file', this.files);
            }

            if (self.domain_id > 0) {
                formData.append('id', self.domain_id);
            }
            self.saveLoading = true;
            httpPost('m=system&c=domain_list&a=saveDomain', formData).then(function (res) {
                if (res.data.error == 0) {
                    message.success(res.data.msg, function () {

                        self.$emit("child-event");
                    });
                } else {

                    message.error(res.data.msg);
                }
            }).finally(function () {
                setTimeout(function () {
                    self.saveLoading = false;
                }, 2000);
            });
        }
    },
};
</script>