<template>
    <div class="tableDome" style="top: 40px;">
        <div class="tableDome_tip">
            <el-alert :title="lc('admin_system_00144')" type="info" :closable="false"></el-alert>
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
                            <div class="TableTite">{{ lc('admin_system_00166') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input :placeholder="lc('wap_user_00076')" v-model="domainInfo.title"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01040') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00161') }}</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="domainInfo.mode" :placeholder="lc('wap_user_00100')">
                                    <el-option v-for="(item,key) in modeOptionS" :key="key" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_system_00161') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-show="domainInfo.mode == 1">
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00168') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input :placeholder="lc('wap_user_00076')" v-model="domainInfo.domain"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01041') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-show="domainInfo.mode == 2">
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00162') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input :placeholder="lc('wap_user_00076')" v-model="domainInfo.indexdir"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01042') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00163') }}</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="domainInfo.fz_type" :placeholder="lc('wap_user_00100')">
                                    <el-option v-for="(item,key) in typeOptionS" :key="key" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_system_00163') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-show="domainInfo.fz_type == 1">
                        <td>
                            <div class="TableTite">{{ lc('wap_com_00015') }}</div>
                        </td>
                        <td>
                            <div class="TableSelect" style="display: flex;align-items: center;">
                                <el-select v-model="domainInfo.province" :placeholder="lc('wap_user_00100')" @change="handelProOption">
                                    <el-option v-for="(item,key) in provinceOptionS" :key="key" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                                <el-select v-model="domainInfo.cityid" :placeholder="lc('wap_user_00100')" style="margin: 0 6px;"
                                    @change="handelCityOption">
                                    <el-option v-for="(item,key) in cityOptionS" :key="key" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                                <el-select v-model="domainInfo.three_cityid" :placeholder="lc('wap_user_00100')" @change="handelCountyOption">
                                    <el-option v-for="(item,key) in countyOptionS" :key="key" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_system_00160') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-show="domainInfo.fz_type == 2">
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00167') }}</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="domainInfo.hy" :placeholder="lc('wap_user_00100')">
                                    <el-option v-for="(item,key) in hyOptionS" :key="key" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_system_00164') }}</span>
                            </div>
                        </td>
                    </tr>

                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00169') }}</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="domainInfo.style" :placeholder="lc('wap_user_00100')">
                                    <el-option v-for="(item,key) in styleOptionS" :key="key" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_system_00146') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00170') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input :placeholder="lc('wap_user_00076')" v-model="domainInfo.tpl"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_system_00145') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00153') }}</div>
                        </td>
                        <td>
                            <div class="setBasicIput">
                                <el-switch v-model="isType" :active-text="lc('member_com_00287')"></el-switch>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_system_00147') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00157') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="2" :placeholder="lc('wap_user_00076')"
                                    v-model="domainInfo.webtitle"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_system_00157') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00152') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="2" :placeholder="lc('wap_user_00076')"
                                    v-model="domainInfo.webkeyword"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_system_00152') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00156') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="2" :placeholder="lc('wap_user_00076')"
                                    v-model="domainInfo.webmeta"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_system_00156') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00154') }}</div>
                        </td>
                        <td>
                            <div class="TableUpload">
                                <el-upload class="upload-demo" :accept="pic_accept" :action="upLogoUrl" :auto-upload="false"
                                    :show-file-list="false" :on-change="upLogoChange">
                                    <el-button slot="trigger" size="small" type="primary">{{ lc('admin_system_00158') }}</el-button>
                                    <img class="el-upload-list__item-thumbnail" v-if="domainInfo.weblogo"
                                        :src="domainInfo.weblogo" alt=lc('admin_system_00155') />
                                </el-upload>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01043') }}<br>{{ lc('admin_01044') }}</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="saveDomain" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
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
                label: lc('admin_system_00159')
            }, {
                value: 2,
                label: lc('admin_system_00171')
            }],
            typeOptionS: [{
                value: 1,
                label: lc('admin_system_00165')
            }, {
                value: 2,
                label: lc('wap_00507')
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
                message.error(lc('admin_vue_00126') + this.picType + lc('common_02005') + '!');
                return false;
            }

            //  判断图片大小
            let isLtNumM = file.size / 1024 / 1024 < this.picMaxSize;
            if (!isLtNumM) {
                message.error(lc('admin_vue_00127') + this.picMaxSize + 'MB!');
                return false;
            }

            // 预览文件处理
            this.domainInfo.weblogo = URL.createObjectURL(file.raw);
            console.log(lc('admin_vue_00046') + that.domainInfo.weblogo);

            // 复刻文件信息
            this.files = file.raw;
            console.log(this.files);
        },
        saveDomain: function () {
            var self = this;
            let formData = new FormData();
            if (self.domainInfo.title == '') {
                message.error(lc('admin_vue_00047'));
                return false;
            } else {
                formData.append('title', self.domainInfo.title);
            }

            formData.append('mode', self.domainInfo.mode);
            if (self.domainInfo.mode == 1) {
                if (self.domainInfo.domain == '') {
                    message.error(lc('admin_system_00148'));
                    return false;
                } else {
                    formData.append('domain', self.domainInfo.domain);
                }
            }
            if (self.domainInfo.mode == 2) {
                if (self.domainInfo.indexdir == '') {
                    message.error(lc('admin_system_00149'));
                    return false;
                } else {
                    formData.append('indexdir', self.domainInfo.indexdir);
                }
            }

            formData.append('fz_type', self.domainInfo.fz_type);
            if (self.domainInfo.fz_type == 1) {
                if (self.domainInfo.province == '') {

                    message.error(lc('admin_system_00150'));
                    return false;
                } else {

                    formData.append('province', self.domainInfo.province);
                    formData.append('cityid', self.domainInfo.cityid);
                    formData.append('three_cityid', self.domainInfo.three_cityid);
                }
            }
            if (self.domainInfo.fz_type == 2) {
                if (self.domainInfo.hy == '') {

                    message.error(lc('admin_system_00151'));
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