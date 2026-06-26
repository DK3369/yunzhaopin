<template>
    <div class="moduleElHight">
        <div class="fenxiabHeadr">
            <div class="fenxiabHeadTite">
                <span>{yun:}t key='admin_tool_00225'{/yun}</span>
            </div>
            <div class="fenxiabHeadFrom">
                <div class="fenxiabHeadFomList" style="padding-right: 12px;">
                    <el-select v-model="searchType" size="small" placeholder="{yun:}t key='wap_user_00100'{/yun}" @change="changeType">
                        <el-option v-for="item in options" :key="item.value" :label="item.label" :value="item.value">
                        </el-option>
                    </el-select>
                </div>
                <div class="fenxiabHeadFomList">
                    <el-date-picker v-model="startTime" size="small" :value-format="valueFormat" :type="type" placeholder="{yun:}t key='admin_tool_00230'{/yun}">
                    </el-date-picker>
                </div>
                <div class="fenxiabHeadTexts">
                    <span>VS</span>
                </div>
                <div class="fenxiabHeadFomList">
                    <el-date-picker v-model="endTime"  size="small" :type="type" :value-format="valueFormat"  placeholder="{yun:}t key='admin_tool_00230'{/yun}">
                    </el-date-picker>
                </div>
                <div class="fenxiabHeadFomList" style="padding-left: 12px;">
                    <el-button icon="el-icon-search" size="mini" type="primary" @click="search">{yun:}t key='admin_tool_00224'{/yun}</el-button>
                </div>
            </div>
        </div>
        <div class="fenxiabConts">
            <div class="fenxiabConTable">
                <el-table :data="tableData" border stripe :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
                    style="width: 100%">
                    <el-table-column prop="years" label="{yun:}t key='admin_tool_00228'{/yun}">
                    </el-table-column>
                    <el-table-column prop="gerezce" label="{yun:}t key='admin_00073'{/yun}"></el-table-column>
                    <el-table-column prop="login_log" label="{yun:}t key='admin_tool_00223'{/yun}"></el-table-column>
                    <el-table-column prop="jilizce" label="{yun:}t key='admin_tool_00229'{/yun}">
                    </el-table-column>
                    <el-table-column prop="comzce" label="{yun:}t key='admin_00074'{/yun}"></el-table-column>
                    <el-table-column prop="company_login_log" label="{yun:}t key='admin_user_00335'{/yun}"></el-table-column>
                    <el-table-column prop="fabuzhw" label="{yun:}t key='wap_00322'{/yun}">
                    </el-table-column>
                    <el-table-column prop="jilitod" label="{yun:}t key='member_com_00152'{/yun}">
                    </el-table-column>
                    <el-table-column prop="yaoqms" label="{yun:}t key='resume_00029'{/yun}">
                    </el-table-column>
                    <el-table-column prop="jilixza" label="{yun:}t key='wap_com_00042'{/yun}">
                    </el-table-column>
                </el-table>
            </div>
            <div class="fenxiabConTable">
                <el-table :data="tableData2" border stripe :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" :cell-style="addClass" >
                    style="width: 100%">
                    <el-table-column prop="years" label="{yun:}t key='admin_tool_00228'{/yun}">
                    </el-table-column>
                    <el-table-column label="{yun:}t key='admin_00073'{/yun}">
                        <template slot-scope="scope">
                            <div class="moduleFenxias">
                                <div class="moduleFenxText">
                                    <span>{{ scope.row.gerezce }}</span>
                                </div>
                                <div class="moduleFenxIcon" v-if="scope.row.gerezce_percent<'0' &&  scope.row.gerezce !='0'" style="color: #22C77A;">
                                    <i class="el-icon-bottom"></i>
                                    <span>{{scope.row.gerezce_percent}}%</span>
                                </div>
                                <div class="moduleFenxIcon" v-else-if="scope.row.gerezce_percent=='0' || scope.row.gerezce =='0'|| scope.row.gerezce == 0 ">
                                    <span>-</span>
                                </div>
                                <div class="moduleFenxIcon" v-else  style="color: #FD0001;">
                                    <i class="el-icon-top"></i>
                                    <span>{{scope.row.gerezce_percent}}%</span>
                                </div>
                            </div>
                        </template>
                    </el-table-column>
                    <el-table-column label="{yun:}t key='admin_tool_00223'{/yun}">
                        <template slot-scope="scope">
                            <div class="moduleFenxias">
                                <div class="moduleFenxText">
                                    <span>{{ scope.row.login_log }}</span>
                                </div>
                                <div class="moduleFenxIcon" v-if="scope.row.login_log_percent<'0' &&  scope.row.login_log !='0'" style="color: #22C77A;">
                                    <i class="el-icon-bottom"></i>
                                    <span>{{scope.row.login_log_percent}}%</span>
                                </div>
                                <div class="moduleFenxIcon" v-else-if="scope.row.login_log_percent=='0' || scope.row.login_log =='0'|| scope.row.login_log == 0 ">
                                    <span>-</span>
                                </div>
                                <div class="moduleFenxIcon" v-else  style="color: #FD0001;">
                                    <i class="el-icon-top"></i>
                                    <span>{{scope.row.login_log_percent}}%</span>
                                </div>
                            </div>
                        </template>
                    </el-table-column>
                    <el-table-column label="{yun:}t key='admin_tool_00229'{/yun}">
                        <template slot-scope="scope">
                            <div class="moduleFenxias">
                                <div class="moduleFenxText">
                                    <span>{{ scope.row.jilizce }}</span>
                                </div>
                                <div class="moduleFenxIcon" v-if="scope.row.jilizce_percent<'0' &&  scope.row.jilizce !='0'" style="color: #22C77A;">
                                    <i class="el-icon-bottom"></i>
                                    <span>{{scope.row.jilizce_percent}}%</span>
                                </div>
                                <div class="moduleFenxIcon" v-else-if="scope.row.jilizce_percent=='0' || scope.row.jilizce == '0' || scope.row.jilizce == 0">
                                    <span>-</span>
                                </div>
                                <div class="moduleFenxIcon" v-else style="color: #FD0001;">
                                    <i class="el-icon-top"></i>
                                    <span>{{scope.row.jilizce_percent}}%</span>
                                </div>
                            </div>
                        </template>
                    </el-table-column>
                    <el-table-column label="{yun:}t key='admin_00074'{/yun}">
                        <template slot-scope="scope">
                            <div class="moduleFenxias">
                                <div class="moduleFenxText">
                                    <span>{{ scope.row.comzce }}</span>
                                </div>
                                <div class="moduleFenxIcon" v-if="scope.row.comzce_percent<'0' &&  scope.row.comzce !='0'" style="color: #22C77A;">
                                    <i class="el-icon-bottom"></i>
                                    <span>{{scope.row.comzce_percent}}%</span>
                                </div>
                                <div class="moduleFenxIcon" v-else-if="scope.row.comzce_percent=='0' || scope.row.comzce == '0' || scope.row.comzce == 0">
                                    <span>-</span>
                                </div>
                                <div class="moduleFenxIcon" v-else  style="color: #FD0001;">
                                    <i class="el-icon-top"></i>
                                    <span>{{scope.row.comzce_percent}}%</span>
                                </div>
                            </div>
                        </template>
                    </el-table-column>
                    <el-table-column label="{yun:}t key='admin_user_00335'{/yun}">
                        <template slot-scope="scope">
                            <div class="moduleFenxias">
                                <div class="moduleFenxText">
                                    <span>{{ scope.row.company_login_log }}</span>
                                </div>
                                <div class="moduleFenxIcon" v-if="scope.row.company_login_log_percent<'0' &&  scope.row.company_login_log !='0'" style="color: #22C77A;">
                                    <i class="el-icon-bottom"></i>
                                    <span>{{scope.row.login_log_percent}}%</span>
                                </div>
                                <div class="moduleFenxIcon" v-else-if="scope.row.company_login_log_percent=='0' || scope.row.company_login_log =='0'|| scope.row.company_login_log == 0 ">
                                    <span>-</span>
                                </div>
                                <div class="moduleFenxIcon" v-else  style="color: #FD0001;">
                                    <i class="el-icon-top"></i>
                                    <span>{{scope.row.company_login_log_percent}}%</span>
                                </div>
                            </div>
                        </template>
                    </el-table-column>
                    <el-table-column label="{yun:}t key='wap_00322'{/yun}">
                        <template slot-scope="scope">
                            <div class="moduleFenxias">
                                <div class="moduleFenxText">
                                    <span>{{ scope.row.fabuzhw }}</span>
                                </div>

                                <div class="moduleFenxIcon" v-if="scope.row.fabuzhw_percent<'0'  &&  scope.row.fabuzhw !='0'" style="color: #22C77A;">
                                    <i class="el-icon-bottom"></i>
                                    <span>{{scope.row.fabuzhw_percent}}%</span>
                                </div>
                                <div class="moduleFenxIcon" v-else-if="scope.row.fabuzhw_percent=='0' || scope.row.fabuzhw == '0' || scope.row.fabuzhw == 0">
                                    <span>-</span>
                                </div>
                                <div class="moduleFenxIcon" v-else="scope.row.fabuzhw>'0'"  style="color: #FD0001;">
                                    <i class="el-icon-top"></i>
                                    <span>{{scope.row.fabuzhw_percent}}%</span>
                                </div>
                            </div>
                        </template>
                    </el-table-column>
                    <el-table-column label="{yun:}t key='member_com_00152'{/yun}">
                        <template slot-scope="scope">
                            <div class="moduleFenxias">
                                <div class="moduleFenxText">
                                    <span>{{ scope.row.jilitod }}</span>
                                </div>

                                <div class="moduleFenxIcon" v-if="scope.row.jilitod_percent<'0' &&  scope.row.jilitod !='0'" style="color: #22C77A;">
                                    <i class="el-icon-bottom"></i>
                                    <span>{{scope.row.jilitod_percent}}%</span>
                                </div>
                                <div class="moduleFenxIcon" v-else-if="scope.row.jilitod_percent=='0' || scope.row.jilitod == '0' || scope.row.jilitod == 0">
                                    <span>-</span>
                                </div>
                                <div class="moduleFenxIcon" v-else  style="color: #FD0001;">
                                    <i class="el-icon-top"></i>
                                    <span>{{scope.row.jilitod_percent}}%</span>
                                </div>
                            </div>
                        </template>
                    </el-table-column>
                    <el-table-column label="{yun:}t key='resume_00029'{/yun}">
                        <template slot-scope="scope">
                            <div class="moduleFenxias">
                                <div class="moduleFenxText">
                                    <span>{{ scope.row.yaoqms }}</span>
                                </div>
                                <div class="moduleFenxIcon" v-if="scope.row.yaoqms_percent<'0' && scope.row.yaoqms !='0'" style="color: #22C77A;">
                                    <i class="el-icon-bottom"></i>
                                    <span>{{scope.row.yaoqms_percent}}%</span>
                                </div>
                                <div class="moduleFenxIcon" v-else-if="scope.row.yaoqms_percent=='0' ||  scope.row.yaoqms=='0'|| scope.row.yaoqms==0">
                                    <span>-</span>
                                </div>
                                <div class="moduleFenxIcon" v-else style="color: #FD0001;">
                                    <i class="el-icon-top"></i>
                                    <span>{{scope.row.yaoqms_percent}}%</span>
                                </div>
                            </div>
                        </template>
                    </el-table-column>
                    <el-table-column label="{yun:}t key='wap_com_00042'{/yun}">
                        <template slot-scope="scope">
                            <div class="moduleFenxias">
                                <div class="moduleFenxText">
                                    <span>{{ scope.row.jilixza }}</span>
                                </div>
                                <div class="moduleFenxIcon" v-if="scope.row.jilixza_percent<'0' && scope.row.jilixza !='0'" style="color: #22C77A;">
                                    <i class="el-icon-bottom"></i>
                                    <span>{{scope.row.jilixza_percent}}%</span>
                                </div>
                                <div class="moduleFenxIcon" v-else-if="scope.row.jilixza_percent=='0' ||  scope.row.jilixza=='0'|| scope.row.jilixza==0">
                                    <span>-</span>
                                </div>
                                <div class="moduleFenxIcon" v-else style="color: #FD0001;">
                                    <i class="el-icon-top"></i>
                                    <span>{{scope.row.jilixza_percent}}%</span>
                                </div>
                            </div>
                        </template>
                    </el-table-column>
                </el-table>
            </div>
        </div>
    </div>
