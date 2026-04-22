<template>
    <div class="tableDome" style="top: 40px;">
        <div class="tableDome_tip">
            <el-alert title="同一域名不得绑定多个城市，多个域名可以绑定同一城市，但是前台选择城市域名跳转会以最后添加域名为基准，外部访问则不作限制！" type="info" :closable="false"></el-alert>
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
                            <div class="TableTite">域名备注</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="请输入内容" v-model="domainInfo.title"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>如：北京站</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">分站形式</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="domainInfo.mode" placeholder="请选择">
                                    <el-option v-for="(item,key) in modeOptionS" :key="key" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>分站形式</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-show="domainInfo.mode == 1">
                        <td>
                            <div class="TableTite">绑定域名</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="请输入内容" v-model="domainInfo.domain"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>如：beijing.hr135.com(不带http://)</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-show="domainInfo.mode == 2">
                        <td>
                            <div class="TableTite">分站目录</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="请输入内容" v-model="domainInfo.indexdir"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>如：beijing(自定义城市、行业拼音目录)</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">分站类型</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="domainInfo.fz_type" placeholder="请选择">
                                    <el-option v-for="(item,key) in typeOptionS" :key="key" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>分站类型</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-show="domainInfo.fz_type == 1">
                        <td>
                            <div class="TableTite">所在地区</div>
                        </td>
                        <td>
                            <div class="TableSelect" style="display: flex;align-items: center;">
                                <el-select v-model="domainInfo.province" placeholder="请选择" @change="handelProOption">
                                    <el-option v-for="(item,key) in provinceOptionS" :key="key" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                                <el-select v-model="domainInfo.cityid" placeholder="请选择" style="margin: 0 6px;"
                                    @change="handelCityOption">
                                    <el-option v-for="(item,key) in cityOptionS" :key="key" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                                <el-select v-model="domainInfo.three_cityid" placeholder="请选择" @change="handelCountyOption">
                                    <el-option v-for="(item,key) in countyOptionS" :key="key" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>分站地区</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-show="domainInfo.fz_type == 2">
                        <td>
                            <div class="TableTite">所属行业</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="domainInfo.hy" placeholder="请选择">
                                    <el-option v-for="(item,key) in hyOptionS" :key="key" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>分站行业</span>
                            </div>
                        </td>
                    </tr>

                    <tr>
                        <td>
                            <div class="TableTite">风格分站</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="domainInfo.style" placeholder="请选择">
                                    <el-option v-for="(item,key) in styleOptionS" :key="key" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>选择您需要绑定的模板风格目录，绑定成功后通过该域名访问人才网会自动进入到该风格下</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">首页模板</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="请输入内容" v-model="domainInfo.tpl"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>如：index.htm(后缀是.htm 不填则是默认siteindex.htm模板)</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">是否启用该域名</div>
                        </td>
                        <td>
                            <div class="setBasicIput">
                                <el-switch v-model="isType" active-text="开启"></el-switch>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>提示：停用该域名则不在前台选择城市处显示并且外部访问不对该域名进行解析</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">分站首页标题</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="2" placeholder="请输入内容"
                                    v-model="domainInfo.webtitle"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>分站首页标题</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">分站首页关键词</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="2" placeholder="请输入内容"
                                    v-model="domainInfo.webkeyword"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>分站首页关键词</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">分站首页描述</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="2" placeholder="请输入内容"
                                    v-model="domainInfo.webmeta"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>分站首页描述</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">分站LOGO</div>
                        </td>
                        <td>
                            <div class="TableUpload">
                                <el-upload class="upload-demo" :accept="pic_accept" :action="upLogoUrl" :auto-upload="false"
                                    :show-file-list="false" :on-change="upLogoChange">
                                    <el-button slot="trigger" size="small" type="primary">选取Logo</el-button>
                                    <img class="el-upload-list__item-thumbnail" v-if="domainInfo.weblogo"
                                        :src="domainInfo.weblogo" alt="分站Logo" />
                                </el-upload>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>建议尺寸：最大适应范围：300px*100px 之间,可根据LOGO形状定义高度70px效果最好；<br>默认为空则调用网站Logo</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="saveDomain" :disabled="saveLoading">提交</el-button>
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
                label: '二级域名'
            }, {
                value: 2,
                label: '首页目录'
            }],
            typeOptionS: [{
                value: 1,
                label: '地区分站'
            }, {
                value: 2,
                label: '行业分站'
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
                message.error('上传头像图片只能是 （' + this.picType + '） 格式!');
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
                    message.error('请填写二级域名分站所绑定的域名');
                    return false;
                } else {
                    formData.append('domain', self.domainInfo.domain);
                }
            }
            if (self.domainInfo.mode == 2) {
                if (self.domainInfo.indexdir == '') {
                    message.error('请填写首页目录分站所在目录');
                    return false;
                } else {
                    formData.append('indexdir', self.domainInfo.indexdir);
                }
            }

            formData.append('fz_type', self.domainInfo.fz_type);
            if (self.domainInfo.fz_type == 1) {
                if (self.domainInfo.province == '') {

                    message.error('请选择地区分站所在地区');
                    return false;
                } else {

                    formData.append('province', self.domainInfo.province);
                    formData.append('cityid', self.domainInfo.cityid);
                    formData.append('three_cityid', self.domainInfo.three_cityid);
                }
            }
            if (self.domainInfo.fz_type == 2) {
                if (self.domainInfo.hy == '') {

                    message.error('请选择行业分站所说行业');
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