<template>
	<div class="setUpload">
	        <div class="uploadTable">
	            <table class="tableVue">
	                <thead>
	                    <tr align="left">
	                        <th width="180">{yun:}t key='member_com_00021'{/yun}</th>
	                        <th width="360">{yun:}t key='member_user_00181'{/yun}</th>
	                        <th>{yun:}t key='member_com_00207'{/yun}</th>
	                    </tr>
	                </thead>
	                <tbody>
	                    <tr>
	                        <td>
	                            <div class="TableTite">{yun:}t key='admin_system_00331'{/yun}</div>
	                        </td>
	                        <td>
	                            <div class="TableInpt">
	                                <el-input v-model="list.sy_webname" placeholder="{yun:}t key='wap_user_00076'{/yun}">
	                                </el-input>
	                            </div>
	                        </td>
	                        <td>
	                            <div class="TableShuom">
	                                <span>{yun:}t key='admin_system_00322'{/yun}</span>
	                            </div>
	                        </td>
	                    </tr>
						<tr>
						    <td>
						        <div class="TableTite">{yun:}t key='admin_01014'{/yun}</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_weburl" placeholder="{yun:}t key='wap_user_00076'{/yun}">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>{yun:}t key='admin_01015'{/yun}</span>
						        </div>
						    </td>
						</tr>
	                    <tr>
	                        <td>
	                            <div class="TableTite">{yun:}t key='admin_01016'{/yun}</div>
	                        </td>
	                        <td>
	                            <div class="TableButn">
                                    <el-switch v-model="list.sy_web_online_status">
                                    </el-switch>
	                            </div>
	                        </td>
	                        <td>
	                            <div class="TableShuom">
	                                <span>{yun:}t key='admin_system_00323'{/yun}</span>
	                            </div>
	                        </td>
	                    </tr>
                        <tr>
	                        <td>
	                            <div class="TableTite">{yun:}t key='admin_system_00318'{/yun}</div>
	                        </td>
	                        <td>
	                            <div class="TableButn">
                                    <el-input v-model="list.sy_listnum" placeholder="{yun:}t key='wap_user_00076'{/yun}" onkeyup="this.value=this.value.replace(/[^0-9.]/g,'')" >
                                    	<span slot="suffix" class="slotspan">{yun:}t key='common_02088'{/yun}</span>
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
                                <div class="TableTite">{yun:}t key='admin_01017'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input type="textarea" :rows="2" autosize placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.sy_webclose">
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
                                <div class="TableTite">{yun:}t key='admin_01018'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input type="textarea" :rows="2" autosize placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.sy_webkeyword">
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_system_00312'{/yun}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_01019'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input type="textarea" :rows="2" autosize placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.sy_webmeta">
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_system_00313'{/yun}</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_system_00327'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input type="textarea" :rows="2" autosize placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.sy_webcopyright">
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_system_00320'{/yun}</span>
                                </div>
                            </td>
                        </tr>
						<tr>
						    <td>
						        <div class="TableTite">{yun:}t key='admin_system_00324'{/yun}</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_webemail" placeholder="{yun:}t key='wap_user_00076'{/yun}">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>{yun:}t key='admin_system_00325'{/yun}</span>
						        </div>
						    </td>
						</tr>
                        <tr>
						    <td>
						        <div class="TableTite">{yun:}t key='admin_system_00330'{/yun}</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_webmoblie" @input="inputIntNumber($event, 'list', 'sy_webmoblie')" placeholder="{yun:}t key='wap_user_00076'{/yun}">
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
						        <div class="TableTite">{yun:}t key='default_00255'{/yun}</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_webtel" placeholder="{yun:}t key='wap_user_00076'{/yun}">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>{yun:}t key='admin_system_00317'{/yun}</span>
						        </div>
						    </td>
						</tr>
                        <tr>
						    <td>
						        <div class="TableTite">{yun:}t key='admin_system_00326'{/yun}</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_webrecord" placeholder="{yun:}t key='wap_user_00076'{/yun}">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>{yun:}t key='admin_system_00325'{/yun}</span>
						        </div>
						    </td>
						</tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_system_00329'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_websecord" placeholder="{yun:}t key='wap_user_00076'{/yun}">
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
                                <div class="TableTite">{yun:}t key='admin_01020'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_perfor" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_system_00321'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_hrlicense" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                </div>
                            </td>
                        </tr>
                        <tr>
						    <td>
						        <div class="TableTite">{yun:}t key='wap_com_00102'{/yun}</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_freewebtel" placeholder="{yun:}t key='wap_user_00076'{/yun}">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>{yun:}t key='admin_system_00325'{/yun}</span>
						        </div>
						    </td>
						</tr>
						<tr>
						    <td>
						        <div class="TableTite">{yun:}t key='admin_01021'{/yun}</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_comwebtel" placeholder="{yun:}t key='wap_user_00076'{/yun}">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>{yun:}t key='admin_system_00314'{/yun}</span>
						        </div>
						    </td>
						</tr>
                        <tr>
                            <td>
                                <div class="TableTite">{yun:}t key='admin_user_00223'{/yun}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_worktime" placeholder="{yun:}t key='wap_user_00076'{/yun}">
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>{yun:}t key='admin_system_00316'{/yun}</span>
                                </div>
                            </td>
                        </tr>
						<tr>
						    <td>
						        <div class="TableTite">{yun:}t key='wap_com_00101'{/yun}</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_qq" @input="inputIntNumber($event, 'list', 'sy_qq')" placeholder="{yun:}t key='wap_user_00076'{/yun}">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>{yun:}t key='admin_system_00319'{/yun}</span>
						        </div>
						    </td>
						</tr>
						<tr>
						    <td>
						        <div class="TableTite">{yun:}t key='wap_00040'{/yun}</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_webadd" placeholder="{yun:}t key='wap_user_00076'{/yun}">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>{yun:}t key='admin_system_00328'{/yun}</span>
						        </div>
						    </td>
						</tr>
						<tr>
						    <td>
						        <div class="TableTite">{yun:}t key='admin_01022'{/yun}</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						          <el-input type="textarea" :rows="2" autosize placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.sy_webtongji">
						          </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						         {yun:}t key='admin_01023'{/yun}
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