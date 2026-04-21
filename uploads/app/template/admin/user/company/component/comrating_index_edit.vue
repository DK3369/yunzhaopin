<template>
    <!--会员-企业-套餐服务：套餐设置-设置会员套餐-->
    <div v-loading="loading" class="drawerModlue">
        <div class="drawerModInfo drawerModInfoOne">
            <div class="adminBoldTips">
                <el-alert title="企业会员等级关乎您的收入问题，请谨慎添加和修改并注意整体的合理性。" show-icon type="success"></el-alert>
                
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>会员模式</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.type" label="1">套餐模式</el-radio>
                    <el-radio v-model="ruleForm.type" label="2">时间模式</el-radio>
                </div>
                <div class="drawerModTips">
                    <el-alert title="套餐模式针对下载简历等数量控制，时间模式在服务有效期内，不限数量，可以限制每日上限"
                              type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>会员名称</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.name" style="margin-right: 8px"></el-input>
                    <el-checkbox v-model="ruleForm.youhuiBool" label="优惠活动" border size="medium"></el-checkbox>
                </div>
                <div class="drawerModTips">
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>服务价格</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.service_price" placeholder="请输入数字">
                        <span slot="suffix" class="slotspan">元</span>
                    </el-input>
                </div>
                <div class="drawerModTips">
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>服务时间</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.service_time" placeholder="请输入数字">
                        <span slot="suffix" class="slotspan">天</span>
                    </el-input>
                </div>
                <div class="drawerModTips">
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>等级排序</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.sort" placeholder="请输入数字"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="大前小后" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
			
			<div class="drawerModLis">
			    <div class="drawerModTite">
			        <span>最长有效天数</span>
			    </div>
			    <div class="drawerModInpt">
			        <el-input v-model="ruleForm.max_time" placeholder="请输入数字">
			            <span slot="suffix" class="slotspan">天</span>
			        </el-input>
			    </div>
			    <div class="drawerModTips">
					<el-alert title="最长有效期不能小于服务时间,为0时不限。" type="info" show-icon :closable="false">
                    </el-alert>
                    <el-alert title="不管企业是否被暂停，超过最长有效期后都作过期处理。" type="info" show-icon :closable="false">
                    </el-alert>
			    </div>
			</div>
			<div class="drawerModLis">
			    <div class="drawerModTite">
			        <span>可暂停次数</span>
			    </div>
			    <div class="drawerModInpt">
			        <el-input v-model="ruleForm.suspend_num" placeholder="请输入数字">
			            <span slot="suffix" class="slotspan">次</span>
			        </el-input>
			    </div>
			    <div class="drawerModTips">
					<el-alert title="最长有效期内可暂停次数" type="info" show-icon :closable="false">
					</el-alert>
				</div>
			</div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>会员状态</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.display" label="1">启用</el-radio>
                    <el-radio v-model="ruleForm.display" label="0">不启用</el-radio>
                </div>
                <div class="drawerModTips">
                    <el-alert title="前台是否可见" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>会员图标</span>
                </div>
                <div class="drawerModInpt" style="display: flex;align-items: center;">
                    <el-upload ref="compic" :action="uploadAction" :on-change="uploadChange" :accept="pic_accept"
                               :show-file-list="false">
                        <el-button size="small" type="primary">上传图片</el-button>
                    </el-upload>
                    <div class="up_sy_logo_div" style="margin-left: 15px; position: relative;">
                        <el-image v-if="ruleForm.com_pic" style="width:40px;height:40px;" :src="ruleForm.com_pic"
                                  :preview-src-list="ruleForm.com_pic ? [ruleForm.com_pic] : []"></el-image>
                    <span style=" display: block;width:150px;;position: absolute;left:60px;top:50%; margin-top: -10px;"><el-link type="danger" :underline="false" v-if="ruleForm.com_pic" @click="delpic"  icon="el-icon-delete">删除图标</el-link></span>
                    </div>
                </div>
                <div class="drawerModTips">
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>等级说明</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.explains" type="textarea"
                              :autosize="{ minRows: 2, maxRows: 3}"></el-input>
                </div>
                <div class="drawerModTips">
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>赠送积分</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.integral_buy" placeholder="请输入数字" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')">
                    </el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="企业购买套餐赠送一定积分用于职位置顶、紧急招聘等服务" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>增值折扣</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.service_discount" placeholder="请输入数字"><span slot="suffix"
                                                                                                 class="slotspan">%</span>
                    </el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="折扣针对不同等级会员所购买增值包服务享有的优惠！（0、100表示无折扣，88表示八八折）"
                              type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis drawerModInFlex">
                <div class="drawerModTite">
                    <span>基础套餐</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.resume" placeholder="请输入数字">
                        <template slot="prepend">下载简历</template>
                        <span slot="suffix" class="slotspan">份</span>
                    </el-input>
                    <el-input v-model="ruleForm.interview" placeholder="请输入数字" style="margin-left: 10px;">
                        <template slot="prepend">邀请面试</template>
                        <span slot="suffix" class="slotspan">份</span>
                    </el-input>
                    <el-input v-model="ruleForm.breakjob_num" placeholder="请输入数字">
                        <template slot="prepend">刷新职位</template>
                        <span slot="suffix" class="slotspan">次</span>
                    </el-input>
                    <el-input v-model="ruleForm.zph_num" placeholder="请输入数字" style="margin-left: 10px;">
                        <template slot="prepend">招聘会报名</template>
                        <span slot="suffix" class="slotspan">次</span>
                    </el-input>
                    
                </div>
                <div class="drawerModTips">
                    <el-alert v-if="ruleForm.type == 2"
                              title="时间会员 - 基础套餐限制每日操作数量；例如：每日最多下载 X 份简历" type="warning"
                              show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>职位套餐</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.job_num" placeholder="请输入数字">
                        <span slot="suffix" class="slotspan">份</span>
                    </el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="可上架职位总数，套餐模式和时间模式同步，不做区分" type="info" show-icon
                              :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis drawerModInFlex">
                <div class="drawerModTite">
                    <span>职位推广</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.top_num" placeholder="请输入数字">
                        <template slot="prepend">置顶天数</template>
                        <span slot="suffix" class="slotspan">天</span>
                    </el-input>
                    <el-input v-model="ruleForm.urgent_num" placeholder="请输入数字" style="margin-left: 10px;">
                        <template slot="prepend">紧急天数</template>
                        <span slot="suffix" class="slotspan">天</span>
                    </el-input>
                    <el-input v-model="ruleForm.rec_num" placeholder="请输入数字">
                        <template slot="prepend">推荐天数</template>
                        <span slot="suffix" class="slotspan">天</span>
                    </el-input>
                </div>
                <div class="drawerModTips">
                </div>
            </div>
            <div class="drawerModLis drawerModInFlex">
                <div class="drawerModTite">
                    <span>每日免费</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.freerefresh_num" placeholder="请输入数字">
                        <template slot="prepend">刷新职位</template>
                        <span slot="suffix" class="slotspan">次</span>
                    </el-input>
                    <el-input v-model="ruleForm.freelook_num" placeholder="请输入数字" style="margin-left: 10px;">
                        <template slot="prepend">查看投递</template>
                        <span slot="suffix" class="slotspan">份</span>
                    </el-input>
                    
                </div>
                <div class="drawerModTips">
                    <el-alert
                            title="刷新职位：每天可免费刷新职位数量，企业客户刷新职位操作，优先扣除每日免费刷新套餐。超出后，需花费基础套餐刷新点数。"
                            type="info" show-icon :closable="false"></el-alert>
                    <el-alert
                            title="查看投递：针对求职者主动投递的简历，企业每天可以免费下载、查看其联系方式的数量。超出后，需花费基础套餐下载简历点数。"
                            type="info" show-icon :closable="false"></el-alert>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">提交
            </el-button>
        </div>
    </div>
