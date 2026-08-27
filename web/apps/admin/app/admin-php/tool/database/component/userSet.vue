<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert :title="lc('admin_tool_00059')" type="success" :closable="false"></el-alert>
        </div>
        <div class=" moduleTable">

            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="200">{{ lc('member_com_00021') }}</th>
                    <th width="400">{{ lc('member_user_00181') }}</th>
                    <th>{{ lc('member_com_00207') }}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_tool_00269') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_user_sexs" :placeholder="lc('wap_user_00100')">
                                <el-option v-for="sex in sexOptions" :key="sex.value" :label="sex.label" :value="sex.value"></el-option>
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
                        <div class="TableTite">{{ lc('admin_tool_00270') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_user_marriage" :placeholder="lc('wap_user_00100')">
                                <el-option v-for="marriage in marriageOptions" :key="marriage.value" :label="marriage.label" :value="marriage.value"></el-option>
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
                        <div class="TableTite">{{ lc('admin_tool_00271') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_user_edu" :placeholder="lc('wap_user_00100')">
                                <el-option v-for="edu in eduOptions" :key="edu.value" :label="edu.label" :value="edu.value"></el-option>
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
                        <div class="TableTite">{{ lc('admin_tool_00272') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_user_exp" :placeholder="lc('wap_user_00100')">
                                <el-option v-for="exp in expOptions" :key="exp.value" :label="exp.label" :value="exp.value"></el-option>
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
                        <div class="TableTite">{{ lc('admin_tool_00273') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_user_nationality" placeholder=" "></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_tool_00274') }}</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" size="medium" @click="submitLocoyConfig" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
            </div>
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
			locoy_config: Object,
			user_set: Number
		},
		watch: {
			locoy_config: {
				handler (n, v){
				},
				deep: true
			},
			user_set: {
				handler(newValue, oldValue) {
					if (newValue == 1) {
						this.getCache();
					}
				},
				deep: true,
				immediate: true
			}
		},
		data: function () {
			return {
				sexOptions: [],
				marriageOptions: [],
                eduOptions: [],
                expOptions: [],
                saveLoading: false
			}
		},
		methods: {
			async getCache() {
				let res = await httpPost('m=tool&c=dataCollection&a=getUserCache');
				if (res.data.error == 0) {
					let data = res.data.data;

                    var userSexArr = data.userSexArr;
                    userSexArr.forEach((item) => {
                        this.sexOptions.push({value: item.id, label: item.name})
                    });

                    var userMarriageArr = data.userMarriageArr;
                    userMarriageArr.forEach((item) => {
                        this.marriageOptions.push({value: item.id, label: item.name})
                    });

                    var userEduArr = data.userEduArr;
                    userEduArr.forEach((item) => {
                        this.eduOptions.push({value: item.id, label: item.name})
                    });
                    var userExpArr = data.userExpArr;
                    userExpArr.forEach((item) => {
                        this.expOptions.push({value: item.id, label: item.name})
                    });
				}
			},
            submitLocoyConfig: function () {
                let that = this;
                let params = {
                    locoyConfig: 1,

                    locoy_user_sexs: that.locoy_config.locoy_user_sexs,
                    locoy_user_marriage: that.locoy_config.locoy_user_marriage,
                    locoy_user_edu: that.locoy_config.locoy_user_edu,
                    locoy_user_exp: that.locoy_config.locoy_user_exp,
                    locoy_user_nationality: that.locoy_config.locoy_user_nationality
                };
                that.saveLoading = true;
                httpPost('m=tool&c=dataCollection&a=setLocoyConfig', params).then(function (res) {
                    if (res.data.error == 0) {

                        message.success(res.data.msg);
                    } else {

                        message.error(res.data.msg);
                    }
                }).finally(function () {
                    setTimeout(function () {
                        that.saveLoading = false;
                    }, 2000);
                });
            },
		},
	};
</script>
<style scoped>
    .moduleTable {
        max-height: calc(100% - (60px + 10px));
    }
</style>