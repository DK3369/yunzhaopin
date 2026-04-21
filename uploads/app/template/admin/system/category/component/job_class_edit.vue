<template>
    <div class="tableDome" style="top: 40px;">
        <div class="moduleTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="200">名称</th>
                    <th width="500">状态</th>
                    <th>说明</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">类别名称</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input placeholder="请输入内容" v-model="ruleForm.position">
                                <!-- <span slot="suffix" class="slotspan">天</span> -->
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>类别名称</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">英文名称/拼音</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input placeholder="请输入内容" v-model="ruleForm.e_name">
                                <!-- <span slot="suffix" class="slotspan">天</span> -->
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>英文名称/拼音</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">一级分类</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="ruleForm.nid" placeholder="请选择" @change="getClass(ruleForm.nid)" clearable>
                                <el-option v-for="item in position" :key="item.id" :label="item.name" :value="item.id"></el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>一级分类</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">二级分类</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="ruleForm.keyid" placeholder="请选择" clearable>
                                <el-option v-for="item in positionTwo" :key="item.id" :label="item.name"
                                    :value="item.id">
                                </el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>二级分类</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">职位样本描述</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input type="textarea" placeholder="请输入内容" v-model="ruleForm.content">
                                <!-- <span slot="suffix" class="slotspan">天</span> -->
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>职位样本描述</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">排　　序</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.sort" placeholder="请输入数字" @input="inputIntNumber($event, 'ruleForm', 'sort')"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>越小越在前</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">提交</el-button>
        </div>
    </div>
</template>
<script>
module.exports = {
    props: {
        tid: {type: [Number, String], default: 0},//第二级
        id: {type: [Number, String], default: 0},//第三级
    },
    data: function () {
        return {
            ruleForm: {
                id: 0,//提交时的id
                position: '',//类别名称
                e_name: '',//英文名称/拼音
                nid: null,//一级分类
                keyid: null,//二级分类
                content: null,//职位样本描述
                sort: '',//排序
            },
            position: [],//一级分类
            positionTwo: [],//第二级分类
            submitLoading: false,
        }
    },
    mounted() {
        if (this.tid > 0 || this.id > 0) {
            //修改
            this.ruleForm.id = this.tid > 0 ? (this.tid) : (this.id > 0 ? this.id : 0);
            this.getInfo();
        } else {
            //添加
            this.ruleForm.id = 0;
            this.getPosition();
        }
    },
    methods: {
        getInfo() {
            let _this = this;
            let params = {};
            if (this.id) {
                params.id = this.id;
            }
            if (this.tid) {
                params.tid = this.tid;
            }
            httpPost('m=system&c=category_job_class&a=classadd', params).then(function (response) {
                let res = response.data;
                if (res.data.info) {
                    _this.ruleForm.position = res.data.info.name;
                    _this.ruleForm.e_name = res.data.info.e_name;
                    _this.ruleForm.content = res.data.info.content;
                    _this.ruleForm.sort = res.data.info.sort;
                    if (res.data.type === 'two') {
                        _this.ruleForm.nid = res.data.info.keyid;
                    } else if (res.data.type === 'three') {
                        _this.ruleForm.nid = res.data.job.keyid;
                        _this.ruleForm.keyid = res.data.info.keyid;
                        _this.positionTwo = res.data.class2;
                    }
                }
                _this.position = res.data.position;
            }).catch(function (error) {
                console.log(error);
            });
        },
        getPosition() {
            let _this = this;
            httpPost('m=system&c=category_job_class&a=getJobClass', {}, {hideloading: true}).then(function (response) {
                let res = response.data;
                _this.position = res.data;
            }).catch(function (error) {
                console.log(error);
            });
        },
        getClass(nid) {
            let _this = this;
            if (nid <= 0) {
                _this.ruleForm.keyid = null;
                _this.positionTwo = [];
                return false;
            }
            this.ruleForm.keyid = null;
            httpPost('m=system&c=category_job_class&a=get_class', {nid: nid}).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    _this.positionTwo = res.data;
                } else {
                    _this.positionTwo = [];
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        submitForm(formName) {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.ruleForm));
            params.submit = 'submit';
            if (params.position == '') {
                message.error('类别名称不能为空！');
                return;
            }
            _this.submitLoading = true;
            httpPost('m=system&c=category_job_class&a=save', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                    _this.$emit("child-event-getlist");
                } else {
                    message.error(res.msg);
                }
            }).catch(function (error) {
                console.log(error);
            }).finally(function () {
                _this.submitLoading = false;
            });
        },
        inputIntNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9]/g, '');
        },
    },
};
</script>
<style scoped>
.moduleTable {
    max-height: calc(100% - (60px + 20px));
}
</style>