</template>

<script>
module.exports = {
    data: function () {
        return {
            startTime: '',
            endTime: '',
            type:'month',
            valueFormat:'yyyy-MM',
            searchType:'2',
            tableData: [],
            tableData2: [],
            options: [{
                value: '1',
                label: window.yunAdminT("{yun:}t key='admin_tool_00227'{/yun}")
            }, {
                value: '2',
                label: window.yunAdminT("{yun:}t key='admin_tool_00226'{/yun}")
            }],
            value: '',
            statistics : ['gerezce','login_log','jilizce','comzce','company_login_log','fabuzhw', 'jilitod', 'liaotan', 'yaoqms', 'jilixza']
        }
    },

    mounted() {
    },
    methods: {
        addClass:function({row,rowIndex,columnIndex}){
            if (row.years == "{yun:}t key='member_com_00348'{/yun}" && columnIndex !=0){
                var i = columnIndex -1;
                let key = this.statistics[i];
                if (row[key+"_percent"]<'0' && row[key]!='0'){
                    return 'background: #E9FFF5';
                }else if (row[key+"_percent"] =='0' || row[key] =='0' || row[key] ==0 ){

                }else{
                    return 'background: #FFEEEE';
                }
            }
        },
        changeType:function(){

            this.startTime = '';
            this.endTime = '';
           if (this.searchType == '1'){
               this.$nextTick(function () {
                   this.type ='year';
                   this.valueFormat='yyyy';
               })
           }  else if (this.searchType == '2'){
               this.$nextTick(function () {
                   this.type ='month';
                   this.valueFormat='yyyy-MM';
               })
           }
        },
        search:function(){
                let that = this;
                let params = {};
                params.type = this.searchType;
                params.startTime = this.startTime
            if (!this.startTime || !this.endTime){
                message.error(window.yunAdminT("{yun:}t key='admin_tool_00222'{/yun}"));return;
            }
                params.endTime = this.endTime
                httpPost('m=tool&c=dataBoard&a=fenxiabiao', params).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        that.tableData = res.data.firstResult;
                        that.tableData2 = res.data.secondResult;
                    }
                }).catch(function (error) {
                    console.log(error);
                })

        }

    }
};
</script>
<style scoped></style>