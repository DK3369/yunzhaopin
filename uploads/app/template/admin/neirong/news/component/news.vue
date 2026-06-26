<template>
    <div style="height: 100%;">
        <div class="modulemoreSeachs">
            <div class="moduleSeachleft">
                <div class="moduleInptList" style="margin-bottom: 8px;">
                    <el-input size="small" placeholder="{yun:}t key='admin_00340'{/yun}" v-model="keyword" class="input-with-select" clearable>
                        <el-select v-model="type" slot="prepend" placeholder="{yun:}t key='wap_user_00103'{/yun}">
                            <el-option label="{yun:}t key='wap_user_00103'{/yun}" value="1"></el-option>
                            <el-option label="{yun:}t key='admin_00172'{/yun}" value="2"></el-option>
                        </el-select>
                    </el-input>
                </div>
                <div class="tableSeachInpt tableSeachInptsmall">
                    <el-select size="small" style="margin-right: 0;" v-model="publish" slot="prepend" placeholder="{yun:}t key='admin_user_weipin_00030'{/yun}" clearable @change="search">
                        <el-option label="{yun:}t key='common_01940'{/yun}" value="1"></el-option>
                        <el-option label="{yun:}t key='admin_user_00179'{/yun}" value="3"></el-option>
                        <el-option label="{yun:}t key='admin_user_00178'{/yun}" value="7"></el-option>
                        <el-option label="{yun:}t key='admin_user_00180'{/yun}" value="15"></el-option>
                        <el-option label="{yun:}t key='admin_user_00175'{/yun}" value="30"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt tableSeachInpElect">
                    <el-cascader size="small" v-model="cate" :options="class_cascader" :props="{ checkStrictly: true }"
                        :show-all-levels="false" placeholder="{yun:}t key='admin_00170'{/yun}" clearable>
                    </el-cascader>
                </div>
                <div class="tableSeachInpt">
                    <el-button type="primary" icon="el-icon-search" size="mini" @click="search">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
                </div>
            </div>
            <div class="moduleSeachButn">
                <div class="tableSeachInpt">
                    <div class="new_bth" style="height: 28px; line-height: 28px;">
                        <el-link :underline="false" icon="el-icon-document-add" @click="add({id: ''})">{yun:}t key='admin_00794'{/yun}</el-link>
                    </div>
                </div>
            </div>
        </div>
        <div class="moduleElTable" style="height: calc(100% - (60px + 50px + 20px + 12px));">
            <el-table :data="tableData" stripe border style="width: 100%;" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" @selection-change="handleSelectionChange" ref="multipleTable" :empty-text="emptytext" :default-sort="{ prop: 'id', order: 'descending' }" @sort-change='sortChange' v-loading="loading">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" label="{yun:}t key='admin_00807'{/yun}" sortable="custom" width="110">
                </el-table-column>
                <el-table-column label="{yun:}t key='member_com_00043'{/yun}" min-width="180" max-width="300" show-overflow-tooltip>
                    <template slot-scope="props">
                        <el-link :href="props.row.url" target="_blank" class="admin_cz_sc" :style="props.row.color ? 'color:'+props.row.color : ''">{{props.row.title}}
                            <div class="admin_mb5" v-html="props.row.titype"></div>
                        </el-link>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='admin_00170'{/yun}" width="130">
                    <template slot-scope="props">
                        <el-tag type=" " size="small">
                            <el-link type="primary" :href="props.row.classurl" target="_blank">{{ props.row.name }}
                            </el-link>
                        </el-tag>
                    </template>
                </el-table-column>
                <el-table-column prop="showtime" label="{yun:}t key='admin_00808'{/yun}" width="130">
                    <template slot-scope="props">
                        {yun:}t key='admin_00795'{/yun}
                        <div style="padding-top:5px;color:#999;">{yun:}t key='admin_00153'{/yun}</div>
                    </template>
                </el-table-column>
                <el-table-column prop="datetime_n" label="{yun:}t key='admin_user_weipin_00030'{/yun}" sortable="custom" width="135">
                </el-table-column>
                <el-table-column prop="hits" label="{yun:}t key='wap_com_00112'{/yun}" sortable="custom" width="115">
                </el-table-column>
                <el-table-column prop="sort" label="{yun:}t key='member_com_00022'{/yun}" sortable="custom" width="95">
                </el-table-column>
                <el-table-column prop="zd" label="{yun:}t key='admin_user_weipin_00050'{/yun}" width="75">
                    <template slot-scope="scope">
                        <span>{{ dnamearr[scope.row.did] }}</span>
                        <el-link type="primary" @click="fp(scope.row)">{yun:}t key='admin_user_weipin_00048'{/yun}</el-link>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="140" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="small " type=" " @click="add(scope.row)">{yun:}t key='wap_js_00073'{/yun}</el-button>
                            <el-button type="danger" size="small " @click="delrow(scope.row.id)">{yun:}t key='common.delete'{/yun}
                            </el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="bottomButnBull">
                <div class="bottomButnBlak">
                    <el-checkbox v-model="checkedAll" @change="selectAllBottom">{yun:}t key='wap_js_00074'{/yun}</el-checkbox>
                    <el-button size="mini" @click="delAllBottom">{yun:}t key='member_com_00055'{/yun}</el-button>
                </div>
                <div class="bottomButnNone">
                    <el-popover placement="top-start" width="460" trigger="hover">
                        <div class="bottomButnGend">
                            <el-button size="mini" @click="setsx(1)">{yun:}t key='admin_00796'{/yun}</el-button>
                            <el-button size="mini" @click="setsx(2)">{yun:}t key='admin_00797'{/yun}</el-button>
                            <el-button size="mini" @click="fpAllBottom">{yun:}t key='admin_user_00279'{/yun}</el-button>
                            <el-button size="mini" @click="fpClassAllBottom">{yun:}t key='admin_00164'{/yun}</el-button>
                        </div>
                        <div class="bottomButnMore" slot="reference">{yun:}t key='common.more'{/yun}</div>
                    </el-popover>
                </div>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange" :current-page="currentPage" :page-sizes="pageSizes" :page-size="perPage" layout="total, sizes, prev, pager, next, jumper" :total="total" :pager-count="pagerCount">
                </el-pagination>
            </div>
        </div>
        <!--分配站点弹窗-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_user_weipin_00029'{/yun}" width="350px" :visible.sync="drawerfp" :modal-append-to-body="false">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{yun:}t key='admin_00798'{/yun}</span>
                        </div>
                        <div class="toolClasCont">
                            <span>{{curr_data.title}}</span>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{yun:}t key='admin_user_weipin_00020'{/yun}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-select v-model="curr_data.did" placeholder="{yun:}t key='wap_user_00100'{/yun}" filterable>
                                <el-option v-for="(item,index) in dnamearr" :key="index" :label="item" :value="index">
                                </el-option>
                            </el-select>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawerfp = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="fpSave(1)" :disabled="submitLoading">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!--批量分配站点-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_00162'{/yun}" width="300px" :visible.sync="drawerfpmultiple" :modal-append-to-body="false">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{yun:}t key='admin_user_weipin_00020'{/yun}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-select v-model="multipledid" placeholder="{yun:}t key='wap_user_00100'{/yun}" filterable>
                                <el-option v-for="(item,index) in dnamearr" :key="index" :label="item" :value="index">
                                </el-option>
                            </el-select>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawerfpmultiple = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="fpSave(2)" :disabled="submitLoading">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!--批量转移分类-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_00164'{/yun}" width="400px" :visible.sync="drawerClassMultiple" :modal-append-to-body="false">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{yun:}t key='admin_00799'{/yun}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-select v-model="classid" filterable placeholder="{yun:}t key='admin_00159'{/yun}">
                                <el-option v-for="item in classarr" :key="item.id" :label="item.name" :value="item.id">
                                </el-option>
                            </el-select>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span></span>
                        </div>
                        <div class="toolClasCont">
                            <span style="color:red;">{yun:}t key='admin_00157'{/yun}</span>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawerClassMultiple = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="saveClass" :disabled="submitLoading">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!--批量设置、取消属性-->
        <div class="modluDrawer">
            <el-dialog :title="sx_type == 1 ? lc('admin_vue_00118') : lc('admin_00163')" width="400px" :visible.sync="drawersxmultiple" :modal-append-to-body="false">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{yun:}t key='admin_00800'{/yun}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-checkbox-group v-model="sx_arr">
                                <el-checkbox v-for="(item,index) in propertys" :key="index" :label="index">{{item}}</el-checkbox>
                            </el-checkbox-group>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{yun:}t key='admin_00801'{/yun}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-input :value="selectedItem.join(',')" readonly></el-input>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawersxmultiple = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="savesx" :disabled="submitLoading">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!-- 新增、修改弹窗 -->
        <div class="modluDrawer">
            <el-drawer :title="curr_data.id ? lc('admin_vue_00119') : lc('admin_vue_00120')" :close-on-press-escape="false" :wrapper-closable="false" :visible.sync="draweradd" append-to-body :modal-append-to-body="false" :show-close="true" :with-header="true" size="880px">
                <div class="drawerModlue">
                    <div class="tableDome_tip tableDoAlert">
                        <div class="shiTopAllTips">
                            <span>{yun:}t key='admin_00152'{/yun}</span>
                        </div>
                    </div>
                    <div class="moduleTable" style="margin-top:10px;">
                        <table class="tableVue">
                            <thead>
                                <tr align="left">
                                    <th width="100">{yun:}t key='member_com_00021'{/yun}</th>
                                    <th width="500">{yun:}t key='wap_user_00102'{/yun}</th>
                                    <th width=" ">{yun:}t key='member_com_00207'{/yun}</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr>
                                    <td>
                                        <div class="TableTite">{yun:}t key='admin_00167'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableSelect">
                                            <el-select v-model="curr_data.nid" filterable placeholder="{yun:}t key='admin_00159'{/yun}">
                                                <el-option v-for="item in classarr" :key="item.id" :label="item.name" :value="item.id">
                                                </el-option>
                                            </el-select>
                                        </div>
                                    </td>
                                    <td>
                                        <div class="TableShuom">
                                            <span> </span>
                                        </div>
                                    </td>
                                </tr>
                                <tr>
                                    <td>
                                        <div class="TableTite">{yun:}t key='member_com_00043'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableInpt">
                                            <el-input v-model="curr_data.title_all" placeholder="{yun:}t key='admin_00809'{/yun}"></el-input>
                                        </div>
                                    </td>
                                    <td>
                                        <div class="TableShuom">
                                            <span> </span>
                                        </div>
                                    </td>
                                </tr>
                                <tr>
                                    <td>
                                        <div class="TableTite">{yun:}t key='admin_user_00126'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableSelect">
                                            <el-select v-model="curr_data.did" placeholder="{yun:}t key='wap_user_00100'{/yun}" filterable>
                                                <el-option v-for="(item,index) in dnamearr" :key="index" :label="item" :value="index">
                                                </el-option>
                                            </el-select>
                                        </div>
                                    </td>
                                    <td>
                                        <div class="TableShuom">
                                            <span> </span>
                                        </div>
                                    </td>
                                </tr>
                                <tr>
                                    <td>
                                        <div class="TableTite">{yun:}t key='admin_00171'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="" style=" display:flex">
                                            <div class="" style=" margin-right:20px;">
                                                <el-date-picker v-model="curr_data.starttime_n" type="date" value-format="yyyy-MM-dd" placeholder="{yun:}t key='wap_user_00087'{/yun}">
                                                </el-date-picker>
                                            </div>
                                            <div class="">
                                                <el-date-picker v-model="curr_data.endtime_n" type="date" value-format="yyyy-MM-dd" placeholder="{yun:}t key='wap_user_00096'{/yun}">
                                                </el-date-picker>
                                            </div>
                                        </div>
                                    </td>
                                    <td>
                                        <div class="TableShuom">
                                            <span>{yun:}t key='admin_00154'{/yun} </span>
                                        </div>
                                    </td>
                                </tr>
                                <tr>
                                    <td>
                                        <div class="TableTite">{yun:}t key='admin_00168'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableInpt">
                                            <el-input v-model="curr_data.author" placeholder="{yun:}t key='admin_00161'{/yun}"></el-input>
                                        </div>
                                    </td>
                                    <td>
                                        <div class="TableShuom">
                                            <span> </span>
                                        </div>
                                    </td>
                                </tr>
                                <tr>
                                    <td>
                                        <div class="TableTite">{yun:}t key='admin_00802'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableInpt">
                                            <el-input v-model="curr_data.source" placeholder="{yun:}t key='admin_00810'{/yun}"></el-input>
                                        </div>
                                    </td>
                                    <td>
                                        <div class="TableShuom">
                                            <span> </span>
                                        </div>
                                    </td>
                                </tr>
                                <tr>
                                    <td>
                                        <div class="TableTite">{yun:}t key='admin_00803'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableInpt">
                                            <el-input v-model="curr_data.keyword" placeholder="{yun:}t key='admin_00811'{/yun}"></el-input>
                                        </div>
                                    </td>
                                    <td>
                                        <div class="TableShuom">
                                            <span> {yun:}t key='admin_00158'{/yun} </span>
                                        </div>
                                    </td>
                                </tr>
                                <tr>
                                    <td>
                                        <div class="TableTite">{yun:}t key='admin_00169'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableInpt">
                                            <el-input type="textarea" :rows="2" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="curr_data.description">
                                            </el-input>
                                        </div>
                                    </td>
                                    <td>
                                        <div class="TableShuom">
                                            <span> </span>
                                        </div>
                                    </td>
                                </tr>
                                <tr>
                                    <td>
                                        <div class="TableTite">{yun:}t key='admin_00804'{/yun}</div>
                                    </td>
                                    <td colspan="2">
                                        <div class="TableInpt">
                                            <textarea type="textarea" id="projectBasis" class="editor" name="projectBasis" cols="150" rows="30">
                                        </textarea>
                                        </div>
                                    </td>
                                </tr>
                                <tr>
                                    <td>
                                        <div class="TableTite">{yun:}t key='admin_00165'{/yun}</div>
                                    </td>
                                    <td class="drawerModInpt" style="display: flex">
                                        <el-upload class="upload-demo" style="display: flex" :action="''" :auto-upload="false" :accept="pic_accept" :show-file-list="false" :on-change="picChange">
                                            <el-button size="small" type="primary" plain icon="el-icon-plus">{yun:}t key='wap_00540'{/yun}
                                            </el-button>
                                        </el-upload>
                                        <img style="width: 208px; height: 167px;padding-left: 5px;" v-if="curr_data.picurl" :src="curr_data.picurl">
                                    </td>
                                    <td>
                                        <div class="TableShuom">
                                            <span> {yun:}t key='admin_user_company_00029'{/yun}</span>
                                        </div>
                                    </td>
                                </tr>
                                <tr v-if="propertys_cnt">
                                    <td>
                                        <div class="TableTite">{yun:}t key='admin_00805'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class=" ">
                                            <el-checkbox-group v-model="curr_data.describe_arr">
                                                <el-checkbox v-for="(item,index) in propertys" :key="index" :label="index">{{item}}
                                                </el-checkbox>
                                            </el-checkbox-group>
                                        </div>
                                    </td>
                                    <td>
                                        <div class="TableShuom">
                                            <span> </span>
                                        </div>
                                    </td>
                                </tr>
                                <tr>
                                    <td>
                                        <div class="TableTite">{yun:}t key='admin_00806'{/yun}</div>
                                    </td>
                                    <td>
                                        <div class="TableInpt">
                                            <el-input v-model="curr_data.sort" placeholder="{yun:}t key='wap_user_00076'{/yun}" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"></el-input>
                                        </div>
                                    </td>
                                    <td>
                                        <div class="TableShuom">
                                            <span> </span>
                                        </div>
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                    <div class="setBasicButn" style="border: none;">
                        <el-button type="primary" size="medium" @click="save" :disabled="submitLoading">{yun:}t key='common.submit'{/yun}</el-button>
                    </div>
                </div>
            </el-drawer>
        </div>
    </div>
