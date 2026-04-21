<template>
  <div>
    <!--广告管理 调用-->
    <div style="margin-top: -20px;">
      <div class="wxsettip_small">广告调用方式</div>
      <div class="">
        <template v-if="info.is_end == '1'">
          广告已过期，不可调用。
        </template>
        <template v-else-if="info.is_open == '0'">
          广告停用，不可调用
        </template>
        <template v-else>
          <el-radio v-model="type" label="1" @input="handleType">站内调用</el-radio>
          <el-radio v-model="type" label="2" @input="handleType">站外调用</el-radio>
        </template>
      </div>
      <div v-if="type != null">
        <div class="wxsettip_small">广告调用代码
          <el-tooltip class="item" effect="dark" content="复制(CTRL+C)以下內容并添加到模板中" placement="right-start"><i class="el-icon-warning-outline"></i>
            <el-button>右上</el-button>
          </el-tooltip>
        </div>
        <el-input id="elementCode" v-model="code" placeholder=""></el-input>
        <div class="wxsettip">将JS代码复制 粘贴到需要投放该广告的页面</div>
    <!--    <div class="wxsettip"> 可以在-->
    <!--      <el-link href="https://work.weixin.qq.com" target="_blank">后台模板管理中修改</el-link>-->
    <!--    </div>-->
        <div slot="footer" class="dialog-footer">
          <el-button id="copyBtn" type="primary"
                     data-clipboard-action="copy"
                     data-clipboard-target="#elementCode"
                     @click="handleCopyValue">复制代码</el-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
module.exports = {
  props: {
    id: Number,
  },
  data: function () {
    return {
      info: {},
      type: null,
      code: '',
    }
  },
  mounted() {
    console.log('ad_preview mounted')
  },
  methods: {
    getInfo() {
      let _this = this;
      let params = {id: this.id};
      httpPost('m=yunying&c=ad&a=preview', params).then(function (response) {
        let res = response.data;
        if (res.error === 0) {
          _this.info = res.data;
        } else {
          message.error('暂无数据');
        }
      }).catch(function (error) {
        console.log(error);
      });
    },
    handleType(val) {
      if (this.info.hasOwnProperty('id')) {
        if (val == '1') {
          this.code = "{yun:}ad cid=" + this.info.class_id + " id=" + this.info.id + "{/yun}";
        } else if (val == '2') {
          this.code = "<script src='" + this.info.src + "' language='javascript'><\/script>";
        }
      }
    },
    /**
     * 复制代码
     */
    handleCopyValue() {
      let clipboard = new ClipboardJS('#copyBtn'); // 获取点击按钮的元素
      /* 注意此事件监听是异步的   */
      clipboard.on('success', (e) => {
        e.clearSelection();
        // 释放内存
        clipboard.destroy();
        message.success('复制成功');
      });
      // 复制失败
      clipboard.on('error', (e) => {
        // 释放内存
        clipboard.destroy();
        message.error('复制失败');
      });
    },
  },
  watch: {
    id: {
      handler: function (newValue, oldValue) {
        console.log('ad_preview watch id', newValue);
        if (newValue) {
          this.getInfo();
        }
      },
      deep: true,
      immediate: true
    },
  }
}
</script>

<style scoped>
.dialog_item {
  margin-top: 25px;
  display: flex;
}
.item_span {
  width: 90px;
  text-align: right;
  display: block;
}
.dialog-footer{
  padding: 10px 0 0;
  text-align: right;
  -webkit-box-sizing: border-box;
  box-sizing: border-box;
}
</style>