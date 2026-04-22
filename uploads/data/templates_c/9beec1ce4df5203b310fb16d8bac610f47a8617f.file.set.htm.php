<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 18:14:37
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/member/user/set.htm" */ ?>
<?php /*%%SmartyHeaderCode:206618312269e89f8dd1d503-23562939%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '9beec1ce4df5203b310fb16d8bac610f47a8617f' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/member/user/set.htm',
      1 => 1700725935,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '206618312269e89f8dd1d503-23562939',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'wap_style' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e89f8dd2df20_43409739',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e89f8dd2df20_43409739')) {function content_69e89f8dd2df20_43409739($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/member/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<div class="none" id="yunvue">
 <!-- 页面整体部分 -->
      <div class="issue_post_body">
       <div class="issue_post_body_card">
        <div onclick="navigateTo('index.php?c=info')" class="post_body_card_job">
            <div class="body_card_job_box">
                <div class="card_job_box_post">基本资料</div>
                <div class="card_job_box_name">完善个人基本资料</div>
            </div>
            <div class="body_card_job_icon">
                <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/issue_add.png" alt="" width="100%">
            </div>
        </div>
           <div class="post_body_card_job" onclick="navigateTo('index.php?c=ident')">
               <div class="body_card_job_box">
                   <div class="card_job_box_post">认证与绑定</div>
               </div>
               <div class="body_card_job_icon">
                   <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/issue_add.png" alt="">
               </div>
           </div>
           <div class="post_body_card_job" onclick="navigateTo('index.php?c=safe')">
               <div class="body_card_job_box">
                   <div class="card_job_box_post">账号与安全</div>
               </div>
               <div class="body_card_job_icon">
                   <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/issue_add.png" alt="">
               </div>
           </div>
           <div onclick="navigateTo('index.php?c=privacy')" class="post_body_card_job"> <div class="body_card_job_box">
               <div class="card_job_box_post">隐私设置</div>
               <div class="card_job_box_name">设置简历公开、保密</div>
           </div>
               <div class="body_card_job_icon">
                   <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/issue_add.png" alt="">
               </div>
           </div>
        <div onclick="navigateTo('<?php echo smarty_function_url(array('m'=>'wap','c'=>'advice'),$_smarty_tpl);?>
')" class="post_body_card_job">
            <div class="body_card_job_box">
                <div class="card_job_box_post">意见反馈</div>
            </div>
            <div class="body_card_job_icon">
                <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/issue_add.png" alt="">
            </div>
        </div>
       </div>
       <div class="logout_btn" onclick="islogout('<?php echo smarty_function_url(array('m'=>'wap','c'=>'loginout'),$_smarty_tpl);?>
','确认退出吗？');">退出登录</div>
      </div>
</div>
<?php echo '<script'; ?>
>
    new Vue({
        el: '#yunvue',
        data: {
            info: {}
        },
        created() {
            this.getInfo();
        },
        methods:{
            getInfo:function(){
                showLoading();
                var self = this;
                $.post('<?php echo smarty_function_url(array('d'=>'wxapp','h'=>'user','m'=>'set','c'=>'getInfo'),$_smarty_tpl);?>
',{rand: Math.random()},function(res){
                    hideLoading();
                    $("#yunvue").css('display', 'block');
                    self.info = res.data;

                },'json');
            }
        }
    })
<?php echo '</script'; ?>
>
</body>
</html>  <?php }} ?>
