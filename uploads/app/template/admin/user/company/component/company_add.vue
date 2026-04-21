<template>
    <div style="margin:10px;" v-if="islook">
        <div class="moduleTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="200">名称</th>
                    <th width="400">状态</th>
                    <th>说明</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td colspan="3">会员信息</td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">用户名</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input placeholder="请输入用户名" v-model="username" @blur="checkuname">
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">设置密码</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center; ">
                            <el-input type="password" @mousedown.native="pwdMousedown" @input="pwdchange" @focus="readonlyCtl(false)" @blur="readonlyCtl(true)" :readonly="pwdreadonly" placeholder="请输入密码" v-model="password">
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td colspan="3">企业信息</td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">企业全称</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-autocomplete style="width: 400px;" class="inline-input" v-model="name"
                                             :fetch-suggestions="querySearch"
                                             placeholder="请输入企业全称"
                                             :trigger-on-focus="false"
                            ></el-autocomplete>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span style="color:red;">下拉框为已存在企业</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">简称</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input placeholder="请输入企业简称" v-model="shortname">
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">从事行业</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-select v-model="hy" placeholder="请选择">
                                <el-option v-for="(item, index) in cache.industry_index" :key="index"
                                           :label="cache.industry_name[item]"
                                           :value="item">
                                </el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">企业性质</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-select v-model="pr" placeholder="请选择">
                                <el-option v-for="(item,index) in cache.comdata.job_pr" :key="index"
                                           :label="cache.comclass_name[item]"
                                           :value="item">
                                </el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">企业规模</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-select v-model="mun" placeholder="请选择">
                                <el-option v-for="(item,index) in cache.comdata.job_mun" :key="index"
                                           :label="cache.comclass_name[item]"
                                           :value="item">
                                </el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">所在地</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-cascader v-model="sel_city" :options="cache.cities" :props="{checkStrictly: true }" @change="citychange" filterable
                                         collapse-tags clearable></el-cascader>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">公司地址</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-autocomplete style="width: 100%;"
                                             popper-class="my-autocomplete"
                                             :debounce="1000"
                                             v-model="address"
                                             :fetch-suggestions="addressKeyup"
                                             placeholder="请输入公司地址"
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
                        <div class="yunyinDiaList" v-if="mapurl">
                            <div class="yunyinDiaInpt" style="width: 100%;">
                                <div class="TableInpt TableInptCoor" style="position:relative; width: 100%;">
                                    <div id="comadd_conrtainer" style="width:100%;height:300px; position:relative; z-index:1"></div>
                                </div>
                            </div>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">联系人</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input placeholder="请输入联系人" v-model="linkman">
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">联系手机</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input placeholder="请输入联系手机" v-model="moblie">
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">固定电话</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input style="width:100px;" placeholder="如：0527" v-model="areacode" maxlength="7">
                            </el-input>
                            <el-input style="width:230px;margin-left: 12px;" placeholder="固定电话" v-model="telphone" maxlength="8">
                            </el-input>
                            <el-input style="width:100px;margin-left: 12px;" placeholder="分机号" v-model="exten" maxlength="4">
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">邮箱</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input placeholder="请输入邮箱" v-model="email">
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">企业简介</div>
                    </td>
                    <td colspan="2">
                        <div style="border: 1px solid #ccc;">
                            <div id="toolbar-container-comdesc"><!-- 工具栏 --></div>
                            <div id="editor-container-comdesc" style="height: 300px;"><!-- 编辑器 --></div>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">消息通知</div>
                    </td>
                    <td>
                        <el-checkbox v-model="sendmsg">发送短信</el-checkbox>
                        <el-checkbox v-model="sendemail">发送邮件</el-checkbox>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td colspan="3">{{integral_pricename}}套餐</td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">赠送{{integral_pricename}}数量</div>
                    </td>
                    <td>
                        <el-input placeholder="请输入赠送积分数量" v-model="integral">
                        </el-input>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">会员套餐</div>
                    </td>
                    <td>
                        <el-select v-model="rating_name" placeholder="请选择">
                            <el-option v-for="(item, index) in ratingarr" :key="index" :label="item"
                                       :value="index">
                            </el-option>
                        </el-select>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save">提交</el-button>
        </div>
    </div>
