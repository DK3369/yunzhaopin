<template>
    <div style="overflow: hidden; position: relative; height: 100%;" v-if="islook">
        <div style="overflow: hidden; position: relative; height: 100%;">
            <div class="uploadTable" style="padding:0px 20px; width: calc(100% - 18px);">
                <table class="tableVue">
                    <thead>
                    <tr align="left">
                        <th width="120">名称</th>
                        <th width=" ">状态</th>
                    </tr>
                    </thead>
                    <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">职位名称</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="curr_job.name" placeholder="请输入职位名称"></el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">职位类别</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-cascader style="width: 480px;" v-model="sel_jobtype" :options="job_types" :props="{checkStrictly: true}" filterable clearable></el-cascader>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">薪水待遇</div>
                        </td>
                        <td>
                            <div style="display: flex;">
                                <div class="TableInpt">
                                    <el-input v-model="curr_job.minsalary" :disabled="mychecked" placeholder="最低薪资"></el-input>
                                </div>
                                <div class="TableInptline">-</div>
                                <div class="TableInpt">
                                    <el-input v-model="curr_job.maxsalary" :disabled="mychecked" placeholder="最高薪资"></el-input>
                                </div>
                                <el-checkbox style="margin-left:10px;" v-model="mychecked" label="面议" size="medium" border></el-checkbox>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">招聘人数</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="curr_job.zp_num" placeholder="请输入招聘人数"></el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">招聘要求</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-select v-model="curr_job.exp" placeholder="请选择经验要求">
                                    <el-option v-for="item in cacheData.comdata.job_exp" :key="'exp_'+item" :label="item == 0 ? '经验'+cacheData.comclass_name[item] : cacheData.comclass_name[item]" :value="item"></el-option>
                                </el-select>
                                <el-select style="margin-left:10px;" v-model="curr_job.edu" placeholder="请选择学历要求">
                                    <el-option v-for="item in cacheData.comdata.job_edu" :key="'edu_'+item" :label="item == 0 ? '经验'+cacheData.comclass_name[item] : cacheData.comclass_name[item]" :value="item"></el-option>
                                </el-select>
                                <el-checkbox style="margin-left:10px;" v-model="curr_job.is_graduate" label="可接受应届生" size="medium" border></el-checkbox>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">描述职位</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <div id="jobeditor—wrapper" style="border: 1px solid #ccc; width: 100%;">
                                    <div id="jobtoolbar-container"><!-- 工具栏 --></div>
                                    <div id="jobeditor-container" style="height: 260px;"><!-- 编辑器 --></div>
                                </div>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">工作地点</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-select v-model="curr_job.link_id" placeholder="请选择" style="width: 480px;">
                                    <el-option key="-1" :label="jobCompany.linkmsg + '【企业默认】'" value="-1"></el-option>
                                    <el-option v-for="(item, index) in jobAddressList" :key="index" :label="item.linkmsg" :value="item.id"></el-option>
                                </el-select>
                                <el-button style="margin-left: 10px;" size="small" type="primary" @click="addAddr" round>新增</el-button>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">从事行业</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="curr_job.hy" placeholder="请选择">
                                    <el-option v-for="(item, index) in cacheData.industry_index" :key="index" :label="cacheData.industry_name[item]" :value="item"></el-option>
                                </el-select>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">性别要求</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="curr_job.sex" placeholder="请选择">
                                    <el-option v-for="(item, index) in cacheData.com_sex" :key="index" :label="index == 3 ? '性别' + item : item" :value="index"></el-option>
                                </el-select>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">婚姻要求</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="curr_job.marriage" placeholder="请选择">
                                    <el-option v-for="item in cacheData.comdata.job_marriage" :key="item" :label="item == 0 ? '婚姻' + cacheData.comclass_name[item] : cacheData.comclass_name[item]" :value="item"></el-option>
                                </el-select>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">到岗位时间</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="curr_job.report" placeholder="请选择">
                                    <el-option v-for="item in cacheData.comdata.job_report" :key="item" :label="item == 0 ? '到岗时间' + cacheData.comclass_name[item] : cacheData.comclass_name[item]" :value="item"></el-option>
                                </el-select>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">年龄要求</div>
                        </td>
                        <td>
                            <div style="display: flex;">
                                <div class="TableInpt">
                                    <el-input type="number" v-model="curr_job.zp_minage" placeholder="请输入最小年龄">
                                        <template slot="append">岁</template>
                                    </el-input>
                                </div>
                                <div style="padding:10px;" class="">-</div>
                                <div class="TableInpt">
                                    <el-input type="number" v-model="curr_job.zp_maxage" placeholder="请输入最大年龄">
                                        <template slot="append">岁</template>
                                    </el-input>
                                </div>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">福利待遇</div>
                        </td>
                        <td>
                            <el-checkbox-group v-model="checkedwelfare">
                                <el-checkbox v-for="(item, index) in curr_job.all_welfare" :label="item" :key="index">{{item}}</el-checkbox>
                            </el-checkbox-group>
                            <el-input style="margin-left: 0px;" class="input-new-tag" v-if="inputVisible" v-model="inputValue" ref="saveTagInput" size="small" @keyup.enter.native="welfareInputConfirm" @blur="welfareInputConfirm"></el-input>
                            <el-button v-else style="margin-left: 0px;" class="button-new-tag" size="small" @click="showInput">+ 新增</el-button>
                        </td>
                    </tr>
                    <tr v-if="cacheData.comdata.job_lang && cacheData.comdata.job_lang.length > 0">
                        <td>
                            <div class="TableTite">语言要求</div>
                        </td>
                        <td>
                            <el-checkbox-group v-model="checkedlang">
                                <el-checkbox v-for="item in cacheData.comdata.job_lang" :label="item" :key="item">{{cacheData.comclass_name[item]}}</el-checkbox>
                            </el-checkbox-group>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">联系方式设置</div>
                        </td>
                        <td>
                            <div class="job_set_list">短信通知：
                                <el-radio v-model="curr_job.is_message" label="1">开启</el-radio>
                                <el-radio v-model="curr_job.is_message" label="2">关闭</el-radio>
                            </div>
                            <div class="job_set_list">邮件通知：
                                <el-radio v-model="curr_job.is_email" label="1">开启</el-radio>
                                <el-radio v-model="curr_job.is_email" label="2">关闭</el-radio>
                            </div>
                            <div class="job_set_list">联系方式：
                                <el-radio v-model="curr_job.is_link" label="1">公开</el-radio>
                                <el-radio v-model="curr_job.is_link" label="3">隐藏</el-radio>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">投递要求设置</div>
                        </td>
                        <td>
                            <div class="job_set_list">经验要求：
                                <el-select v-model="curr_job.exp_req" placeholder="请选择">
                                    <el-option label="不限" :value="0"></el-option>
                                    <el-option v-for="item in cacheData.userdata.user_word" :key="item" :label="cacheData.userclass_name[item]" :value="item"></el-option>
                                </el-select>
                            </div>
                            <div class="job_set_list">学历要求：
                                <el-select v-model="curr_job.edu_req" placeholder="请选择">
                                    <el-option label="不限" :value="0"></el-option>
                                    <el-option v-for="item in cacheData.userdata.user_edu" :key="item" :label="cacheData.userclass_name[item]" :value="item"></el-option>
                                </el-select>
                            </div>
                            <div class="job_set_list">性别要求：
                                <el-select v-model="curr_job.sex_req" placeholder="请选择">
                                    <el-option v-for="(item,index) in cacheData.com_sexreq" :key="index" :label="item" :value="index"></el-option>
                                </el-select>
                            </div>
                            <div class="job_set_list" style="display: flex; align-items: center;">
                                <div>年龄要求：</div>
                                <div style="display: flex; align-items: center; position: relative; width: calc(100% - 120px);">
                                    <div class="TableInpt">
                                        <el-input v-model="curr_job.minage_req" placeholder="请输入内容">
                                            <template slot="append">岁</template>
                                        </el-input>
                                    </div>
                                    <div class="TableInptline">-</div>
                                    <div class="TableInpt">
                                        <el-input v-model="curr_job.maxage_req" placeholder="请输入内容">
                                            <template slot="append">岁</template>
                                        </el-input>
                                    </div>
                                </div>
                            </div>
                        </td>
                    </tr>
                    <tr v-if="curr_job.id">
                        <td>
                            <div class="TableTite">职位状态</div>
                        </td>
                        <td>
                            <el-radio v-model="curr_job.status" label="0">招聘中</el-radio>
                            <el-radio v-model="curr_job.status" label="1">已下架</el-radio>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">浏览量</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="curr_job.jobhits" placeholder="请输入浏览量"></el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">曝光量</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="curr_job.jobexpoure" placeholder="请输入曝光量"></el-input>
                            </div>
                        </td>
                    </tr>
                    <tr v-if="curr_job.id">
                        <td>
                            <div class="TableTite">审核状态</div>
                        </td>
                        <td>
                            <div class="job_set_list">
                                <font v-if="curr_job.state == 1" color="blue">已审核</font>
                                <font v-else-if="curr_job.state == 3" color="red">未通过</font>
                                <font v-else color="red">未审核</font>
                            </div>
                        </td>
                    </tr>
                    </tbody>
                </table>
            </div>
            <div class="setBasicButn" style="border: none; height: 80px;">
                <el-button type="primary" size="medium" :loading="save_load" @click="jobsave">提交</el-button>
            </div>
        </div>
        <!--新增工作地址弹窗-->
        <el-drawer title="新增工作地址" :visible.sync="addressdrawer" append-to-body :wrapper-closable="false" size="60%">
            <div class="yunyinDialog" style="padding-right: 20px;">
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>联系人</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-input v-model="link_man" placeholder="请输入联系人"></el-input>
                    </div>
                </div>
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>手机号码</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-input v-model="link_moblie" placeholder="请输入手机号码"></el-input>
                    </div>
                </div>
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>固定电话</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-input v-model="link_phone" placeholder="请输入固定电话"></el-input>
                    </div>
                </div>
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>联系邮箱</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-input v-model="email" placeholder="请输入联系邮箱"></el-input>
                    </div>
                </div>
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>所在区域</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-cascader style="width: 100%;" v-model="sel_city" :options="city_types" :props="{checkStrictly: true}"  filterable collapse-tags clearable></el-cascader>
                    </div>
                </div>
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>详细地址</span>
                    </div>
                    <div class="yunyinDiaInpt yunyinDiautoco">
						<el-autocomplete style="width: 100%;"
							popper-class="my-autocomplete"
							:debounce="1000"
							v-model="address"
							:fetch-suggestions="addressKeyup"
							placeholder="请输入内容"
							@select="poiSearchClick">
							<i class="el-icon-location-outline el-input__icon" slot="suffix" @click="localsearch('全国')"></i>
							<template slot-scope="{ item }">
								<div class="autocompLtite">
									<div class="name">{{ item.name }}</div>
									<span class="addr">{{ item.address }}</span>
								</div>
							</template>
						</el-autocomplete>
					</div>
                </div>
                <div class="yunyinDiaList" v-if="mapurl">
                    <div class="yunyinDiaTite">
                        <span></span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <div class="TableInpt TableInptCoor" style="position:relative; width: 100%;">
                            <div id="conrtainer" style="width:100%;height:300px; position:relative; z-index:1"></div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="setBasicButn" style="border: none; height: 80px;">
                <el-button @click="addressdrawer = false">取 消</el-button>
                <el-button type="primary" @click="addressSubmit" :loading="save_load">确 定</el-button>
            </div>
        </el-drawer>
    </div>
