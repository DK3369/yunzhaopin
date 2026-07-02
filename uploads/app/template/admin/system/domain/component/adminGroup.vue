<template>
    <div class="tableDome" style="top: 40px; width: initial;">
        <div class="tableDome_tip">
            <el-alert :title="lc('admin_01033')" type="info" :closable="false"></el-alert>
        </div>
        <div class="moduleTable" style="height: calc(100% - 80px); max-height: initial;">
            <div style="overflow-y: auto; position: relative; width: 100%; height: calc(100% - 80px);">
                <table class="quanxiantable">
                    <tbody>
                        <tr>
                            <td bgcolor="#F5F7FA">{{ lc('admin_system_00212') }}</td>
                            <td colspan="2">
                                <div class="quanxiantext">
                                    <div class="quanxiantext_in">
                                        <el-input v-model="groupInfo.group_name" :placeholder="lc('admin_01034')"
                                            size="small"></el-input>
                                    </div>
                                    <div class="quanxiantext_in">
                                        <el-select v-model="groupInfo.did" :placeholder="lc('admin_system_00143')" size="small">
                                            <el-option v-for="item in domainOptionS" :key="item.value" :label="item.label"
                                                :value="item.value"></el-option>
                                        </el-select>
                                    </div>
                                </div>
                            </td>
                        </tr>
                        <tr align="left">
                            <th width="150" bgcolor="#F5F7FA">{{ lc('admin_user_company_00362') }}</th>
                            <th width="150" bgcolor="#F5F7FA">{{ lc('admin_user_company_00364') }}</th>
                            <th bgcolor="#F5F7FA">{{ lc('admin_system_00214') }}</th>
                        </tr>
                        <tr>
                            <td valign="top">
                                <div class="jsnavbox">
                                    <div v-for="(item, index) in navigation" :key="index"
                                        :class="item.id == checkedOne ? 'jsnavlist jsnavlist_cur' : 'jsnavlist'"
                                        @click="handleOneChange(item.id)">{{ item.name }}<i
                                            class="jsnavlisticon el-icon-arrow-right"></i></div>
                                </div>
                            </td>
                            <td valign="top">
                                <div class="jsnavbox">
                                    <div v-for="(item2, index2) in oneMenu[checkedOne]" :key="index2"
                                        :class="item2.id == checkedTwo ? 'jsnavlist jsnavlist_cur' : 'jsnavlist'"
                                        @click="handleTwoChange(item2.id)">{{ item2.name }}<i
                                            class="jsnavlisticon el-icon-arrow-right"></i></div>
                                </div>
                            </td>
                            <td valign="top" style="padding:0px;">
                                <table class="tableqx">
                                    <tr v-for="(item3, index3) in twoMenu[checkedTwo]" :key="index3">
                                        <td width="150" class="tableqxtd" @click="currChecked(item3.id)">
                                            <el-checkbox-group v-model="checkedThreeIds" @change="handleCheckedThreeChange">
                                                <el-checkbox :label="item3.id">{{ item3.name }}</el-checkbox>
                                            </el-checkbox-group>
                                        </td>
                                        <td>
                                            <el-checkbox-group v-model="checkedFourIds">
                                                <el-checkbox v-for="(item4, index4) in threeMenu[item3.id]" :key="index4"
                                                    :label="item4.id">{{
                                                        item4.name }}</el-checkbox>
                                            </el-checkbox-group>
                                        </td>
                                    </tr>
                                </table>
                            </td>
                        </tr><!--  -->
                    </tbody>
                </table>
            </div>
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" size="medium" @click="saveGroup" :loading="saveLoading">{{ lc('common.submit') }}</el-button>
            </div>
        </div>
    </div>
