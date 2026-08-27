<template>
<div id="daohaapp" class="moduleElenAl">
    	<div class="moduleSeachs">
    	    <div class="">{{ lc('admin_system_00681') }}</div>
    	    <div class="moduleSeachButn">
    	       
    	        <el-button type="primary" icon="el-icon-document-add" size="small" @click="addShow = true">{{ lc('admin_system_00680') }}</el-button>
    	    </div>
    	</div>

        <div class="moduleElTable">
            <el-table ref="multipleTable" :data="tableData" border style="width: 100%" :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="100%" @selection-change="selectionChange" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" :label="lc('admin_system_00682')" width="80"></el-table-column>
				<el-table-column :label="lc('admin_00219')"  >
				    <template #default="scope">
				        <div class="moduleElTaPax" v-if="editname_id==scope.row.id">
                            <el-input id="inputref" :placeholder="lc('wap_user_00076')" v-model="editname" :data-preval="scope.row.name" data-type="name" @blur="editChange" clearable></el-input>
				        </div>
                        <div class="moduleElTaPax" v-else>
                            <span>{{scope.row.name}}</span>
                            <img src="/admin/php-admin/images/bine.png" @click="editcolumn('name',scope.row.name,scope.row.id)" alt="">
                        </div>
				    </template>
				</el-table-column>
				<el-table-column :label="lc('admin_vue_00044')" >
				    <template #default="scope">
                        <div class="moduleElTaPax" v-if="editsort_id==scope.row.id">
                            <el-input id="inputref" :placeholder="lc('wap_user_00076')" v-model="editsort" @input="inputIntNumber($event, 'editsort', '')" :data-preval="scope.row.sort" data-type="sort" @blur="editChange" clearable></el-input>
                        </div>
				        <div class="moduleElTaPax" v-else>
				            <span>{{scope.row.sort}}</span>
				            <img src="/admin/php-admin/images/bine.png" @click="editcolumn('sort',scope.row.sort,scope.row.id)" alt="">
				        </div>
				    </template>
				</el-table-column>
                <el-table-column fixed="right" :label="lc('member_user_00048')" width="80">
                    <template #default="scope">
                        <div class="cz_button">
						  <el-button size="small" @click="deleteClass(scope.row.id)" type="danger">{{ lc('wap_js_00077') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="modulecz modulePagButn">
                <el-checkbox v-model="allchecked" @change="allcheckChange">{{ lc('wap_js_00074') }}</el-checkbox> 
                <el-button @click="deleteClassAll" size="small">{{ lc('member_com_00055') }}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background 
                    @current-change="handleCurrentChange"
                    :hide-on-single-page="true"
                    :current-page="currentPage"
                    :total="total"
                    :page-size="perPage"
                    layout="total,prev, pager, next, jumper">
                </el-pagination>
            </div>
        </div>
        <el-dialog :title="lc('admin_system_00234')"  width="30%" v-model="addShow" :modal-append-to-body="false" @close="addclose">
            <div class="hydialog_item">
                <span>{{ lc('admin_system_00683') }}</span>
                <el-input type="textarea"  v-model="classname" style="flex: 1;"></el-input>
            </div>
            <i class="el-icon-info" style="margin-top: 10px;">{{ lc('admin_system_00091') }}</i>
            <template #footer><div class="dialog-footer">
                <el-button type="primary" @click="addclass">{{ lc('wap_js_00091') }}</el-button>
            </div></template>
        </el-dialog>
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
                    tableData: [],
                    total:0,
                    perPage:10,
                    currentPage:1,
                    prevPage: 0,
                    editname_id:'',
                    editsort_id:'',
                    editname:'',
                    editsort:'',

                    addShow:false,
                    classname:'',

                    allchecked:false,
                    choosedata:[],

                    loading: true,
					emptytext: window.lc('wap_js_00113'),
                }
            },
            components: {},
            created: function () {
                this.getList();
            },
            methods: {
                inputIntNumber(val, form, key) {
                    this.$data[form] = val.replace(/[^0-9]/g,'');
                },
                async getList() {
                    let that = this;
                    let params = {
                        page: that.currentPage,
                    }
                    that.loading = true;
                    that.emptytext = window.lc('admin_user_weipin_00026');
                    httpPost('m=system&c=singleclass&a=index', params).then(function(result){
                        var res = result.data
                        if (res.error == 0) {
                            that.tableData = res.data.list
                            that.total = parseInt(res.data.total)
                            that.perPage = parseInt(res.data.perPage)
                            if (that.prevPage != that.currentPage) {
                                that.prevPage = that.currentPage;
                                that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                            }
                            that.loading = false;
							if (that.tableData.length === 0){
                                that.emptytext = window.lc('wap_js_00113');
                            }
                        }
                    }).catch(function(e){
                        console.log(e)
                    })
                },
                handleCurrentChange(val) {
                    this.currentPage = val;
                    this.getList();
                },
                editcolumn:function(type,def,id){
                    
                    this[`edit${type}_id`] = id;
                    this[`edit${type}`] = def;

                    this.$nextTick(() => {
                        if (timer) {
                             clearTimeout(timer);
                        }
                        timer = setTimeout(() => {
                           document.getElementById('inputref').focus();
                        }, 100);
                    })
                    
                },
                async editChange(e){

                    var that = this;
                    
                    var preval = e.target.dataset.preval;
                    var type = e.target.dataset.type;

                    var val = this[`edit${type}`];
                    var id = this[`edit${type}_id`];
                    
                    if(val==preval){
                        
                        this[`edit${type}_id`] = '';
                        this[`edit${type}`] = '';

                    }else{
                        if(type=='name' && val==''){
                            this[`edit${type}_id`] = '';
                            message.error(window.lc('admin_00208'));return;
                        }
                        var param = {id:id};
                        param[`${type}`] = val;

                        httpPost('m=system&c=singleclass&a=ajax',param).then(function(result){
                            
                            for(let i in that.tableData){
                                if(that.tableData[i].id==id){
                                    that.tableData[i][`${type}`] = val;break;
                                }
                            }

                            that[`edit${type}_id`] = '';
                            that[`edit${type}`] = '';
                            message.success(window.lc('admin_user_company_00208'),function(){
                                that.getList()
                            });
                        }).catch(function(e){
                            console.log(e)
                        })
                    }
                    
                },
                async addclass() {
                    let that = this;

                    var position = this.classname.split("\n");
                    var name=position.join("-");

                    if(name==''){
                        message.error(window.lc('admin_00208'));return;
                    }
                    let params = {
                        name: name,
                    }
                    
                    httpPost('m=system&c=singleclass&a=add', params).then(function(result){
                        
                        var res = result.data;
                        if(res.error==1){
                            message.error(window.lc('admin_system_00130'));return;
                        }else if(res.error==2){
                            message.success(window.lc('admin_system_00138'),function(){location.reload();});return;
                        }else if(res.error==3){
                            message.error(window.lc('admin_system_00137'),function(){location.reload();});return;
                        }
                    }).catch(function(e){
                        console.log(e)
                    })
                },
                addclose:function(){
                    this.classname = '';
                },
                allcheckChange:function(){
                   
                    this.$refs.multipleTable.toggleAllSelection();
                    
                },
                selectionChange:function(e){
                    if(this.tableData.length != e.length){
                        this.allchecked = false;
                    }else{
                        this.allchecked = true;
                    }
                    this.choosedata = e;
                },
                deleteClass:function(id){
                    var _this = this;
                    
                    var params = {
                        del:id
                    };
                    delConfirm(_this,params,this.deleteClassPost)
                },
                deleteClassAll:function(){
                    var _this = this;
                    var idarr = [];
                    if(this.choosedata.length>0){
                        for(let i in this.choosedata){
                            idarr.push(this.choosedata[i].id);
                        }
                    }else{
                        message.error(window.lc('member_com_00084'));return;
                    }
                    var params = {
                        del:idarr
                    };
                    
                    delConfirm(_this,params,this.deleteClassPost)
                },
                async deleteClassPost(params) {

                    let that = this;

                    httpPost('m=system&c=singleclass&a=del', params).then(function(result){
                        
                        var res = result.data;
                        if(res.error==9){
                            message.success(res.msg,function(){that.getList()});return;
                        }else{
                            message.error(res.msg);return;
                        }
                    }).catch(function(e){
                        console.log(e)
                    })
                },
            }
        }
</script>
