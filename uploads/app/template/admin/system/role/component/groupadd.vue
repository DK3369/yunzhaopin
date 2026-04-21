<template>
	<div class="tableDome" style="position: relative;">
		<div class="tableDome_tip">
			<el-alert title="管理员根据网站运营需求，添加不同类型的管理员！管理员类型分为：“CRM、超级管理员、分站管理员”用户权限组成相关设置，超级管理员可以根据运营需求设置"
				type="success" :closable="false">
			</el-alert>
		</div>
		<div class="moduleTable" style="max-height: calc(100% - 130px);">
			<table class="quanxiantable" v-if="islook">
				<tbody>
					<tr>
						<td bgcolor="#F5F7FA">用户组名称</td>
						<td colspan="2">
							<div class="quanxiantext">
								<div class="quanxiantext_in">
									<el-input v-model="groupinfo.group_name" placeholder="请输入名称" size="small"></el-input>
								</div>
								<div class="quanxiantip">
									<span>名称如：技术部 ，财务部，销售部等</span>
								</div>
							</div>
						</td>
					</tr>
					<tr align="left">
						<th width="150" bgcolor="#F5F7FA">一级类别</th>
						<th width="150"  bgcolor="#F5F7FA">二级类别</th>
						<th  bgcolor="#F5F7FA">功能管理权限</th>
					</tr>
					<tr>
						<td valign="top">
							<div class="jsnavbox">
								<div v-for="(item,index) in navs" :key="index" :class="item.id == checked_one ? 'jsnavlist jsnavlist_cur' : 'jsnavlist'" @click="handleOneChange(item.id)">{{item.name}}<i class="jsnavlisticon el-icon-arrow-right"></i></div>
							</div>
						</td>
						<td valign="top">
							<div class="jsnavbox">
								<div v-for="(item2,index2) in one[checked_one]" :key="index2" :class="item2.id == checked_two ? 'jsnavlist jsnavlist_cur' : 'jsnavlist'" @click="handleTwoChange(item2.id)">{{item2.name}}<i class="jsnavlisticon el-icon-arrow-right"></i></div>
							</div>
						</td>
						<td valign="top" style="padding:0px;">
							<table class="tableqx">
								<tr v-for="(item3,index3) in two[checked_two]" :key="index3">
									<td width="150" class="tableqxtd" @click="currCheckedId(item3.id)">
										<el-checkbox-group v-model="checked_three_ids" @change="handleCheckedThreeChange">
											<el-checkbox :label="item3.id">{{item3.name}}</el-checkbox>
										</el-checkbox-group>
									</td>
									<td>
										<el-checkbox-group v-model="checked_four_ids">
											<el-checkbox v-for="(item4,index4) in three[item3.id]" :key="index4" :label="item4.id">{{item4.name}}</el-checkbox>
										</el-checkbox-group>
									</td>
								</tr>
							</table>
						</td>
					</tr>
				</tbody>
			</table>
			
		</div>
		<div class="setBasicButn" style="border: none;">
			<el-button type="primary" size="medium" @click="submitForm" :disabled="saveLoading">提交</el-button>
		</div>
	</div>
</template>
<script>
	module.exports = {
	    props:{
	        id_v: {
				type: String,
				default: ''
			}
	    },
		data: function() {
			return {
				id: '',
				navs: [],
				one: [],
				two: [],
				three: [],
				groupinfo: {group_name: ''},
				power: [],
				checked: true,
				islook: false,
				checked_one: 0,
				checked_two: 0,
				checked_three_ids: [],
				checked_four_ids: [],
				one_children_ids: [],
				two_children_ids: [],
				curr_checked_id: 0,
				saveLoading:false
			}
		},
		watch: {
			id_v: {
				handler(val) {
					this.id = val;
					this.getInfo();
				},
				immediate: true,
				deep: true,
			},
		},
		methods: {
			handleOneChange(id){
				var that = this
				that.checked_one = parseInt(id)
				that.checked_two = parseInt(that.one[that.checked_one][0].id)
			},
			handleTwoChange(id){
				var that = this
				that.checked_two = parseInt(id)
			},
			currCheckedId(id){
				this.curr_checked_id = id
			},
			handleCheckedThreeChange(val) {
				var that = this
				if (that.checked_three_ids.includes(that.curr_checked_id)) {// 新增
					if (that.three[that.curr_checked_id]) {
						that.three[that.curr_checked_id].forEach(function(item, index) {
							if (!that.checked_four_ids.includes(item.id)) {
								that.checked_four_ids.push(item.id)
							}
						})
					}
				} else {// 取消
					if (that.three[that.curr_checked_id]) {
						that.three[that.curr_checked_id].forEach(function(id_item, index) {
							if (that.checked_four_ids.includes(id_item.id)) {
								that.checked_four_ids = that.checked_four_ids.filter(item => item != id_item.id)
							}
						})
					}
				}
			},
			async getInfo() {
				var that = this
				let res = await httpPost('m=system&c=role_ugroup&a=info', { id: this.id });
				if (res.data.error == 0) {
					let data = res.data.data;
					that.navs = data.navigation
					that.one = data.one_menu
					that.two = data.two_menu
					that.three = data.three_menu
					that.groupinfo = data.admin_group
					that.power = data.power
					that.checked_one = parseInt(that.navs[0].id)
					that.checked_two = parseInt(that.one[that.checked_one][0].id)
					that.checked_three_ids = data.checked_three_ids
					that.checked_four_ids = data.checked_four_ids
					that.one_children_ids = data.one_children_ids,
					that.two_children_ids = data.two_children_ids,
					that.islook = true
				}
			},
			submitForm() {
				let that = this;
				let checked_two_ids = [],checked_one_ids = [];
				// 根据选中的三级菜单id来获取需要选中提交的二级菜单id
				if (that.checked_three_ids.length) {
					that.checked_three_ids.forEach(function(ck_three_id){
						for(parentid in that.two_children_ids){
							// 三级菜单被选中且对应的二级菜单未被选中，将二级菜单id添加到需要选中提交的二级菜单id数组中
							if (that.two_children_ids[parentid].includes(ck_three_id) && !checked_two_ids.includes(parentid)) {
								checked_two_ids.push(parentid)
							}
						}
					})
				}
				// 根据选中的二级菜单id来获取需要选中提交的一级菜单id
				if (checked_two_ids.length) {
					checked_two_ids.forEach(function(ck_two_id){
						for(parentid in that.one_children_ids){
							// 二级菜单被选中且对应的一级菜单未被选中，将一级菜单id添加到需要选中提交的一级菜单id数组中
							if (that.one_children_ids[parentid].includes(ck_two_id) && !checked_one_ids.includes(parentid)) {
								checked_one_ids.push(parentid)
							}
						}
					})
				}
				let params = {
					group_name: that.groupinfo.group_name,
					one_ids: checked_one_ids,
					two_ids: checked_two_ids,
					three_ids: that.checked_three_ids,
					four_ids: that.checked_four_ids
				};
				if (that.id) {
					params.groupid = that.id;
				}
				that.saveLoading = true;
				httpPost('m=system&c=role_ugroup&a=save', params).then(function (res) {
					if (res.data.error == 0) {
						message.success(res.data.msg, function () {
                            that.$emit("child-event");
                        })
					} else {
						message.error(res.data.msg);
					}
				}).finally(function () {
                    setTimeout(function () {
                        that.saveLoading = false;
                    }, 2000);
                });
			},
		}
	};
</script>
