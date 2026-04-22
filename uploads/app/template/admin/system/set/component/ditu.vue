<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="180">名称</th>
                        <th width="380">状态</th>
                        <th>说明</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">IP跳转到当前城市</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio-group v-model="list.map_tocity">
                                    <el-radio label="1">跳转</el-radio>
                                    <el-radio label="2">保持默认坐标</el-radio>
                                </el-radio-group>

                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>IP跳转到当前城市</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">高德地图Web端Key</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="list.map_key" placeholder="请输入内容"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>服务平台必须是“Web端(JS API)”</span><a href="https://lbs.amap.com" target="_balnk">申请地址</a>
                            </div>
                        </td>
                    </tr>
					<tr>
                        <td>
                            <div class="TableTite">高德地图Web端安全秘钥</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="list.map_secret" placeholder="请输入内容"></el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">高德地图Web服务Key</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="list.bmap_webak" placeholder="请输入内容"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>服务平台必须是“Web服务”</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">默认坐标</div>
                        </td>
                        <td>
                            <div class="TableInpt TableInptCoor">
                                <div class="TableInCooLis">
                                    <el-input v-model="list.map_x"  placeholder="">
                                        <template slot="prepend">X</template>
                                    </el-input>
                                </div>
                                <div class="TableInCooLis">
                                    <el-input v-model="list.map_y" placeholder="">
                                        <template slot="prepend">Y</template>
                                    </el-input>
                                </div>
                                <div>
                                    <el-button type="primary" @click="getclick">{{ title }}</el-button>
                                </div>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>默认坐标</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-show="dituShow">
                        <td>
                            <div class="TableTite">获取坐标：</div>
                        </td>
                        <td>
                            <div class="TableInpt TableInptCoor" style=" position:relative; z-index:0px;width: 600px;">
                                <div id="conrtainer" style="width:100%;height:300px; position:relative; z-index:1"></div>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>默认坐标</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium"  @click="save" :disabled="saveLoading">提交</el-button>
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
            title:'点击获取坐标',
            uri:"m=system&c=",
            saveLoading: false,
			dituShow: false
        }
    },
    mounted() {
        var mapurl = this.list.mapurl+"&callback=onAMapCallback";
        this.writeJs(mapurl);
    },
    methods: {
        writeJs(url) {
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
                // 插入script脚本
                let scriptNode = document.createElement('script');
                scriptNode.setAttribute('type', 'text/javascript');
                scriptNode.setAttribute('src', url);
                document.body.appendChild(scriptNode);
            });
        },
        getclick:function(){
            if (this.list.map_key){
				this.dituShow = true;
                this.openMap();
            }else{
				message.error('请先设置Web端Key');
			}
        },
        openMap:function (){
            x = this.list.map_x?this.list.map_x :116.404;
            y = this.list.map_y?this.list.map_y :39.915;
            var _this = this;
            var data=get_map_config();
            if(data && data.indexOf('map_x')>-1){
                var config=eval('('+data+')');
                var rating,map_control_type,map_control_anchor;
                if (!x && !y) { x = config.map_x; y = config.map_y; }

                var map = new AMap.Map('conrtainer', {
				  zoom: 15,
				  center: [x,y]
				});
                var marker = new AMap.Marker({
					position: new AMap.LngLat(x,y)
				});
                map.add(marker);
                map.on("click",function(e){
                    var lngLat = e.lnglat;
                    _this.list.map_x = lngLat.lng
                    _this.list.map_y = lngLat.lat
                    map.clearMap();
					var marker = new AMap.Marker({
						position: new AMap.LngLat(lngLat.lng, lngLat.lat)
					});
                    map.add(marker);
                });
            }
        },
        save:function (){
            let _this = this;
            let url = this.uri+"set_config&a=save";
            let ruleForm = {
                map_tocity: _this.list.map_tocity,
                content: _this.list.map_key,
                con_post:_this.list.bmap_webak,
				map_secret: _this.list.map_secret,
                pytoken: _this.list.pytoken,
                map_x: _this.list.map_x,
                map_y: _this.list.map_y,
                ctype:'mapconfig',
            };
            _this.saveLoading = true;
            httpPost(url, ruleForm).then(function (response) {
                var res = response.data;
                if (res.error == 0) {
                    message.success('操作成功', function () {
						localStorage.setItem('systemTab', 'fifth');
                        location.reload();
                    });
                } else {
                    message.error(res.msg);
                }
            }).finally(function () {
                setTimeout(function () {
                    _this.saveLoading = false;
                }, 2000);
            });
        }
    }
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
<style scoped></style>