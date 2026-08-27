<template>
    <div v-loading="loading">
        <div style="overflow: hidden; position: relative; display: flex; align-items: center;">
        <el-select v-model="hyId" multiple :multiple-limit="multiple ? max : 1" :placeholder="lc('admin_00061')"
                   filterable remote :remote-method="remoteClassList" @change="classChange" @remove-tag="classRemove">
            <el-option v-for="opitem in classOptions" :key="opitem.id" :label="opitem.name"
                       :value="opitem.id" :disabled="opitem.disabled">
                <span :style="hyId.indexOf(opitem.id) > -1 ? 'color:#409eff' : ''">
                    <span style="float: left; font-size: 14px;font-weight:bold;">{{opitem.name}}</span>
                    <span style="float: right; color: #a5a5a5; font-size: x-small;" v-if="opitem.upname!=''">{{opitem.upname}}</span>
                </span>
            </el-option>
        </el-select>
        <div>
            <el-button type="text" icon="el-icon-s-operation" style="width:25px; margin-right: 25px;"
                       @click="hyOpen"></el-button>
        </div>
        </div>

        <!-- Select headhunter industry category -->
        <div class="modluDrawer">
            <el-drawer v-model="hyVisible" :with-header="false" :modal-append-to-body="false" append-to-body
                       :show-close="true" size="60%">
                <div class="modluDrawerContents">
                    <div class="modluDrawerTi9te">
                        <div>{{ lc('admin_00060') }}</div>
                        <div class="shuytans">
                            <el-input v-model="searchHy" :placeholder="lc('admin_00061')"
                                      @input="handleSearchHy">
                                <template #prefix><i class="el-input__icon el-icon-search"></i></template>
                            </el-input>
                        </div>
                        <button aria-label="close drawer" type="button" class="el-drawer__close-btn"
                                style="right: 2px;position: absolute;" @click="hyVisible = false"><i
                                class="el-dialog__close el-icon el-icon-close"></i></button>
                    </div>
                    <div class="xuanzleibie" v-if="classList.length > 0">
                        <ul>
                            <template v-for="(oneItem, oneIndex) in classList">
                                <li v-if="!oneItem.hide" :key="oneIndex">
                                    <!-- First level -->
                                    <div class="xuanzlOne pointer" :data-id="oneItem.id" :data-name="oneItem.name"
                                        :data-one="oneIndex" :data-level="1"
                                        :class="selectHyId.indexOf(oneItem.id) > -1 ? 'class-selected' : ''"
                                        @click="handleSelectHy">{{ oneItem.name }}</div>
                                    <div class="xuanzlTwo">
                                        <!-- Second level -->
                                        <template v-for="(twoItem, twoIndex) in oneItem.children">
                                            <div  v-if="!twoItem.hide" :key="twoIndex" class="xuanzlTwoList">
                                                <div class="xuanzNamte blue">
                                                    <!--<i class="el-icon-remove"></i>-->
                                                    <span v-if="multiple && selectHyId.indexOf(oneItem.id) > -1"
                                                        class="class-disabled">{{ twoItem.name }}</span>
                                                    <span v-else :data-id="twoItem.id" :data-name="twoItem.name"
                                                        :data-one="oneIndex" :data-two="twoIndex" :data-level="2"
                                                        :class="selectHyId.indexOf(twoItem.id) > -1 ? 'class-selected' : ''"
                                                        @click="handleSelectHy">{{ twoItem.name }}</span>
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
                                <el-tag v-for="(selectClass, selectIndex) in selectHyClass" :key="selectIndex"
                                        closable size="small" @close="handleCloseHy(selectClass.id)">
                                    {{ selectClass.name }}
                                </el-tag>
                            </div>
                        </div>
                        <div class="footTextburn">
                            <el-button type="primary" size="small" round @click="handleSubmitHy">{{ lc('wap_com_00019') }}</el-button>
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

                hyId: [],
                hyClass: [],

                classOptions: [],

                hyVisible: false,
                searchHy: '',
                selectHyId: [],
                selectHyClass: [],

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

                httpPost('m=common&c=cache&a=getlthy', params, {hideLoading: true}).then(function (response) {
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
                                            disabled: that.hyId.indexOf(oneItem.id) !== -1, // Disable second-level selection when first level is selected
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
                    hyClass = this.hyClass
                    hyClassLen = hyClass.length;

                if (hyClassLen > valLen) { // Remove
                    for (var i = 0; i < hyClassLen; i++) {
                        if (val.indexOf(hyClass[i].id) === -1) { // Clear items that are no longer selected
                            this.hyClass.splice(i, 1);
                            break;
                        }
                    }
                } else { // Add
                    for (var i = 0; i < classOptions.length; i++) {
                        if (classOptions[i].id == id) { // Get selected value data
                            if (this.multiple) {
                                this.hyClass.push({id: classOptions[i].id, name: classOptions[i].name});

                                let childrenIds = classOptions[i].childrenIds,
                                    index = -1;
                                if (childrenIds && childrenIds.length > 0 && this.hyId.length > 0) { // {{ lc('common_01285') }}
                                    for (var j = 0; j < childrenIds.length; j++) {
                                        index = this.hyId.indexOf(childrenIds[j]);
                                        if (index > -1) { // Find selected child item
                                            this.hyId.splice(index, 1); // Remove child item
                                            this.hyClass.splice(index, 1);
                                        }
                                    }
                                }
                            } else {
                                this.hyClass = [{id: classOptions[i].id, name: classOptions[i].name}];
                            }
                            break;
                        }
                    }
                }

                this.$emit("confirm", {hyId: this.hyId});
            },
            // Remove category
            classRemove(val) {
                let that = this;

                that.hyClass.forEach(function(item, index){
                    if (val == item.id) {
                        that.hyClass.splice(index, 1);
                    }
                })
            },

            // Open dialog
            hyOpen() {
                this.hyVisible = true;
                if (this.hyId.length > 0) {
                    this.selectHyId = deepClone(this.hyId);
                    this.selectHyClass = deepClone(this.hyClass);
                } else {
                    this.selectHyId = [];
                    this.selectHyClass = [];
                }
                if (this.searchHy !== '') { // Clear search text
                    this.searchHy = '';
                }
                this.searchClass(''); // Rebuild category list
            },

            // Select category
            async handleSelectHy(event) {
                let that = this,
                    dataset = event.currentTarget.dataset,
                    id = dataset.id,
                    name = dataset.name,
                    selectHyId = this.selectHyId,
                    max = that.max,
                    index = selectHyId.indexOf(id),
                    level = dataset.level,
                    one = dataset.one;

                if (index > -1) { // Toggle off repeated clicks
                    that.selectHyId.splice(index, 1);
                    that.selectHyClass.splice(index, 1);
                    return true;
                }

                if (that.multiple) { // Multi-select
                    if (level == 1) { // Selecting first level clears selected second-level items
                        that.handleSelectClass(one);
                    }

                    if (selectHyId.length >= max) {
                        message.warning(lc('admin_vue_00125') + max + lc('common_02104'));
                        return false;
                    }
                    that.selectHyId.push(id);
                    that.selectHyClass.push({id: id, name: name});
                } else { // Single-select
                    that.selectHyId = [id];
                    that.selectHyClass = [
                        {id: id, name: name}
                    ]; // Single-select replaces selected value
                }
            },
            // Remove selected category
            handleCloseHy(id) {
                let index = this.selectHyId.indexOf(id);

                if (index > -1) {
                    this.selectHyId.splice(index, 1);
                    this.selectHyClass.splice(index, 1);
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

                if (twoClassList && twoClassList.length > 0 && that.selectHyId.length > 0) { // Clear selected second-level items when first level is selected
                    twoClassList.forEach(function (item, index) {
                        twoIndex = that.selectHyId.indexOf(item.id);
                        if (twoIndex > -1) { // Find selected second-level item
                            that.selectHyId.splice(twoIndex, 1); // {{ lc('admin_00065') }}
                            that.selectHyClass.splice(twoIndex, 1);
                        }
                    })
                }
            },

            // Dialog search
            handleSearchHy() {
                this.debouncedSearchHandler();
            },
            debouncedSearchHandler() {
                let that = this;
                if (that.timer) {
                    clearTimeout(that.timer);
                }
                that.timer = setTimeout(() => {
                    that.searchClass(that.searchHy);
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
            handleSubmitHy() {
                let that = this;

                // Call parent page method to process selected data
                this.hyId = deepClone(this.selectHyId);
                this.hyClass = deepClone(this.selectHyClass);
                this.classOptions = deepClone(this.selectHyClass);

                let timer = setTimeout(() => {
                    that.classOptions = [];
                    timer = null;
                }, 500); // Clear search dropdown options

                this.hyVisible = false;
                this.$emit("confirm", {hyId: this.hyId});
            },

            // Handle selected values
            handleSelected() {
                let that = this,
                    selected = this.selected;

                if (this.searchHy !== '') { // Reload category data when the previous dialog session searched
                    this.searchHy = '';
                    this.searchClass('');
                }

                this.selectHyId = [];
                this.selectHyClass = [];
                this.hyId = [];
                this.hyClass = [];
                this.classOptions = [];

                if (selected) {
                    for (let key in selected) {
                        this.selectHyId.push(key);
                        this.selectHyClass.push({id: key, name: selected[key]});
                    }
                    this.hyId = deepClone(this.selectHyId);
                    this.hyClass = deepClone(this.selectHyClass);
                    this.classOptions = deepClone(this.selectHyClass);

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