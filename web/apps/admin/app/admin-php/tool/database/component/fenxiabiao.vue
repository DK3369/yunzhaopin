<template>
    <div class="moduleElHight">
        <div class="fenxiabHeadr">
            <div class="fenxiabHeadTite">
                <span>{{ lc('admin_tool_00225') }}</span>
            </div>
            <div class="fenxiabHeadFrom">
                <div class="fenxiabHeadFomList" style="padding-right: 12px;">
                    <el-select v-model="searchType" size="small" :placeholder="lc('wap_user_00100')" @change="changeType">
                        <el-option v-for="item in options" :key="item.value" :label="item.label" :value="item.value">
                        </el-option>
                    </el-select>
                </div>
                <div class="fenxiabHeadFomList">
                    <el-date-picker v-model="startTime" size="small" :value-format="valueFormat" :type="type" :placeholder="lc('admin_tool_00230')">
                    </el-date-picker>
                </div>
                <div class="fenxiabHeadTexts">
                    <span>VS</span>
                </div>
                <div class="fenxiabHeadFomList">
                    <el-date-picker v-model="endTime"  size="small" :type="type" :value-format="valueFormat"  :placeholder="lc('admin_tool_00230')">
                    </el-date-picker>
                </div>
                <div class="fenxiabHeadFomList" style="padding-left: 12px;">
                    <el-button icon="el-icon-search" size="small" type="primary" @click="search">{{ lc('admin_tool_00224') }}</el-button>
                </div>
            </div>
        </div>
        <div class="fenxiabConts">
            <div class="fenxiabConTable">
                <el-table :data="tableData" border stripe :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
                    style="width: 100%">
                    <el-table-column prop="years" :label="lc('admin_tool_00228')">
                    </el-table-column>
                    <el-table-column prop="gerezce" :label="lc('admin_00073')"></el-table-column>
                    <el-table-column prop="login_log" :label="lc('admin_tool_00223')"></el-table-column>
                    <el-table-column prop="jilizce" :label="lc('admin_tool_00229')">
                    </el-table-column>
                    <el-table-column prop="comzce" :label="lc('admin_00074')"></el-table-column>
                    <el-table-column prop="company_login_log" :label="lc('admin_user_00335')"></el-table-column>
                    <el-table-column prop="fabuzhw" :label="lc('wap_00322')">
                    </el-table-column>
                    <el-table-column prop="jilitod" :label="lc('member_com_00152')">
                    </el-table-column>
                    <el-table-column prop="yaoqms" :label="lc('resume_00029')">
                    </el-table-column>
                    <el-table-column prop="jilixza" :label="lc('wap_com_00042')">
                    </el-table-column>
                </el-table>
            </div>
            <div class="fenxiabConTable">
                <el-table :data="tableData2" border stripe :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" :cell-style="addClass" >
                    style="width: 100%">
                    <el-table-column prop="years" :label="lc('admin_tool_00228')">
                    </el-table-column>
                    <el-table-column :label="lc('admin_00073')">
                        <template #default="scope">
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
                    <el-table-column :label="lc('admin_tool_00223')">
                        <template #default="scope">
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
                    <el-table-column :label="lc('admin_tool_00229')">
                        <template #default="scope">
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
                    <el-table-column :label="lc('admin_00074')">
                        <template #default="scope">
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
                    <el-table-column :label="lc('admin_user_00335')">
                        <template #default="scope">
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
                    <el-table-column :label="lc('wap_00322')">
                        <template #default="scope">
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
                    <el-table-column :label="lc('member_com_00152')">
                        <template #default="scope">
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
                    <el-table-column :label="lc('resume_00029')">
                        <template #default="scope">
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
                    <el-table-column :label="lc('wap_com_00042')">
                        <template #default="scope">
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
                label: window.yunAdminT(lc('admin_tool_00227'))
            }, {
                value: '2',
                label: window.yunAdminT(lc('admin_tool_00226'))
            }],
            value: '',
            statistics : ['gerezce','login_log','jilizce','comzce','company_login_log','fabuzhw', 'jilitod', 'liaotan', 'yaoqms', 'jilixza']
        }
    },

    mounted() {
    },
    methods: {
        addClass:function({row,rowIndex,columnIndex}){
            if (row.years == lc('member_com_00348') && columnIndex !=0){
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
                message.error(window.yunAdminT(lc('admin_tool_00222')));return;
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