</template>
<script>
    let editor_desc = null,toolbar_desc = null,editorInterval = null;
    const {createEditor, createToolbar} = window.wangEditor;
    module.exports = {
        props: {
            pricename: {
                type: String,
                default: function(){
                    return ''
                }
            },
            rates: {
                type: Object,
                default: function(){
                    return {}
                }
            },
        },
        data: function () {
            return {
                username: '',
                password: '',
                name: '',
                shortname: '',
                hy: '',
                pr: '',
                mun:'',
                address:'',
                linkman:'',
                moblie:'',
                areacode: '',
                telphone:'',
                exten:'',
                email:'',
                sendmsg:false,
                sendemail: false,
                integral: '',
                rating_name: '',
                sel_city: [],
                ratingarr: [],
                cache: null,
                integral_pricename: '',
                cionly: 0,
                pwdreadonly: true,
                islook: false,

                x:'',
                y:'',
                address_v:'',
                mapurl: '',
                mapsecret: '',
            }
        },
        watch: {
            pricename: {
                handler(val) {
                    this.integral_pricename = val;
                },
                immediate: true,
                deep: true,
            },
            rates: {
                handler(val) {
                    this.ratingarr = val;
                },
                immediate: true,
                deep: true,
            },
        },
        beforeDestroy() {
            editor_desc = null; 
            toolbar_desc = null;
            editorInterval = null;
        },
        methods: {
            //用来阻止第二次或更多次点击密码输入框时下拉用户密码清单的框一闪而过的问题
            pwdMousedown(){
                var that = this
                this.pwdreadonly = true
                setTimeout(function(){ that.pwdreadonly = false, 100})
            },
            // 防止密码框内容清楚后展示用户密码清单
            pwdchange: function(val){
                var that = this
                if (val == '') {
                    this.pwdreadonly = true
                    setTimeout(function(){ that.pwdreadonly = false, 100})
                }
            },
            // 修改密码框readonly属性，防止密码框展示浏览器记录的密码信息
            readonlyCtl: function(res){
                var that = this
                setTimeout(function(){
                    that.pwdreadonly = res
                }, 200)
            },
            citychange: function (data) {
                this.sel_city = data
            },
            getinfo: function(){
                var that = this
                httpPost('m=user&c=company&a=add', {}).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        that.cache = res.data.cache
                        that.rating_name = res.data.com_rating
                        if (res.data.cionly == 1) {
                            that.cionly = 1
                        } else {
                            that.cionly = 0
                        }
                        that.mapurl = res.data.mapurl;
                        that.mapsecret = res.data.mapsecret;


                        that.islook = true;
                        that.$nextTick(_ => {
                            var map_url = that.mapurl+"&callback=onAMapCallback";
                            that.writeJs(map_url, that.mapsecret).then(value => {
                                that.openMap();
                            });
                        });
                        clearInterval(editorInterval);
                        editorInterval = setInterval(()=>{
                            if (editor_desc !== null){
                                clearInterval(editorInterval);
                            }else{
                                that.initEditor();
                            }
                        },300);
                    }
                });
            },
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
            getMap:function (){

                var map = new AMap.Map('comadd_conrtainer', {
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
            // 用户名是否重复检测
            checkuname: function(e){
                httpPost('m=user&c=company&a=checkUsername', {username: e.target.value}).then(function (response) {
                    let res = response.data;
                    if (res.error != 0) {
                        message.error(res.msg);
                        return false
                    }
                });
            },

            querySearch(queryString, cb) {
                httpPost('m=user&c=company&a=checkComName', {companyName: queryString}).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        cb(res.data);
                    }
                });
            },
            initEditor() {
                let editorConfig = {
                    MENU_CONF: {
                        uploadImage: {
                            server: baseUrl + 'm=index&c=uploadfile',
                            fieldName: 'file'
                        }
                    }
                };
                // 企业简介
                if (!editor_desc) {
                    editor_desc = createEditor({
                        selector: '#editor-container-comdesc',
                        html: '',
                        config: editorConfig,
                        mode: 'simple'
                    });
                }
                if (!toolbar_desc) {
                    toolbar_desc = createToolbar({
                        editor: editor_desc,
                        selector: '#toolbar-container-comdesc',
                        config: {
                            excludeKeys: ['blockquote', 'header1', 'header2', 'header3', '|', 'through', 'todo', '|', 'insertVideo', 'insertTable', 'codeBlock', '|', 'undo', 'redo', '|',]
                        },
                        mode: 'simple'
                    });
                }
                editor_desc.setHtml('');
            },
            async save() {
                let that = this;
                if(that.username == '') {
                    message.error('请输入用户名');
                    return false;
                }
                if(that.password == '') {
                    message.error('请输入密码');
                    return false;
                }
                if(that.name == '') {
                    message.error('请输入企业全称');
                    return false;
                }
                if(that.hy == '') {
                    message.error('请选择从事行业');
                    return false;
                }
                if(that.pr == '') {
                    message.error('请选择企业性质');
                    return false;
                }
                if(that.mun == '') {
                    message.error('请选择企业规模');
                    return false;
                }
                if(that.cionly == '1' && (that.sel_city.length == 0 || that.sel_city[0] == '')) {
                    message.error('请选择所在地');
                    return false;
                }
                if(that.cionly == '0' && (that.sel_city.length <= 1 || that.sel_city[1] == '')) {
                    message.error('请选择所在地');
                    return false;
                }
                if(that.address == '') {
                    message.error('公司地址不能为空');
                    return false;
                }
                if(that.linkman == '') {
                    message.error('请输入联系人');
                    return false;
                } else{
                    var link_name = that.linkman;
                    var link_man = link_name.replace(/[-_ ]/g,'');// 去掉空格
                    if(!link_man){
                        return ;
                    }
                    var test = link_man.replace(/[0-9]/g,'');
                    if (!test){
                        message.error('联系人不支持全数字');
                        return false;
                    }else{
                        if(/\d/.test(link_man)){
                            if(link_man.length>8){
                                // obj.value = obj.value.substring(0,8);
                                message.error('联系人填写字数不能超过8个');
                                return false;
                            }
                        }
                    }
                }
                if(that.moblie == '') {
                    message.error('联系手机不能为空');
                    return false;
                } else {
                    var obj = that.moblie;
                    if(isjsMobile(obj) == false) {
                        message.error('联系手机格式错误');
                        return false;
                    }
                }
                var obj = that.email;
                var myreg = /^([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9\-]+@([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9]+\.[a-zA-Z]{2,3}$/;
                if(obj != "" && !myreg.test(obj)) {
                    message.error('联系邮箱格式错误');
                    return false;
                }
                // 去除html标签后判断内容是否为空
                var regex = /(<([^>]+)>)/ig
                var content = editor_desc.getHtml().replace(regex, "")
                if (content == "") {
                    message.error('请输入企业简介！')
                    return false;
                }
                if(that.rating_name == '') {
                    message.error('请选择会员套餐！')
                    return false;
                }
                if(that.sendmsg == true && that.moblie == '') {
                    message.error('请输入联系手机！')
                    return false;
                }
                if(that.sendemail==true&&that.email == '') {
                    message.error('请输入联系邮箱！')
                    return false;
                }
                var params = {
                    username: that.username,
                    password: that.password,
                    name: that.name,
                    shortname: that.shortname,
                    hy: that.hy,
                    pr: that.pr,
                    mun: that.mun,
                    address: that.address,
                    x: that.x,
                    y: that.y,
                    linkman: that.linkman,
                    moblie: that.moblie,
                    areacode: that.areacode,
                    telphone: that.telphone,
                    exten: that.exten,
                    email: that.email,
                    sendmsg:that.sendmsg,
                    sendemail: that.sendemail,
                    status:'1',
                    integral: that.integral,
                    rating_name: that.rating_name,
                    content:editor_desc.getHtml(),
                    submit: 1
                };
                if (that.sel_city[0]) {
                    params.provinceid = that.sel_city[0]
                }
                if (that.sel_city[1]) {
                    params.cityid = that.sel_city[1]
                }
                if (that.sel_city[2]) {
                    params.three_cityid = that.sel_city[2]
                }
                httpPost('m=user&c=company&a=add', params).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        message.success(res.msg, function () {
                            that.$parent.$parent.comadddrawer = false
                            that.resetData();
                            that.$parent.$parent.getList();
                        });
                    } else {
                        message.error(res.msg);
                    }
                });
            },
            resetData() {
                this.username =  '';
                this.password =  '';
                this.name =  '';
                this.shortname =  '';
                this.hy =  '';
                this.pr =  '';
                this.mun = '';
                this.address = '';
                this.linkman = '';
                this.moblie = '';
                this.areacode =  '';
                this.telphone = '';
                this.exten = '';
                this.email = '';
                this.sendmsg = false;
                this.sendemail =  false;
                this.integral =  '';
                this.rating_name =  '';
                this.sel_city =  [];
                this.cache =  null;
                this.integral_pricename =  '';
                this.cionly =  0;
                this.pwdreadonly =  true;
                this.x = '';
                this.y = '';
                this.address_v = '';
                this.mapurl =  '';
                this.mapsecret =  '';
            }
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
<style scoped>
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
        font-size: 28px;
        color: #8c939d;
        width: 148px;
        height: 148px;
        line-height: 148px;
        text-align: center;
    }

    .avatar {
        width: 148px;
        height: 148px;
        display: block;
    }
</style>