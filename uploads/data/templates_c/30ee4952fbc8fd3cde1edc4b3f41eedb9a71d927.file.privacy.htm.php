<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:41:14
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/member/user/privacy.htm" */ ?>
<?php /*%%SmartyHeaderCode:212787412969e897baa4d308-67298219%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '30ee4952fbc8fd3cde1edc4b3f41eedb9a71d927' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/member/user/privacy.htm',
      1 => 1700725935,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '212787412969e897baa4d308-67298219',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'wap_style' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e897baa639c6_69200107',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e897baa639c6_69200107')) {function content_69e897baa639c6_69200107($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/member/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<div id="yunvue" class="none">
<div class="mui-content">
    <div class="privacy_title">
        <p>提示：设置为公开时,企业才能搜索到默认简历</p>
    </div>
    <div class="privacy_body">
        <ul class="privacy_body_card">
            <li  @click="changeStatus(1)" value="1">
                <a class="mui-navigate-right">
                    <div class="privacy_body_card_title">简历公开</div>
                     <span class="privacy_list_p">我正在找工作，希望企业关注我的简历</span>
                </a>
                    <div  class="mui-table-pitch-on"  v-if="status==1">
                    <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/table-view_yes.png" alt="" width="100%" height="100%">
                </div>
            </li>
            <li  @click="changeStatus(3)" value="3">
                <a class="mui-navigate-right">
                    <div class="privacy_body_card_title">仅投递公司可见</div>
                     <span class="privacy_list_p">可投递简历，仅投递公司可见</span>
                </a>
                    <div class="mui-table-pitch-on" v-if="status==3">
                    <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/table-view_yes.png" alt="" width="100%" height="100%">
                </div>
            </li>
            <li  @click="changeStatus(2)" value="2">
                <a class="mui-navigate-right">
                    <div class="privacy_body_card_title">简历保密</div>
                     <span class="privacy_list_p">不找工作，企业不能搜索到您的简历</span>
               </a>
                <div class="mui-table-pitch-on" v-if="status==2">
                    <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/table-view_yes.png" alt="" width="100%" height="100%">
                </div>
            </li>
            <li class="privacy_list_af" @click="pageTo('index.php?c=blacklist')">
               <div>
                <div class="privacy_body_card_title">屏蔽企业 </div>
                <span class="privacy_list_p">我不希望某些企业搜索到我的简历</span>
               </div>
                <div class="mui-table-pitch-on">
                    <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/issue_add.png" alt="" width="100%" height="100%">
                </div>
            </li>
        </ul>
    </div>
   
</div>



</div>
<?php echo '<script'; ?>
 type="text/javascript">  
    var jobid = '<?php echo $_GET['jobid'];?>
';
        jobid = jobid ? jobid : ''; 
    var yunvue =  new Vue({
        el: '#yunvue',
        data() {
            return {
               status: 0
            };
        },        
        created() {
            this.privacy();
        },
        methods:{
            privacy: function(){
                showLoading();
                let that = this;
                $.post('<?php echo smarty_function_url(array('d'=>'wxapp','h'=>'user','m'=>'privacy','c'=>'privacy'),$_smarty_tpl);?>
', {rand:Math.random()}, function(data){
                    hideLoading();
                    if (data) {
                        that.status = data.data;                       
                    }
                    $("#yunvue").css('display', 'block');
                },'json');
            },
            pageTo:function(url){
                window.location.href=url;
            },
            changeStatus:function(e){
                var that = this;
                var status = e;
                var paramer = {                   
                    status: status,
                };
                showLoading('设置中');
                $.post('<?php echo smarty_function_url(array('d'=>'wxapp','h'=>'user','m'=>'privacy','c'=>'up'),$_smarty_tpl);?>
', paramer, function(data){
                    hideLoading();
                    if (data.error == 1) {
                        var list = [];
                        list = data.data;
                        that.status = list.status;
						window.localStorage.setItem("needRefresh", 1);
                        showToast('操作成功');
                        if(jobid){
                            // 从职位详情页来的，修改后要刷新职位页面
                            history.back();
                        }
                    }
                },'json');
            }
        },
    });
<?php echo '</script'; ?>
> 
</body>
</html><?php }} ?>
