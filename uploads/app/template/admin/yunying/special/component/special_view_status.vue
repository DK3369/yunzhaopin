<template>
  <div style="padding: 10px 20px;">
    <div class="wxsettip_small">{yun:}t key='admin_user_weipin_00032'{/yun}</div>
    <template>
      <el-radio v-model="ruleForm.status" label="1">{yun:}t key='member_user_00042'{/yun}</el-radio>
      <el-radio v-model="ruleForm.status" label="2">{yun:}t key='wap_user_00167'{/yun}</el-radio>
    </template>
    <div class="wxsettip_small">{yun:}t key='member_user_00062'{/yun} </div>
    <el-input type="textarea" :rows="2" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="ruleForm.statusbody" style="margin-bottom: 10px;"></el-input>
    <span slot="footer" class="dialog-footer">
      <el-button @click="handleCancel">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
      <el-button type="primary" @click="submitForm('ruleForm')">{yun:}t key='wap_com_00019'{/yun}</el-button>
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
        message.error("{yun:}t key='admin_01311'{/yun}");
        return false;
      }

      httpPost('m=yunying&c=special_special&a=statuscom', params).then(function (response) {
        let res = response.data;
        if (res.error === 0) {
          message.success("{yun:}t key='wap_js_00159'{/yun}");
          _this.$emit("child-event");
        } else {
          message.error("{yun:}t key='model_00003'{/yun}");
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