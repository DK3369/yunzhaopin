<template>
    <div class="moduleElHight">
        <div class="moduleSeachbig">
            <div class="tableSeachInpt tableSeachInptsmall tableSeacFromer" style="padding: 2px 0;">
				<el-input v-model="search_params.keyword" @keyup.enter.native="search" placeholder="请输入搜索内容" size="small" clearable>
					<el-select v-model="search_params.ctype" size="small" slot="prepend" placeholder="用户名">
					    <el-option label="企业名称" value="1"></el-option>
					    <el-option label="备注" value="2"></el-option>
					</el-select>
				</el-input>
            </div>
            <!--收起部分-->
            <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
                <el-select v-if="searchlist" v-model="search_params.rating" size="small" slot="prepend" placeholder="会员等级" clearable @change="search">
                    <el-option v-for="(item, index) in searchlist.rating.value" :key="index" :label="item"
                               :value="index">
                    </el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
                <el-select v-if="searchlist" v-model="search_params.time" size="small" slot="prepend" placeholder="到期时间" clearable @change="search">
                    <el-option v-for="(item, index) in searchlist.time.value" :key="index" :label="item"
                               :value="index">
                    </el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" icon="el-icon-search" size="mini" @click="search">查询</el-button>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" plain icon="el-icon-plus" size="mini" @click="add">新增名企
                </el-button>
            </div>
            <div class="tableSeachzk" :class="{ 'searchbutnKai': seachbutn }" style="margin-bottom: 8px;">
                <el-button type="info" class="zhankai" @click="seachbutn = !seachbutn, tableHig = !tableHig"
                           aria-disabled="false" size="mini" plain>展开<i class="el-icon-arrow-down el-icon--right"></i>
                </el-button>
                <el-button type="info" class="shouqi" @click="seachbutn = !seachbutn, tableHig = !tableHig"
                           aria-disabled="false" size="mini" plain>合并<i class="el-icon-arrow-up el-icon--right"></i>
                </el-button>
            </div>
        </div>
        <div class="admin_datatip">
            <i class="el-icon-document"></i> 数据统计：<span class="admin_datatip_n">已过期：{{gqNum}} 条</span>
        </div>
        <div class="moduleElTable" :class="{ 'moduleElTableHig': tableHig }"
             style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="tableData" style="width: 100%" stripe
                      @mousedown.native="mouseDownHandler"
                      @mouseup.native="mouseUpHandler"
                      @mousemove.native="mouseMoveHandler"
                      @sort-change='sortChange' :default-sort="{ prop: 'uid', order: 'descending' }"
                      :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
                      @selection-change="handleSelectionChange" ref="multipleTable" v-loading="loading">
                <template slot="empty">
                    <p>{{dataText}}</p>
                </template>
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="uid" label="用户ID" width="120" sortable="custom"></el-table-column>
                <el-table-column prop="name" label="企业名称" min-width="180" show-overflow-tooltip>
                    <template slot-scope="props">
                        <div class="layuiSmallImg">
                            <a href="javascript:;" class="layuiSmallImgUp" @click="jumpToMember(props.row.uid);">{{props.row.username}}</a>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label=" 等级 " prop="rating" width="130"></el-table-column>
                <el-table-column prop=" " label="名企图片" width="150">
                    <template slot-scope="props">
                        <div class="layuiSmallImg">
                            <el-image v-if="props.row.hot_pic" :src="props.row.hot_pic" :preview-src-list="[props.row.hot_pic]"  style="width:48px;height:48px"></el-image>
                            <span v-else>无</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="" label="服务价格">
                    <template slot-scope="props">
                        {{props.row.service_price}}元
                    </template>
                </el-table-column>
                <el-table-column prop="time_start" label="开始时间" width="150">
                    <template slot-scope="props">
                        {{props.row.time_start_n}}
                    </template>
                </el-table-column>
                <el-table-column prop="time_end" label="结束时间" width="150">
                    <template slot-scope="props">
                        <span v-if="props.row.time_end_n == '已到期'" style="color: red">已到期</span>
                        <span v-else>{{props.row.time_end_n}}</span>
                    </template>
                </el-table-column>
                <el-table-column prop="beizhu" label="备注" width="150">
                    <template slot-scope="props">
                        <span v-if="props.row.beizhu">{{props.row.beizhu}}</span>
                        <span v-else>未备注</span>
                    </template>
                </el-table-column>
                <el-table-column prop="sort" label="排序" width="150" sortable="custom"></el-table-column>
                <el-table-column label="操作" width="140" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini" plain @click="edit(scope.row)">修改</el-button>
                            <el-button type="danger" size="mini"  @click="delrow(scope.row.uid)">删除</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" @change="selectAllBottom">全选</el-checkbox>
                <el-button @click="delAllBottom" size="mini">批量删除</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange"
                               @current-change="handleCurrentChange"
                               :current-page="currentPage" :page-sizes="pageSizes"
                               :page-size="perPage"
                               layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
        <!--名企弹出-->
        <div class="modluDrawer">
            <el-dialog :title="title" :visible.sync="drawermq" :with-header="true" :modal-append-to-body="false"
                       :show-close="true" width="620px">
                <addhotjob :hotinfo="info" :hascom="isedit"></addhotjob>
            </el-dialog>
        </div>
    </div>
