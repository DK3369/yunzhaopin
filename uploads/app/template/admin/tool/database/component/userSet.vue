<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert title="{yun:}t key='admin_tool_00059'{/yun}" type="success" :closable="false"></el-alert>
        </div>
        <div class=" moduleTable">

            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="200">{yun:}t key='member_com_00021'{/yun}</th>
                    <th width="400">{yun:}t key='member_user_00181'{/yun}</th>
                    <th>{yun:}t key='member_com_00207'{/yun}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_tool_00269'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_user_sexs" placeholder="{yun:}t key='wap_user_00100'{/yun}">
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
                        <div class="TableTite">{yun:}t key='admin_tool_00270'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_user_marriage" placeholder="{yun:}t key='wap_user_00100'{/yun}">
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
                        <div class="TableTite">{yun:}t key='admin_tool_00271'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_user_edu" placeholder="{yun:}t key='wap_user_00100'{/yun}">
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
                        <div class="TableTite">{yun:}t key='admin_tool_00272'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_user_exp" placeholder="{yun:}t key='wap_user_00100'{/yun}">
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
                        <div class="TableTite">{yun:}t key='admin_tool_00273'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_user_nationality" placeholder=" "></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_tool_00274'{/yun}</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" size="medium" @click="submitLocoyConfig" :disabled="saveLoading">{yun:}t key='common.submit'{/yun}</el-button>
            </div>
        </div>
    </div>
</template>

<script>
	module.exports = {
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