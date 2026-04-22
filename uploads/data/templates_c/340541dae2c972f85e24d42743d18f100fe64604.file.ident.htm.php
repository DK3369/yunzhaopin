<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:42:59
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/default/register/ident.htm" */ ?>
<?php /*%%SmartyHeaderCode:72507293969e89823023f59-19833971%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '340541dae2c972f85e24d42743d18f100fe64604' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/default/register/ident.htm',
      1 => 1700725928,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '72507293969e89823023f59-19833971',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'title' => 0,
    'keywords' => 0,
    'description' => 0,
    'style' => 0,
    'config' => 0,
    'type' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e89823044f60_71554317',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e89823044f60_71554317')) {function content_69e89823044f60_71554317($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
<html xmlns="http://www.w3.org/1999/xhtml">

    <head>
        <meta http-equiv="Content-Type" content="text/html;charset=utf-8" />
        <title><?php echo $_smarty_tpl->tpl_vars['title']->value;?>
</title>
        <meta name="keywords" content="<?php echo $_smarty_tpl->tpl_vars['keywords']->value;?>
" />
        <meta name="description" content="<?php echo $_smarty_tpl->tpl_vars['description']->value;?>
" />
        <link rel="stylesheet" href="<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/style/css.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" type="text/css" />
        <link rel="stylesheet" href="<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/style/login.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" type="text/css" />
        <?php echo '<script'; ?>
>
            var weburl = "<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
",
                integral_pricename = '<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
',
                pricename = '<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
',
                code_web = '<?php echo $_smarty_tpl->tpl_vars['config']->value['code_web'];?>
',
                code_kind = '<?php echo $_smarty_tpl->tpl_vars['config']->value['code_kind'];?>
';
        <?php echo '</script'; ?>
>
    </head>

    <body>
        <div class="register_header">
            <div class="reg_w980">
                <div class="reg_logo">
                    <a href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
"><img src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_ossurl'];?>
/<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_logo'];?>
" class="png"></a>
                </div>
                <div class="reg_msg">欢迎注册</div>
                <a href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
" class="reg_fh yun_text_color fr">返回首页</a>
            </div>
        </div>
         <div class="clear"></div>
        <!--content  start-->
            <div class="reg_content">
        <div class="reg_stepbox">
        <ul class="reg_stepbox_list">
        <li class="reg_stepcur"><span class="reg_stepbox_n">1</span><em class="reg_stepbox_name">创建账户</em></li>
        <li class="reg_stepcur"><span class="reg_stepbox_line"></span></li>
        <li  class="reg_stepcur"><span class="reg_stepbox_n">2</span><em class="reg_stepbox_name">设置身份信息</em></li>
        <li  class="reg_stepcur"><span class="reg_stepbox_line"></span></li>
        <li><span class="reg_stepbox_n"><i class="reg_stepbox_cgicon"></i></span><em class="reg_stepbox_name">注册成功</em></li>
        </ul>
        </div>
        
            <div class="logoin_cont fl"> 
              <div class="reg_content_tit">请从下方选择您的角色类型，享受专属功能</div>
            
           <ul class="reg_role">
<li>
<a href="<?php echo smarty_function_url(array('m'=>'register','c'=>'ident','usertype'=>1),$_smarty_tpl);?>
">
<div class="regok_box_icon"><span class="regok_box_user_icon"><i class="regok_box_user_icon_gr"></i></span></div>
<div class="regok_box_user">我要找工作</div><div class="regok_box_user_p">免费发布简历，海量职位随意挑</div>
<div  class="reg_role_bth">进入用户中心<i class="reg_role_bth_line"></i></div></a></li>
<li>
<a href="<?php echo smarty_function_url(array('m'=>'register','c'=>'ident','usertype'=>2),$_smarty_tpl);?>
">
<div class="regok_box_icon"><span class="regok_box_user_icon"><i class="regok_box_user_icon_qy"></i></span></div>
<div class="regok_box_user">我要招人</div><div class="regok_box_user_p">发布招聘信息，收取简历投递</div><div  class="reg_role_bth">进入用户中心<i class="reg_role_bth_line"></i></div></a></li>


</ul>
            
             </div>
         <div class="clear"></div>
        <div class="reg_footer">如果您有任何疑问或在注册过程中遇到任何困难，您都可以通过以下方式与我们联系
            <div class="">电话： <?php echo $_smarty_tpl->tpl_vars['config']->value['sy_freewebtel'];?>
 <span class="reg_footer_yz">客服QQ：<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_qq'];?>
</span> </div>
                     </div>
         </div>
         
        
       

        <style>
            .icon2 {
                background-image: none;
                background-repeat: no-repeat;
            }
        </style>
        <!--content  end-->
        <div class="clear"></div>
        <?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['tplstyle']->value)."/jquery/jquery_js.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

        <link href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/layui/css/layui.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet" type="text/css" />
        <?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/layui/layui.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
        <?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/layui/phpyun_layer.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
        <?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/public.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" language="javascript"><?php echo '</script'; ?>
>
        
        <!--[if IE 6]>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/png.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
>
  DD_belatedPNG.fix('.png,.#bg');
<?php echo '</script'; ?>
>
<![endif]-->
        <?php echo '<script'; ?>
>
            <?php if ($_smarty_tpl->tpl_vars['type']->value==1) {?>
                    $(document).ready(function() {
                        $(document).keydown(function(e) {
                            var ev = document.all ? window.event : e;
                            if(ev.keyCode == 13) {
                                check_user(1, 'vcode_imgs');
                            }
                        });
                        $("#xieyi1").keyup(function(event) {
                            var myEvent = event || window.event;
                            if(myEvent.keyCode == 13) {
                                check_user(1, 'vcode_imgs');
                            } else {
                                return;
                            };
                        });
                    }) 
					<?php } elseif ($_smarty_tpl->tpl_vars['type']->value==2) {?>
                            $(document).ready(function() {
                                $(document).keydown(function(e) {
                                    var ev = document.all ? window.event : e;
                                    if(ev.keyCode == 13) {
                                        check_user(2, 'vcode_img');
                                    }
                                });
                                $("#xieyi2").keyup(function(event) {
                                    var myEvent = event || window.event;
                                    if(myEvent.keyCode == 13) {
                                        check_user(2, 'vcode_img');
                                    } else {
                                        return;
                                    };
                                });
                            }) <?php } elseif ($_smarty_tpl->tpl_vars['type']->value==3) {?>
                                $(document).ready(function() {
                                    $(document).keydown(function(e) {
                                        var ev = document.all ? window.event : e;
                                        if(ev.keyCode == 13) {
                                            check_user(3, 'vcodeimg');
                                        }
                                    });
                                    $("#xieyi3").keyup(function(event) {
                                        var myEvent = event || window.event;
                                        if(myEvent.keyCode == 13) {
                                            check_user(3, 'vcodeimg');
                                        } else {
                                            return;
                                        };
                                    });
                                }) <?php }?>
        <?php echo '</script'; ?>
>
              <div style="display:none">  <?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['tplstyle']->value)."/footer.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>
</div><?php }} ?>
