<template>
  <div style="padding: 10px 20px;">
    <div class="wxsettip_small">审核操作</div>
    <template>
      <el-radio v-model="ruleForm.status" label="1">已通过</el-radio>
      <el-radio v-model="ruleForm.status" label="2">未通过</el-radio>
    </template>
    <div class="wxsettip_small">审核说明 </div>
    <el-input type="textarea" :rows="2" placeholder="请输入内容" v-model="ruleForm.statusbody" style="margin-bottom: 10px;"></el-input>
    <span slot="footer" class="dialog-footer">
      <el-button @click="handleCancel">取 消</el-button>
      <el-button type="primary" @click="submitForm('ruleForm')">确 定</el-button>
    </span>
  </div>
</template>
<script>
module.exports = {
  props: {
    pid: {type: String, default: ''},
  },
  data: function () {
    return {
      ruleForm: {
        pid: '',
        status: null,
        statusbody: '',
      },
    }
  },
  mounted() {
  },
  methods: {
    submitForm(formName) {
      let _this = this;
      let params = JSON.parse(JSON.stringify(this.ruleForm));
      if (params.status != '1' && params.status != '2') {
        message.error('请选择审核状态！');
        return false;
      }

      httpPost('m=yunying&c=special_special&a=statuscom', params).then(function (response) {
        let res = response.data;
        if (res.error === 0) {
          message.success('操作成功！');
          _this.$emit("child-event");
        } else {
          message.error('操作失败！');
        }
      }).catch(function (error) {
        console.log(error);
      });
    },
    handleCancel(){
      this.$emit("child-event-close");
    }
  },
  watch: {
    pid: {
      handler: function (newValue, oldValue) {
        if (newValue) {
          this.ruleForm.pid = newValue;
        }
      },
      deep: true,
      immediate: true
    },
  }
};
</script>
<style scoped>

</style>