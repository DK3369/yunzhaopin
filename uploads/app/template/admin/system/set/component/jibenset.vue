<template>
	<div class="setUpload">
	        <div class="uploadTable">
	            <table class="tableVue">
	                <thead>
	                    <tr align="left">
	                        <th width="180">名称</th>
	                        <th width="360">状态</th>
	                        <th>说明</th>
	                    </tr>
	                </thead>
	                <tbody>
	                    <tr>
	                        <td>
	                            <div class="TableTite">网站名称</div>
	                        </td>
	                        <td>
	                            <div class="TableInpt">
	                                <el-input v-model="list.sy_webname" placeholder="请输入内容">
	                                </el-input>
	                            </div>
	                        </td>
	                        <td>
	                            <div class="TableShuom">
	                                <span>如：XXX人才网</span>
	                            </div>
	                        </td>
	                    </tr>
						<tr>
						    <td>
						        <div class="TableTite">网站地址</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_weburl" placeholder="请输入内容">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>如：http://www.hr135.com</span>
						        </div>
						    </td>
						</tr>
	                    <tr>
	                        <td>
	                            <div class="TableTite">网站状态</div>
	                        </td>
	                        <td>
	                            <div class="TableButn">
                                    <el-switch v-model="list.sy_web_online_status">
                                    </el-switch>
	                            </div>
	                        </td>
	                        <td>
	                            <div class="TableShuom">
	                                <span>开启才能正常访问</span>
	                            </div>
	                        </td>
	                    </tr>
                        <tr>
	                        <td>
	                            <div class="TableTite">后台列表页显示条数：</div>
	                        </td>
	                        <td>
	                            <div class="TableButn">
                                    <el-input v-model="list.sy_listnum" placeholder="请输入内容" onkeyup="this.value=this.value.replace(/[^0-9.]/g,'')" >
                                    	<span slot="suffix" class="slotspan">条</span>
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
                                <div class="TableTite">网站关闭原因：</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input type="textarea" :rows="2" autosize placeholder="请输入内容" v-model="list.sy_webclose">
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
                                <div class="TableTite">网站关键词：</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input type="textarea" :rows="2" autosize placeholder="请输入内容" v-model="list.sy_webkeyword">
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>提示：网站关键词作为公共部分详细设置请到系统-》SEO设置单独设置</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">网站描述：</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input type="textarea" :rows="2" autosize placeholder="请输入内容" v-model="list.sy_webmeta">
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>提示：网站描述作为公共部分，详细设置请到系统-》SEO管理设置</span>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">网站版权信息：</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input type="textarea" :rows="2" autosize placeholder="请输入内容" v-model="list.sy_webcopyright">
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>提示：© 可复制使用</span>
                                </div>
                            </td>
                        </tr>
						<tr>
						    <td>
						        <div class="TableTite">站长EMAIL：</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_webemail" placeholder="请输入内容">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>网站客服联系方式</span>
						        </div>
						    </td>
						</tr>
                        <tr>
						    <td>
						        <div class="TableTite">站长手机：</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_webmoblie" @input="inputIntNumber($event, 'list', 'sy_webmoblie')" placeholder="请输入内容">
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
						        <div class="TableTite">站长传真：</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_webtel" placeholder="请输入内容">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>如：021-61190281</span>
						        </div>
						    </td>
						</tr>
                        <tr>
						    <td>
						        <div class="TableTite">ICP备案号：</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_webrecord" placeholder="请输入内容">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>网站客服联系方式</span>
						        </div>
						    </td>
						</tr>
                        <tr>
                            <td>
                                <div class="TableTite">网安备案号：</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_websecord" placeholder="请输入内容">
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
                                <div class="TableTite">ICP经营许可证编号：</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_perfor" placeholder="请输入内容"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">人力资源证编号：</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_hrlicense" placeholder="请输入内容"></el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                </div>
                            </td>
                        </tr>
                        <tr>
						    <td>
						        <div class="TableTite">客服电话</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_freewebtel" placeholder="请输入内容">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>网站客服联系方式</span>
						        </div>
						    </td>
						</tr>
						<tr>
						    <td>
						        <div class="TableTite">企业咨询电话</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_comwebtel" placeholder="请输入内容">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>专门用于企业相关功能咨询的客服电话，如购买套餐、企业审核等</span>
						        </div>
						    </td>
						</tr>
                        <tr>
                            <td>
                                <div class="TableTite">工作时间</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input v-model="list.sy_worktime" placeholder="请输入内容">
                                    </el-input>
                                </div>
                            </td>
                            <td>
                                <div class="TableShuom">
                                    <span>如：工作日 9:00-18:00</span>
                                </div>
                            </td>
                        </tr>
						<tr>
						    <td>
						        <div class="TableTite">客服QQ</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_qq" @input="inputIntNumber($event, 'list', 'sy_qq')" placeholder="请输入内容">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>多个则用半角逗号隔开</span>
						        </div>
						    </td>
						</tr>
						<tr>
						    <td>
						        <div class="TableTite">公司地址</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						            <el-input v-model="list.sy_webadd" placeholder="请输入内容">
						            </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						            <span>公司联系地址</span>
						        </div>
						    </td>
						</tr>
						<tr>
						    <td>
						        <div class="TableTite">统计代码</div>
						    </td>
						    <td>
						        <div class="TableInpt">
						          <el-input type="textarea" :rows="2" autosize placeholder="请输入内容" v-model="list.sy_webtongji">
						          </el-input>
						        </div>
						    </td>
						    <td>
						        <div class="TableShuom">
						         网站底部的统计代码
						        </div>
						    </td>
						</tr>
	                   
	                </tbody>
	            </table>
	        </div>
	        <div class="setBasicButn" style="border: none; height: 80px;">
	            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">提交</el-button>
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
                    message.success('操作成功');
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