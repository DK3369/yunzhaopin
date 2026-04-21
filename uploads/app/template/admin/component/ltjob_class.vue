<template>
    <div v-loading="loading">
        <div style="overflow: hidden; position: relative; display: flex; align-items: center;">
        <el-select v-model="jobId" multiple :multiple-limit="multiple ? max : 1" placeholder="搜索职位名称"
                   filterable remote :remote-method="remoteClassList" @change="classChange" @remove-tag="classRemove">
            <el-option v-for="opitem in classOptions" :key="opitem.id" :label="opitem.name"
                       :value="opitem.id" :disabled="opitem.disabled">
                <span :style="jobId.indexOf(opitem.id) > -1 ? 'color:#409eff' : ''">
                    <span style="float: left; font-size: 14px;font-weight:bold;">{{opitem.name}}</span>
                    <span style="float: right; color: #a5a5a5; font-size: x-small;" v-if="opitem.upname!=''">{{opitem.upname}}</span>
                </span>
            </el-option>
        </el-select>
        <div slot="prefix">
            <el-button type="text" icon="el-icon-s-operation" style="width:25px; margin-right: 25px;"
                       @click="jobOpen"></el-button>
        </div>
        </div>

        <!--请选择猎头职位类别-->
        <div class="modluDrawer">
            <el-drawer :visible.sync="jobVisible" :with-header="false" :modal-append-to-body="false" append-to-body
                       :show-close="true" size="60%">
                <div class="modluDrawerContents">
                    <div class="modluDrawerTi9te">
                        <div>请选择擅长职位</div>
                        <div class="shuytans">
                            <el-input v-model="searchJob" placeholder="搜索职位名称"
                                      @input="handleSearchJob">
                                <i slot="prefix" class="el-input__icon el-icon-search"></i>
                            </el-input>
                        </div>
                        <button aria-label="close drawer" type="button" class="el-drawer__close-btn"
                                style="right: 2px;position: absolute;" @click="jobVisible = false"><i
                                class="el-dialog__close el-icon el-icon-close"></i></button>
                    </div>
                    <div class="xuanzleibie" v-if="classList.length > 0">
                        <ul>
                            <template v-for="(oneItem, oneIndex) in classList">
                                <li v-if="!oneItem.hide" :key="oneIndex">
                                    <!--第一级-->
                                    <div class="xuanzlOne pointer" :data-id="oneItem.id" :data-name="oneItem.name"
                                        :data-one="oneIndex" :data-level="1"
                                        :class="selectJobId.indexOf(oneItem.id) > -1 ? 'class-selected' : ''"
                                        @click="handleSelectJob">{{ oneItem.name }}</div>
                                    <div class="xuanzlTwo">
                                        <!--第二级-->
                                        <template v-for="(twoItem, twoIndex) in oneItem.children">
                                            <div v-if="!twoItem.hide" :key="twoIndex" class="xuanzlTwoList">
                                                <div class="xuanzNamte blue">
                                                    <!--<i class="el-icon-remove"></i>-->
                                                    <span v-if="multiple && selectJobId.indexOf(oneItem.id) > -1"
                                                        class="class-disabled">{{ twoItem.name }}</span>
                                                    <span v-else :data-id="twoItem.id" :data-name="twoItem.name"
                                                        :data-one="oneIndex" :data-two="twoIndex" :data-level="2"
                                                        :class="selectJobId.indexOf(twoItem.id) > -1 ? 'class-selected' : ''"
                                                        @click="handleSelectJob">{{ twoItem.name }}</span>
                                                </div>
                                            </div>
                                        </template>
                                    </div>
                                </li>
                            </template>
                        </ul>
                    </div>
                    <div v-else class="noneResults">
                        <div>
                            <el-empty description="抱歉，没有找到结果！"></el-empty>
                            
                        </div>
                    </div>
                    <div slot="footer" class="dialog-footer dialoFoofetee">
                        <div class="footText">
                            <div class="mingdsc"><span>最多可选择{{ multiple ? max : '1'}}项：</span></div>
                            <div class="mingdEltags" style="padding-top: 4px;">
                                <el-tag v-for="(selectClass, selectIndex) in selectJobClass" :key="selectIndex"
                                        closable size="small" @close="handleCloseJob(selectClass.id)">
                                    {{ selectClass.name }}
                                </el-tag>
                            </div>
                        </div>
                        <div class="footTextburn">
                            <el-button type="primary" size="mini" round @click="handleSubmitJob">确 定</el-button>
                        </div>
                    </div>
                </div>
            </el-drawer>
        </div>
    </div>
