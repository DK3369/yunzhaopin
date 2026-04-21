<template>
    <div v-loading="loading" style="overflow: hidden; position: relative; width: 100%;">
        <div class="mplatejdshds">
            <el-select v-model="jobId" size="small" multiple :multiple-limit="multiple ? max : 1" placeholder="搜索职位类别名称"
                       filterable remote :remote-method="remoteClassList" @change="classChange" @remove-tag="classRemove" v-if="showsearch==true">
                <el-option v-for="opitem in classOptions" :key="opitem.id" :label="opitem.name"
                           :value="opitem.id" :disabled="opitem.disabled" >
                    <span :style="jobId.indexOf(opitem.id) > -1 ? 'color:#409eff' : ''">
                        <span style="float: left; font-size: 14px;font-weight:bold;">{{opitem.name}}</span>
                        <span style="float: right; color: #a5a5a5; font-size: x-small;" v-if="opitem.upname!=''">{{opitem.upname}}</span>
                    </span>
                </el-option>
            </el-select>
            <el-input style="cursor: pointer;" :readonly="true" placeholder="设置不强制填写职位类别项" v-else> </el-input>
            <div slot="prefix">
                <el-button type="text" icon="el-icon-s-operation" style="width:25px; margin-right: 25px;"
                       @click="jobOpen"></el-button>
            </div>
        </div>

        <!--请选择职位类别-->
        <div class="modluDrawer">
            <el-drawer :visible.sync="jobVisible" :with-header="false" :modal-append-to-body="false" append-to-body
                       :show-close="true" size="80%">
                <div class="modluDrawerContents">
                    <div class="modluDrawerTi9te">
                        <div>请选择职位类别</div>
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
                            <li v-for="(oneItem, oneIndex) in classList" :key="oneIndex">
                                <!--第一级-->
                                <div class="xuanzlOne">{{ oneItem.name }}</div>
                                <div class="xuanzlTwo">
                                    <div v-for="(twoItem, twoIndex) in oneItem.children" :key="twoIndex" class="xuanzlTwoList">
                                        <!--有第三级-->
                                        <el-popover v-if="twoItem.children" placement="bottom" width="350" trigger="click">
                                            <div class="xuanzlTwoCont" v-loading="twoItem.children.length == 0">
                                                <!--第二级-->
                                                <div class="xuanzlTwoBit">
                                                    <i class="el-icon-remove"></i>
                                                    <span :data-id="twoItem.id" :data-name="twoItem.name"
                                                          :data-one="oneIndex" :data-two="twoIndex" :data-level="2"
                                                          :class="selectJobId.indexOf(twoItem.id) > -1 ? 'class-selected' : ''"
                                                          @click="handleSelectJob">{{ twoItem.name }}</span>
                                                </div>
                                                <!--第三级-->
                                                <div class="xuanzlTwoTips">
                                                    <template v-for="(threeItem, threeIndex) in twoItem.children">
                                                        <span v-if="multiple && selectJobId.indexOf(twoItem.id) > -1" :key="threeIndex"
                                                              class="class-disabled">{{ threeItem.name }}</span>
                                                        <span v-else :data-id="threeItem.id" :data-name="threeItem.name" :key="'else'+threeIndex"
                                                              :data-one="oneIndex" :data-two="twoIndex"
                                                              :data-three="threeIndex" :data-level="3"
                                                              :class="selectJobId.indexOf(threeItem.id) > -1 ? 'class-selected' : ''"
                                                              @click="handleSelectJob">{{ threeItem.name }}</span>
                                                    </template>
                                                </div>
                                            </div>
                                            <div slot="reference" class="xuanzNamte" :data-id="twoItem.id"
                                                 :data-one="oneIndex" :data-two="twoIndex" @click="childClassList">
                                                <i class="el-icon-circle-plus"></i>
                                                <span :class="selectJobId.indexOf(twoItem.id) > -1 ? 'class-selected' : ''"
                                                      >{{ twoItem.name }}</span>
                                            </div>
                                        </el-popover>
                                        <!--无第三级-->
                                        <div v-else class="xuanzNamte blue">
                                            <i class="el-icon-remove"></i>
                                            <span :data-id="twoItem.id" :data-name="twoItem.name"
                                                  :data-one="oneIndex" :data-two="twoIndex" :data-level="2"
                                                  :class="selectJobId.indexOf(twoItem.id) > -1 ? 'class-selected' : ''"
                                                  @click="handleSelectJob">{{ twoItem.name }}</span>
                                        </div>
                                    </div>
                                </div>
                            </li>
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
            selected: {type: Object, default: null}, // 已选中数据，数据内容如：{167: "通信技术工程师", 168: "有线传输工程师"}
            showsearch: {type: Boolean, default: true},
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
            // 首次加载分类(一级和二级)
            getClassList() {
                let that = this,
                    params = {};

                if (that.searchJob !== '') {
                    params.name = that.searchJob;
                } else {
                    params.level = 2;
                }

                httpPost('m=common&c=cache&a=getJobClass', params, { hideloading: true }).then(function (response) {
                    let res = response.data,
                        classList = res.data.classList;

                    that.classList = classList && classList.length > 0 ? classList : [];
                    that.loading = false;
                })
            },

            // 搜索分类
            remoteClassList(query) {
                if ($.trim(query) !== '') {
                    let that = this;

                    httpPost('m=common&c=cache&a=getJobClass', {name: query}, {hideLoading: true}).then(function (response) {
                        let res = response.data,
                            classList = res.data.classList;

                        // 层级数据转为一级数据
                        if (classList && classList.length > 0) {
                            let newClassList = [],
                                newClassId = [];
                            classList.forEach(function (oneItem, oneIndex) {
                                if (oneItem.children) { // 存在二级
                                    oneItem.children.forEach(function (twoItem, twoIndex) {
                                        if (twoItem.name.includes(query)) { // 二级须包含关键字
                                            newClassList.push({
                                                id: twoItem.id,
                                                name: twoItem.name,
                                                upname: oneItem.name
                                            })
                                            newClassId.push(twoItem.id); // 用来判断存在二级，三级不显示二级的名称
                                        }

                                        if (twoItem.children) { // 存在三级
                                            twoItem.children.forEach(function (threeItem, threeIndex) {
                                                newClassList.push({
                                                    id: threeItem.id,
                                                    name: threeItem.name,
                                                    disabled: that.jobId.indexOf(twoItem.id) !== -1, // 选中二级，三级禁止选择
                                                    upname: newClassId.indexOf(twoItem.id) === -1 ? twoItem.name : ''
                                                })
                                            })
                                        }
                                    })
                                }
                            })
                            that.classOptions = newClassList;
                        } else {
                            that.classOptions = [];
                        }
                    })
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

                                let childrenIds = await this.getJobChildIds(classOptions[i].id),
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
                if (this.searchJob !== '') { // 上一次有搜索过，再次打开重新加载分类数据
                    this.searchJob = '';
                    this.getClassList();
                }
            },

            // 获取子类
            childClassList(event) {
                let that = this,
                    dataset = event.currentTarget.dataset,
                    id = dataset.id,
                    oneIndex = dataset.one,
                    twoIndex = dataset.two;

                if (that.classList[oneIndex]['children'][twoIndex]['children'].length > 0) { // 存在子类,不在触发加载
                    return false;
                }

                httpPost('m=common&c=cache&a=getJobClass', {pid: id}, {hideLoading: true}).then(function (response) {
                    let res = response.data,
                        classList = res.data.classList;

                    that.classList[oneIndex]['children'][twoIndex]['children'] = classList && classList.length > 0 ? classList : false; // 默认为没有下级
                })
            },

            // 获取下级分类ID
            async getJobChildIds(pid) {
                let response = await httpPost('m=common&c=cache&a=getJobChildIds', {pid: pid}, {hideLoading: true});

                return response.data.data;
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
                    one = dataset.one,
                    two = dataset.two;

                if (index > -1) { // 重复点击,取消选中
                    that.selectJobId.splice(index, 1);
                    that.selectJobClass.splice(index, 1);
                    return true;
                }

                if (that.multiple) { // 多选
                    if (level == 2) { // 选择二级 清空三级已选选项
                        let twoClass = that.classList[one]['children'][two],
                            childrenIds = '';
                        if (typeof twoClass.childrenIds === 'undefined') {
                            childrenIds = await that.getJobChildIds(twoClass.id);
                            that.$set(that.classList[one]['children'][two], 'childrenIds', childrenIds);
                        } else {
                            childrenIds = twoClass.childrenIds;
                        }
                        that.handleSelectClass(childrenIds);
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
            handleSelectClass(ids) {
                let that = this,
                    index = -1;

                if (ids && ids.length > 0 && that.selectJobId.length > 0) { // 二级选中，清空下级已选选项
                    ids.forEach(function (id) {
                        index = that.selectJobId.indexOf(id);
                        if (index > -1) { // 检索已选中下级
                            that.selectJobId.splice(index, 1); // 删除下级
                            that.selectJobClass.splice(index, 1);
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
                    that.getClassList();
                    that.timer = null;
                }, 500); // 延迟时间设置为500毫秒
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
                    this.getClassList();
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
            },
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

    .moduleSeachbig .el-button:hover{
        color: initial;
    }
</style>