</template>
<script>
module.exports = {
    props: {
        group_id: Number
    },
    watch: {
        group_id: {
            handler(newValue, oldValue) {
                this.getGroupInfo();
            },
            deep: true,
            immediate: true
        }
    },
    data: function () {
        return {
            groupInfo: {
                group_name: ''
            },

            domainOptionS: [],

            power: [],

            navigation: [],
            oneMenu: [],
            twoMenu: [],
            threeMenu: [],

            checkedOne: 0,
            checkedTwo: 0,
            checkedThreeIds: [],
            checkedFourIds: [],

            oneChildrenIds: [],
            twoChildrenIds: [],

            currCheckedId: 0,
            saveLoading: false
        }
    },
    methods: {
        resetData: function () {

            this.groupInfo = {};
            this.domainOptionS = [];

            this.power = [];

            this.navigation = [];
            this.oneMenu = [];
            this.twoMenu = [];
            this.threeMenu = [];

            this.checkedOne = 0;
            this.checkedTwo = 0;
            this.checkedThreeIds = [];
            this.checkedFourIds = [];

            this.oneChildrenIds = [];
            this.twoChildrenIds = [];

            this.currCheckedId = 0;
        },
        getGroupInfo: function () {
            var self = this;
            httpPost('m=system&c=domain_group&a=groupInfo', { id: this.group_id }).then(function (res) {
                if (res.data.error == 0) {
                    let data = res.data.data;

                    self.resetData();

                    self.navigation = data.navigation;
                    self.oneMenu = data.one_menu;
                    self.twoMenu = data.two_menu;
                    self.threeMenu = data.three_menu;

                    self.checkedOne = parseInt(self.navigation[0].id);
                    self.checkedTwo = parseInt(self.oneMenu[self.checkedOne][0].id);
                    self.checkedThreeIds = data.checked_three_ids;
                    self.checkedFourIds = data.checked_four_ids;

                    self.oneChildrenIds = data.one_children_ids;
                    self.twoChildrenIds = data.two_children_ids;

                    self.groupInfo = data.groupInfo;

                    data.domain.forEach(item => {
                        self.domainOptionS.push({ value: item.id, label: item.title });
                    });

                }
            });
        },
        handleOneChange(id) {
            var that = this
            that.checkedOne = parseInt(id);
            that.checkedTwo = parseInt(that.oneMenu[that.checkedOne][0].id);
        },
        handleTwoChange(id) {
            var that = this
            that.checkedTwo = parseInt(id);
        },
        currChecked(id) {
            this.currCheckedId = id;
        },
        handleCheckedThreeChange(val) {
            var that = this
            if (that.checkedThreeIds.includes(that.currCheckedId)) {// {{ lc('admin_user_company_00028') }}
                if (that.threeMenu[that.currCheckedId]) {
                    that.threeMenu[that.currCheckedId].forEach(function (item, index) {
                        if (!that.checkedFourIds.includes(item.id)) {
                            that.checkedFourIds.push(item.id)
                        }
                    })
                }
            } else {// {{ lc('common.cancel') }}
                if (that.threeMenu[that.currCheckedId]) {
                    that.threeMenu[that.currCheckedId].forEach(function (id_item, index) {
                        if (that.checkedFourIds.includes(id_item.id)) {
                            that.checkedFourIds = that.checkedFourIds.filter(item => item != id_item.id)
                        }
                    })
                }
            }
        },
        saveGroup: function () {
            let that = this;
            let checkedTwoIds = [], checkedOneIds = [];
            // Build submitted second-level menu ids from selected third-level menu ids
            if (that.checkedThreeIds.length) {
                that.checkedThreeIds.forEach(function (ck_three_id) {
                    for (parentId in that.twoChildrenIds) {
                        // Add the parent second-level menu id when a selected third-level menu has no selected parent yet
                        if (that.twoChildrenIds[parentId].includes(ck_three_id) && !checkedTwoIds.includes(parentId)) {
                            checkedTwoIds.push(parentId)
                        }
                    }
                })
            }
            // Build submitted first-level menu ids from selected second-level menu ids
            if (checkedTwoIds.length) {
                checkedTwoIds.forEach(function (ck_two_id) {
                    for (parentId in that.oneChildrenIds) {
                        // Add the parent first-level menu id when a selected second-level menu has no selected parent yet
                        if (that.oneChildrenIds[parentId].includes(ck_two_id) && !checkedOneIds.includes(parentId)) {
                            checkedOneIds.push(parentId)
                        }
                    }
                })
            }
            let params = {
                group_name: that.groupInfo.group_name,
                did: that.groupInfo.did,
                one_ids: checkedOneIds,
                two_ids: checkedTwoIds,
                three_ids: that.checkedThreeIds,
                four_ids: that.checkedFourIds
            };
            if (that.group_id) {
                params.groupid = that.group_id;
            }
            that.saveLoading = true;
            httpPost('m=system&c=domain_group&a=saveGroup', params).then(function (res) {
                if (res.data.error == 0) {
                    message.success(res.data.msg, function () {
                        that.$emit("child-event");
                    });
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