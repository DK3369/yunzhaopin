<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 18:20:02
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/member/com/news.htm" */ ?>
<?php /*%%SmartyHeaderCode:8592410469e8a0d210ba82-50018490%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '0efab244d3e14c228839839c290f3ca4eebfae02' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/member/com/news.htm',
      1 => 1700725932,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '8592410469e8a0d210ba82-50018490',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'config' => 0,
    'now_url' => 0,
    'rows' => 0,
    'v' => 0,
    'uid' => 0,
    'pagenav' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8a0d21266d6_35277057',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8a0d21266d6_35277057')) {function content_69e8a0d21266d6_35277057($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
if (!is_callable('smarty_modifier_date_format')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/modifier.date_format.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<div class="w1000">
	<div class="admin_mainbody"> 
	<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/left.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

		<div class=right_box>
			<div class="newmember_tit">
				<ul>
					<li><a href="index.php?c=info">基本信息</a></li>
					<li><a href="index.php?c=info&act=side">补充信息</a></li>
					<li><a href="index.php?c=address">地址管理</a></li>
					<li><a href="index.php?c=show"> 公司相册</a></li>
					<li><a href="index.php?c=uppic"> 公司LOGO</a></li>
					<li><a href="index.php?c=product">产品介绍</a></li>
					<li class="newmember_titcur"><a href="index.php?c=news">公司资讯</a></li>
					<?php if ($_smarty_tpl->tpl_vars['config']->value['map_key']) {?>
					<li><a href="index.php?c=map">公司地图</a></li>
					<?php }?>
					<li><a href="index.php?c=comtpl">个性化模板</a></li>

				</ul>
			</div>
			<div class="newmember_screenbox">
				<div class="newmember_screen">
					<div class="com_topbth_box">
						<input  class="com_topbth_input"type="button" value="添加资讯" onclick="location.href='index.php?c=news&act=add'" />
					</div>
				</div>
			</div>
			<div class="clear"></div>
			<div class=admincont_box>
		     	<div class="com_body">
							<iframe id="supportiframe" name="supportiframe" onload="returnmessage('supportiframe');" style="display:none"></iframe>
							<form action="<?php echo $_smarty_tpl->tpl_vars['now_url']->value;?>
&act=del" method="post" target="supportiframe" id='myform' class='layui-form'>
								<div id="job_checkbokid">
									<div class="com_tablebox">
										<table class="com_table">
											<?php if ($_smarty_tpl->tpl_vars['rows']->value) {?>
											<tr>
												<th width="20">&nbsp;</th>
												<th>新闻标题</th>
												<th>添加时间</th>
												<th>状态 </th>
												<th width="200">操作</th>
											</tr>
											<?php }?>
											<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['rows']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
											<tr>
												<td><input class="job_news_input" type="checkbox" name="delid[]" value="<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
" lay-skin="primary"></td>
												<td><a href="<?php echo smarty_function_url(array('m'=>'company','c'=>'newsshow','id'=>'`$uid`','nid'=>'`$v.id`'),$_smarty_tpl);?>
" target="_blank"><?php echo $_smarty_tpl->tpl_vars['v']->value['title'];?>
</a></td>
												<td align="center"><?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['v']->value['ctime'],'%Y-%m-%d');?>
 </td>
												<td align="center">
													<?php if ($_smarty_tpl->tpl_vars['v']->value['status']=="0") {?>
													<span class="wate_verify">等待审核</span>
													<?php } elseif ($_smarty_tpl->tpl_vars['v']->value['status']=="1") {?> 
													<span class="y_verify">已审核</span>
													<?php } elseif ($_smarty_tpl->tpl_vars['v']->value['status']=="2") {?> 
													<span class="n_verify">未通过</span>
													<span class="com_show_b_line">|</span>
													<a class="job_news_reason" href="javascript:;" onclick="show_msg('<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
')">原因</a>
													<?php }?>
												</td>
												<td>
													<a href="<?php echo smarty_function_url(array('m'=>'company','c'=>'newsshow','id'=>$_smarty_tpl->tpl_vars['uid']->value,'nid'=>$_smarty_tpl->tpl_vars['v']->value['id']),$_smarty_tpl);?>
" target="_blank" class=" com_bth cblue">预览</a>
													<a href="index.php?c=news&act=edit&id=<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
" class=" com_bth cblue">修改</a> 
													<a href="javascript:void(0)" class=" com_bth cblue" onclick="layer_del('确定要删除该新闻？','index.php?c=news&act=del&id=<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
')">删除</a>
												</td>
											</tr>
											<?php }
if (!$_smarty_tpl->tpl_vars['v']->_loop) {
?>

											<?php if ($_GET['keyword']!='') {?>
											<tr>
												<td colspan="8" class="table_end">
													<div class="msg_no">没有搜索相关资讯。</div>
												</td>
											</tr>
											<?php } else { ?>
											<tr>
												<td colspan="8" class="table_end">
													<div class="com_msg_no">
														<p class="com_msg_no_name">未添加资讯信息</p>
														<p>添加资讯信息有利于宣传企业文化信息</p>
														<a href="index.php?c=news&act=add" class="com_msg_no_bth com_submit">点击添加</a>
													</div>
												</td>
											</tr>
											<?php }?>

											<?php } ?>

											<?php if ($_smarty_tpl->tpl_vars['rows']->value) {?>
											<tr>
												<td colspan="8" class="table_end">
													<div class="com_Release_job_bot"> 
														<span class="com_Release_job_qx">
															<input id='checkAll' type="checkbox" lay-filter="allcom" lay-skin="primary"> 全选
														</span>
														<input class='c_btn_02' type="button" name="subdel" value="批量删除" onclick="return really('delid[]');">
													</div>
													<div class="diggg"><?php echo $_smarty_tpl->tpl_vars['pagenav']->value;?>
</div>
												</td>
											</tr>
											<?php }?>
										</table>
									</div>
								</div>
							</form>
							<div class="clear"></div>
						</div>
				 
			</div>
		</div>
	</div>
</div>
<?php echo '<script'; ?>
>
	layui.use(['form', 'layer', 'laydate'], function() {
		var $ = layui.$,
			form = layui.form,
			layer = layui.layer;
		form.on('checkbox(allcom)', function(data) {
			$("input[name='delid[]']").each(function() {
				this.checked = data.elem.checked;
			});
			form.render('checkbox');
		});
	});

	function show_msg(id) {
		$.post("index.php?c=news&act=show", {
			id: id
		}, function(data) {
			if (data) {
				data = eval('(' + data + ')');
				$("#msgs").html(data.statusbody);
				$.layer({
					type: 1,
					title: '查看原因',
					closeBtn: [0, true],
					border: [10, 0.3, '#000', true],
					area: ['400px', 'auto'],
					page: {
						dom: "#showmsg"
					}
				});
			}
		});
	}
<?php echo '</script'; ?>
>
<div id="showmsg" style="display:none; width: 400px;">
	<div>
		<div id="infobox">
			<div class="admin_Operating_sh" style="padding:10px 20px; min-height: 100px; ">

				审核未通过原因：<span id="msgs"></span>

			</div>
		</div>
	</div>
</div>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/footer.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php }} ?>