</template>
<script>
var ue = null;
module.exports = {
    data: function() {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            loading: false,
            pagerCount: 5,
            emptytext: "{yun:}t key='wap_js_00113'{/yun}",
            keyword: '',
            checkedAll: false,
            selectedItem: [],
            tableData: [],
            currentPage: 1,
            perPage: 0,
            pageSizes: [],
            total: 0,
            islook: false,
            sort_type: '',
            sort_col: '',
            type: '1',
            oneclass: [],
            class_cascader: [],
            dnamearr: {},
            publish: '',
            cate: '',
            weburl: localStorage.getItem("sy_weburl"),
            curr_data: {
                nid: '',
                title: '',
                did: '0',
                starttime_n: '',
                endtime_n: '',
                author: '',
                source: '',
                keyword: '',
                description: '',
                picurl: '',
                describe_arr: [],
                sort: 0
            },
            drawerfp: false,
            drawerfpmultiple: false,
            multipledid: '',
            draweradd: false,
            today: '',
            piclist: [],
            propertys: [],
            classarr: [],
            propertys_cnt: 0,
            drawersxmultiple: false,
            sx_type: 1,
            sx_arr: [],
            classid: '',
            drawerClassMultiple: false,

            submitLoading: false,
            prevPage: 0
        }
    },
    mounted() {
        // this.getCacheInfo()
    },
    created: function() {

        this.getList();
        this.getCacheInfo();
    },
    methods: {
        fpClassAllBottom() {
            if (!this.selectedItem.length) {
                message.error("{yun:}t key='admin_user_weipin_00001'{/yun}");
                return false;
            }
            this.drawerClassMultiple = true
        },
        saveClass() {
            let that = this;
            if (this.classid == '') {
                message.error("{yun:}t key='admin_01333'{/yun}");
                return false;
            }
            let params = {
                id: this.selectedItem.join(','),
                nid: this.classid
            };
            that.submitLoading = true;
            httpPost('m=neirong&c=news&a=changeClass', params).then(function(response) {
                if (response.data.error == 0) {
                    message.success(response.data.msg, function() {
                        that.getList();
                        that.drawerClassMultiple = false
                    });
                } else {
                    message.error(response.data.msg);
                }
            }).catch(function(error) {
                console.log(error);
            }).finally(function() {
                that.submitLoading = false;
            });
        },
        setsx(sx_tp) {
            if (!this.selectedItem.length) {
                message.error("{yun:}t key='admin_user_weipin_00001'{/yun}");
                return false;
            }
            this.sx_type = sx_tp
            this.sx_arr = []
            this.drawersxmultiple = true
        },
        savesx() {
            let that = this;
            let params = {
                proid: this.selectedItem.join(','),
                type: this.sx_type == 1 ? 'add' : 'del',
                describe: this.sx_arr
            };
            that.submitLoading = true;
            httpPost('m=neirong&c=news&a=savepro', params).then(function(response) {
                if (response.data.error == 0) {
                    message.success(response.data.msg, function() {
                        that.getList();
                        that.drawersxmultiple = false
                    });
                } else {
                    message.error(response.data.msg);
                }
            }).catch(function(error) {
                console.log(error);
            }).finally(function() {
                that.submitLoading = false;
            });
        },
        picChange(file) {
            if (file.raw.type != 'image/png' && file.raw.type != 'image/jpeg') {
                message.error("{yun:}t key='admin_00155'{/yun}");
                return false
            }
            if (file.raw.size / 1024 > 500) {
                message.error("{yun:}t key='admin_00156'{/yun}");
                return false
            }
            var tmp = deepClone(this.curr_data)
            // 预览文件处理
            tmp.picurl = URL.createObjectURL(file.raw);
            // 复刻文件信息
            this.piclist[0] = file.raw;
            this.curr_data = tmp
        },
        fp(data) {
            this.curr_data = deepClone(data);
            this.drawerfp = true
        },
        fpAllBottom() {
            if (!this.selectedItem.length) {
                message.error("{yun:}t key='admin_user_weipin_00001'{/yun}");
                return false;
            }
            this.multipledid = '"
            this.drawerfpmultiple = true
        },
        fpSave(tp) {
            var that = this
            let params = {}
            if (tp == 1) { // {yun:}t key='admin_user_weipin_00029'{/yun}
                params.uid = that.curr_data.id
                params.did = that.curr_data.did
            } else { // {yun:}t key='admin_00162'{/yun}
                if (!that.selectedItem.length) {
                    message.error("请选择要分配的数据');
                    return false;
                }
                if (that.multipledid == '') {
                    message.error("{yun:}t key='admin_00166'{/yun}");
                    return false;
                }
                params.uid = that.selectedItem.join(',')
                params.did = that.multipledid
            }
            that.submitLoading = true;
            httpPost('m=neirong&c=news&a=checksitedid', params).then(function(response) {
                if (response.data.error == 0) {
                    message.success(response.data.msg, function() {
                        if (tp == 1) {
                            that.drawerfp = false
                        } else {
                            that.drawerfpmultiple = false
                        }
                        that.getList();
                    });
                } else {
                    message.error(response.data.msg);
                }
            }).catch(function(error) {
                console.log(error);
            }).finally(function() {
                that.submitLoading = false;
            });
        },
        initEditor() {
            var that = this;
            ue = UE.getEditor('projectBasis", {
                wordCount: false,           // 关闭字数统计
                elementPathEnabled: false,  //{yun:}t key='common.close'{/yun}elementPath {yun:}t key='common_05704'{/yun}
                autoHeightEnabled: false,   //关闭自适应高度，超出部分以滚动条形式展示
                initialFrameHeight: 480,    //默认的编辑区域高度
                initialFrameWidth: 600,     //初始化编辑器宽度,{yun:}t key='wap_js_00098'{/yun}1000
                zIndex: 2000
            });

            ue.ready(function() {
                if (that.curr_data.content) {
                    ue.setContent(that.curr_data.content);
                } else {
                    ue.setContent("');
                }
            });


        },
        add(row) {
            var that = this;
            let params = {add: 1};
            if (row.id){
                params.id = row.id;
            }
            httpPost('m=neirong&c=news&a=addnews', params).then(function (result) {
                var res = result.data;
                that.submitLoading = false;
                if (res.error == 0) {
                    if (row.id) {
                        that.curr_data = deepClone(row);
                        that.curr_data.content = res.data.content;
                    } else {
                        that.curr_data = {
                            nid: '',
                            title: '',
                            did: '0',
                            starttime_n: '',
                            endtime_n: '',
                            author: '',
                            source: '',
                            keyword: '',
                            description: '',
                            picurl: '',
                            describe_arr: [],
                            sort: 0,
                        };
                    }
                    setTimeout(function() {
                        that.initEditor();
                    }, 100);
                    that.draweradd = true;
                } else {
                    message.error(res.msg);
                }
            }).catch(function(e) {
                console.log(e)
            })
        },
        handleSelectionChange(val) {
            this.selectedItem = [];
            let _this = this;
            if (val.length) {
                val.forEach(item => {
                    _this.selectedItem.push(item.id);
                });
            }
            if (_this.selectedItem.length == 0) {
                _this.checkedAll = false;
            } else {
                if (_this.selectedItem.length == _this.tableData.length) {
                    _this.checkedAll = true;
                } else {
                    _this.checkedAll = false;
                }
            }
        },
        selectAllBottom(value) {
            value ? this.$refs.multipleTable.toggleAllSelection() : this.$refs.multipleTable.clearSelection();
        },
        handleSizeChange(val) {
            this.perPage = val;
            this.getList()
        },
        handleCurrentChange(val) {
            this.currentPage = val;
            this.getList()
        },
        sortChange: function(column) {
            if (column.order == 'descending') {
                this.sort_type = 'desc';
            } else if (column.order == 'ascending') {
                this.sort_type = 'asc';
            } else {
                this.sort_type = '';
            }
            this.sort_col = column.prop
            if (this.sort_col == 'datetime_n') {
                this.sort_col = 'datetime'
            }
            this.search();
        },
        search() {
            this.currentPage = 1;
            this.getList();
        },
        async getList() {
            let that = this;
            let params = {
                page: that.currentPage,
                pageSize: that.perPage
            }
            if (that.keyword) {
                params.keyword = that.keyword
            }
            if (that.end) {
                params.end = that.end
            }
            if (that.type) {
                params.type = that.type
            }
            if (that.publish) {
                params.publish = that.publish
            }
            if (that.cate) {
                params.cate = that.cate
            }
            if (that.sort_type && that.sort_col) {
                params.order = that.sort_type
                params.t = that.sort_col
            }
            that.loading = true;
            that.emptytext = "{yun:}t key='admin_user_weipin_00026'{/yun}";
            httpPost('m=neirong&c=news&a=index', params, { hideloading: true }).then(function(result) {
                var res = result.data
                if (res.error == 0) {
                    that.tableData = res.data.list
                    that.perPage = parseInt(res.data.perPage)
                    that.pageSizes = res.data.pageSizes
                    that.total = parseInt(res.data.total);
                    if (that.prevPage != that.currentPage) {
                        that.prevPage = that.currentPage;
                        that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    }
                    that.loading = false;
                    if (that.tableData.length === 0) {
                        that.emptytext = "{yun:}t key='wap_js_00113'{/yun}";
                    }
                }
            }).catch(function(e) {
                console.log(e)
            })
        },
        async getCacheInfo() {
            let that = this;
            httpPost('m=neirong&c=news&a=getCache', {}, { hideloading: true }).then(function(result) {
                var res = result.data
                if (res.error == 0) {
                    that.dnamearr = res.data.Dname
                    that.oneclass = res.data.one_class
                    that.class_cascader = res.data.class_cascader
                    that.today = res.data.today
                    that.classarr = res.data.class_arr
                    that.propertys = res.data.property
                    that.propertys_cnt = Object.keys(that.propertys).length

                }
            }).catch(function(e) {
                console.log(e)
            })

        },
        save() {
            var that = this;
            if (!that.curr_data.nid) {
                message.error("{yun:}t key='admin_01333'{/yun}");
                return false;
            }
            if (!that.curr_data.title_all) {
                message.error("{yun:}t key='admin_00809'{/yun}");
                return false;
            }
            that.curr_data.content = ue.getContent();
            // 去除html标签后判断内容是否为空
            var regex = /(<([^>]+)>)/ig
            var result = ue.getContent().replace(regex, "");
            if (!result) {
                message.error(lc('admin_vue_00061'));
                return false;
            }
            var params = new FormData();
            if (that.piclist.length) {
                params.append('pic[]', this.piclist[0])
            }
            params.append('id', this.curr_data.id);
            params.append('title', this.curr_data.title_all);
            params.append('nid', this.curr_data.nid);
            params.append('did', this.curr_data.did);
            params.append('starttime', this.curr_data.starttime_n);
            params.append('endtime', this.curr_data.endtime_n);
            params.append('author', this.curr_data.author);
            params.append('source', this.curr_data.source);
            params.append('keyword', this.curr_data.keyword);
            params.append('description', this.curr_data.description);
            params.append('content', this.curr_data.content);
            params.append('describe', this.curr_data.describe_arr.join(','));
            params.append('sort', this.curr_data.sort);
            that.submitLoading = true;
            httpPost('m=neirong&c=news&a=addnews', params).then(function(result) {
                var res = result.data;
                that.submitLoading = false;
                if (res.error == 0) {
                    message.success(res.msg, function() {
                        that.draweradd = false
                        that.keyword = ''
                        that.end = ''
                        that.piclist = [];
                        that.getList()
                    })
                } else {
                    message.error(res.msg);
                }
            }).catch(function(e) {
                console.log(e)
            })
        },
        delrow(id) {
            delConfirm(this, id, this.delete);
        },
        delAllBottom() {
            if (!this.selectedItem.length) {
                message.error("{yun:}t key='admin_00136'{/yun}");
                return false;
            }
            delConfirm(this, this.selectedItem, this.delete);
        },
        async delete(id) {
            let that = this;
            let params = {
                del: id
            };
            httpPost('m=neirong&c=news&a=delnews', params).then(function(response) {
                if (response.data.error == 0) {
                    message.success(response.data.msg);
                    that.getList();
                } else {
                    message.error(response.data.msg);
                }
            }).catch(function(error) {
                console.log(error);
            })
        },
    },

};
</script>
<style scoped>
.drawerModInfoOne .drawerModInpt .el-button {
    margin-left: 0;
}
</style>