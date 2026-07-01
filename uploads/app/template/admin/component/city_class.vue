<template>
    <div v-loading="loading">
        <div style="overflow: hidden; position: relative; display: flex; align-items: center;">
        <el-select v-model="cityId" size="small" multiple :multiple-limit="multiple ? max : 1" :placeholder="lc('admin_00041')"
                   filterable remote :remote-method="remoteClassList" @change="classChange" @remove-tag="classRemove">
            <el-option v-for="opitem in classOptions" :key="opitem.id" :label="opitem.name"
                       :value="opitem.id" :disabled="opitem.disabled">
                <span :style="cityId.indexOf(opitem.id) > -1 ? 'color:#409eff' : ''">
                    <span style="float: left; font-size: 14px;font-weight:bold;">{{opitem.name}}</span>
                    <span style="float: right; color: #a5a5a5; font-size: x-small;" v-if="opitem.upname!=''">{{opitem.upname}}</span>
                </span>
            </el-option>
        </el-select>
        <div slot="prefix">
            <el-button type="text" icon="el-icon-location-information" style="width:25px; margin-right: 25px;"
                       @click="cityOpen"></el-button>
        </div>
        </div>

        <!-- Select city category -->
        <div class="modluDrawer">
            <el-drawer :visible.sync="cityVisible" :with-header="false" :modal-append-to-body="false" append-to-body
                       :show-close="true" size="60%">
                <div class="modluDrawerContents">
                    <div class="modluDrawerTi9te">
                        <div>{{ lc('admin_00042') }}</div>
                        <div class="shuytans">
                            <el-input v-model="searchCity" :placeholder="lc('admin_00043')"
                                      @keyup.native="handleSearchCity">
                                <i slot="prefix" class="el-input__icon el-icon-search"></i>
                            </el-input>
                        </div>
                        <button aria-label="close drawer" type="button" class="el-drawer__close-btn"
                                style="right: 2px;position: absolute;" @click="cityVisible = false"><i
                                class="el-dialog__close el-icon el-icon-close"></i></button>
                    </div>
                    <div class="xuanzleibie" v-if="classList.length > 0">
                        <ul>
                            <li v-for="(oneItem, oneIndex) in classList" :key="oneIndex">
                                <!-- First level -->
                                <div class="xuanzlOne pointer" :data-id="oneItem.id" :data-name="oneItem.name"
                                     :data-one="oneIndex" :data-level="1"
                                     :class="selectCityId.indexOf(oneItem.id) > -1 ? 'class-selected' : ''"
                                     @click="handleSelectCity">{{ oneItem.name }}</div>
                                <div class="xuanzlTwo">
                                    <div v-for="(twoItem, twoIndex) in oneItem.children" :key="twoIndex" class="xuanzlTwoList">
                                        <!-- Has third level -->
                                        <el-popover v-if="twoItem.children" placement="bottom" width="350" trigger="click">
                                            <div class="xuanzlTwoCont" v-loading="twoItem.children.length == 0">
                                                <!-- Second level -->
                                                <div class="xuanzlTwoBit">
                                                    <i class="el-icon-remove"></i>
                                                    <span v-if="multiple && selectCityId.indexOf(oneItem.id) > -1"
                                                          class="class-disabled">{{ twoItem.name }}</span>
                                                    <span v-else :data-id="twoItem.id" :data-name="twoItem.name"
                                                          :data-one="oneIndex" :data-two="twoIndex" :data-level="2"
                                                          :class="selectCityId.indexOf(twoItem.id) > -1 ? 'class-selected' : ''"
                                                          @click="handleSelectCity">{{ twoItem.name }}</span>
                                                </div>
                                                <!-- Third level -->
                                                <div class="xuanzlTwoTips">
                                                    <template v-for="(threeItem, threeIndex) in twoItem.children">
                                                        <span v-if="multiple && (selectCityId.indexOf(oneItem.id) > -1 || selectCityId.indexOf(twoItem.id) > -1)"
                                                            :key="threeIndex" class="class-disabled">{{ threeItem.name }}</span>
                                                        <span v-else :key="'else'+threeIndex" :data-id="threeItem.id" :data-name="threeItem.name"
                                                              :data-one="oneIndex" :data-two="twoIndex"
                                                              :data-three="threeIndex" :data-level="3"
                                                              :class="selectCityId.indexOf(threeItem.id) > -1 ? 'class-selected' : ''"
                                                              @click="handleSelectCity">{{ threeItem.name }}</span>
                                                    </template>
                                                </div>
                                            </div>
                                            <div slot="reference" class="xuanzNamte" :data-id="twoItem.id"
                                                 :data-one="oneIndex" :data-two="twoIndex" @click="childClassList">
                                                <i class="el-icon-circle-plus"></i>
                                                <span v-if="multiple && selectCityId.indexOf(oneItem.id) > -1"
                                                      class="class-disabled">{{ twoItem.name }}</span>
                                                <span v-else
                                                      :class="selectCityId.indexOf(twoItem.id) > -1 ? 'class-selected' : ''"
                                                      >{{ twoItem.name }}</span>
                                            </div>
                                        </el-popover>
                                        <!-- No third level -->
                                        <div v-else class="xuanzNamte blue">
                                            <i class="el-icon-remove"></i>
                                            <span v-if="multiple && selectCityId.indexOf(oneItem.id) > -1"
                                                  class="class-disabled">{{ twoItem.name }}</span>
                                            <span v-else :data-id="twoItem.id" :data-name="twoItem.name"
                                                  :data-one="oneIndex" :data-two="twoIndex" :data-level="2"
                                                  :class="selectCityId.indexOf(twoItem.id) > -1 ? 'class-selected' : ''"
                                                  @click="handleSelectCity">{{ twoItem.name }}</span>
                                        </div>
                                    </div>
                                </div>
                            </li>
                        </ul>
                    </div>
                    <div v-else>
                        <div>{{ lc('admin_00039') }}</div>
                    </div>
                    <div slot="footer" class="dialog-footer dialoFoofetee">
                        <div class="footText">
                            <div class="mingdsc"><span>{{ lc('admin_00390') }}</span></div>
                            <div class="mingdEltags" style="padding-top: 4px;">
                                <el-tag v-for="(selectClass, selectIndex) in selectCityClass" :key="selectIndex"
                                        closable type="" size="small" @close="handleCloseCity(selectClass.id)">
                                    {{ selectClass.name }}
                                </el-tag>
                            </div>
                        </div>
                        <div class="footTextburn">
                            <el-button type="primary" size="mini" round @click="handleSubmitCity">{{ lc('wap_com_00019') }}</el-button>
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
            selected: {type: Object, default: null} // Selected data, for example: {1911: lc('admin_00047'), 1912: lc('admin_00048')}
        },
        data: function () {
            return {
                loading: true,

                classList: [],

                cityId: [],
                cityClass: [],

                classOptions: [],

                cityVisible: false,
                searchCity: '',
                selectCityId: [],
                selectCityClass: [],

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

                if (that.searchCity !== '') {
                    params.name = that.searchCity;
                } else {
                    params.level = 2;
                }

                httpPost('m=common&c=cache&a=getCityClass', params, {hideloading: true}).then(function (response) {
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

                    httpPost('m=common&c=cache&a=getCityClass', {name: query}, {hideLoading: true}).then(function (response) {
                        let res = response.data,
                            classList = res.data.classList;

                        // Flatten hierarchical data to first-level options
                        if (classList && classList.length > 0) {
                            let newClassList = [],
                                newClassId = [];
                            classList.forEach(function (oneItem, oneIndex) {
                                if (oneItem.name.includes(query)) { // First-level item must contain the keyword
                                    newClassList.push({
                                        id: oneItem.id,
                                        name: oneItem.name,
                                        upname: ''
                                    })
                                    newClassId.push(oneItem.id); // Used to hide first-level name when first-level item already exists
                                }
                                if (oneItem.children) { // Has second-level children
                                    oneItem.children.forEach(function (twoItem, twoIndex) {
                                        if (twoItem.name.includes(query)) { // Second-level item must contain the search keyword
                                            newClassList.push({
                                                id: twoItem.id,
                                                name: twoItem.name,
                                                disabled: that.cityId.indexOf(oneItem.id) !== -1, // Disable second-level selection when first level is selected
                                                upname: newClassId.indexOf(oneItem.id) === -1 ? oneItem.name : ''
                                            })
                                            newClassId.push(twoItem.id); // Used to hide second-level name when second-level item already exists
                                        }

                                        if (twoItem.children) { // Has third-level children
                                            twoItem.children.forEach(function (threeItem, threeIndex) {
                                                newClassList.push({
                                                    id: threeItem.id,
                                                    name: threeItem.name,
                                                    disabled: that.cityId.indexOf(oneItem.id) !== -1 || that.selectCityId.indexOf(twoItem.id) !== -1, // Disable third-level selection when first or second level is selected
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
                    cityClass = this.cityClass,
                    cityClassLen = cityClass.length;

                if (cityClassLen > valLen) { // Remove
                    for (var i = 0; i < cityClassLen; i++) {
                        if (val.indexOf(cityClass[i].id) === -1) { // Clear items that are no longer selected
                            this.cityClass.splice(i, 1);
                            break;
                        }
                    }
                } else { // Add
                    for (var i = 0; i < classOptions.length; i++) {
                        if (classOptions[i].id == id) {
                            if (this.multiple) {
                                this.cityClass.push({id: classOptions[i].id, name: classOptions[i].name});

                                let childrenIds = await this.getCityChildIds(classOptions[i].id),
                                    index = -1;
                                if (childrenIds && childrenIds.length > 0 && this.cityId.length > 0) { // {{ lc('common_01285') }}
                                    for (var j = 0; j < childrenIds.length; j++) {
                                        index = this.cityId.indexOf(childrenIds[j]);
                                        if (index > -1) { // Find selected child item
                                            this.cityId.splice(index, 1); // Remove child item
                                            this.cityClass.splice(index, 1);
                                        }
                                    }
                                }
                            } else {
                                this.cityClass = [{id: classOptions[i].id, name: classOptions[i].name}];
                            }
                            break;
                        }
                    }
                }

                this.$emit("confirm", {cityId: this.cityId});
            },
            // Remove category
            classRemove(val) {
                let that = this;

                that.cityClass.forEach(function(item, index){
                    if (val == item.id) {
                        that.cityClass.splice(index, 1);
                    }
                })
            },

            // Open dialog
            cityOpen() {
                this.cityVisible = true;
                if (this.cityId.length > 0) {
                    this.selectCityId = deepClone(this.cityId);
                    this.selectCityClass = deepClone(this.cityClass);
                } else {
                    this.selectCityId = [];
                    this.selectCityClass = [];
                }
                if (this.searchCity !== '') { // Reload category data when the previous dialog session searched
                    this.searchCity = '';
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

                httpPost('m=common&c=cache&a=getCityClass', {pid: id}, {hideLoading: true}).then(function (response) {
                    let res = response.data,
                        classList = res.data.classList;

                    that.classList[oneIndex]['children'][twoIndex]['children'] = classList && classList.length > 0 ? classList : false; // Default to no child categories
                })
            },

            // Get child category IDs
            async getCityChildIds(pid) {
                let response = await httpPost('m=common&c=cache&a=getCityChildIds', {pid: pid}, {hideLoading: true});

                return response.data.data;
            },

            // Select category
            async handleSelectCity(event) {
                let that = this,
                    dataset = event.currentTarget.dataset,
                    id = dataset.id,
                    name = dataset.name,
                    selectCityId = this.selectCityId,
                    max = this.max,
                    index = selectCityId.indexOf(id),
                    level = dataset.level,
                    one = dataset.one,
                    classList = that.classList;

                if (index > -1) { // Toggle off repeated clicks
                    that.selectCityId.splice(index, 1);
                    that.selectCityClass.splice(index, 1);
                    return true;
                }

                if (that.multiple) { // Multi-select
                    if (level == 1 || level == 2) {
                        let childrenIds = [],
                            oneClass = classList[one];

                        if (level == 1) { // Selecting first level clears selected child items
                            if (typeof oneClass.childrenIds === 'undefined') {
                                childrenIds = await that.getCityChildIds(oneClass.id);
                                that.$set(that.classList[one], 'childrenIds', childrenIds);
                            } else {
                                childrenIds = oneClass.childrenIds;
                            }
                            that.handleSelectClass(childrenIds);
                        } else if (level == 2) { // Selecting second level clears selected third-level items
                            let two = dataset.two,
                                twoClass = oneClass['children'][two];
                            if (typeof twoClass.childrenIds === 'undefined') {
                                childrenIds = await that.getCityChildIds(twoClass.id);
                                that.$set(that.classList[one]['children'][two], 'childrenIds', childrenIds);
                            } else {
                                childrenIds = twoClass.childrenIds;
                            }
                            that.handleSelectClass(childrenIds);
                        }
                    }

                    if (selectCityId.length >= max) {
                        message.warning(lc('admin_00045') + max + lc('common_02104'));
                        return false;
                    }
                    that.selectCityId.push(id);
                    that.selectCityClass.push({id: id, name: name});
                } else { // Single-select
                    that.selectCityId = [id];
                    that.selectCityClass = [
                        {id: id, name: name}
                    ]; // Single-select replaces selected value
                }
            },
            // Remove selected category
            handleCloseCity(id) {
                let index = this.selectCityId.indexOf(id);

                if (index > -1) {
                    this.selectCityId.splice(index, 1);
                    this.selectCityClass.splice(index, 1);
                }
            },
            /**
             * Child category handling
             * @params ids All child IDs
             */
            handleSelectClass(ids) {
                let that = this,
                    index = -1;

                if (ids && ids.length > 0 && that.selectCityId.length > 0) { // Clear selected child items when first or second level is selected
                    ids.forEach(function (id) {
                        index = that.selectCityId.indexOf(id);
                        if (index > -1) { // Find selected child item
                            that.selectCityId.splice(index, 1); // Remove child item
                            that.selectCityClass.splice(index, 1);
                        }
                    })
                }
            },

            // Dialog search
            handleSearchCity() {
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
            handleSubmitCity() {
                let that = this;

                // Call parent page method to process selected data
                this.cityId = deepClone(this.selectCityId);
                this.cityClass = deepClone(this.selectCityClass);
                this.classOptions = deepClone(this.selectCityClass);

                let timer = setTimeout(() => {
                    that.classOptions = [];
                    timer = null;
                }, 500); // Clear search dropdown options

                this.cityVisible = false;
                this.$emit("confirm", {cityId: this.cityId});
            },

            // Handle selected values
            handleSelected() {
                let that = this,
                    selected = this.selected;

                if (this.searchCity !== '') { // Reload category data when the previous dialog session searched
                    this.searchCity = '';
                    this.getClassList();
                }

                this.selectCityId = [];
                this.selectCityClass = [];
                this.cityId = [];
                this.cityClass = [];
                this.classOptions = [];

                if (selected) {
                    for (let key in selected) {
                        this.selectCityId.push(key);
                        this.selectCityClass.push({id: key, name: selected[key]});
                    }
                    this.cityId = deepClone(this.selectCityId);
                    this.cityClass = deepClone(this.selectCityClass);
                    this.classOptions = deepClone(this.selectCityClass);

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
<style>
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