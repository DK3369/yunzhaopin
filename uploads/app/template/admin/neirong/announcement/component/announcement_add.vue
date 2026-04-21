<template>
    <div class="drawerModlue">
        <div class="moduleTable" style="max-height: calc(100% - 80px);">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="100">参数</th>
                    <th>内容</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">公告标题</div>
                    </td>
                    <td>
                        <div class="TableInpt w_400">
                            <el-input placeholder="请输入公告标题" v-model="ruleForm.title"></el-input>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">使用范围</div>
                    </td>
                    <td>
                        <div class="TableSelect w_400" style="display: flex;align-items: center;">
                            <el-select v-model="ruleForm.did" placeholder="请选择">
                                <el-option v-for="(item, key) in domainList" :key="key" :label="item" :value="key"></el-option>
                            </el-select>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">关键词</div>
                    </td>
                    <td style="display: flex;">
                        <div class="w_400" style="display: flex;align-items: center;">
                            <el-input placeholder="请输入关键词" v-model="ruleForm.keyword"></el-input>
                        </div>
                        <div class="TableShuom" style="padding: 6px;">
                            <span><i class="el-icon-warning"></i>多关键字,请用,隔开,请不要为空</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">开始时间</div>
                    </td>
                    <td style="display: flex;">
                        <div class="TableInpt w_400">
                            <el-date-picker v-model="ruleForm.startime" type="date" :picker-options="pickerOptions" style="width: 100%;" placeholder="选择开始时间"></el-date-picker>
                        </div>
                        <div class="TableShuom" style="padding: 6px;">
                            <span><i class="el-icon-warning"></i>默认当前时间</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">结束时间</div>
                    </td>
                    <td style="display: flex;">
                        <div class="TableInpt w_400">
                            <el-date-picker v-model="ruleForm.endtime" type="date" :picker-options="pickerOptions" style="width: 100%;" placeholder="选择结束时间"></el-date-picker>
                        </div>
                        <div class="TableShuom" style="padding: 6px;">
                            <span><i class="el-icon-warning"></i>默认时间为永久</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">描述</div>
                    </td>
                    <td>
                        <div class="TableInpt w_400">
                            <el-input type="textarea" placeholder="请输入描述" v-model="ruleForm.description"></el-input>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">公告内容</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <textarea type="textarea" id="projectBasis" class="editor" name="projectBasis" cols="150" rows="30" style="width: 80%">
                            </textarea>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">提交</el-button>
        </div>
    </div>
</template>
<!-- script -->
<script>
    module.exports = {
        props: ['id'],
        data: function () {
            return {
                pickerOptions: {
                    disabledDate(time) {
                        // 禁止选择过去日期（不能选择今天）
                        return time.getTime() < Date.now();
                    },
                },
                ruleForm: {},
                domainList: [],
                saveLoading: false,
                ue:''
            }
        },
        mounted() {
            var ue = UE.getEditor('projectBasis', {
                wordCount: false,           // 关闭字数统计
                elementPathEnabled: false,  //关闭elementPath 元素路径
                autoHeightEnabled: false,   //关闭自适应高度，超出部分以滚动条形式展示
                initialFrameHeight: 480,    //默认的编辑区域高度
                initialFrameWidth: 600      //初始化编辑器宽度,默认1000
            });

        },
        created: function () {
            this.getInfo();
        },
        methods: {
            getInfo() {
                let that = this;
                httpPost('m=neirong&c=announcement&a=add', {id: that.id ? that.id : ''}).then(function (response) {
                    let res = response.data,
                        data = res.data,
                        info = data.info;

                    that.domainList = data.domainList;
                    if (that.id) {
                        that.ruleForm = {
                            id: info.id,
                            title: info.title,
                            did: info.did,
                            keyword: info.keyword,
                            startime: info.startime > 0 ? new Date(info.startime_n) : new Date(),
                            endtime: info.endtime > 0 ? new Date(info.endtime_n) : '',
                            description: info.description,
                        };
                        if (info.content){
                            UE.getEditor('projectBasis').setContent(info.content);
                        }
                        // editor.setHtml(info.content);
                    } else {
                        that.ruleForm = {
                            did: '-1',
                            startime: new Date()
                        };
                    }
                })
            },

            save() {
                let that = this,
                    params = that.ruleForm;

                if (typeof params.title == 'undefined' || params.title == '') {
                    message.warning('请输入公告标题');
                    return;
                }
                if (typeof params.keyword == 'undefined' || params.keyword == '') {
                    message.warning('请输入公告关键字');
                    return;
                }

                if (params.startime && params.endtime && params.startime > params.endtime) {
                    message.warning('结束时间必须大于开始时间');
                    return;
                }

                if (typeof params.description == 'undefined' || params.description == '') {
                    message.warning('请输入公告描述');
                    return;
                }


                if (that.saveLoading) {
                    return false;
                }
                that.saveLoading = true;

                params.content = UE.getEditor('projectBasis').getContent();

                params.submit = true;
                httpPost('m=neirong&c=announcement&a=add', params).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        message.error(res.msg, function () {
                            that.saveLoading = false;
                        });
                    } else {
                        that.$emit("child-event");
                        message.success(res.msg, function () {
                            that.saveLoading = false;
                        });
                    }
                })
            },
        },
        watch: {
            id: function (val, oldVal) {
                this.ruleForm = {};
                UE.getEditor('projectBasis').setContent('');
                this.getInfo();
            },
        }
    };
</script>
<style>
    .w_400{ width: 400px;}
    .tableVue .TableSelect .el-select{ width: 400px;}
</style>