</template>
<script>
    module.exports = {
        data: function () {
            return {
                mouseFlag: false,
                mouseOffset: 0,
                loading: false,
                dataText: '数据加载中',
                input3: '',
                input: '',
                select: '',
                value: true,
                value1: '',
                checked: '',
                drawermq: false,
                seachbutn: true,
                tableHig: true,
                search_params:{
                    ctype: '1',
                    keyword: '',
                    rating:'',
                    time: '',
                },
                checkedAll: false,
                selectedItem: [],
                tableData: [],
                currentPage: 1,
                perPage: 0,
                pageSizes: [],
                total: 0,
                gqNum: 0,
                searchlist: null,
                imageUrl: '',
                items: [],
                islook: false,
                sort_type: '',
                sort_col: '',
                title: '',
                isedit: false,
                info: {
                    uid: '',
                    username: '',
                    service_price: '',
                    rating: '',
                    rating_id: '',
                    sort: '',
                    beizhu: '',
                    hot_pic: '',
                },
                timeRange: ['', ''],
                com_arr: [],
                mqlogolist: [],
                prevPage: 0
            }
        },
        created() {
			var that = this;
			let params = window.parent.homeapp.$route.params;
			let query = window.parent.homeapp.$route.query;
			
			if (!$.isEmptyObject(query.params)) {
				params = {...params,...query.params};
			}
			
			if (!$.isEmptyObject(params)) {
				delete params.activeName;
				this.getParams(params);
			}
            this.getList();
        },
        mounted() {
            var that = this
            setTimeout(function () {
                that.getTjNum()
                that.getSearchFun()
            }, 200)
        },
        components: {
            'addhotjob': httpVueLoader('./addhotjob.vue'),
        },
        methods: {
            mouseDownHandler(e) {
                this.mouseOffset = e.clientX;
                this.mouseFlag = true;
            },
            mouseUpHandler(e) {
                this.mouseFlag = false;
            },
            mouseMoveHandler(e) {
                // 这里面需要注意，通过ref需要那个那个包含table元素的父元素
                let divData = this.$refs.multipleTable.bodyWrapper;
                if (this.mouseFlag) {
                    // 设置水平方向的元素的位置
                    divData.scrollLeft -= (- this.mouseOffset + (this.mouseOffset = e.clientX));
                }
            },





			getParams:function(params={},search=false){
				var that = this;
				for(let i in params){
					if(typeof that.search_params[i]!='undefined'){
						that.search_params[i] = params[i];
					}
				}
				if(search){
					this.search();
				}
			},
            // 添加名企业搜索企业
            getComArr(query) {
                var that = this
                if (query !== '') {
                    setTimeout(() => {
                        httpPost('m=neirong&c=zhaopinhui&a=getcomlist', {comname: query}).then(function (response) {
                            if (response.data.error == 0) {
                                that.com_arr = response.data.data
                            } else {
                                message.error('获取参会企业失败');
                            }
                        }).catch(function (error) {
                            console.log(error);
                        })
                    }, 200);
                } else {
                    this.options = [];
                }
            },
            edit: function(data){
                this.info = data
                this.info.hot_pic_n = this.info.hot_pic
                this.info.time_end_n = this.info.time_end_nn
                this.title = '修改名企'
                this.isedit = true
                this.drawermq = true
            },
            add: function(){
                this.info = {
                    uid: '',
                    username: '',
                    service_price: '',
                    rating: '',
                    rating_id: '',
                    sort: '',
                    beizhu: '',
                    hot_pic_n: '',
                    time_start_n: '',
                    time_end_n: ''
                }
                this.isedit = false
                this.title = '新增名企'
                this.drawermq = true
            },
            sortChange: function (column) {
                if (column.order == 'descending') {
                    this.sort_type = 'desc';
                } else if (column.order == 'ascending') {
                    this.sort_type = 'asc';
                } else {
                    this.sort_type = '';
                }
                this.sort_col = column.prop
                this.search();
            },
            // 获取名企数量统计
            getTjNum: function () {
                var that = this;
                httpPost('m=user&c=hotjob&a=hotNum', {}, {hideloading: true}).then(function (result) {
                    var res = result.data;
                    if (res.error == 0) {
                        that.gqNum = res.data.hoted ? res.data.hoted : 0
                    }
                }).catch(function (e) {
                    console.log(e)
                })
            },
            // 跳转到会员中心
            jumpToMember: function (uid) {
                let tmpWin = window.open('', '_blank')
                var params = {uid: uid}
                httpPost('m=user&c=company&a=Imitate', params).then(function (result) {
                    var res = result.data;
                    if (res.error == 0) {
                        tmpWin.location = res.data
                    }
                }).catch(function (e) {
                    tmpWin.close()
                })
            },
            handleSelectionChange(val) {
                this.selectedItem = [];
                let _this = this;
                if (val.length) {
                    val.forEach(item => {
                        _this.selectedItem.push(item.uid);
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
                this.tableData = []
                this.getList()
            },
            handleCurrentChange(val) {
                this.currentPage = val;
                this.getList()
            },
            search() {
                this.currentPage = 1;
                this.getList();
            },
            getSearchFun:function(){
                let that = this;
                httpPost('m=user&c=hotjob&a=getSearchData', {},{hideloading: true}).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        that.searchlist = res.data;
                    }
                })
            },
            async getList() {
                let that = this;
                let params = {
                    page: that.currentPage,
                    pageSize: that.perPage
                }
                if (that.search_params.ctype) {
                    params.ctype = that.search_params.ctype
                }
                if (that.search_params.keyword) {
                    params.keyword = that.search_params.keyword
                }
				if (that.search_params.rating) {
                    // console.log(that.searchlist.rating.value);return ;
				    params.rating_name = that.searchlist.rating.value[that.search_params.rating]
				}
				if (that.search_params.time) {
				    params.time = that.search_params.time
				}
				if (that.search_params.keyword) {
				    params.keyword = that.search_params.keyword
				}
                if (that.sort_type && that.sort_col) {
                    params.order = that.sort_type
                    params.t = that.sort_col
                }
                that.loading = true;
                httpPost('m=user&c=hotjob&a=index', params, {hideloading: true}).then(function (result) {
                    var res = result.data
                    if (res.error == 0) {
                        that.tableData = res.data.list
                        that.perPage = parseInt(res.data.perPage)
                        that.pageSizes = res.data.pageSizes
                        that.total = parseInt(res.data.total)
                        that.loading = false;
                        if(that.prevPage != that.currentPage){
                            that.prevPage = that.currentPage;
                            that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                            scrollToTop()
                        }
                        if (that.tableData.length === 0) {
                            that.dataText = "暂无数据";
                        }
                    }
                }).catch(function (e) {
                    console.log(e)
                })
            },
            delrow(id) {
                delConfirm(this, id, this.delete);
            },
            delAllBottom() {
                if (!this.selectedItem.length) {
                    message.error('请选择要删除的数据');
                    return false;
                }
                delConfirm(this, this.selectedItem, this.delete);
            },
            async delete(id) {
                let that = this;
                let params = {
                    del: id
                };
                httpPost('m=user&c=hotjob&a=del', params).then(function (response) {
                    if (response.data.error == 0) {
                        message.success(response.data.msg);
                        that.getList();
                    } else {
                        message.error(response.data.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                })
            },
        },
    };
</script>
<style>
    .avatar-uploader .el-upload {
        border: 1px dashed #d9d9d9;
        border-radius: 6px;
        cursor: pointer;
        position: relative;
        overflow: hidden;
    }

    .avatar-uploader .el-upload:hover {
        border-color: #409EFF;
    }

    .avatar-uploader-icon {
        font-size: 14px;
        color: #8c939d;
        width: 48px;
        height: 48px;
        line-height: 48px;
        text-align: center;
    }

    .avatar {
        width: 48px;
        height: 48px;
        display: block;
    }
    .tableSeacFromer{
    margin-right: 8px;
}
.tableSeacFromer .el-input-group__prepend{
    padding: 0;
    background: none;
}
.tableSeacFromer .el-select{
    margin-right: 0;
    width: 160px;
}
.tableSeacFromer .el-input{
    margin-right: 0;
}
</style>