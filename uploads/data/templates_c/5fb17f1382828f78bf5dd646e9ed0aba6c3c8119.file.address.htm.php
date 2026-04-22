<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 18:20:01
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/member/com/address.htm" */ ?>
<?php /*%%SmartyHeaderCode:170987397569e8a0d19d62b8-60988029%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '5fb17f1382828f78bf5dd646e9ed0aba6c3c8119' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/member/com/address.htm',
      1 => 1700725932,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '170987397569e8a0d19d62b8-60988029',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'config' => 0,
    'defLink' => 0,
    'now_url' => 0,
    'rows' => 0,
    'v' => 0,
    'pagenav' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8a0d19ee9a0_41010610',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8a0d19ee9a0_41010610')) {function content_69e8a0d19ee9a0_41010610($_smarty_tpl) {?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<div class="w1000">
    <div class="admin_mainbody">
        <?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/left.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>


   
   <div class=right_box>
       <div class="newmember_tit">
           <ul>
               <li><a href="index.php?c=info">基本信息</a></li>
               <li><a href="index.php?c=info&act=side">补充信息</a></li>
               <li class="newmember_titcur"><a href="index.php?c=address">地址管理</a></li>
               <li><a href="index.php?c=show"> 公司相册</a></li>
               <li><a href="index.php?c=uppic"> 公司LOGO</a></li>
               <li><a href="index.php?c=product">产品介绍</a></li>
               <li><a href="index.php?c=news">公司资讯</a></li>
               <?php if ($_smarty_tpl->tpl_vars['config']->value['map_key']) {?>
               <li><a href="index.php?c=map">公司地图</a></li>
               <?php }?>
               <li><a href="index.php?c=comtpl">个性化模板</a></li>
           </ul>
       </div>
       <div class="newmember_screenbox">
           <div class="newmember_screen">
               <div class="com_topbth_box">
                   <input class="com_topbth_input newAddressBtn" type="button" value="新增地址"
                          data-provinceid="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['provinceid'];?>
" data-cityid="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['cityid'];?>
"
                          data-three_cityid="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['three_cityid'];?>
"
                          data-address="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['address'];?>
" data-x="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['x'];?>
"
                          data-y="<?php echo $_smarty_tpl->tpl_vars['defLink']->value['y'];?>
">
               </div>
           </div>
       </div>
       <div class="clear"></div>
       <div class=admincont_box>
           <div class="com_body">
               <iframe id="supportiframe" name="supportiframe" onload="returnmessage('supportiframe');"
                       style="display:none"></iframe>
               <form action='<?php echo $_smarty_tpl->tpl_vars['now_url']->value;?>
&act=delAllAddress' target="supportiframe" method="post" id='myform'
                     class='layui-form'>
                   <div class="  ">
                       <table class="com_table   ">
                           <tr>
                               <th></th>
                               <th>联系人</th>
                               <th align="left">联系方式</th>
                               <th>所在区域</th>
                               <th>详细地址</th>
                               <th width="230">操作</th>
                           </tr>
                           <tr>
                               <td align="center">
                                   <input type="checkbox" disabled class="newcom_user_infoheck" lay-skin="primary"/>
                               </td>
                               <td align="center">
                                   <?php echo $_smarty_tpl->tpl_vars['defLink']->value['link_man'];?>

                               </td>
                               <td align="left">
                                   <?php if ($_smarty_tpl->tpl_vars['defLink']->value['link_moblie']) {?>手机：<?php echo $_smarty_tpl->tpl_vars['defLink']->value['link_moblie'];?>
<br><?php }?>
                                   <?php if ($_smarty_tpl->tpl_vars['defLink']->value['link_phone']) {?>固话：<?php echo $_smarty_tpl->tpl_vars['defLink']->value['link_phone'];?>
<br><?php }?>
                                   <?php if ($_smarty_tpl->tpl_vars['defLink']->value['email']) {?>邮箱：<?php echo $_smarty_tpl->tpl_vars['defLink']->value['email'];
}?>
                               </td>
                               <td align="center"><?php echo $_smarty_tpl->tpl_vars['defLink']->value['city'];?>
</td>

                               <td align="center"><?php echo $_smarty_tpl->tpl_vars['defLink']->value['address'];?>
</td>
                               <td align="center">默认企业地址</td>
                           </tr>

                           <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['rows']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
                           <tr>
                               <td align="center">
                                   <input type="checkbox" name="delid[]" value="<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
"
                                          class="newcom_user_infoheck" lay-skin="primary"/>
                               </td>
                               <td align="center">
                                   <?php echo $_smarty_tpl->tpl_vars['v']->value['link_man'];?>

                               </td>
                               <td align="left">
                                   <?php if ($_smarty_tpl->tpl_vars['v']->value['link_moblie']) {?>手机：<?php echo $_smarty_tpl->tpl_vars['v']->value['link_moblie'];?>
<br><?php }?>
                                   <?php if ($_smarty_tpl->tpl_vars['v']->value['link_phone']) {?>固话：<?php echo $_smarty_tpl->tpl_vars['v']->value['link_phone'];?>
<br><?php }?>
                                   <?php if ($_smarty_tpl->tpl_vars['v']->value['email']) {?>邮箱：<?php echo $_smarty_tpl->tpl_vars['v']->value['email'];?>
 <?php }?>
                               </td>
                               <td align="center"><?php echo $_smarty_tpl->tpl_vars['v']->value['city'];?>
</td>

                               <td align="center"><?php echo $_smarty_tpl->tpl_vars['v']->value['link_address'];?>
</td>
                               <td align="center">
                                   <a href="javascript:;" class="com_bth cblue newAddressBtn"
                                      data-id="<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
" data-link_man="<?php echo $_smarty_tpl->tpl_vars['v']->value['link_man'];?>
"
                                      data-link_moblie="<?php echo $_smarty_tpl->tpl_vars['v']->value['link_moblie'];?>
"
                                      data-link_phone="<?php echo $_smarty_tpl->tpl_vars['v']->value['link_phone'];?>
" data-email="<?php echo $_smarty_tpl->tpl_vars['v']->value['email'];?>
"
                                      data-provinceid="<?php echo $_smarty_tpl->tpl_vars['v']->value['provinceid'];?>
" data-cityid="<?php echo $_smarty_tpl->tpl_vars['v']->value['cityid'];?>
"
                                      data-three_cityid="<?php echo $_smarty_tpl->tpl_vars['v']->value['three_cityid'];?>
"
                                      data-address="<?php echo $_smarty_tpl->tpl_vars['v']->value['link_address'];?>
" data-x="<?php echo $_smarty_tpl->tpl_vars['v']->value['x'];?>
"
                                      data-y="<?php echo $_smarty_tpl->tpl_vars['v']->value['y'];?>
">修改</a>
                                   <a href="javascript:;" class="com_bth cblue delAddressA" data-id="<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
">删除</a>
                               </td>
                           </tr>
                           <?php } ?>
                           <tr>
                               <td align="center">
                                   <input type="checkbox" lay-skin="primary" lay-filter='allAddress'>
                               </td>
                               <td colspan="7"><input class="c_btn_02" type="button" name="subdel" value="批量删除"
                                                      onclick="return really('delid[]');"></td>
                           </tr>
                           <tr>
                               <td colspan="8" class="table_end">
                                   <div class="diggg"><?php echo $_smarty_tpl->tpl_vars['pagenav']->value;?>
</div>
                               </td>
                           </tr>
                       </table>
                   </div>
               </form>
           </div>
       </div>
   </div>
    <?php echo '<script'; ?>
>
        layui.use(['form'], function () {
            var form = layui.form,
                $ = layui.$;

            form.on('checkbox(allAddress)', function (data) {
                $("input[name='delid[]']").each(function () {
                    this.checked = data.elem.checked;
                });
                form.render('checkbox');
            });
        });
        $('.delAddressA').off('click').on('click', function () {

            var thisAddressId = parseInt($(this).attr('data-id'));

            if (isNaN(thisAddressId) || thisAddressId < 1) {
                layer.msg('地址数据错误！', 2, 8);
                return false;
            }

            layer.confirm('确定要删除招聘地址吗？', function () {

                var i = loadlayer();

                $.post("index.php?c=address&act=delAddress", {id: thisAddressId}, function (data) {

                    layer.closeAll();

                    var res = eval('(' + data + ')');

                    if (res.errcode == 9) {

                        layer.msg(res.msg, 2, 9, function () {
                            location.reload();
                        });
                    } else {

                        layer.msg(res.msg, 2, 8);
                    }
                    return false;
                });
            });
        });
    <?php echo '</script'; ?>
>
</div>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/newAddress.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/footer.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>
<?php }} ?>
