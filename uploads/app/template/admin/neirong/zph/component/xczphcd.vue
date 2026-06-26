<template>
    <div class="moduleElHight">
        <div class="modulemoreSeach">
            <div class="moduleSeachleft">
                <div class="tableSeachInpt">
                    <el-input placeholder="{yun:}t key='admin_00340'{/yun}" clearable size="small" prefix-icon="el-icon-search" v-model="keyword">
                    </el-input>
                </div>
                <div class="tableSeachInpt">
                    <el-button type="primary" icon="el-icon-search" size="mini" @click="search">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
                </div>
            </div>
            <div class="moduleSeachButn">
                <div class="tableSeachInpt">
                    <el-button type="primary" icon="el-icon-document-add" size="mini" @click="toadd({id: ''})">{yun:}t key='admin_00829'{/yun}</el-button>
                </div>
            </div>
        </div>
        <div class="admin_datatip"><i class="el-icon-document"></i> {yun:}t key='admin_00830'{/yun}
        </div>
        <div class="moduleElTable moduleElMoreLive" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="tableData" style="width: 100%" stripe @selection-change="handleSelectionChange"
                      :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" ref="multipleTable" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" label="{yun:}t key='member_com_00345'{/yun}" width="80" sortable="custom"></el-table-column>
                <el-table-column prop="name" label="{yun:}t key='member_com_00021'{/yun}">
                    <template slot-scope="scope">
                        <el-input v-if="scope.row[scope.column.property + 'isShow']"
                                  :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                                  v-model="scope.row.name" @blur="alterData(scope, 1)"></el-input>
                        <span v-else>{{ lc("admin_level1_category_value", [scope.row.name]) }}
                            <img src="../../../admin/images/bine.png" alt="" style="margin-left: 4px;" width="14"
                                 height="14" @click="editData(scope, 1)">
                        </span>
                    </template>
                </el-table-column>
                <el-table-column prop="sort" label="{yun:}t key='member_com_00022'{/yun}">
                    <template slot-scope="scope">
                        <el-input v-if="scope.row[scope.column.property + 'isShow']"
                                  :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                                  v-model="scope.row.sort" @blur="alterData(scope, 1)"></el-input>
                        <span v-else>{{ scope.row.sort }}
                            <img src="../../../admin/images/bine.png" alt="" style="margin-left: 4px;" width="14"
                                 height="14" @click="editData(scope, 1)">
                        </span>
                    </template>
                </el-table-column>
                <el-table-column prop="pic_n" label="{yun:}t key='admin_00832'{/yun}" width="180">
                    <template slot-scope="props">
                        <div class="demo-image__preview">
                            <el-image style="width: 20px; height: 20px" :src="props.row.pic_n" :preview-src-list="srcList">
                            </el-image>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='member_user_00048'{/yun}" fixed="right" width="200">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini" plain @click="toadd(scope.row)">{yun:}t key='wap_js_00073'{/yun}</el-button>
                            <el-button size="mini" plain @click="todtl(scope.row.id)">{yun:}t key='admin_00294'{/yun}</el-button>
                            <el-button type="danger" size="mini" @click="delrow(scope.row.id, 1)">{yun:}t key='common.delete'{/yun}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" @change="selectAllBottom">{yun:}t key='wap_js_00074'{/yun}</el-checkbox>
                <el-button @click="delAllBottom(1)" size="mini">{yun:}t key='member_com_00055'{/yun}</el-button>
            </div>
        </div>
        <!-- 场地详细 -->
        <el-drawer title="{yun:}t key='admin_00833'{/yun}" :visible.sync="drawerdtl" append-to-body :modal-append-to-body="false" size="80%">
            <div class="moduleElHight" style="padding: 0 20px;">
                <div class="modulemoreSeach" style="width: 100%; padding-bottom: 10px;">
                    <div class="">
                    </div>
                    <div class="moduleHeadrButn" >
                        <el-button type="primary" icon="el-icon-document-add" @click="toadd({id: ''})" size="mini">{yun:}t key='admin_00823'{/yun}
                        </el-button>
                    </div>
                </div>
                <div class="moduleElTable" style="height: calc(100% - 92px);">
                    <el-table :data="spaceTableData" style="width: 100%;margin-bottom: 20px;" row-key="id" border
                          default-expand-all :tree-props="{children: 'children', hasChildren: 'hasChildren'}"
                          :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" @selection-change="handleSubTblSelectionChange" ref="multipleSubTable" v-loading="loading" :empty-text="emptytext">
                    <el-table-column type="selection" width="55"></el-table-column>
                    <el-table-column prop="id" label="{yun:}t key='member_com_00345'{/yun}" width="180">
                    </el-table-column>
                    <el-table-column label="{yun:}t key='member_com_00021'{/yun}" property="name">
                        <template slot-scope="scope">
                            <el-input v-if="scope.row[scope.column.property + 'isShow']"
                                      :ref="'edit_' + scope.column.property + scope.$index" :id="'edit_' + scope.column.property + scope.$index"
                                      v-model="scope.row.name" @blur="alterData(scope, 2)"></el-input>
                            <span v-else>{{ scope.row.name }}
                            <img src="../../../admin/images/bine.png" alt="" style="margin-left: 4px;" width="14"
                                 height="14" @click="editData(scope, 2)">
                        </span>
                        </template>
                    </el-table-column>
                    <el-table-column label="{yun:}t key='wap_00563'{/yun}" property="price">
                        <template slot-scope="scope">
                            <el-input v-if="scope.row[scope.column.property + 'isShow']"
                                      :ref="'edit_' + scope.column.property + scope.$index" :id="'edit_' + scope.column.property + scope.$index"
                                      v-model="scope.row.price" @blur="alterData(scope, 2)"></el-input>
                            <span v-else>{{ scope.row.price }}
                                <img src="../../../admin/images/bine.png" alt="" style="margin-left: 4px;" width="14"
                                     height="14" @click="editData(scope, 2)">
                            </span>
                        </template>
                    </el-table-column>
                    <el-table-column label="{yun:}t key='member_com_00022'{/yun}" property="sort">
                        <template slot-scope="scope">
                            <el-input v-if="scope.row[scope.column.property + 'isShow']"
                                      :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                                      v-model="scope.row.sort" @blur="alterData(scope, 2)"></el-input>
                            <span v-else>
                                {{ scope.row.sort }}
                                <img src="../../../admin/images/bine.png" alt="" style="margin-left: 4px;" width="14"
                                     height="14" @click="editData(scope, 2)">
                            </span>
                        </template>
                    </el-table-column>
                    <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="140" fixed="right">
                        <template slot-scope="scope">
                            <div class="cz_button">
                                <el-button type="danger" size="small "
                                           @click="delrow(scope.row.id, 2)">{yun:}t key='common.delete'{/yun}
                                </el-button>
                            </div>
                        </template>
                    </el-table-column>
                </el-table>
                </div>
                <div class="modulePaging" style="padding-left: 20px;">
                    <div class="">
                        <div class="">
                            <el-checkbox v-model="checkedSubTblAll" @change="selectSubTblAllBottom">{yun:}t key='wap_js_00074'{/yun}</el-checkbox>
                            <el-button @click="delAllBottom(2)" size="mini">{yun:}t key='member_com_00055'{/yun}</el-button>
                        </div>
                    </div>
                </div>
            </div>
        </el-drawer>
        <!--修改场地、新增场地-->
        <div class="modluDrawer">
            <el-drawer :title="curr_data.id ? lc('admin_vue_00121') : lc('admin_00829')" :visible.sync="draweradd" append-to-body :modal-append-to-body="false" :show-close="true"
                       :with-header="true" size="50%">
                <div class="processDrawer"  style="overflow-y: auto; position: relative; width: 100%; height: 100%;">
                    <div>
                        <div class="drawerModLis">
                            <div class="drawerModTite">
                                <span>{yun:}t key='admin_00292'{/yun}</span>
                            </div>
                            <div class="drawerModInpt">
                                <el-input v-if="curr_data.id == ''" type="textarea" :rows="2" placeholder="{yun:}t key='admin_00288'{/yun}" v-model="curr_data.name">
                                </el-input>
                                <el-input v-else placeholder="{yun:}t key='admin_00288'{/yun}" v-model="curr_data.name">
                                </el-input>
                            </div>
                            <div class="drawerModTips" v-if="curr_data.id">
                                <el-alert title="{yun:}t key='admin_00834'{/yun}" type="info" show-icon :closable="false">
                                </el-alert>
                            </div>
                        </div>
                        <div class="drawerModLis" v-if="curr_data.id == ''">
                            <div class="drawerModTite">
                                <span>{yun:}t key='admin_00290'{/yun}</span>
                            </div>
                            <div class="drawerModInpt">
                                <el-select v-model="fir_id" placeholder="{yun:}t key='wap_user_00100'{/yun}" @change="firChange">
                                    <el-option label="{yun:}t key='wap_user_00100'{/yun}" value="">
                                    </el-option>
                                    <el-option v-for="item in tableData" :key="item.id" :label="item.name" :value="item.id">
                                    </el-option>
                                </el-select>
                            </div>
                            <div class="drawerModTips">
                                <el-alert title="{yun:}t key='admin_00835'{/yun}" type="info" show-icon :closable="false">
                                </el-alert>
                            </div>
                        </div>
                        <div class="drawerModLis" v-if="curr_data.id == ''">
                            <div class="drawerModTite">
                                <span>{yun:}t key='admin_00291'{/yun}</span>
                            </div>
                            <div class="drawerModInpt">
                                <el-select v-model="sec_id" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                    <el-option label="{yun:}t key='wap_user_00100'{/yun}" value="">
                                    </el-option>
                                    <el-option v-for="item in secClass" :key="item.id" :label="item.name" :value="item.id">
                                    </el-option>
                                </el-select>
                            </div>
                            <div class="drawerModTips">
                                <el-alert title="{yun:}t key='admin_00836'{/yun}" type="info" show-icon
                                          :closable="false">
                                </el-alert>
                            </div>
                        </div>
                        <div class="drawerModLis" v-if="curr_data.keyid == '' || fir_id == ''">
                            <div class="drawerModTite">
                                <span>{yun:}t key='admin_00289'{/yun}</span>
                            </div>
                            <div class="drawerModInpt">
                                <el-upload class="avatar-uploader" :action="uploadAction" :accept="pic_accept" :show-file-list="false" :on-change="picChange">
                                    <img style="width:200px;" v-if="curr_data.pic_n" :src="curr_data.pic_n" class="avatar">
                                    <el-button v-else type="primary" icon="el-icon-document-add">{yun:}t key='wap_00540'{/yun}</el-button>
                                </el-upload>
                            </div>
                        </div>
                        <div class="drawerModLis" v-if="curr_data.keyid != '' || fir_id != ''">
                            <div class="drawerModTite">
                                <span>{yun:}t key='wap_00563'{/yun}</span>
                            </div>
                            <div class="drawerModInpt">
                                <el-input v-model="curr_data.price" placeholder="{yun:}t key='admin_00837'{/yun}"
                                          @input="inputIntNumber($event, 'curr_data', 'price')">
                                    <template slot="append">{yun:}t key='wap_user_00008'{/yun}</template>
                                </el-input>
                            </div>
                        </div>
                        <div class="drawerModLis">
                            <div class="drawerModTite">
                                <span>{yun:}t key='member_com_00022'{/yun}</span>
                            </div>
                            <div class="drawerModInpt">
                                <el-input v-model="curr_data.sort" placeholder="{yun:}t key='admin_00814'{/yun}"
                                          @input="inputIntNumber($event, 'curr_data', 'sort')"></el-input>
                            </div>
                            <div class="drawerModTips">
                                <el-alert title="{yun:}t key='admin_00218'{/yun}" type="info" show-icon :closable="false">
                                </el-alert>
                            </div>
                        </div>
                        <div class="drawerModLis">
                            <div class="drawerModTite">
                                <span>{yun:}t key='admin_00831'{/yun}</span>
                            </div>
                            <div class="drawerModInpt">
                                <div style="border: 1px solid #ccc;">
                                    <div id="toolbar-container"><!-- 工具栏 --></div>
                                    <div id="editor-container" style="height: 300px;"><!-- 编辑器 --></div>
                                </div>
                            </div>
                        </div>
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
    let editor = null, toolbar = null,editorInterval = null;
    const {createEditor, createToolbar} = window.wangEditor;
    module.exports = {
        data: function () {
            return {
                pic_accept: localStorage.getItem("pic_accept"),
                emptytext: "{yun:}t key='wap_js_00113'{/yun}",
                loading: false,
                submitLoading: false,
                srcList: [],
                keyword: '',
                checkedAll: false,
                checkedSubTblAll: false,
                selectedItem: [],
                tableData: [],
                spaceTableData: [],
                curr_data: {},
                draweradd: false,
                drawerdtl: false,
                piclist: [],
                fir_id: '',
                sec_id: '',
                secClass: [],
                tableHig: true,
                del_type: 1,
                del_id: '',
                islook: false,
                uploadAction: baseUrl + 'm=common&c=common_upload'
            }
        },
        mounted() {

        },
        beforeDestroy() {
            editor = null; 
            toolbar = null;
            editorInterval = null;
        },
        methods: {
            todtl(id){
                this.del_id = id
                this.getdtlinfo()
            },
            getdtlinfo(){
                var that = this
                that.loading = true;
                that.emptytext = "{yun:}t key='admin_user_weipin_00026'{/yun}";
                httpPost('m=neirong&c=zph_space&a=up', {id: that.del_id}, {hideloading: true}).then(function (response) {
                    let res = response.data;
                    if (res.error === 0) {
                        that.spaceTableData = res.data
                        if (that.spaceTableData.length > 0) {
                            that.drawerdtl = true
                        } else {
                            that.drawerdtl = false
                        }
                        that.loading = false;
                        if (that.spaceTableData.length === 0){
                            that.emptytext = "{yun:}t key='wap_js_00113'{/yun}";
                        }
                    } else {
                        message.error("{yun:}t key='admin_00187'{/yun}");
                    }
                }).catch(function (error) {
                    console.log(error);
                });
            },
            handleSubTblSelectionChange(val) {
                this.selectedItem = [];
                let _this = this;
                if (val.length) {
                    val.forEach(item => {
                        _this.selectedItem.push(item.id);
                    });
                }
                if (_this.selectedItem.length == 0) {
                    _this.checkedSubTblAll = false;
                } else {
                    var all_len = 1
                    if (this.spaceTableData[0].children) {
                        this.spaceTableData[0].children.forEach((row) => {
                            if (row.children) {
                                all_len += row.children.length
                            }
                        })
                        all_len += this.spaceTableData[0].children.length
                    }
                    if (_this.selectedItem.length == all_len) {
                        _this.checkedSubTblAll = true;
                    } else {
                        _this.checkedSubTblAll = false;
                    }
                }
            },
            selectSubTblAllBottom(value) {
                // 处理嵌套数据选中
                if (this.spaceTableData[0].children) {
                    this.spaceTableData[0].children.forEach((row) => {
                        if (row.children) {
                            row.children.forEach((subrow) => {
                                if(this.selectedItem.indexOf(subrow.id) < 0) {// 三级分类选中处理
                                    this.$refs.multipleSubTable.toggleRowSelection(subrow, value)
                                    if (value) {
                                        this.selectedItem.push(subrow.id)
                                    }
                                }
                            })
                        }
                        if(this.selectedItem.indexOf(row.id) < 0) {// 二级分类选中处理
                            this.$refs.multipleSubTable.toggleRowSelection(row, value)
                            if (value) {
                                this.selectedItem.push(row.id)
                            }
                        }
                    })
                }
                if (!value) {
                    this.selectedItem = []
                }
                value ? this.$refs.multipleSubTable.toggleAllSelection() : this.$refs.multipleSubTable.clearSelection();
            },
            editData(scope, type) {
                let index = scope.$index;
                let row = scope.row;
                let column = scope.column;
                this.oldData = JSON.parse(JSON.stringify(row));
                let copyRow = JSON.parse(JSON.stringify(row));
                if (type == 1) {
                    copyRow[column.property + "isShow"] = true;
                    this.$set(this.tableData, index, copyRow);
                } else {
                    if (row.children) {
                        if (row.keyid == '') {// 一级分类修改
                            this.spaceTableData[0][column.property + "isShow"] = true;
                        } else {// 二级分类修改
                            this.spaceTableData[0].children.forEach((item)=>{
                                if (item.id == row.id) {
                                    item[column.property + "isShow"] = true
                                }
                            })
                        }
                    } else {// 三级分类修改
                        this.spaceTableData[0].children.forEach((item)=>{
                            if (item.id == row.keyid) {
                                item.children.forEach((subitem)=>{
                                    if (subitem.id == row.id) {
                                        subitem[column.property + "isShow"] = true
                                    }
                                })
                            }
                        })
                    }
                    var tmp = deepClone(this.spaceTableData)
                    this.spaceTableData = tmp
                }
                this.$nextTick(() => {
                    let ref = null
                    if (type == 1) {
                        ref = column.property + index;
                    } else {
                        ref = 'edit'+column.property + index;
                    }
                    $("#" + ref).focus();
                });
            },
            alterData(scope, type) {
                if (this.oldData == null) {
                    return false;
                }
                let index = scope.$index;
                let row = scope.row;
                let column = scope.column;
                let copyRow = JSON.parse(JSON.stringify(row));
                if (type == 1) {
                    copyRow[column.property + "isShow"] = false;
                    this.$set(this.tableData, index, copyRow);
                } else {// 管理弹窗表格修改名称、排序
                    if (row.children) {
                        if (row.keyid == '') {// 一级分类修改
                            this.spaceTableData[0][column.property + "isShow"] = false;
                        } else {// 二级分类修改
                            this.spaceTableData[0].children.forEach((item)=>{
                                if (item.id == row.id) {
                                    item[column.property + "isShow"] = false
                                }
                            })
                        }
                    } else {// 三级分类修改
                        this.spaceTableData[0].children.forEach((item)=>{
                            if (item.id == row.keyid) {
                                item.children.forEach((subitem)=>{
                                    if (subitem.id == row.id) {
                                        subitem[column.property + "isShow"] = false
                                    }
                                })
                            }
                        })
                    }
                    var tmp = deepClone(this.spaceTableData)
                    this.spaceTableData = tmp
                }
                if (row[column.property] === this.oldData[column.property]) {
                    return false;
                }
                let _this = this;
                let sendData = {id: row.id};
                sendData[column.property] = row[column.property];
                httpPost('m=neirong&c=zph_space&a=ajax', sendData, {hideloading: true}).then(function (response) {
                    let res = response.data;
                    if (res.error === 0) {
                        message.success("{yun:}t key='admin_user_company_00208'{/yun}");
                    } else {
                        message.error("{yun:}t key='admin_00187'{/yun}");
                    }
                    _this.oldData = null;
                    // if (type == 1) {
                    _this.getList();
                    // }
                }).catch(function (error) {
                    console.log(error);
                });
            },
            // 一级分类修改
            firChange(data){
                var that = this
                that.sec_id = ''
                that.secClass = []
                if (data) {
                    httpPost('m=neirong&c=zph_space&a=ajaxspace", {id: data}).then(function (result) {
                        var res = result.data
                        if (res.error == 0) {
                            that.secClass = res.data
                        }
                    }).catch(function (e) {
                        console.log(e)
                    })
                }
            },
            // {yun:}t key='admin_00289'{/yun}
            picChange(file) {
                var tmp = deepClone(this.curr_data)
                // 预览文件处理
                tmp.pic_n = URL.createObjectURL(file.raw);
                // 复刻文件信息
                this.piclist[0] = file.raw;
                this.curr_data = tmp
            },
            initEditor(content=null) {
                var that = this
                clearInterval(editorInterval);
                editorInterval = setInterval(()=>{
                    if (editor !== null){
                        clearInterval(editorInterval);
                        if(content!==null){
                            editor.setHtml(content);
                        }
                    }else{
                        let editorConfig = {
                            MENU_CONF: {
                                uploadImage: {
                                    server: baseUrl + "m=index&c=uploadfile',
                                    fieldName: 'file"
                                }
                            }
                        };
                        // {yun:}t key='wap_00566'{/yun}
                        if (!editor) {
                            editor = createEditor({
                                selector: "#editor-container',
                                html: '',
                                config: editorConfig,
                                mode: 'simple'
                            });
                        }
                        if (!toolbar) {
                            toolbar = createToolbar({
                                editor: editor,
                                selector: '#toolbar-container',
                                config: {
                                    excludeKeys: ['blockquote', 'header1', 'header2', 'header3', '|', 'through', 'todo', '|', 'insertVideo', 'insertTable', 'codeBlock', '|', 'undo', 'redo', '|',]
                                },
                                mode: 'simple'
                            });
                        }
                    }
                },300);
                
            },
            toadd(data){
                var that = this
                httpPost('m=neirong&c=zph_space&a=add', {add:1}).then(function (response) {
                    if (data.id) {
                        that.curr_data = deepClone(data)
                    } else {
                        that.fir_id = ''
                        that.sec_id = ''
                        that.curr_data = {id: '', content: '', keyid: ''}
                    }
                    that.draweradd = true
                    that.initEditor(that.curr_data.content);
                }).catch(function (error) {
                    console.log(error);
                }).finally(function () {
                    that.submitLoading = false;
                });
            },
            search() {
                this.currentPage = 1;
                this.getList();
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
            async getList() {
                let that = this;
                let params = {}
                if (that.keyword) {
                    params.keyword = that.keyword
                }
                that.loading = true;
                that.emptytext = "{yun:}t key='admin_user_weipin_00026'{/yun}";
                httpPost('m=neirong&c=zph_space&a=index', params, {hideloading: true}).then(function (result) {
                    var res = result.data
                    if (res.error == 0) {
                        that.tableData = res.data.list
                        that.srcList = res.data.pics
                        that.loading = false;
                        if (that.tableData.length === 0){
                            that.emptytext = "{yun:}t key='wap_js_00113'{/yun}";
                        }
                    }
                }).catch(function (e) {
                    console.log(e)
                })
            },
            delrow(id, type) {
                this.del_type = type
                delConfirm(this, id, this.delete);
            },
            delAllBottom(type) {
                if (!this.selectedItem.length) {
                    message.error("{yun:}t key='admin_00136'{/yun}");
                    return false;
                }
                this.del_type = type
                delConfirm(this, this.selectedItem, this.delete);
            },
            async delete(id) {
                let that = this;
                let params = {
                    del: id
                };
                httpPost('m=neirong&c=zph_space&a=del', params).then(function (response) {
                    if (response.data.error == 0) {
                        message.success(response.data.msg, function(){
                            if (that.del_type == 2) {
                                that.getdtlinfo();
                            }
                            that.getList();
                        });
                    } else {
                        message.error(response.data.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                })
            },
            inputIntNumber(val, form, key) {
                this.$data[form][key] = val.replace(/[^0-9]/g,'');
            },
            async save() {
                let that = this;
                if (that.curr_data.name == '') {
                    message.error(lc('admin_vue_00071'));
                    return false;
                }
                that.curr_data.content = editor.getHtml()
                if (that.fir_id) {
                    that.curr_data.keyid = that.fir_id
                }
                if (that.sec_id) {
                    that.curr_data.keyid = that.sec_id
                }
             
                if (that.piclist.length) {
                    that.curr_data.pic = this.piclist[0];
                }
                let params = that.curr_data;
                var formData = new FormData();
                Object.keys(params).forEach((key) => {
                    if (Array.isArray(params[key])) {
                        params[key].forEach((v) => {
                            formData.append(key + '[]', v);
                        });
                    } else {
                        formData.append(key, params[key]);
                    }
                });
                this.submitLoading = true;

                httpPost('m=neirong&c=zph_space&a=add', formData).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        message.success(res.msg, function () {
                            that.getdtlinfo()
                            that.getList();
                        });
                    } else {
                        message.error(res.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                }).finally(function () {
                    that.submitLoading = false;
                });
                that.draweradd = false

            },
        },
    };
</script>
<style scoped></style>