</template>

<script>
module.exports = {
    props: {
        id: {type: [Number, String], default: 0},
        config: {type: [Object], default: {}},
    },
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            loading: false,
            submitLoading: false,
            ruleForm: {
                id: this.id,
                category: 1,
                //会员模式
                type: null,
                //会员名称
                name: '',
                //优惠活动
                youhuiBool: false,
                //优惠价格
                yh_price: '',
                //优惠日期
                time: null,
                //服务价格
                service_price: '',
                //服务时间
                service_time: '',
                //等级排序
                sort: '',
                //会员状态
                display: null,
                //会员图标
                com_pic: '',
                //等级说明
                explains: '',
                //赠送
                integral_buy: '0',
                //增值折扣
                service_discount: '',
                //下载简历
                resume: '',
                //邀请面试
                interview: '',
                //刷新职位
                breakjob_num: '',
                //招聘会报名
                zph_num: '',
                //
                chat_num: '',
                //职位套餐
                job_num: '',
                //置顶天数
                top_num: '',
                //紧急天数
                urgent_num: '',
                //推荐天数
                rec_num: '',
                //刷新职位
                freerefresh_num: '',
                //查看投递
                freelook_num: '',
                //主动聊天
                freeactchat_num: '',
                //回复聊天
                freerepchat_num: '',
				//可暂停次数
				suspend_num: 0,
				//最长有效期天数
				max_time:0
            },
            file: [],//暂存文件
            haspic:false,
            uploadAction: baseUrl + 'm=common&c=common_upload'
        }
    },
    created() {
        if (this.id > 0) {
            this.getInfo();
        }
    },
    methods: {
        delpic(){
            if(this.haspic){
                var params = {id:this.id};
                delConfirm(this, params, this.delpicPost,'确定要删除？');
            }else if (this.file.length !== 0) {
                this.file = [];
                this.ruleForm.com_pic = '';
                this.$refs.compic.clearFiles();
            }
        },
        delpicPost(params) {
            let _this = this;
            httpPost('m=user&c=company_comrating&a=delpic', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success('删除成功！');
                    _this.ruleForm.com_pic = '';
                    _this.file = [];
                    _this.$refs.compic.clearFiles();
                } else {
                    message.error('删除失败！');
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        uploadChange(file) {
            this.ruleForm.com_pic = URL.createObjectURL(file.raw);
            // 复刻文件信息
            this.file = file.raw;
        },
        getInfo() {
            let _this = this;
            let params = {id: this.id};
            _this.loading = true;
            httpPost('m=user&c=company_comrating&a=rating', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    let row = res.data;
                    if (row.yh_price < 1) {
                        _this.ruleForm.youhuiBool = false;
                    } else {
                        _this.ruleForm.youhuiBool = true;
                    }
                    Object.keys(_this.ruleForm).forEach((key) => {
                        if (row.hasOwnProperty(key)) {
                            _this.ruleForm[key] = row[key];
                        }
                    });
                    if(_this.ruleForm.com_pic!=''){
                        _this.haspic = true;
                    }
                }
                _this.loading = false;
            }).catch(function (error) {
                console.log(error);
            });
        },
        submitForm(formName) {
            // this.$refs[formName].validate((valid) => {if (valid) {}});
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.ruleForm));
            params.time = Array.isArray(params.time) ? params.time.join('~') : null;
            if (params.youhuiBool) {
                params.youhui = 1;
            } else {
                delete params.youhui
            }
            delete params.youhuiBool;
            delete params.com_pic;
            
            
            if (params.name == '') {
                message.error('会员名称不能为空！');
                return false;
            }
            if (parseFloat(params.service_price) < 0) {
                message.error('请填写服务金额！');
                return false;
            }
            if (params.youhuiBool) {
                if (parseFloat(params.yh_price) <= 0 || parseFloat(params.yh_price) > parseFloat(params.service_price)) {
                    message.error('请填写合理的优化价格！');
                    return false;
                }
                if (!params.time) {
                    message.error('请选择优惠时间！');
                    return false;
                }
            }
			
			if(parseInt(params.max_time)>0 && parseInt(params.service_time) > parseInt(params.max_time)){
				message.error('最长有效期不能小于服务时间！');
				return false;
			}
			
            let formData = new FormData();
            Object.keys(params).forEach((key) => {
                if (Array.isArray(params[key])) {
                    params[key].forEach((v) => {
                        formData.append(key + '[]', v);
                    });
                } else {
                    formData.append(key, params[key]);
                }
            });
            if (this.file.length !== 0) {
                formData.append('file', this.file);
            }
            _this.submitLoading = true;
            httpPost('m=user&c=company_comrating&a=saveclass', formData).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                    _this.$emit("child-event-list");
                } else {
                    message.error(res.msg);
                }
            }).catch(function (error) {
                console.log(error);
            }).finally(function () {
                _this.submitLoading = false;
            });
        },
    },
    watch: {
        "ruleForm.yh_price": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.yh_price = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.service_price": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.service_price = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.service_time": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.service_time = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.sort": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.sort = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.integral_buy": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.integral_buy = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.service_discount": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    if (newValue < 0) {
                        newValue = 0;
                    } else if (newValue > 100) {
                        newValue = 100;
                    }
                    this.ruleForm.service_discount = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.resume": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.resume = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.interview": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.interview = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.breakjob_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.breakjob_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.zph_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.zph_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        
        
        "ruleForm.job_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    if (newValue < 0) {
                        newValue = 0;
                    }
                    this.ruleForm.job_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.top_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.top_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.urgent_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.urgent_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.rec_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.rec_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.freerefresh_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.freerefresh_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.freelook_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.freelook_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        
		"ruleForm.suspend_num": {
		    handler(newValue, oldValue) {
		        if (typeof (newValue) == 'string') {
		            newValue = newValue.replace(/[^0-9.]/g, '');
		            this.ruleForm.suspend_num = newValue;
		        }
		    },
		    deep: true,
		    immediate: true
		},
		"ruleForm.max_time": {
		    handler(newValue, oldValue) {
		        if (typeof (newValue) == 'string') {
		            newValue = newValue.replace(/[^0-9.]/g, '');
		            this.ruleForm.max_time = newValue;
		        }
		    },
		    deep: true,
		    immediate: true
		},
	},
};
</script>
<style scoped>
.mt-10 {
    margin-top: 10px;
}

.coupon-select {
    display: flex;
    align-items: center;
}

.title-inner {
    color: rgb(144, 147, 153);
    font-size: 14px;
    width: 97px;
    text-align: center;
    padding: 0 10px;
}
</style>