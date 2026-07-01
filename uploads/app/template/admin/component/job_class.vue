<template>
    <div v-loading="loading" style="overflow: hidden; position: relative; width: 100%;">
        <div class="mplatejdshds">
            <el-select v-model="jobId" size="small" multiple :multiple-limit="multiple ? max : 1" :placeholder="lc('admin_00055')"
                       filterable remote :remote-method="remoteClassList" @change="classChange" @remove-tag="classRemove" v-if="showsearch==true">
                <el-option v-for="opitem in classOptions" :key="opitem.id" :label="opitem.name"
                           :value="opitem.id" :disabled="opitem.disabled" >
                    <span :style="jobId.indexOf(opitem.id) > -1 ? 'color:#409eff' : ''">
                        <span style="float: left; font-size: 14px;font-weight:bold;">{{opitem.name}}</span>
                        <span style="float: right; color: #a5a5a5; font-size: x-small;" v-if="opitem.upname!=''">{{opitem.upname}}</span>
                    </span>
                </el-option>
            </el-select>
            <el-input style="cursor: pointer;" :readonly="true" :placeholder="lc('admin_00052')" v-else> </el-input>
            <div slot="prefix">
                <el-button type="text" icon="el-icon-s-operation" style="width:25px; margin-right: 25px;"
                       @click="jobOpen"></el-button>
            </div>
        </div>

        <!-- Select job category -->
        <div class="modluDrawer">
            <el-drawer :visible.sync="jobVisible" :with-header="false" :modal-append-to-body="false" append-to-body
                       :show-close="true" size="80%">
                <div class="modluDrawerContents">
                    <div class="modluDrawerTi9te">
                        <div>{{ lc('wap_com_00272') }}</div>
                        <div class="shuytans">
                            <el-input v-model="searchJob" :placeholder="lc('admin_00057')"
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
                                <!-- First level -->
                                <div class="xuanzlOne">{{ oneItem.name }}</div>
                                <div class="xuanzlTwo">
                                    <div v-for="(twoItem, twoIndex) in oneItem.children" :key="twoIndex" class="xuanzlTwoList">
                                        <!-- Has third level -->
                                        <el-popover v-if="twoItem.children" placement="bottom" width="350" trigger="click">
                                            <div class="xuanzlTwoCont" v-loading="twoItem.children.length == 0">
                                                <!-- Second level -->
                                                <div class="xuanzlTwoBit">
                                                    <i class="el-icon-remove"></i>
                                                    <span :data-id="twoItem.id" :data-name="twoItem.name"
                                                          :data-one="oneIndex" :data-two="twoIndex" :data-level="2"
                                                          :class="selectJobId.indexOf(twoItem.id) > -1 ? 'class-selected' : ''"
                                                          @click="handleSelectJob">{{ twoItem.name }}</span>
                                                </div>
                                                <!-- Third level -->
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
                                        <!-- No third level -->
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
                            <el-empty :description="lc('admin_00039')"></el-empty>
                            
                        </div>
                    </div>
                    <div slot="footer" class="dialog-footer dialoFoofetee">
                        <div class="footText">
                            <div class="mingdsc"><span>{{ lc('admin_00390') }}</span></div>
                            <div class="mingdEltags" style="padding-top: 4px;">
                                <el-tag v-for="(selectClass, selectIndex) in selectJobClass" :key="selectIndex"
                                        closable size="small" @close="handleCloseJob(selectClass.id)">
                                    {{ selectClass.name }}
                                </el-tag>
                            </div>
                        </div>
                        <div class="footTextburn">
                            <el-button type="primary" size="mini" round @click="handleSubmitJob">{{ lc('wap_com_00019') }}</el-button>
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
            multiple: {type: Boolean, default: false}, // Selection mode: false for single-select, true for multi-select
            max: {type: Number, default: 5}, // Maximum selection count for multi-select mode
            selected: {type: Object, default: null}, // Selected data, for example: {167: lc('common_01417'), 168: lc('admin_00056')}
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
            // Initial category load for first and second levels
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

            // Search categories
            remoteClassList(query) {
                if ($.trim(query) !== '') {
                    let that = this;

                    httpPost('m=common&c=cache&a=getJobClass', {name: query}, {hideLoading: true}).then(function (response) {
                        let res = response.data,
                            classList = res.data.classList;

                        // Flatten hierarchical data to first-level options
                        if (classList && classList.length > 0) {
                            let newClassList = [],
                                newClassId = [];
                            classList.forEach(function (oneItem, oneIndex) {
                                if (oneItem.children) { // Has second-level children
                                    oneItem.children.forEach(function (twoItem, twoIndex) {
                                        if (twoItem.name.includes(query)) { // Second-level item must contain the keyword
                                            newClassList.push({
                                                id: twoItem.id,
                                                name: twoItem.name,
                                                upname: oneItem.name
                                            })
                                            newClassId.push(twoItem.id); // Used to hide second-level name when second-level item already exists
                                        }

                                        if (twoItem.children) { // Has third-level children
                                            twoItem.children.forEach(function (threeItem, threeIndex) {
                                                newClassList.push({
                                                    id: threeItem.id,
                                                    name: threeItem.name,
                                                    disabled: that.jobId.indexOf(twoItem.id) !== -1, // Disable third-level selection when second level is selected
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

            // Category changed
            async classChange(val) {
                let classOptions = this.classOptions,
                    valLen = val.length,
                    id = val[valLen-1],
                    jobClass = this.jobClass
                    jobClassLen = jobClass.length;

                if (jobClassLen > valLen) { // Remove
                    for (var i = 0; i < jobClassLen; i++) {
                        if (val.indexOf(jobClass[i].id) === -1) { // Clear items that are no longer selected
                            this.jobClass.splice(i, 1);
                            break;
                        }
                    }
                } else { // Add
                    for (var i = 0; i < classOptions.length; i++) {
                        if (classOptions[i].id == id) { // Get selected value data
                            if (this.multiple) {
                                this.jobClass.push({id: classOptions[i].id, name: classOptions[i].name});

                                let childrenIds = await this.getJobChildIds(classOptions[i].id),
                                    index = -1;
                                if (childrenIds && childrenIds.length > 0 && this.jobId.length > 0) { // {{ lc('common_01285') }}
                                    for (var j = 0; j < childrenIds.length; j++) {
                                        index = this.jobId.indexOf(childrenIds[j]);
                                        if (index > -1) { // Find selected child item
                                            this.jobId.splice(index, 1); // Remove child item
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
            // Remove category
            classRemove(val) {
                let that = this;

                that.jobClass.forEach(function(item, index){
                    if (val == item.id) {
                        that.jobClass.splice(index, 1);
                    }
                })
            },

            // Open dialog
            jobOpen() {
                this.jobVisible = true;
                if (this.jobId.length > 0) {
                    this.selectJobId = deepClone(this.jobId);
                    this.selectJobClass = deepClone(this.jobClass);
                } else {
                    this.selectJobId = [];
                    this.selectJobClass = [];
                }
                if (this.searchJob !== '') { // Reload category data when the previous dialog session searched
                    this.searchJob = '';
                    this.getClassList();
                }
            },

            // Load child categories
            childClassList(event) {
                let that = this,
                    dataset = event.currentTarget.dataset,
                    id = dataset.id,
                    oneIndex = dataset.one,
                    twoIndex = dataset.two;

                if (that.classList[oneIndex]['children'][twoIndex]['children'].length > 0) { // Child categories already exist, skip loading
                    return false;
                }

                httpPost('m=common&c=cache&a=getJobClass', {pid: id}, {hideLoading: true}).then(function (response) {
                    let res = response.data,
                        classList = res.data.classList;

                    that.classList[oneIndex]['children'][twoIndex]['children'] = classList && classList.length > 0 ? classList : false; // Default to no child categories
                })
            },

            // Get child category IDs
            async getJobChildIds(pid) {
                let response = await httpPost('m=common&c=cache&a=getJobChildIds', {pid: pid}, {hideLoading: true});

                return response.data.data;
            },

            // Select category
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

                if (index > -1) { // Toggle off repeated clicks
                    that.selectJobId.splice(index, 1);
                    that.selectJobClass.splice(index, 1);
                    return true;
                }

                if (that.multiple) { // Multi-select
                    if (level == 2) { // Selecting second level clears selected third-level items
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
                        message.warning(lc('admin_00045') + max + lc('common_02104'));
                        return false;
                    }
                    that.selectJobId.push(id);
                    that.selectJobClass.push({id: id, name: name});
                } else { // Single-select
                    that.selectJobId = [id];
                    that.selectJobClass = [
                        {id: id, name: name}
                    ]; // Single-select replaces selected value
                }
            },
            // Remove selected category
            handleCloseJob(id) {
                let index = this.selectJobId.indexOf(id);

                if (index > -1) {
                    this.selectJobId.splice(index, 1);
                    this.selectJobClass.splice(index, 1);
                }
            },
            /**
             * Child category handling
             * @params ids All child IDs
             */
            handleSelectClass(ids) {
                let that = this,
                    index = -1;

                if (ids && ids.length > 0 && that.selectJobId.length > 0) { // Clear selected child items when second level is selected
                    ids.forEach(function (id) {
                        index = that.selectJobId.indexOf(id);
                        if (index > -1) { // Find selected child item
                            that.selectJobId.splice(index, 1); // Remove child item
                            that.selectJobClass.splice(index, 1);
                        }
                    })
                }
            },

            // Dialog search
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
                }, 500); // Delay is 500 ms
            },

            // Confirm selected category
            handleSubmitJob() {
                let that = this;

                // Call parent page method to process selected data
                this.jobId = deepClone(this.selectJobId);
                this.jobClass = deepClone(this.selectJobClass);
                this.classOptions = deepClone(this.selectJobClass);

                let timer = setTimeout(() => {
                    that.classOptions = [];
                    timer = null;
                }, 500); // Clear search dropdown options

                this.jobVisible = false;
                this.$emit("confirm", {jobId: this.jobId});
            },

            // Handle selected values
            handleSelected() {
                let that = this,
                    selected = this.selected;

                if (this.searchJob !== '') { // Reload category data when the previous dialog session searched
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
                    }, 500); // Clear search dropdown options
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