</template>
<script>
    module.exports = {
        props: {
            multiple: {type: Boolean, default: false}, // 选择方式 false-单选/true-多选
            max: {type: Number, default: 5}, // 多选下有效，最多选择几个
            selected: {type: Object, default: null} // 已选中数据，数据内容如：{167: "通信技术工程师", 168: "有线传输工程师"}
        },
        data: function () {
            return {
                loading: true,

                classList: [],

                jobId: [],
                jobClass: [],

                classOptions: [],

                jobVisible: false,
                searchJob: '',
                selectJobId: [],
                selectJobClass: [],

                timer: null,
            }
        },
        created() {
            this.getClassList();
            this.handleSelected();
        },
        methods: {
            // 首次加载分类
            getClassList() {
                let that = this,
                    params = {};

                httpPost('m=common&c=cache&a=getltjob', params, {hideLoading: true}).then(function (response) {
                    let res = response.data,
                        classList = res.data.classList;

                    that.classList = classList && classList.length > 0 ? classList : [];
                    that.loading = false;
                })
            },

            // 搜索分类 - 下拉框
            remoteClassList(query) {
                if ($.trim(query) !== '') {
                    let that = this;

                    that.searchClass(query); // 本地JS搜索

                    let classList = deepClone(that.classList);

                    // 层级数据转为一级数据
                    if (classList && classList.length > 0) {
                        let newClassList = [],
                            newClassId = [];
                        classList.forEach(function (oneItem, oneIndex) {
                            if (oneItem.name.includes(query)) { // 一级须包含关键字
                                newClassList.push({
                                    id: oneItem.id,
                                    name: oneItem.name,
                                    upname: '',
                                    childrenIds: oneItem.children && oneItem.children.length > 0 ? oneItem.children.map(row => row.id) : []
                                })
                                newClassId.push(oneItem.id); // 用来判断存在一级，二级不显示一级的名称
                            }
                            if (oneItem.children && oneItem.children.length > 0) { // 存在二级
                                oneItem.children.forEach(function (twoItem, twoIndex) {
                                    if (twoItem.name.includes(query)) { // 二级须包含销售关键字
                                        newClassList.push({
                                            id: twoItem.id,
                                            name: twoItem.name,
                                            disabled: that.jobId.indexOf(oneItem.id) !== -1, // 选中一级，二级禁用选择
                                            upname: newClassId.indexOf(oneItem.id) === -1 ? oneItem.name : ''
                                        })
                                    }
                                })
                            }
                        })
                        that.classOptions = newClassList;
                    } else {
                        that.classOptions = [];
                    }
                } else {
                    this.classOptions = [];
                }
            },

            // 分类变更
            async classChange(val) {
                let classOptions = this.classOptions,
                    valLen = val.length,
                    id = val[valLen-1],
                    jobClass = this.jobClass
                    jobClassLen = jobClass.length;

                if (jobClassLen > valLen) { // 删除
                    for (var i = 0; i < jobClassLen; i++) {
                        if (val.indexOf(jobClass[i].id) === -1) { // 未在已选中列表的，清除
                            this.jobClass.splice(i, 1);
                            break;
                        }
                    }
                } else { // 增加
                    for (var i = 0; i < classOptions.length; i++) {
                        if (classOptions[i].id == id) { // 获取选中值数据
                            if (this.multiple) {
                                this.jobClass.push({id: classOptions[i].id, name: classOptions[i].name});

                                let childrenIds = classOptions[i].childrenIds,
                                    index = -1;
                                if (childrenIds && childrenIds.length > 0 && this.jobId.length > 0) { // 清空下级已选选项
                                    for (var j = 0; j < childrenIds.length; j++) {
                                        index = this.jobId.indexOf(childrenIds[j]);
                                        if (index > -1) { // 检索已选中下级
                                            this.jobId.splice(index, 1); // 删除下级
                                            this.jobClass.splice(index, 1);
                                        }
                                    }
                                }
                            } else {
                                this.jobClass = [{id: classOptions[i].id, name: classOptions[i].name}];
                            }
                            break;
                        }
                    }
                }

                this.$emit("confirm", {jobId: this.jobId});
            },
            // 移除分类
            classRemove(val) {
                let that = this;

                that.jobClass.forEach(function(item, index){
                    if (val == item.id) {
                        that.jobClass.splice(index, 1);
                    }
                })
            },

            // 打开弹窗
            jobOpen() {
                this.jobVisible = true;
                if (this.jobId.length > 0) {
                    this.selectJobId = deepClone(this.jobId);
                    this.selectJobClass = deepClone(this.jobClass);
                } else {
                    this.selectJobId = [];
                    this.selectJobClass = [];
                }
                if (this.searchJob !== '') { // 清空搜索内容
                    this.searchJob = '';
                }
                this.searchClass(''); // 重新整理分类
            },

            // 选中分类
            async handleSelectJob(event) {
                let that = this,
                    dataset = event.currentTarget.dataset,
                    id = dataset.id,
                    name = dataset.name,
                    selectJobId = this.selectJobId,
                    max = that.max,
                    index = selectJobId.indexOf(id),
                    level = dataset.level,
                    one = dataset.one;

                if (index > -1) { // 重复点击,取消选中
                    that.selectJobId.splice(index, 1);
                    that.selectJobClass.splice(index, 1);
                    return true;
                }

                if (that.multiple) { // 多选
                    if (level == 1) { // 选择一级 清空二级已选选项
                        that.handleSelectClass(one);
                    }

                    if (selectJobId.length >= max) {
                        message.warning('最多选择' + max + '项');
                        return false;
                    }
                    that.selectJobId.push(id);
                    that.selectJobClass.push({id: id, name: name});
                } else { // 单选
                    that.selectJobId = [id];
                    that.selectJobClass = [
                        {id: id, name: name}
                    ]; // 单选覆盖选中值
                }
            },
            // 删除选中分类
            handleCloseJob(id) {
                let index = this.selectJobId.indexOf(id);

                if (index > -1) {
                    this.selectJobId.splice(index, 1);
                    this.selectJobClass.splice(index, 1);
                }
            },
            /**
             * 下级分类处理
             * @params ids 所有下级ID
             */
            handleSelectClass(one) {
                let that = this,
                    classList = that.classList,
                    twoClassList = classList[one]['children'];

                if (twoClassList && twoClassList.length > 0 && that.selectJobId.length > 0) { // 一级选中，清空二级已选选项
                    twoClassList.forEach(function (item, index) {
                        twoIndex = that.selectJobId.indexOf(item.id);
                        if (twoIndex > -1) { // 检索已选中二级
                            that.selectJobId.splice(twoIndex, 1); // 删除二级
                            that.selectJobClass.splice(twoIndex, 1);
                        }
                    })
                }
            },

            // 弹出层搜索
            handleSearchJob() {
                this.debouncedSearchHandler();
            },
            debouncedSearchHandler() {
                let that = this;
                if (that.timer) {
                    clearTimeout(that.timer);
                }
                that.timer = setTimeout(() => {
                    that.searchClass(that.searchJob);
                    that.timer = null;
                }, 500); // 延迟时间设置为500毫秒
            },

            // 搜索分类
            searchClass(query) {
                let that = this,
                    classList = deepClone(that.classList),
                    twoList = [];

                if (classList && classList.length > 0) {
                    classList.forEach(function(oneItem, oneKey) {
                        if (oneItem.name.includes(query)) { // 一级须包含关键字
                            classList[oneKey].hide = false; // 显示一级分类
                        } else {
                            classList[oneKey].hide = true; // 隐藏一级分类
                        }
                        twoList = oneItem.children;
                        if (twoList && twoList.length > 0) {
                            twoList.forEach(function(twoItem, twoKey) {
                                if (twoItem.name.includes(query)) { // 二级须包含关键字
                                    classList[oneKey].hide = false; // 存在二级一级也显示
                                    classList[oneKey]['children'][twoKey].hide = false; // 二级分类标记显示
                                } else {
                                    classList[oneKey]['children'][twoKey].hide = true; // 隐藏二级分类
                                }
                            })
                        }
                    })
                    that.classList = classList;
                }
            },

            // 确认选中分类
            handleSubmitJob() {
                let that = this;

                // 调用引入页面方法,自行处理数据
                this.jobId = deepClone(this.selectJobId);
                this.jobClass = deepClone(this.selectJobClass);
                this.classOptions = deepClone(this.selectJobClass);

                let timer = setTimeout(() => {
                    that.classOptions = [];
                    timer = null;
                }, 500); // 清空搜索下拉选项

                this.jobVisible = false;
                this.$emit("confirm", {jobId: this.jobId});
            },

            // 处理选中值
            handleSelected() {
                let that = this,
                    selected = this.selected;

                if (this.searchJob !== '') { // 上一次有搜索过，再次打开重新加载分类数据
                    this.searchJob = '';
                    this.searchClass('');
                }

                this.selectJobId = [];
                this.selectJobClass = [];
                this.jobId = [];
                this.jobClass = [];
                this.classOptions = [];

                if (selected) {
                    for (let key in selected) {
                        this.selectJobId.push(key);
                        this.selectJobClass.push({id: key, name: selected[key]});
                    }
                    this.jobId = deepClone(this.selectJobId);
                    this.jobClass = deepClone(this.selectJobClass);
                    this.classOptions = deepClone(this.selectJobClass);

                    let timer = setTimeout(() => {
                        that.classOptions = [];
                        timer = null;
                    }, 500); // 清空搜索下拉选项
                }
            },
        },
        watch: {
            selected: function(val, oldVal) {
                this.handleSelected();
            }
        }
    }
</script>
<style scoped>
    .uploadTable {
        width: calc(100% - 40px);
    }

    .moreTop {
        padding-top: 10px;
    }

    .titleTwoSpace {
        padding-left: 50px;
    }

    .moreInOne {
        display: flex;
    }

    .fw {
        font-weight: 900;
        color: #0a0a0a;
    }
    .mingdEltags{
        overflow: hidden;
        position: relative;
        display: flex;
        align-items: center;
        padding-top: 3px;
    }
    .mingdEltags .el-tag{
        overflow: hidden;
        position: relative;
        margin: 3px 4px !important;
    }
</style>