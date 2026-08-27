<template>
	<div class="setUpload">
	        <div class="uploadTable">
	            <table class="tableVue">
	                <thead>
	                    <tr align="left">
	                        <th width="180">{{ lc('member_com_00021') }}</th>
	                        <th width="360">{{ lc('member_user_00181') }}</th>
	                        <th>{{ lc('member_com_00207') }}</th>
	                    </tr>
	                </thead>
	                <tbody>
	                    <tr>
	                        <td>
	                            <div class="TableTite">{{ lc('admin_system_00331') }}</div>
	                        </td>
	                        <td>
	                            <div class="TableInpt">
	                                <el-input v-model="list.sy_webname" :placeholder="lc('wap_user_00076')">
	                                </el-input>
	                            </div>
	                        </td>
	                        <td>
	                            <div class="TableShuom">
	                                <span>{{ lc('admin_system_00322') }}</span>
	                            </div>
	                        </td>
	                    </tr>
						<tr>
						    <td>
						        <div class="TableTite">{{ lc('admin_01014') }}</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_weburl" :placeholder="lc('wap_user_00076')">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>{{ lc('admin_01015') }}</span>
						        </div>
						    </td>
						</tr>
	                    <tr>
	                        <td>
	                            <div class="TableTite">{{ lc('admin_01016') }}</div>
	                        </td>
	                        <td>
	                            <div class="TableButn">
                                    <el-switch v-model="list.sy_web_online_status">
                                    </el-switch>
	                            </div>
	                        </td>
	                        <td>
	                            <div class="TableShuom">
	                                <span>{{ lc('admin_system_00323') }}</span>
	                            </div>
	                        </td>
	                    </tr>
                        <tr>
	                        <td>
	                            <div class="TableTite">{{ lc('admin_system_00318') }}</div>
	                        </td>
	                        <td>
	                            <div class="TableButn">
                                    <el-input v-model="list.sy_listnum" :placeholder="lc('wap_user_00076')" onkeyup="this.value=this.value.replace(/[^0-9.]/g,'')" >
                                    	<template #suffix><span class="slotspan">{{ lc('common_02088') }}</span></template>
                                    </el-input>
	                            </div>
	                        </td>
	                        <td>
	                            <div class="TableShuom">
	                                
	                            </div>
	                        </td>
	                    </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_01017') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input type="textarea" :rows="2" autosize :placeholder="lc('wap_user_00076')" v-model="list.sy_webclose">
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
                                <div class="TableTite">{{ lc('admin_01018') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input type="textarea" :rows="2" autosize :placeholder="lc('wap_user_00076')" v-model="list.sy_webkeyword">
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_system_00312') }}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_01019') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input type="textarea" :rows="2" autosize :placeholder="lc('wap_user_00076')" v-model="list.sy_webmeta">
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_system_00313') }}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_system_00327') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input type="textarea" :rows="2" autosize :placeholder="lc('wap_user_00076')" v-model="list.sy_webcopyright">
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_system_00320') }}</span>
                                </div>
                            </td>
                        </tr>
						<tr>
						    <td>
						        <div class="TableTite">{{ lc('admin_system_00324') }}</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_webemail" :placeholder="lc('wap_user_00076')">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>{{ lc('admin_system_00325') }}</span>
						        </div>
						    </td>
						</tr>
                        <tr>
						    <td>
						        <div class="TableTite">{{ lc('admin_system_00330') }}</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_webmoblie" @input="inputIntNumber($event, 'list', 'sy_webmoblie')" :placeholder="lc('wap_user_00076')">
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
						        <div class="TableTite">{{ lc('default_00255') }}</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_webtel" :placeholder="lc('wap_user_00076')">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>{{ lc('admin_system_00317') }}</span>
						        </div>
						    </td>
						</tr>
                        <tr>
						    <td>
						        <div class="TableTite">{{ lc('admin_system_00326') }}</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_webrecord" :placeholder="lc('wap_user_00076')">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>{{ lc('admin_system_00325') }}</span>
						        </div>
						    </td>
						</tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_system_00329') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_websecord" :placeholder="lc('wap_user_00076')">
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_01020') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_perfor" :placeholder="lc('wap_user_00076')"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_system_00321') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_hrlicense" :placeholder="lc('wap_user_00076')"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                </div>
                            </td>
                        </tr>
                        <tr>
						    <td>
						        <div class="TableTite">{{ lc('wap_com_00102') }}</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_freewebtel" :placeholder="lc('wap_user_00076')">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>{{ lc('admin_system_00325') }}</span>
						        </div>
						    </td>
						</tr>
						<tr>
						    <td>
						        <div class="TableTite">{{ lc('admin_01021') }}</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_comwebtel" :placeholder="lc('wap_user_00076')">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>{{ lc('admin_system_00314') }}</span>
						        </div>
						    </td>
						</tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_user_00223') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_worktime" :placeholder="lc('wap_user_00076')">
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{{ lc('admin_system_00316') }}</span>
                                </div>
                            </td>
                        </tr>
						<tr>
						    <td>
						        <div class="TableTite">{{ lc('wap_com_00101') }}</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_qq" @input="inputIntNumber($event, 'list', 'sy_qq')" :placeholder="lc('wap_user_00076')">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>{{ lc('admin_system_00319') }}</span>
						        </div>
						    </td>
						</tr>
						<tr>
						    <td>
						        <div class="TableTite">{{ lc('wap_00040') }}</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_webadd" :placeholder="lc('wap_user_00076')">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>{{ lc('admin_system_00328') }}</span>
						        </div>
						    </td>
						</tr>
						<tr>
						    <td>
						        <div class="TableTite">{{ lc('admin_01022') }}</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						          <el-input type="textarea" :rows="2" autosize :placeholder="lc('wap_user_00076')" v-model="list.sy_webtongji">
						          </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						         {{ lc('admin_01023') }}
						        </div>
						    </td>
						</tr>
	                   
	                </tbody>
	            </table>
	        </div>
	        <div class="setBasicButn" style="border: none; height: 80px;">
	            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
	        </div>
	    </div>