</template>
<script>
    let jobeditor = null, jobtoolbar = null,editorInterval = null;
    var local;
    const {createEditor, createToolbar} = window.wangEditor;
    module.exports = {
        props: {
            jid: {
                type: String,
                default: function () {
                    return ''
                }
            },
            jtypes: {
                type: Array,
                default: function () {
                    return []
                }
            },
            ctypes: {
                type: Array,
                default: function () {
                    return []
                }
            },
            comid: {
                type: String,
                default: function () {
                    return ''
                }
            },
        },
        data: function () {
            return {
                inputVisible: false,
                inputValue: '',
                job_types: [],
                city_types: [],
                jobid: '',
                cuid: '',
                curr_job: {
                    is_link: "1",
                },
                sel_jobtype: [],
                jobCompany: null,
                jobAddressList: [],
                mychecked: false,
                cache_userdata: null,
                cache_userclassname: null,
                cache_com_sexreq: null,
                cacheData: null,
                checkedwelfare: [],
                checkedlang: [],
                showJob: false,
                islook: false,
                addressdrawer: false,
                link_man: '',
                link_moblie: '',
                link_phone: '',
                email: '',
                sel_city: [],
                address: '',
                x: '116.404',
                y: '39.915',
                mapurl: '',
				mapsecret: '',
                save_load: false,
				poiSearchArr:[],
				timer:null,
				address_v:'',
            }
        },
        mounted() {

        },
        beforeDestroy() {
            jobeditor = null; 
            jobtoolbar = null;
            editorInterval = null;
        },
        watch: {
            jid: {
                handler(val) {
                    this.jobid = val;
                },
                immediate: true,
                deep: true,
            },
            jtypes: {
                handler(val) {
                    this.job_types = val;
                },
                immediate: true,
                deep: true,
            },
            ctypes: {
                handler(val) {
                    this.city_types = val;
                },
                immediate: true,
                deep: true,
            },
            comid: {
                handler(val) {
                    this.cuid = val;
                },
                immediate: true,
                deep: true,
            },
        },
        methods: {
            writeJs: function(url, secret) {
                return new Promise((resolve, reject) => {
                    // 如果已加载直接返回
                    if (typeof window.AMap !== 'undefined') {
                        resolve(window.AMap);
                        return true;
                    }
                    // 地图异步加载回调处理
                    window.onAMapCallback = function () {
                        resolve(AMap);
                    };
					// 设置安全密钥
					window._AMapSecurityConfig = {
						securityJsCode: secret,
					}
                    // 插入script脚本
                    let scriptNode = document.createElement('script');
                    scriptNode.setAttribute('type', 'text/javascript');
                    scriptNode.setAttribute('src', url);
                    document.body.appendChild(scriptNode);
                });
            },
            addAddr: function(){
                this.sel_city = []
                if (this.jobCompany.provinceid > 0) {
                    this.sel_city.push(this.jobCompany.provinceid)
                }
                if (this.jobCompany.cityid > 0) {
                    this.sel_city.push(this.jobCompany.cityid)
                }
                if (this.jobCompany.three_cityid > 0) {
                    this.sel_city.push(this.jobCompany.three_cityid)
                }
                this.address = this.jobCompany.address
                this.x = this.jobCompany.x
                this.y = this.jobCompany.y
                var map_url = this.mapurl+"&callback=onAMapCallback";
                this.writeJs(map_url, this.mapsecret).then(value => {
                    this.openMap();
                });
                this.addressdrawer = true
            },

            getMap:function (){
				var map = new AMap.Map('conrtainer', {
				  zoom: 15,
				  center: [this.x, this.y]
				});
				var marker = new AMap.Marker({
					position: new AMap.LngLat(this.x, this.y)
				});
                map.add(marker);
                return map;
            },
            openMap:function (){
                var that = this;
                var data=get_map_config();
                if(data && data.indexOf('map_x')>-1){
                    var config=eval('('+data+')');
                    var rating,map_control_type,map_control_anchor;
                    if (!that.x && !that.y) {
                        that.x = config.map_x;
                        that.y = config.map_y;
                    }
                    var map = that.getMap();
					map.on("click",function(e){
						var lngLat = e.lnglat;
						that.x = lngLat.lng
						that.y = lngLat.lat
						map.clearMap();
						var marker = new AMap.Marker({
							position: new AMap.LngLat(lngLat.lng, lngLat.lat)
						});
						map.add(marker);
					});
                }
            },
            addressKeyup:function(queryString, cb){
            				
				this.localsearch(queryString, cb)
			},
			localsearch:function(city='',cb){
				var that = this;
				var address = city;
				
				if (this.mapurl && address.length > 3) {
					var map = this.getMap();
					map.clearMap();
					
					var params = {address: address};
					httpPost('m=common&c=cache&a=poi', params).then(function (result) {
						var res = result.data
						if (res.error == 0) {
							var e = res.data;
							if(e.status == '1' && e.tips.length > 0){
								that.poiSearchArr = e.tips;
								setTimeout(function(){
									map.clearMap();
									
								},200);
							}
							cb(that.poiSearchArr);
							that.address_v = that.address;
						}
					}).catch(function (e) {
						console.log(e)
					});
					
				}
		
			},
			poiSearchClick:function(item){
				
				var that = this;
				that.address = that.address_v;
				var location = item.location;
				var c = location.split(',');
				
				this.x = c[0];
				this.y = c[1];
				var map = that.getMap();
				// 设置marker
				var lngLat = new AMap.LngLat(c[0],c[1]);
				map.setZoomAndCenter(17,lngLat);
				var marker = new AMap.Marker({
					position: lngLat
				});
				map.add(marker);
				// 地图监听点击事件
				map.on("click",function(e){
					var lngLat = e.lnglat;
					this.x = lngLat.lng;
					this.y = lngLat.lat;
					map.clearMap();
					var marker = new AMap.Marker({
						position: new AMap.LngLat(lngLat.lng, lngLat.lat)
					});
					map.add(marker);
				});
				this.poiSearchArr = [];
			},
            addressSubmit: function () {
                var that = this
                that.link_man = that.link_man.replace(/[-_ ]/g, '');// 去掉空格
                if (that.link_man == '') {
                    message.error('请填写联系人')
                    return false
                } else {
                    var test = that.link_man.replace(/[0-9]/g, '');
                    if (!test) {
                        message.error('联系人不支持全数字')
                        return false
                    } else {
                        if (/\d/.test(that.link_man)) {
                            if (that.link_man.length > 8) {
                                // obj.value = obj.value.substring(0,8);
                                message.error('联系人填写字数不能超过8个')
                                return false
                            }
                        }
                    }
                }
                if (that.link_moblie == '' && that.link_phone == '') {
                    message.error('请填写联系电话')
                    return false
                } else if (that.link_moblie != '' && !isjsMobile(that.link_moblie)) {
                    message.error('手机号码格式错误')
                    return false
                }
                if (that.sel_city[0] && that.cacheData['city_type'][that.sel_city[0]]) {
                    if (!that.sel_city[1] > 0) {
                        message.error('请选择所在区域二级城市')
                        return false
                    }
                }
                if (that.address == '') {
                    message.error('请选择填写详细地址')
                    return false
                }
                var params = {
                    uid: that.jobCompany.uid,
                    link_man: that.link_man,
                    link_moblie: that.link_moblie,
                    link_phone: that.link_phone,
                    email: that.email,
                    provinceid: that.sel_city[0],
                    cityid: that.sel_city[1],
                    three_cityid: that.sel_city[2],
                    link_address: that.address,
                    x: that.x,
                    y: that.y,
                    is_link: 2
                }
                that.save_load = true;
                httpPost('m=user&c=company_job&a=saveAddress', params).then(function (result) {
                    var res = result.data
                    if (res.error == 0) {
                        message.success(res.msg, function () {
                            if (res.data.link_id) {
                                that.curr_job.link_id = res.data.link_id + ''
                            }
                            if (res.data.addressList) {
                                that.jobAddressList = res.data.addressList
                            }
                            that.addressdrawer = false
                        })
                    } else {
                        message.error(res.msg)
                    }
                }).catch(function (e) {
                    console.log(e)
                }).finally(function () {
                    setTimeout(function () {
                        that.save_load = false;
                    }, 2000);
                });
            },
            showInput() {
                this.inputVisible = true;
                this.$nextTick(_ => {
                    this.$refs.saveTagInput.$refs.input.focus();
                });
            },
            welfareInputConfirm() {
                let inputValue = this.inputValue;
                if (inputValue) {
                    this.curr_job.all_welfare.push(inputValue);
                    this.checkedwelfare.push(inputValue)
                }
                this.inputVisible = false;
                this.inputValue = '';
            },
            // 职位修改
            edit: function () {
                var that = this
                var params = {}
                if (that.jobid) {
                    params.id = that.jobid
                } else if (that.cuid) {
                    params.uid = that.cuid
                }
                httpPost('m=user&c=company_job&a=add', params).then(function (result) {
                    var res = result.data
                    if (res.error == 0) {
                        that.mapurl = res.data.mapurl;
						that.mapsecret = res.data.mapsecret;
                        that.curr_job = res.data.show;
                        that.jobCompany = res.data.company;
                        that.jobAddressList = res.data.addressList;
                        if (params.id) {
                            that.checkedwelfare = that.curr_job.job_welfare ? that.curr_job.job_welfare : []
                            that.checkedlang = that.curr_job.lang ? that.curr_job.lang : []
                            that.sel_jobtype = []
                            if (res.data.show.job1) {
                                that.sel_jobtype.push(res.data.show.job1)
                            }
                            if (res.data.show.job1_son) {
                                that.sel_jobtype.push(res.data.show.job1_son)
                            }
                            if (res.data.show.job_post > 0) {
                                that.sel_jobtype.push(res.data.show.job_post)
                            }
                            if (that.curr_job.minsalary == 0 && that.curr_job.maxsalary == 0 && that.curr_job.id) {
                                that.curr_job.minsalary = ''
                                that.curr_job.maxsalary = ''
                                that.mychecked = true
                            } else {
                                if (that.curr_job.maxsalary == 0){
                                    that.curr_job.maxsalary = '';
                                }
                                that.mychecked = false
                            }
                            if ((that.curr_job.minage_req == 0 || !that.curr_job.minage_req) && that.curr_job.id) {
                                that.curr_job.minage_req = ''
                            }
                            if ((that.curr_job.maxage_req == 0 || !that.curr_job.maxage_req) && that.curr_job.id) {
                                that.curr_job.maxage_req = ''
                            }
                            if ((that.curr_job.zp_minage == 0 || !that.curr_job.zp_minage) && that.curr_job.id) {
                                that.curr_job.zp_minage = ''
                            }
                            if ((that.curr_job.zp_maxage == 0 || !that.curr_job.zp_maxage) && that.curr_job.id) {
                                that.curr_job.zp_maxage = ''
                            }
                            if (that.curr_job.sex_req == 0 && that.curr_job.id) {
                                that.curr_job.sex_req = '3'
                            }
                            if (that.curr_job.exp == 0 && that.curr_job.id) {
                                that.curr_job.exp = ''
                            }
                            if (that.curr_job.edu == 0 && that.curr_job.id) {
                                that.curr_job.edu = ''
                            }
                            if (that.curr_job.hy == 0 && that.curr_job.id) {
                                that.curr_job.hy = ''
                            }
                            if (that.curr_job.sex == 0 && that.curr_job.id) {
                                that.curr_job.sex = ''
                            }
                            if (that.curr_job.marriage == 0 && that.curr_job.id) {
                                that.curr_job.marriage = ''
                            }
                            if (that.curr_job.report == 0 && that.curr_job.id) {
                                that.curr_job.report = ''
                            }
                            if (that.curr_job.is_link == 1 || that.curr_job.link_id == 0) {// 使用企业默认地址
                                that.curr_job.link_id = '-1';
                            }
                            that.curr_job.exp_req = parseInt(that.curr_job.exp_req)>0?that.curr_job.exp_req:0;
                            that.curr_job.edu_req = parseInt(that.curr_job.edu_req)>0?that.curr_job.edu_req:0;
                        }
                        that.cacheData = res.data.cache
                        if (params.uid) {
                            that.curr_job = {
                                name: '',
                                minsalary: '',
                                maxsalary: '',
                                zp_num: '',
                                exp: '',
                                edu: '',
                                is_graduate: false,
                                link_id: '-1',
                                description: '',
                                hy: '',
                                sex: '',
                                marriage: '',
                                report: '',
                                zp_minage: '',
                                zp_maxage: '',
                                all_welfare: res.data.default_welfare,
                                lang: [],
                                is_message: '',
                                is_email: '',
                                is_link: '1',
                                exp_req: 0,
                                edu_req: 0,
                                sex_req: '3',
                                minage_req: '',
                                maxage_req: '',
                                jobhits: '',
                                jobexpoure: ''
                            }
                            that.mychecked = false
                            that.sel_jobtype = ['', '', '']
                            that.checkedwelfare = that.jobCompany.welfare != '' ? that.jobCompany.welfare.split(',') : []
                            that.checkedlang = []
                            that.job_types = res.data.job_types
                            that.city_types = res.data.city_types
                        }
                        that.cache_userdata = res.data.cache_userdata
                        that.cache_userclassname = res.data.cache_userclassname
                        that.cache_com_sexreq = res.data.cache_com_sexreq
                        that.islook = true
                        that.initEditor(that.curr_job.description);
                    }
                }).catch(function (e) {
                    console.log(e)
                })
            },
            initEditor: function (content=null) {
                var that = this
                clearInterval(editorInterval);
                editorInterval = setInterval(function () {
                    if (jobeditor !== null){
                        clearInterval(editorInterval);
                        if(content!==null){
                            jobeditor.setHtml(content);
                        }
                    }else{
                        let editorConfig = {
                            MENU_CONF: {
                                uploadImage: {
                                    server: baseUrl + 'm=index&c=uploadfile',
                                    fieldName: 'file'
                                }
                            }
                        };
                        jobeditor = null;
                        if (!jobeditor) {
                            jobeditor = createEditor({
                                selector: '#jobeditor-container',
                                html: '',
                                config: editorConfig,
                                mode: 'simple'
                            });
                            
                        }
                        
                        jobtoolbar = null;
                        if (!jobtoolbar) {
                            jobtoolbar = createToolbar({
                                editor: jobeditor,
                                selector: '#jobtoolbar-container',
                                config: {
                                    // 去除工具栏key
                                    excludeKeys: ['blockquote', 'header1', 'header2', 'header3', '|', 'through', 'color', 'bgColor', 'insertLink', 'todo', '|', 'insertVideo', 'insertTable', 'codeBlock', '|', 'undo', 'redo', '|',]
                                },
                                mode: 'simple'
                            });
                        }
                    }
                },300);
                
            },
            jobsave: function () {
                var that = this
                if (that.curr_job.name == '') {
                    message.error('职位名称不能为空！')
                    return false;
                }
                that.curr_job.job1 = that.sel_jobtype[0] ? that.sel_jobtype[0] : '0'
				that.curr_job.job1_son = that.sel_jobtype[1] ? that.sel_jobtype[1] : '0'
				that.curr_job.job_post = that.sel_jobtype[2] ? that.sel_jobtype[2] : '0'
				
				if (that.jionly == '1') {
					if (that.curr_job.job1 == '0') {
						message.error('请选择职位类别！')
						return false;
					}
				} else {
					if (that.curr_job.job1_son == '0') {
						message.error('请选择职位类别！')
						return false;
					}
				}
                if (that.mychecked == 0) {// 非薪资面议
                    if (that.curr_job.minsalary == '' || that.curr_job.minsalary == '0') {
                        message.error('请填写工资！')
                        return false;
                    }
                    if (that.curr_job.maxsalary) {
                        if (parseInt(that.curr_job.maxsalary) < parseInt(that.curr_job.minsalary)) {
                            message.error('最高工资必须大于最低工资！')
                            return false;
                        } else if (parseInt(that.curr_job.maxsalary) == parseInt(that.curr_job.minsalary)) {
                            message.error('最高工资必须大于最低工资，如是固定工资只需填写最低工资！')
                            return false;
                        }
                    }
                }else{
                    that.curr_job.minsalary = 0;
                    that.curr_job.maxsalary = 0;
                }
                if (that.curr_job.zp_num == '') {
                    message.error('请填写招聘人数！')
                    return false;
                }
                // 去除html标签后判断内容是否为空
                var regex = /(<([^>]+)>)/ig
                var content = jobeditor.getHtml().replace(regex, "")
				
                if (content == "") {
                    message.error('职位描述不能为空！')
                    return false;
                } else {
                    that.curr_job.content = jobeditor.getHtml()
                }
                if (that.curr_job.link_id == '') {
                    message.error('请选择工作地点！')
                    return false;
                }
                if (that.curr_job.zp_minage != '' && that.curr_job.zp_minage < 16) {
                    message.error('法律规定：禁止招收未满16周岁未成年人！')
                    return false;
                }
                if (that.curr_job.zp_maxage != '' && (that.curr_job.zp_maxage < 16 || that.curr_job.zp_maxage > 99)) {
                    message.error('请设置合理的年龄区间！')
                    return false;
                }
                if (that.curr_job.minage_req != '' && that.curr_job.minage_req < 16) {
                    message.error('法律规定：禁止招收未满16周岁未成年人！')
                    return false;
                }
                if (that.curr_job.maxage_req != '' && (that.curr_job.maxage_req < 16 || that.curr_job.maxage_req > 99)) {
                    message.error('请设置合理的年龄区间！')
                    return false;
                }
                if (parseInt(that.curr_job.jobexpoure) < parseInt(that.curr_job.jobhits)) {
                    message.error('职位曝光量不能低于职位浏览次数！')
                    return false;
                }
                if (!that.curr_job.id) {
                    that.curr_job.uid = that.cuid
                }
                that.curr_job.save = 1
                that.curr_job.checked_welfare = that.checkedwelfare
                that.curr_job.checked_lang = that.checkedlang
                that.save_load = true;
                httpPost('m=user&c=company_job&a=add', that.curr_job).then(function (result) {
                    that.save_load = false;
                    var res = result.data
                    if (res.error == 0) {
                        message.success(res.msg, function () {
                            that.$parent.$parent.getList()
                            if (that.curr_job.id) {// 编辑
                                that.$parent.$parent.drawerEditJob = false
                            } else {// 新增
                                that.$parent.$parent.drawerAddJob = false
                            }
                        })
                    } else {
                        message.error(res.msg)
                    }
                }).catch(function (e) {
                    console.log(e)
                })
            },
        },
    };

    function get_map_config(){
        var config="";
        var weburl = localStorage.getItem("sy_weburl");
        $.ajax( {
            async : false,
            type : "post",
            url : weburl + '/index.php?m=ajax&c=mapconfig',
            data : {id:""},
            success : function(set) {
                config=set;
            }
        });
        return config;
    }
</script>
<style>
    .avatar-uploader .el-upload {
        border: 1px dashed #d9d9d9;
        border-radius: 6px;
        cursor: pointer;
        position: relative;
        overflow: hidden;
    }

    .avatar-uploader .el-upload:hover {
        border-color: #409EFF;
    }

    .avatar-uploader-icon {
        font-size: 14px;
        color: #8c939d;
        width: 48px;
        height: 48px;
        line-height: 48px;
        text-align: center;
    }

    .avatar {
        width: 48px;
        height: 48px;
        display: block;
    }

    .job_set_list {
        font-size: 14px;
        color: #606266;
        overflow: hidden;
        position: relative;
        padding: 4px 0;
    }
	.my-autocomplete {
	  li {
		line-height: normal;
		padding: 7px;
	
		.name {
		  text-overflow: ellipsis;
		  overflow: hidden;
		}
		.addr {
		  font-size: 12px;
		  color: #b4b4b4;
		}
	
		.highlighted .addr {
		  color: #ddd;
		}
	  }
	}
</style>