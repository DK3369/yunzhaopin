<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="180">名称</th>
                    <th width="400">状态</th>
                    <th>说明</th>
                </tr>
                </thead>
                <tbody>

                <tr>
                    <td>
                        <div class="TableTite">公众号名称</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="configdata.wx_name"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>在微信公众平台获取</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">URL</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="configdata.backurl" :disabled="true"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span> </span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">Token</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="configdata.wx_token"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>自定义填写，需和微信公众平台一致</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="post">提交</el-button>
        </div>
    </div>
</template>

<script>
module.exports = {
    props: {
        config: {
            type: Object,
            default: {}
        },
    },

    data: function () {
        return {
            configdata: {}
        }
    },
    watch: {
        config: {
            handler(val, oldVal) {
                this.configdata = val;
            },
            immediate: true,
            deep: true,
        }
    },
    methods: {
        post: function () {
            this.$emit('post-set', {type: 'wxgzh', config: this.configdata})
        }
    },
};
</script>
<style scoped></style>