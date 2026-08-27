<template>
    <div v-loading="loading">
        <div style="overflow: hidden; position: relative; display: flex; align-items: center;">
        <el-select v-model="jobId" multiple :multiple-limit="multiple ? max : 1" :placeholder="lc('admin_00057')"
                   filterable remote :remote-method="remoteClassList" @change="classChange" @remove-tag="classRemove">
            <el-option v-for="opitem in classOptions" :key="opitem.id" :label="opitem.name"
                       :value="opitem.id" :disabled="opitem.disabled">
                <span :style="jobId.indexOf(opitem.id) > -1 ? 'color:#409eff' : ''">
                    <span style="float: left; font-size: 14px;font-weight:bold;">{{opitem.name}}</span>
                    <span style="float: right; color: #a5a5a5; font-size: x-small;" v-if="opitem.upname!=''">{{opitem.upname}}</span>
                </span>
            </el-option>
        </el-select>
        <div>
            <el-button type="text" icon="el-icon-s-operation" style="width:25px; margin-right: 25px;"
                       @click="jobOpen"></el-button>
        </div>
        </div>

        <!-- Select headhunter job category -->
        <div class="modluDrawer">
            <el-drawer v-model="jobVisible" :with-header="false" :modal-append-to-body="false" append-to-body
                       :show-close="true" size="60%">
                <div class="modluDrawerContents">
                    <div class="modluDrawerTi9te">
                        <div>{{ lc('admin_00067') }}</div>
                        <div class="shuytans">
                            <el-input v-model="searchJob" :placeholder="lc('admin_00057')"
                                      @input="handleSearchJob">
                                <template #prefix><i class="el-input__icon el-icon-search"></i></template>
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
                                    <!-- First level -->
                                    <div class="xuanzlOne pointer" :data-id="oneItem.id" :data-name="oneItem.name"
                                        :data-one="oneIndex" :data-level="1"
                                        :class="selectJobId.indexOf(oneItem.id) > -1 ? 'class-selected' : ''"
                                        @click="handleSelectJob">{{ oneItem.name }}</div>
                                    <div class="xuanzlTwo">
                                        <!-- Second level -->
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
                            <el-empty :description="lc('admin_00039')"></el-empty>
                            
                        </div>
                    </div>
                    <div class="dialog-footer dialoFoofetee">
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
                            <el-button type="primary" size="small" round @click="handleSubmitJob">{{ lc('wap_com_00019') }}</el-button>
                        </div>
                    </div>
                </div>
            </el-drawer>
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
        props: {
            multiple: {type: Boolean, default: false}, // Selection mode: false for single-select, true for multi-select
            max: {type: Number, default: 5}, // Maximum selection count for multi-select mode
            selected: {type: Object, default: null} // Selected data, for example: {167: lc('common_01417'), 168: lc('admin_00056')}
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
            // Initial category load
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

            // Search categories for dropdown
            remoteClassList(query) {
                if ($.trim(query) !== '') {
                    let that = this;

                    that.searchClass(query); // Local JS search

                    let classList = deepClone(that.classList);

                    // Flatten hierarchical data to first-level options
                    if (classList && classList.length > 0) {
                        let newClassList = [],
                            newClassId = [];
                        classList.forEach(function (oneItem, oneIndex) {
                            if (oneItem.name.includes(query)) { // First-level item must contain the keyword
                                newClassList.push({
                                    id: oneItem.id,
                                    name: oneItem.name,
                                    upname: '',
                                    childrenIds: oneItem.children && oneItem.children.length > 0 ? oneItem.children.map(row => row.id) : []
                                })
                                newClassId.push(oneItem.id); // Used to hide first-level name when first-level item already exists
                            }
                            if (oneItem.children && oneItem.children.length > 0) { // Has second-level children
                                oneItem.children.forEach(function (twoItem, twoIndex) {
                                    if (twoItem.name.includes(query)) { // Second-level item must contain the search keyword
                                        newClassList.push({
                                            id: twoItem.id,
                                            name: twoItem.name,
                                            disabled: that.jobId.indexOf(oneItem.id) !== -1, // Disable second-level selection when first level is selected
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

                                let childrenIds = classOptions[i].childrenIds,
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
                if (this.searchJob !== '') { // Clear search text
                    this.searchJob = '';
                }
                this.searchClass(''); // Rebuild category list
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
                    one = dataset.one;

                if (index > -1) { // Toggle off repeated clicks
                    that.selectJobId.splice(index, 1);
                    that.selectJobClass.splice(index, 1);
                    return true;
                }

                if (that.multiple) { // Multi-select
                    if (level == 1) { // Selecting first level clears selected second-level items
                        that.handleSelectClass(one);
                    }

                    if (selectJobId.length >= max) {
                        message.warning(lc('admin_vue_00125') + max + lc('common_02104'));
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
            handleSelectClass(one) {
                let that = this,
                    classList = that.classList,
                    twoClassList = classList[one]['children'];

                if (twoClassList && twoClassList.length > 0 && that.selectJobId.length > 0) { // Clear selected second-level items when first level is selected
                    twoClassList.forEach(function (item, index) {
                        twoIndex = that.selectJobId.indexOf(item.id);
                        if (twoIndex > -1) { // Find selected second-level item
                            that.selectJobId.splice(twoIndex, 1); // {{ lc('admin_00065') }}
                            that.selectJobClass.splice(twoIndex, 1);
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
                    that.searchClass(that.searchJob);
                    that.timer = null;
                }, 500); // Delay is 500 ms
            },

            // Search categories
            searchClass(query) {
                let that = this,
                    classList = deepClone(that.classList),
                    twoList = [];

                if (classList && classList.length > 0) {
                    classList.forEach(function(oneItem, oneKey) {
                        if (oneItem.name.includes(query)) { // First-level item must contain the keyword
                            classList[oneKey].hide = false; // Show first-level category
                        } else {
                            classList[oneKey].hide = true; // Hide first-level category
                        }
                        twoList = oneItem.children;
                        if (twoList && twoList.length > 0) {
                            twoList.forEach(function(twoItem, twoKey) {
                                if (twoItem.name.includes(query)) { // Second-level item must contain the keyword
                                    classList[oneKey].hide = false; // Show first-level category when a child matches
                                    classList[oneKey]['children'][twoKey].hide = false; // Mark second-level category as visible
                                } else {
                                    classList[oneKey]['children'][twoKey].hide = true; // Hide second-level category
                                }
                            })
                        }
                    })
                    that.classList = classList;
                }
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
                    }, 500); // Clear search dropdown options
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