</template>
    
<script>
const httpPost = (...a) => window.httpPost(...a)
const lc = (...a) => window.lc(...a)
const message = typeof window !== 'undefined' && window.message ? window.message : { success(){}, error(){}, warning(){}, confirm(){}, alert(){}, open(){} }
const delConfirm = (...a) => window.delConfirm(...a)
const formatDate = (...a) => window.formatDate(...a)
const formatMonth = (...a) => window.formatMonth(...a)
const formatDatetime = (...a) => window.formatDatetime(...a)
const deepClone = (...a) => window.deepClone(...a)
const scrollToTop = (...a) => window.scrollToTop(...a)
const isEmpty = (...a) => window.isEmpty(...a)
const isArray = (...a) => window.isArray(...a)
const $ = typeof window !== 'undefined' && window.$ ? window.$ : Object.assign(function(){ return { length: 0 } }, {})
const echarts = typeof window !== 'undefined' && window.echarts ? window.echarts : { init(){ return { setOption(){}, resize(){} } }, graphic: { LinearGradient: function(){} } }

export default {
    props: {
        list: Object
    },
    data: function () {
        return {
            input: '',
            uri:"m=system&c=",
            saveLoading: false,
        }
    },
    methods: {
    	inputIntNumber(val, form, key) {
            this.$props[form][key] = val.replace(/[^0-9]/g,'');
        },
        save:function (){
            let _this = this;
            let url = this.uri+"set_config&a=save";
            let ruleForm = {
                sy_webname: this.list.sy_webname,
                sy_weburl: this.list.sy_weburl,
                sy_webkeyword: this.list.sy_webkeyword,
                sy_webmeta: this.list.sy_webmeta,
                sy_webcopyright: this.list.sy_webcopyright,
                sy_webtongji: this.list.sy_webtongji,
                sy_webemail: this.list.sy_webemail,
                sy_webmoblie: this.list.sy_webmoblie,
                sy_webrecord: this.list.sy_webrecord,
				sy_websecord: this.list.sy_websecord,
                sy_perfor: this.list.sy_perfor,
                sy_hrlicense: this.list.sy_hrlicense,
                sy_webtel: this.list.sy_webtel,
                sy_qq: this.list.sy_qq,
                sy_freewebtel: this.list.sy_freewebtel,
                sy_comwebtel: this.list.sy_comwebtel,
                sy_worktime: this.list.sy_worktime,
                sy_listnum: this.list.sy_listnum,
                sy_webadd: this.list.sy_webadd,
                sy_webclose: this.list.sy_webclose,
                sy_web_online: this.list.sy_web_online_status ? 1 : 2,
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
};
</script>
<style scoped></style>