<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 18:20:04
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/member/com/binding.htm" */ ?>
<?php /*%%SmartyHeaderCode:141627868169e8a0d4b86ed4-97150388%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    'ea82b53201ab5be4dfe461ee5fd09b35d8d93a76' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/member/com/binding.htm',
      1 => 1700725932,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '141627868169e8a0d4b86ed4-97150388',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'member' => 0,
    'company' => 0,
    'cert' => 0,
    'config' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8a0d4ba9510_10298836',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8a0d4ba9510_10298836')) {function content_69e8a0d4ba9510_10298836($_smarty_tpl) {?><?php if (!is_callable('smarty_modifier_date_format')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/modifier.date_format.php';
if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<div class="w1000">
    <div class="admin_mainbody">

        <?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/left.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

        
        <div class=right_box>
                <div class="newmember_tit">
                                <ul>
                                    <li class="newmember_titcur"><a href="index.php?c=binding">认证与绑定</a></li>
                                    <li><a href="index.php?c=vs">密码修改</a></li>
                                    <li><a href="index.php?c=logout">账号注销</a></li>
                                     
                                         
                                </ul>
                            </div>
                              <div class="clear"></div>
                             <div class=admincont_box>
                             <div class="com_body" style="padding-top:15px;">  
             <div class="com_new_tip">
             <span class="com_new_tip_h">温馨小提示</span>上次登录时间：<?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['member']->value['login_date'],"%Y-%m-%d %H:%M:%S");?>
 ,
                        若非本人操作可&nbsp;&nbsp;<a href="index.php?c=vs" class="cblue">修改密码</a>
             </div>
                     
                    <div class="vip_box" style="padding:15px 15px">

                        <div class="Binding_list_box">
                            <div class="Binding_list_box_c">
                                <?php if ($_smarty_tpl->tpl_vars['member']->value['setname']==1) {?>
                                <div class="Binding_list">
                                    <div class="Binding_list_left">
                                        <div class="bingding_yx "><i class="binding_xg_icon"></i></div>
                                    </div>
                                    <div class="Binding_list_cont">
                                        <div class="Binding_list_name"> 修改用户名</div>
                                        <span class="Binding_list_no"><i class="Binding_list_no_icon"></i>可修改</span>
                                        <div class="Binding_list_text">
                                            您有一次修改用户名的机会（仅限一次哦~）
                                        </div>
                                        <div class="Binding_oper">
                                            <a href="index.php?c=setname" class="Binding_submit">立即修改</a>
                                        </div>
                                    </div>
                                </div>
                                <?php }?>
                                <div class="Binding_list">
                                    <div class="Binding_list_left">
                                        <div class="bingding_yx <?php if ($_smarty_tpl->tpl_vars['company']->value['yyzz_status']==1) {?>bingding_yx_cur<?php }?>"><i class="binding_sf_icon"></i></div>
                                    </div>
                                    <div class="Binding_list_cont">
                                        <div class="Binding_list_name">企业资质</div>
                                        <?php if ($_smarty_tpl->tpl_vars['company']->value['yyzz_status']==1) {?>
                                        <div class="Binding_list_text mt10">当前企业资质已验证 <?php if ($_smarty_tpl->tpl_vars['cert']->value['check']) {?> &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
                                            <a href="javascript:;" data_url="<?php echo $_smarty_tpl->tpl_vars['cert']->value['old_check'];?>
"
                                               data_ourl="<?php echo $_smarty_tpl->tpl_vars['cert']->value['old_owner_cert'];?>
"
                                               data_wurl="<?php echo $_smarty_tpl->tpl_vars['cert']->value['old_wt_cert'];?>
"
                                               data_otherurl="<?php echo $_smarty_tpl->tpl_vars['cert']->value['old_other_cert'];?>
" onclick="showpic(this)"
                                               class="Binding_pop_box_msg_cont_pic_p">查看企业资质</a>
                                            <?php }?>
                                        </div>
                                        <div class="Binding_oper">
                                            <a href="index.php?c=binding&act=comcert" class="Binding_submit_qx">修改资质
</a>
                                        </div>
                                        <?php } else { ?> <?php if (!empty($_smarty_tpl->tpl_vars['cert']->value)) {?> <?php if ($_smarty_tpl->tpl_vars['cert']->value['status']==2) {?>
                                        <div class="Binding_list_text" style="color: #F00">审核未通过 <?php if ($_smarty_tpl->tpl_vars['cert']->value['statusbody']) {?>原因：<?php echo $_smarty_tpl->tpl_vars['cert']->value['statusbody'];?>
 <?php }?>
                                            <?php if ($_smarty_tpl->tpl_vars['cert']->value['check']) {?> &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
                                            <a href="javascript:;" data_url="<?php echo $_smarty_tpl->tpl_vars['cert']->value['old_check'];?>
"
                                               data_ourl="<?php echo $_smarty_tpl->tpl_vars['cert']->value['old_owner_cert'];?>
"
                                               data_wurl="<?php echo $_smarty_tpl->tpl_vars['cert']->value['old_wt_cert'];?>
"
                                               data_otherurl="<?php echo $_smarty_tpl->tpl_vars['cert']->value['old_other_cert'];?>
" onclick="showpic(this)"
                                               class="Binding_pop_box_msg_cont_pic_p">查看企业资质</a>
                                            <?php }?>
                                        </div>
                                        <div class="Binding_oper">
                                            <a href="index.php?c=binding&act=comcert" class="Binding_submit_rz">重新上传</a>
                                        </div>
                                        <?php } else { ?>
                                        <div class="Binding_list_text mt10">企业资质已上传，请等待管理员审核 <?php if ($_smarty_tpl->tpl_vars['cert']->value['check']) {?>
                                            &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
                                            <a href="javascript:;" data_url="<?php echo $_smarty_tpl->tpl_vars['cert']->value['old_check'];?>
"
                                               data_ourl="<?php echo $_smarty_tpl->tpl_vars['cert']->value['old_owner_cert'];?>
"
                                               data_wurl="<?php echo $_smarty_tpl->tpl_vars['cert']->value['old_wt_cert'];?>
"
                                               data_otherurl="<?php echo $_smarty_tpl->tpl_vars['cert']->value['old_other_cert'];?>
" onclick="showpic(this)"
                                               class="Binding_pop_box_msg_cont_pic_p">查看企业资质</a>
                                            <?php }?>
                                        </div>
                                        <div class="Binding_oper">
                                            <a href="index.php?c=binding&act=comcert" class="Binding_submit_rz" style="background:#ff5050">重新上传</a>
                                        </div>
                                        <?php }?> <?php } else { ?>
                                        <span class="Binding_list_no"><i class="Binding_list_no_icon"></i>未认证</span>
                                        <div class="Binding_list_text">当前未上传企业资质</div>
                                        <div class="Binding_oper">
                                            <a href="index.php?c=binding&act=comcert" class="Binding_submit">立即验证</a>
                                        </div>
                                        <?php }?> <?php }?>
                                    </div>
                                </div>
                                <div class="Binding_list">
                                    <div class="Binding_list_left">
                                        <div class="bingding_yx <?php if ($_smarty_tpl->tpl_vars['company']->value['moblie_status']==1) {?>bingding_yx_cur<?php }?>"><i class="binding_sj_icon"></i></div>
                                    </div>
                                    <div class="Binding_list_cont">
                                        <div class="Binding_list_name"> 手机认证</div>
                                        <?php if ($_smarty_tpl->tpl_vars['company']->value['moblie_status']==1) {?>
                                        <div class="Binding_list_text mt10">当前手机已认证： <b class="Binding_list_b"><?php echo $_smarty_tpl->tpl_vars['company']->value['linktel'];?>
</b>
                                        </div>
                                        <div class="Binding_oper">
                                            <a href="javascript:getshow('moblie','更换绑定手机号码');" class="Binding_submit_qx">更换手机</a>
                                        </div>
                                        <?php } else { ?>
                                        <span class="Binding_list_no"><i class="Binding_list_no_icon"></i>未认证</span>
                                        <div class="Binding_list_text">当前手机未认证： <b class="Binding_list_b"><?php echo $_smarty_tpl->tpl_vars['company']->value['linktel'];?>
</b>
                                            <br>验证后，可用于直接登录账户，快速找回登录密码，接收账户变动提醒,接受求职者简历投递信息
                                        </div>
                                        <div class="Binding_oper">
                                            <a href="javascript:getshow('moblie','绑定手机号码');" class="Binding_submit">立即认证</a>
                                        </div>
                                        <?php }?>
                                    </div>
                                </div>
                                <div class="Binding_list">
                                    <div class="Binding_list_left">
                                        <div class="bingding_yx <?php if ($_smarty_tpl->tpl_vars['company']->value['email_status']==1) {?>bingding_yx_cur<?php }?>"><i class="binding_yx_icon"></i></div>
                                    </div>
                                    <div class="Binding_list_name"> 邮箱认证</div>
                                    <div class="Binding_list_cont">

                                        <?php if ($_smarty_tpl->tpl_vars['company']->value['email_status']==1) {?>
                                        <div class="Binding_list_text mt10">当前邮箱已认证：<b class="Binding_list_b"><?php echo $_smarty_tpl->tpl_vars['company']->value['linkmail'];?>
</b>
                                        </div>
                                        <div class="Binding_oper">
                                            <a href="javascript:void(0)"
                                               onClick="layer_del('确定要取消绑定？','index.php?c=binding&act=del&type=email');"
                                               class="Binding_submit_qx">取消认证</a>
                                        </div>
                                        <?php } else { ?>
                                        <span class="Binding_list_no"><i class="Binding_list_no_icon"></i>未认证</span>
                                        <div class="Binding_list_text">当前邮箱未认证：<b class="Binding_list_b"><?php echo $_smarty_tpl->tpl_vars['company']->value['linkmail'];?>
</b>
                                            <br>邮箱验证后，可用于直接登录账户，快速找回登录密码，接收账户变动提醒,接受求职者简历投递信息
                                        </div>
                                        <div class="Binding_oper">
                                            <a href="javascript:getshow('email','绑定邮箱');" class="Binding_submit">立即验证</a>
                                        </div>
                                        <?php }?>
                                    </div>
                                </div>
                                <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_qqlogin']=='1') {?>
                                <div class="Binding_list">
                                    <div class="Binding_list_left">
                                        <div class="bingding_yx <?php if ($_smarty_tpl->tpl_vars['member']->value['qqid']!='') {?>bingding_yx_cur<?php }?>"><i class="binding_qq_icon"></i></div>
                                    </div>
                                    <div class="Binding_list_cont">
                                        <div class="Binding_list_name">绑定QQ</div>
                                        <?php if ($_smarty_tpl->tpl_vars['member']->value['qqid']!='') {?>
                                        <div class="Binding_list_text">已绑定QQ号</div>
                                        <div class="Binding_oper ">
                                            <a href="javascript:void(0)" onClick="layer_del('确定要取消绑定？','index.php?c=binding&act=del&type=qqid');" class="Binding_submit_qx">取消绑定</a>
                                        </div>
                                        <?php } else { ?>
                                        <span class="Binding_list_no"><i class="Binding_list_no_icon"></i>未绑定</span>
                                        <div class="Binding_list_text">未绑定QQ号</div>
                                        <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_qqlogin']!='1') {?>
                                        <div class="Binding_oper">
                                            <a href="javascript:void(0)" onclick="layer.msg('对不起，QQ绑定已关闭！', 2, 8);return false;" class="Binding_submit">立即绑定</a>
                                        </div>
                                        <?php } else { ?>
                                        <div class="Binding_oper">
                                            <a href="<?php echo smarty_function_url(array('m'=>'qqconnect','login'=>1),$_smarty_tpl);?>
" class="Binding_submit">立即绑定</a>
                                        </div>
                                        <?php }?> <?php }?>
                                    </div>
                                </div>
                                <?php }?>
                                <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_sinalogin']=='1') {?>
                                <div class="Binding_list">
                                    <div class="Binding_list_left">
                                        <div class="bingding_yx <?php if ($_smarty_tpl->tpl_vars['member']->value['sinaid']!='') {?>bingding_yx_cur<?php }?>"><i class="binding_xl_icon"></i></div>
                                    </div>
                                    <div class="Binding_list_cont">
                                        <div class="Binding_list_name"> 绑定新浪微博</div>
                                        <?php if ($_smarty_tpl->tpl_vars['member']->value['sinaid']!='') {?>
                                        <div class="Binding_list_text">已绑定，可使用新浪微博快速登录</div>
                                        <div class="Binding_oper">
                                            <a href="javascript:void(0)"
                                               onClick="layer_del('确定要取消绑定？','index.php?c=binding&act=del&type=sinaid');"
                                               class="Binding_submit_qx">取消绑定</a>
                                        </div>
                                        <?php } else { ?>
                                        <span class="Binding_list_no"><i class="Binding_list_no_icon"></i>未绑定</span>
                                        <div class="Binding_list_text">授权绑定后，可使用新浪微博快速登录</div>
                                        <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_sinalogin']!='1') {?>
                                        <div class="Binding_oper">
                                            <a href="javascript:void(0)" onclick="layer.msg('对不起，新浪绑定已关闭！', 2, 8);return false;"
                                               class="Binding_submit">立即绑定</a>
                                        </div>
                                        <?php } else { ?>
                                        <div class="Binding_oper">
                                            <a href="<?php echo smarty_function_url(array('m'=>'sinaconnect','login'=>1),$_smarty_tpl);?>
" class="Binding_submit">立即绑定</a>
                                        </div>
                                        <?php }?> <?php }?>
                                    </div>
                                </div>
                                <?php }?>

                                <?php if ($_smarty_tpl->tpl_vars['config']->value['wx_author']=='1') {?>
                                <div class="Binding_list">
                                    <div class="Binding_list_left">
                                        <div class="bingding_yx <?php if ($_smarty_tpl->tpl_vars['member']->value['unionid']!=''||$_smarty_tpl->tpl_vars['member']->value['wxid']!='') {?>bingding_yx_cur<?php }?>"><i class="binding_wx_icon"></i></div>
                                    </div>
                                    <div class="Binding_list_cont">
                                        <div class="Binding_list_name">绑定微信</div>
                                        <?php if ($_smarty_tpl->tpl_vars['member']->value['unionid']!=''||$_smarty_tpl->tpl_vars['member']->value['wxid']!='') {?>
                                            <div class="Binding_list_text">已绑定，可使用微信扫描登录</div>
                                            <div class="Binding_oper">
                                                <a href="javascript:void(0)" onClick="layer_del('确定要取消绑定？','index.php?c=binding&act=del&type=wxid');" class="Binding_submit_qx">取消绑定</a>
                                            </div>
                                        <?php } else { ?>
                                        <span class="Binding_list_no"><i class="Binding_list_no_icon"></i>未绑定</span>
                                        <div class="Binding_list_text">授权绑定后，可使用微信扫描登录</div>
                                        <?php if ($_smarty_tpl->tpl_vars['config']->value['wx_author']!='1') {?>
                                        <div class="Binding_oper">
                                            <a href="javascript:void(0)" onclick="layer.msg('对不起，微信绑定已关闭！', 2, 8); return false;" class="Binding_submit">立即绑定</a>
                                        </div>
                                        <?php } else { ?>
                                        <div class="Binding_oper"><a href="javascript:void(0)" onclick="wxshow();" class="Binding_submit">立即绑定</a></div>
                                        <?php }?> <?php }?>
                                    </div>
                                </div>
                                <?php }?>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
    <!--绑定邮箱弹出框-->
    <div id="email" style="display:none;">
        <div class="Binding_pop_box" style="padding:10px;width:480px;background:#fff">
            <div>
                <div class="Binding_pop_box_list">
                    <span class="Binding_pop_box_list_left"><i class="Binding_pop_box_list_left_i">*</i>我的邮箱：</span>
                    <input type="text" name="email" value="<?php echo $_smarty_tpl->tpl_vars['company']->value['linkmail'];?>
" class="Binding_pop_box_list_text Binding_pop_box_list_textw200">
                </div>
                <div class="Binding_pop_box_list">
                    <span class="Binding_pop_box_list_left"><i class="Binding_pop_box_list_left_i">*</i>验证码：</span>
                    <input type="text" name="email_code" maxlength="6" class="Binding_pop_box_list_text"/>
                    <img id="vcode_img" src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/app/include/authcode.inc.php" style=" margin:0 5px 5px 5px; vertical-align:middle;">
                    <a href="javascript:void(0);" onclick="checkCode('vcode_img');">看不清</a>
                </div>
                <div class="Binding_pop_box_list">
                    <input class=" Binding_pop_bth" type="button" onclick="sendbemail('vcode_img');" value="发送验证邮件" />
                </div>
                <div class="Binding_pop_tip">没收到邮件？</div>
                <div class="Binding_pop_tip_p">
                    1. 请检查您的垃圾箱或者广告箱，邮件有可能被误认为垃圾或者广告邮件；<br/> 2.验证邮件24小时内有效，请尽快登录您的邮箱点击验证链接完成验证
                </div>
            </div>
        </div>
    </div>
    <!--弹出框 end-->

    <!--绑定手机弹出框-->
    <div id="moblie" style=" display:none;">
        <div class="Binding_pop_box" style="padding:10px;width:480px;background:#fff;">

            <div class="Binding_pop_box_list" style=" display:block">
                <span class="Binding_pop_box_list_left"><i class="Binding_pop_box_list_left_i">*</i>手机号码：</span>
                <input type="text" name="moblie" class="Binding_pop_box_list_text" value="<?php echo $_smarty_tpl->tpl_vars['company']->value['linktel'];?>
">
            </div>
            <div class="Binding_pop_box_list">
                <span class="Binding_pop_box_list_left"><i class="Binding_pop_box_list_left_i">*</i>验证码：</span>
                <input type="text" name="phoneimg_code" maxlength="6" class="Binding_pop_box_list_text"/>
                <img id="pcode_img" src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/app/include/authcode.inc.php" style=" margin:0 5px 5px 5px; vertical-align:middle;">
                <a href="javascript:void(0);" onclick="checkCode('pcode_img');">看不清</a>
            </div>
            <div class="Binding_pop_box_list">
                <span class="Binding_pop_box_list_left"><i class="Binding_pop_box_list_left_i">*</i>手机校验码：</span>
                <div class="Binding_pop_right">
                    <input type="text" id="moblie_code" class="Binding_pop_box_list_text">
                    <a href="javascript:;" onclick="sendmoblie('pcode_img');" class="Binding_pop_box_magbth" id="time">获取短信校验码</a>
                </div>
            </div>
            <div class="Binding_pop_box_list">
                <input class="Binding_pop_bth" onclick="check_moblie();" type="button" value="提交"/>
            </div>
            <div class="Binding_pop_tip">收不到短信验证码?</div>
            <div class="Binding_pop_tip_p">
                1.短信验证码有效时效为<?php echo $_smarty_tpl->tpl_vars['config']->value['moblie_codetime'];?>
分钟，超过<?php echo $_smarty_tpl->tpl_vars['config']->value['moblie_codetime'];?>
分钟请点击重新发送；<br>
                2.如您手机无法收取短信或者收取延迟，请关机重启或者联系运营商处理；<br/>
                3.如以上措施还无法解决，请确认用户名，手机号以及邮箱联系我司客服
            </div>
        </div>
    </div>
    <!--弹出框 end-->

    <?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/binding.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>


    <div id="wxcontent" style=" display:none;">
        <div class="yun_wxbd_box">
            <div class="yun_wxbd_img_c">
                <div id="wx_login_qrcode" class="yun_wxbd_img" style="border:1px solid #eee; line-height:180px;">
                    正在获取二维码...
                </div>
                <div id="wx_sx" class="none">
                    <div class="wx_login_show_sxbox">
                        <a href="javascript:void(0);" onclick="getwxbindcode()" class="wx_login_show_sxicon"></a>二维码已失效点击刷新
                    </div>
                </div>
            </div>
            <div class="yun_wxbd_p"> 请使用微信扫描二维码绑定</div>
        </div>
    </div>
</div>

<input type="hidden" id="linktel" value="<?php echo $_smarty_tpl->tpl_vars['company']->value['linktel'];?>
"/>
<input type="hidden" id="linkmail" value="<?php echo $_smarty_tpl->tpl_vars['company']->value['linkmail'];?>
"/>
<input type="hidden" id="send" value="0"/>

<?php echo '<script'; ?>
>
    $('#uploadimg_btn').on('mouseover', function () {
        var that = this;
        layer.tips('<img src="<?php echo smarty_function_url(array('m'=>'upload','c'=>'qrcode','type'=>1),$_smarty_tpl);?>
" alt="手机扫码拍照上传" />', that);
        return false;
    });

    $('#uploadimg_btn').on('mouseout', function () {
        layer.closeAll('tips');
    });
<?php echo '</script'; ?>
>

<?php echo '<script'; ?>
>
    layui.use(['layer', 'upload'], function () {
        var layer = layui.layer,
            upload = layui.upload,
            $ = layui.$;


    });
    var setval,
        setwout;

    function getwxbindcode() {
        $.post('<?php echo smarty_function_url(array('m'=>'login','c'=>'wxlogin'),$_smarty_tpl);?>
', {t: 1}, function (data) {
            if (data == 0) {
                $('#wx_login_qrcode').html('二维码获取失败..');
            } else {
                $('#wx_login_qrcode').html('<img src="' + data + '" width="180" height="180">');
                setval = setInterval(function () {
                    $.post('<?php echo smarty_function_url(array('m'=>'login','c'=>'getwxloginstatus'),$_smarty_tpl);?>
', {t: 1}, function (data) {
                        var data = eval('(' + data + ')');
                        if (data.url != '' && data.msg != '') {
                            clearInterval(setval);
                            setval = null;
                            layer.msg(data.msg, 2, 9, function () {
                                window.location.href = data.url;
                            });
                        } else if (data.url) {
                            window.location.href = data.url;
                        }
                    });
                }, 2000);
                if (setwout) {
                    clearTimeout(setwout);
                    setwout = null;
                }
                setwout = setTimeout(function () {
                    clearInterval(setval);
                    setval = null;
                    var wx_sx = $("#wx_sx").html();
                    $('#wx_login_qrcode').html(wx_sx);
                }, 300 * 1000);
            }
        });
    }

    function wxshow() {
        $.layer({
            type: 1,
            title: '绑定微信',
            border: [10, 0.3, '#000', true],
            area: ['400px', 'auto'],
            page: {dom: "#wxcontent"},
            close: function () {
                if (setval) {
                    clearInterval(setval);
                    setval = null;
                }
                if (setwout) {
                    clearTimeout(setwout);
                    setwout = null;
                }
            }
        });
        getwxbindcode();
    }

    function showpic(obj) {
        var url = $(obj).attr('data_url');
        var ourl = $(obj).attr('data_ourl');
        var wurl = $(obj).attr('data_wurl');
        var otherurl = $(obj).attr('data_otherurl');
        var picjson = {
            "data": []
        }
        if (url) {
            picjson.data.push({"title": '营业执照', "src": url, "thumb": url});
        }
        if (ourl) {
            picjson.data.push({"title": '经办人身份证', "src": ourl, "thumb": ourl});
        }
        if (wurl) {
            picjson.data.push({"title": '委托书/承诺函', "src": wurl, "thumb": wurl});
        }
        if (otherurl) {
            picjson.data.push({"title": '其他材料', "src": otherurl, "thumb": otherurl});
        }
        layer.photos({
            photos: picjson
            , anim: 5 //0-6的选择，指定弹出图片动画类型，默认随机（请注意，3.0之前的版本用shift参数）
        });
    }
<?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/layui.upload.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" type='text/javascript'><?php echo '</script'; ?>
>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/footer.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>
<?php }} ?>
