<template>
    <div v-loading="loading">
        <div style="overflow: hidden; position: relative; display: flex; align-items: center;">
        <el-select v-model="cityId" size="small" multiple :multiple-limit="multiple ? max : 1" placeholder="搜索地区类别名称"
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

        <!--请选择城市类别-->
        <div class="modluDrawer">
            <el-drawer :visible.sync="cityVisible" :with-header="false" :modal-append-to-body="false" append-to-body
                       :show-close="true" size="60%">
                <div class="modluDrawerContents">
                    <div class="modluDrawerTi9te">
                        <div>请选择城市类别</div>
                        <div class="shuytans">
                            <el-input v-model="searchCity" placeholder="搜索城市名称"
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
                                <!--第一级-->
                                <div class="xuanzlOne pointer" :data-id="oneItem.id" :data-name="oneItem.name"
                                     :data-one="oneIndex" :data-level="1"
                                     :class="selectCityId.indexOf(oneItem.id) > -1 ? 'class-selected' : ''"
                                     @click="handleSelectCity">{{ oneItem.name }}</div>
                                <div class="xuanzlTwo">
                                    <div v-for="(twoItem, twoIndex) in oneItem.children" :key="twoIndex" class="xuanzlTwoList">
                                        <!--有第三级-->
                                        <el-popover v-if="twoItem.children" placement="bottom" width="350" trigger="click">
                                            <div class="xuanzlTwoCont" v-loading="twoItem.children.length == 0">
                                                <!--第二级-->
                                                <div class="xuanzlTwoBit">
                                                    <i class="el-icon-remove"></i>
                                                    <span v-if="multiple && selectCityId.indexOf(oneItem.id) > -1"
                                                          class="class-disabled">{{ twoItem.name }}</span>
                                                    <span v-else :data-id="twoItem.id" :data-name="twoItem.name"
                                                          :data-one="oneIndex" :data-two="twoIndex" :data-level="2"
                                                          :class="selectCityId.indexOf(twoItem.id) > -1 ? 'class-selected' : ''"
                                                          @click="handleSelectCity">{{ twoItem.name }}</span>
                                                </div>
                                                <!--第三级-->
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
                                        <!--无第三级-->
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
                        <div>抱歉，没有找到结果！</div>
                    </div>
                    <div slot="footer" class="dialog-footer dialoFoofetee">
                        <div class="footText">
                            <div class="mingdsc"><span>最多可选择{{ multiple ? max : '1'}}项：</span></div>
                            <div class="mingdEltags" style="padding-top: 4px;">
                                <el-tag v-for="(selectClass, selectIndex) in selectCityClass" :key="selectIndex"
                                        closable type="" size="small" @close="handleCloseCity(selectClass.id)">
                                    {{ selectClass.name }}
                                </el-tag>
                            </div>
                        </div>
                        <div class="footTextburn">
                            <el-button type="primary" size="mini" round @click="handleSubmitCity">确 定</el-button>
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
            selected: {type: Object, default: null} // 已选中数据，数据内容如：{1911: "宿城区", 1912: "宿豫区"}
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
            // 首次加载分类(一级和二级)
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

            // 搜索分类
            remoteClassList(query) {
                if ($.trim(query) !== '') {
                    let that = this;

                    httpPost('m=common&c=cache&a=getCityClass', {name: query}, {hideLoading: true}).then(function (response) {
                        let res = response.data,
                            classList = res.data.classList;

                        // 层级数据转为一级数据
                        if (classList && classList.length > 0) {
                            let newClassList = [],
                                newClassId = [];
                            classList.forEach(function (oneItem, oneIndex) {
                                if (oneItem.name.includes(query)) { // 一级须包含关键字
                                    newClassList.push({
                                        id: oneItem.id,
                                        name: oneItem.name,
                                        upname: ''
                                    })
                                    newClassId.push(oneItem.id); // 用来判断存在一级，二级不显示一级的名称
                                }
                                if (oneItem.children) { // 存在二级
                                    oneItem.children.forEach(function (twoItem, twoIndex) {
                                        if (twoItem.name.includes(query)) { // 二级须包含销售关键字
                                            newClassList.push({
                                                id: twoItem.id,
                                                name: twoItem.name,
                                                disabled: that.cityId.indexOf(oneItem.id) !== -1, // 选中一级，二级禁用选择
                                                upname: newClassId.indexOf(oneItem.id) === -1 ? oneItem.name : ''
                                            })
                                            newClassId.push(twoItem.id); // 用来判断存在二级，三级不显示二级的名称
                                        }

                                        if (twoItem.children) { // 存在三级
                                            twoItem.children.forEach(function (threeItem, threeIndex) {
                                                newClassList.push({
                                                    id: threeItem.id,
                                                    name: threeItem.name,
                                                    disabled: that.cityId.indexOf(oneItem.id) !== -1 || that.selectCityId.indexOf(twoItem.id) !== -1, // 选中一级或二级，三级均禁用选择
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
                    cityClass = this.cityClass,
                    cityClassLen = cityClass.length;

                if (cityClassLen > valLen) { // 删除
                    for (var i = 0; i < cityClassLen; i++) {
                        if (val.indexOf(cityClass[i].id) === -1) { // 未在已选中列表的，清除
                            this.cityClass.splice(i, 1);
                            break;
                        }
                    }
                } else { // 增加
                    for (var i = 0; i < classOptions.length; i++) {
                        if (classOptions[i].id == id) {
                            if (this.multiple) {
                                this.cityClass.push({id: classOptions[i].id, name: classOptions[i].name});

                                let childrenIds = await this.getCityChildIds(classOptions[i].id),
                                    index = -1;
                                if (childrenIds && childrenIds.length > 0 && this.cityId.length > 0) { // 清空下级已选选项
                                    for (var j = 0; j < childrenIds.length; j++) {
                                        index = this.cityId.indexOf(childrenIds[j]);
                                        if (index > -1) { // 检索已选中下级
                                            this.cityId.splice(index, 1); // 删除下级
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
            // 移除分类
            classRemove(val) {
                let that = this;

                that.cityClass.forEach(function(item, index){
                    if (val == item.id) {
                        that.cityClass.splice(index, 1);
                    }
                })
            },

            // 打开弹窗
            cityOpen() {
                this.cityVisible = true;
                if (this.cityId.length > 0) {
                    this.selectCityId = deepClone(this.cityId);
                    this.selectCityClass = deepClone(this.cityClass);
                } else {
                    this.selectCityId = [];
                    this.selectCityClass = [];
                }
                if (this.searchCity !== '') { // 上一次有搜索过，再次打开重新加载分类数据
                    this.searchCity = '';
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

                httpPost('m=common&c=cache&a=getCityClass', {pid: id}, {hideLoading: true}).then(function (response) {
                    let res = response.data,
                        classList = res.data.classList;

                    that.classList[oneIndex]['children'][twoIndex]['children'] = classList && classList.length > 0 ? classList : false; // 默认为没有下级
                })
            },

            // 获取下级分类ID
            async getCityChildIds(pid) {
                let response = await httpPost('m=common&c=cache&a=getCityChildIds', {pid: pid}, {hideLoading: true});

                return response.data.data;
            },

            // 选中分类
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

                if (index > -1) { // 重复点击,取消选中
                    that.selectCityId.splice(index, 1);
                    that.selectCityClass.splice(index, 1);
                    return true;
                }

                if (that.multiple) { // 多选
                    if (level == 1 || level == 2) {
                        let childrenIds = [],
                            oneClass = classList[one];

                        if (level == 1) { // 选择一级，清空下级已选项
                            if (typeof oneClass.childrenIds === 'undefined') {
                                childrenIds = await that.getCityChildIds(oneClass.id);
                                that.$set(that.classList[one], 'childrenIds', childrenIds);
                            } else {
                                childrenIds = oneClass.childrenIds;
                            }
                            that.handleSelectClass(childrenIds);
                        } else if (level == 2) { // 选择二级 清空三级已选选项
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
                        message.warning('最多选择' + max + '项');
                        return false;
                    }
                    that.selectCityId.push(id);
                    that.selectCityClass.push({id: id, name: name});
                } else { // 单选
                    that.selectCityId = [id];
                    that.selectCityClass = [
                        {id: id, name: name}
                    ]; // 单选覆盖选中值
                }
            },
            // 删除选中分类
            handleCloseCity(id) {
                let index = this.selectCityId.indexOf(id);

                if (index > -1) {
                    this.selectCityId.splice(index, 1);
                    this.selectCityClass.splice(index, 1);
                }
            },
            /**
             * 下级分类处理
             * @params ids 所有下级ID
             */
            handleSelectClass(ids) {
                let that = this,
                    index = -1;

                if (ids && ids.length > 0 && that.selectCityId.length > 0) { // 一级或二级选中，清空下级已选选项
                    ids.forEach(function (id) {
                        index = that.selectCityId.indexOf(id);
                        if (index > -1) { // 检索已选中下级
                            that.selectCityId.splice(index, 1); // 删除下级
                            that.selectCityClass.splice(index, 1);
                        }
                    })
                }
            },

            // 弹出层搜索
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
                }, 500); // 延迟时间设置为500毫秒
            },

            // 确认选中分类
            handleSubmitCity() {
                let that = this;

                // 调用引入页面方法,自行处理数据
                this.cityId = deepClone(this.selectCityId);
                this.cityClass = deepClone(this.selectCityClass);
                this.classOptions = deepClone(this.selectCityClass);

                let timer = setTimeout(() => {
                    that.classOptions = [];
                    timer = null;
                }, 500); // 清空搜索下拉选项

                this.cityVisible = false;
                this.$emit("confirm", {cityId: this.cityId});
            },

            // 处理选中值
            handleSelected() {
                let that = this,
                    selected = this.selected;

                if (this.searchCity !== '') { // 上一次有搜索过，再次打开重新加载分类数据
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