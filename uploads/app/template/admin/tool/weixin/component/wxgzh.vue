<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="180">{yun:}t key='member_com_00021'{/yun}</th>
                    <th width="400">{yun:}t key='member_user_00181'{/yun}</th>
                    <th>{yun:}t key='member_com_00207'{/yun}</th>
                </tr>
                </thead>
                <tbody>

                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_tool_00636'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="configdata.wx_name"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_tool_00637'{/yun}</span>
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
                            <span>{yun:}t key='admin_tool_00638'{/yun}</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="post">{yun:}t key='common.submit'{/yun}</el-button>
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