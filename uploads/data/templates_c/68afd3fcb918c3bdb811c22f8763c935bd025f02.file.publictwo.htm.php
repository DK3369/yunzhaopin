<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:34:45
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/publichtm/publictwo.htm" */ ?>
<?php /*%%SmartyHeaderCode:56137833169e89635b6c206-65281195%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '68afd3fcb918c3bdb811c22f8763c935bd025f02' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/publichtm/publictwo.htm',
      1 => 1700725936,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '56137833169e89635b6c206-65281195',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'config_wapdomain' => 0,
    'salaryArr' => 0,
    'v' => 0,
    'comdata' => 0,
    'comclass_name' => 0,
    'config' => 0,
    'com_sex' => 0,
    'key' => 0,
    'uptime' => 0,
    'userdata' => 0,
    'userclass_name' => 0,
    'integrity_name' => 0,
    'k' => 0,
    'user_sex' => 0,
    'cityChoosed' => 0,
    'jobChoosed' => 0,
    'searchUrlObj' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e89635b96dc2_15237123',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e89635b96dc2_15237123')) {function content_69e89635b96dc2_15237123($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><div id="publictwo" class="none">
	<!--地区筛选-->
	<van-popup v-model="areaShow" round position="bottom">
	  <van-cascader
	    v-model="areaValue"
	    title="请选择地区"
	    :options="areaOptions"
	    @close="areaShow = false"
	    @finish="onAreaFinish"
	  />
	</van-popup>
	<!--职能筛选-->
	<van-popup v-model="jobShow" round position="bottom">
	  <van-cascader
	    v-model="jobValue"
	    title="请选择职能"
	    :options="jobOptions"
	    @close="jobShow = false"
	    @finish="onJobFinish"
	  />
	</van-popup>
	<!--职位更多筛选-->
	<van-popup v-model="jobmoreShow" closeable position="bottom" round :style="{height:'12.24rem'}">
		<div class="Gengduoj-eject pubtwo">
			<form action="<?php echo $_smarty_tpl->tpl_vars['config_wapdomain']->value;?>
/index.php" method="get" id="job">
				<input type="hidden" name="c" value="<?php if ($_GET['c']) {
echo $_GET['c'];
} else { ?>job<?php }?>" />
				<?php if ($_GET['a']) {?><input type="hidden" name="a" value="<?php echo $_GET['a'];?>
" /><?php }?>
				
		        <div class="conditional_screening_box">
		        <div class="conditional_screening_pv">	<div class="conditional_screening_alltit">全部筛选</div>
		        <div class="conditional_screening_all">
				<div class="conditional_screening_cont">
				
					<div class="conditional_screening_tit"><span class="conditional_screening_tit_n">薪资范围</span></div>
					<ul class="conditional_screening_list">
						<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['salaryArr']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
						<li <?php if ($_GET['minsalary']==$_smarty_tpl->tpl_vars['v']->value['min']||(!$_GET['minsalary']&&$_smarty_tpl->tpl_vars['v']->value['min']==0)) {?>class="conditional_screening_cur"<?php }?>><a href="<?php echo $_smarty_tpl->tpl_vars['config_wapdomain']->value;?>
/index.php?c=job&minsalary=<?php echo $_smarty_tpl->tpl_vars['v']->value['min'];?>
"><?php echo $_smarty_tpl->tpl_vars['v']->value['name'];?>
</a></li>
						<?php } ?>
					</ul>
				</div>

				<div class="clear"></div>
				<div class="conditional_screening_cont">
				
					<div class="conditional_screening_tit">
						<span class="conditional_screening_tit_n">福利待遇</span>
					</div>
					<ul class="conditional_screening_list">
						<li id="welfare" <?php if ($_GET['welfare']=='') {?>class="conditional_screening_cur"<?php }?>  onclick="check_welfare_li('','welfare')"><a href="javascript:void(0)">全部</a></li>
						<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['comdata']->value['job_welfare']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
						<li id="welfare<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if ($_GET['welfare']==$_smarty_tpl->tpl_vars['v']->value) {?>class="conditional_screening_cur"<?php }?> onclick="check_welfare_li('<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
','welfare')"><a href="javascript:void(0)"><?php echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</a></li>
						<?php } ?>
					</ul>	
				</div>
		        <div class="conditional_screening_cont">
					<div class="conditional_screening_tit"><span class="conditional_screening_tit_n">学历要求</span></div>
					<ul class="conditional_screening_list">
					<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['comdata']->value['job_edu']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
					<li id="edu<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" class="<?php if ($_GET['edu']==$_smarty_tpl->tpl_vars['v']->value||($_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value]=='不限'&&!$_GET['edu'])) {?>conditional_screening_cur<?php }?> eduCtrl" onclick="check_edu_li('<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
','edu')"><a href="javascript:void(0)"><?php echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</a></li>
					<?php } ?>
					</ul>
		        </div>
			    <div class="conditional_screening_cont">
					<div class="conditional_screening_tit"><span class="conditional_screening_tit_n">经验要求</span></div>
					<ul class="conditional_screening_list">
					<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['comdata']->value['job_exp']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
					<li id="exp<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" class="<?php if ($_GET['exp']==$_smarty_tpl->tpl_vars['v']->value||($_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value]=='不限'&&!$_GET['exp'])) {?>conditional_screening_cur<?php }?> expCtrl" onclick="check_exp_li('<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
','exp')"><a href="javascript:void(0)"><?php echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</a></li>
					<?php } ?>
					</ul>
		         </div>
				<?php if ($_smarty_tpl->tpl_vars['config']->value['com_job_sexswitch']) {?> 
				<div class="conditional_screening_cont">
					<div class="conditional_screening_tit"><span class="conditional_screening_tit_n">性别要求</span></div>
					<ul class="conditional_screening_list">
					<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['com_sex']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
					<li id="sex<?php echo $_smarty_tpl->tpl_vars['key']->value;?>
" class="<?php if ($_GET['sex']==$_smarty_tpl->tpl_vars['key']->value||($_smarty_tpl->tpl_vars['v']->value=='不限'&&!$_GET['sex'])) {?>conditional_screening_cur<?php }?> sexCtrl" onclick="check_sex_li('<?php echo $_smarty_tpl->tpl_vars['key']->value;?>
','sex')"><a href="javascript:void(0)"><?php echo $_smarty_tpl->tpl_vars['v']->value;?>
</a></li>
				   <?php } ?>
					</ul>
		        </div>
		        <?php }?>
		        <div class="conditional_screening_cont">
		        <div class="conditional_screening_tit"><span class="conditional_screening_tit_n">更新时间</span></div>
					<ul class="conditional_screening_list">
					<li id="uptime" <?php if ($_GET['uptime']=='') {?>class="conditional_screening_cur"<?php }?> onclick="check_uptime_li('','uptime')"><a href="javascript:void(0)">全部</a></li>
					<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['uptime']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
					<li id="uptime<?php echo $_smarty_tpl->tpl_vars['key']->value;?>
" <?php if ($_GET['uptime']==$_smarty_tpl->tpl_vars['key']->value) {?>class="conditional_screening_cur"<?php }?> onclick="check_uptime_li('<?php echo $_smarty_tpl->tpl_vars['key']->value;?>
','uptime')"><a href="javascript:void(0)"><?php echo $_smarty_tpl->tpl_vars['v']->value;?>
</a></li>
					<?php } ?>
					</ul>
		        </div>
		      
		        <div class="conditional_screening_cont">
					<div class="conditional_screening_tit"><span class="conditional_screening_tit_n">公司性质</span></div>
					<ul class="conditional_screening_list">
					<li  id="pr" <?php if ($_GET['pr']=='') {?>class="conditional_screening_cur"<?php }?> onclick="check_pr_li('','pr')"><a href="javascript:void(0)">全部</a></li>
					<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['comdata']->value['job_pr']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
					<li id="pr<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if ($_GET['pr']==$_smarty_tpl->tpl_vars['v']->value) {?>class="conditional_screening_cur"<?php }?> onclick="check_pr_li('<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
','pr')"><a href="javascript:void(0)"><?php echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</a></li>
					<?php } ?>
					</ul>
		        </div>
		        <div class="conditional_screening_cont">
					<div class="conditional_screening_tit"><span class="conditional_screening_tit_n">公司规模</span></div>
					<ul class="conditional_screening_list">
					<li id="mun" <?php if ($_GET['mun']=='') {?>class="conditional_screening_cur"<?php }?> onclick="check_mun_li('','mun')"><a href="javascript:void(0)">全部</a></li>
					<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['comdata']->value['job_mun']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
					<li id="mun<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if ($_GET['mun']==$_smarty_tpl->tpl_vars['v']->value) {?>class="conditional_screening_cur"<?php }?> onclick="check_mun_li('<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
','mun')"><a href="javascript:void(0)"><?php echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</a></li>
					<?php } ?>
					</ul>
		        </div> 
		        </div>
		         <div class="conditional_screening_operation">
		          <div class="conditional_screening_operation_left"><a href="<?php if ($_GET['a']) {
echo smarty_function_url(array('m'=>'wap','c'=>$_GET['c'],'a'=>$_GET['a']),$_smarty_tpl);
} else {
echo smarty_function_url(array('m'=>'wap','c'=>$_GET['c']),$_smarty_tpl);
}?>" class="conditional_screening_operation_cz">重置</a></div>
		          <div class="conditional_screening_operation_right"><a href="javascript:void(0)" onclick="$('#job').submit();" class="conditional_screening_operation_ok">确定</a></div></div>
		        </div>
		        <?php if ($_GET['provinceid']) {?><input type="hidden" name="provinceid" value="<?php echo $_GET['provinceid'];?>
"/><?php }?>
		        <?php if ($_GET['cityid']) {?><input type="hidden" name="cityid" value="<?php echo $_GET['cityid'];?>
"/><?php }?>
		        <?php if ($_GET['three_cityid']) {?><input type="hidden" name="cityid" value="<?php echo $_GET['three_cityid'];?>
"/><?php }?>
		        <?php if ($_GET['jobin']) {?><input type="hidden" name="jobin" value="<?php echo $_GET['jobin'];?>
"/><?php }?>
		      
		        <?php if ($_GET['hy']) {?><input type="hidden" name="hy" id="gdjhyi" value="<?php echo $_GET['hy'];?>
" /><?php }?>
		        <input type="hidden" id="comexp" name="exp" <?php if ($_GET['exp']=='') {?>disabled="disabled"<?php }?>value="<?php echo $_GET['exp'];?>
"/>
		        <input type="hidden" name="pr" id="compr" <?php if ($_GET['pr']=='') {?>disabled="disabled"<?php }?> value="<?php echo $_GET['pr'];?>
" />
		        <input type="hidden" name="mun" id="commun" <?php if ($_GET['mun']=='') {?>disabled="disabled"<?php }?>  value="<?php echo $_GET['mun'];?>
" />
		        <input  type="hidden" id="comedu" name="edu" <?php if ($_GET['edu']=='') {?>disabled="disabled"<?php }?> value="<?php echo $_GET['edu'];?>
" />
		        <input type="hidden" name="uptime" id="comuptime" <?php if ($_GET['uptime']=='') {?>disabled="disabled"<?php }?> value="<?php echo $_GET['uptime'];?>
" />
		        <input type="hidden" name="sex" id="comsex" <?php if ($_GET['sex']=='') {?>disabled="disabled"<?php }?> value="<?php echo $_GET['sex'];?>
" />
		        <input type="hidden" name="welfare" id="comwelfare" <?php if ($_GET['welfare']=='') {?>disabled="disabled"<?php }?> value="<?php echo $_GET['welfare'];?>
" />
				</div>
		       
			</form>
			
		</div>
	</van-popup>
	<!--简历更多筛选-->
	<van-popup v-model="resumemoreShow" closeable position="bottom" round :style="{height:'12.24rem'}">
		<div class="Gengduos-eject pubtwo">
			<form action="<?php echo $_smarty_tpl->tpl_vars['config_wapdomain']->value;?>
/index.php" method="get" id="resume">
				<input type="hidden" name="c" value="<?php if ($_GET['c']) {
echo $_GET['c'];
} else { ?>resume<?php }?>" />
				<div class="conditional_screening_box">
					<div class="conditional_screening_pv">
						  <div class="conditional_screening_pv">	<div class="conditional_screening_alltit">全部筛选</div>
						<div class="conditional_screening_all">
						<div class="conditional_screening_cont">
								<div class="conditional_screening_tit"><span class="conditional_screening_tit_n">经验要求</span></div>
								<ul class="conditional_screening_list" id="Sortexp-Sortexp">
									<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['userdata']->value['user_word']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
									<li id="uexp<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if ($_GET['exp']==$_smarty_tpl->tpl_vars['v']->value) {?>class="conditional_screening_cur"<?php }?> onclick="check_userexp_li('<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
','exp')"><a href="javascript:void(0)"><?php echo $_smarty_tpl->tpl_vars['userclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</a>
									</li>
									<?php } ?>
								</ul>
						</div>
						
							<div class="conditional_screening_cont">
								<div class="conditional_screening_tit"><span class="conditional_screening_tit_n">简历完整度</span></div>
								<ul class="conditional_screening_list">
								<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['k'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['integrity_name']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['k']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
								<li id="integrity<?php echo $_smarty_tpl->tpl_vars['k']->value;?>
" <?php if ($_GET['integrity']==$_smarty_tpl->tpl_vars['k']->value) {?>class="conditional_screening_cur"<?php }?> onclick="check_integrity_li('<?php echo $_smarty_tpl->tpl_vars['k']->value;?>
','integrity')"><a href="javascript:void(0)"><?php echo $_smarty_tpl->tpl_vars['v']->value;?>
</a></li>
							   <?php } ?>
								</ul>
							</div>
							<div class="conditional_screening_cont">
								<div class="conditional_screening_tit"><span class="conditional_screening_tit_n">性别要求</span></div>
								<ul class="conditional_screening_list">
								<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['user_sex']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
								<li id="sex<?php echo $_smarty_tpl->tpl_vars['key']->value;?>
" class="<?php if ($_GET['sex']==$_smarty_tpl->tpl_vars['key']->value||($_smarty_tpl->tpl_vars['v']->value=='不限'&&!$_GET['sex'])) {?>conditional_screening_cur<?php }?> sexCtrl" onclick="check_usex_li('<?php echo $_smarty_tpl->tpl_vars['key']->value;?>
','sex')"><a href="javascript:void(0)"><?php echo $_smarty_tpl->tpl_vars['v']->value;?>
</a></li>
							   <?php } ?>
								</ul>
					        </div>
							<div class="conditional_screening_cont">
								<div class="conditional_screening_tit"><span class="conditional_screening_tit_n">学历要求</span></div>
								<ul class="conditional_screening_list">
								<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['userdata']->value['user_edu']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
								<li id="uedu<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if ($_GET['edu']==$_smarty_tpl->tpl_vars['v']->value) {?>class="conditional_screening_cur"<?php }?> onclick="check_uedu_li('<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
','edu')"><a href="javascript:void(0)"><?php echo $_smarty_tpl->tpl_vars['userclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</a></li>
								<?php } ?>
								</ul>
							</div>
							<div class="conditional_screening_cont">
								<div class="conditional_screening_tit"><span class="conditional_screening_tit_n">到岗时间</span></div>
								<ul class="conditional_screening_list">
								<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['userdata']->value['user_report']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
								<li id="report<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if ($_GET['report']==$_smarty_tpl->tpl_vars['v']->value) {?>class="conditional_screening_cur"<?php }?> onclick="check_report_li('<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
','report')"><a href="javascript:void(0)"><?php echo $_smarty_tpl->tpl_vars['userclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</a></li>
							   <?php } ?>
								</ul>
							</div>
							<div class="conditional_screening_cont">
								<div class="conditional_screening_tit"><span class="conditional_screening_tit_n">更新时间</span></div>
								<ul class="conditional_screening_list">
								<li id="uuptime" <?php if ($_GET['uptime']=='') {?>class="conditional_screening_cur"<?php }?> onclick="check_useruptime_li('','uptime')"><a href="javascript:void(0)">全部</a></li>
								<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['uptime']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
								<li id="uuptime<?php echo $_smarty_tpl->tpl_vars['key']->value;?>
" <?php if ($_GET['uptime']==$_smarty_tpl->tpl_vars['key']->value) {?>class="conditional_screening_cur"<?php }?> onclick="check_useruptime_li('<?php echo $_smarty_tpl->tpl_vars['key']->value;?>
','uptime')"><a href="javascript:void(0)"><?php echo $_smarty_tpl->tpl_vars['v']->value;?>
</a></li>
								<?php } ?>
								</ul>
							</div>
							<div class="conditional_screening_cont">
								<div class="conditional_screening_tit"><span class="conditional_screening_tit_n">个人标签</span></div>
								<ul class="conditional_screening_list">
								<li id="tag" <?php if ($_GET['tag']=='') {?>class="conditional_screening_cur"<?php }?> onclick="check_tag_li('','tag')"><a href="javascript:void(0)">全部</a></li>
								<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['userdata']->value['user_tag']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
								<li id="tag<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if ($_GET['tag']==$_smarty_tpl->tpl_vars['v']->value) {?>class="conditional_screening_cur"<?php }?> onclick="check_tag_li('<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
','tag')"><a href="javascript:void(0)"><?php echo $_smarty_tpl->tpl_vars['userclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</a></li>
								<?php } ?>
								</ul>
							</div>

							<div class="conditional_screening_cont">
								<div class="conditional_screening_tit"><span class="conditional_screening_tit_n">工作性质</span></div>
								<ul class="conditional_screening_list">
								<li id="rtype" <?php if ($_GET['type']=='') {?>class="conditional_screening_cur"<?php }?> onclick="check_rtype_li('','rtype')"><a href="javascript:void(0)">全部</a></li>
								<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['userdata']->value['user_type']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
								<li id="rtype<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if ($_GET['type']==$_smarty_tpl->tpl_vars['v']->value) {?>class="conditional_screening_cur"<?php }?> onclick="check_rtype_li('<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
','rtype')"><a href="javascript:void(0)"><?php echo $_smarty_tpl->tpl_vars['userclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</a></li>
								<?php } ?>
								</ul>
							</div>
		                    
						</div>
						<div class="conditional_screening_operation">
						<div class="conditional_screening_operation_left"><a href="<?php if ($_GET['a']) {
echo smarty_function_url(array('m'=>'wap','c'=>$_GET['c'],'a'=>$_GET['a']),$_smarty_tpl);
} else {
echo smarty_function_url(array('m'=>'wap','c'=>$_GET['c']),$_smarty_tpl);
}?>" class="conditional_screening_operation_cz">重置</a></div>
						<div class="conditional_screening_operation_right"><a href="javascript:void(0)" onclick="$('#resume').submit();" class="conditional_screening_operation_ok">确定</a></div>
						</div>
					</div>
					<?php if ($_GET['provinceid']) {?><input type="hidden" name="provinceid" value="<?php echo $_GET['provinceid'];?>
"/><?php }?>
					<?php if ($_GET['cityid']) {?><input type="hidden" name="cityid" value="<?php echo $_GET['cityid'];?>
"/><?php }?>
					<?php if ($_GET['three_cityid']) {?><input type="hidden" name="cityid" value="<?php echo $_GET['three_cityid'];?>
"/><?php }?>
					<?php if ($_GET['jobin']) {?><input type="hidden" name="jobin" value="<?php echo $_GET['jobin'];?>
"/><?php }?>
					<input type="hidden" name="hy" id="userhy" <?php if ($_GET['hy']=='') {?>disabled="disabled"<?php }?> value="<?php echo $_GET['hy'];?>
" />
					<input type="hidden" id="useredu" name="edu" <?php if ($_GET['edu']=='') {?>disabled="disabled"<?php }?> value="<?php echo $_GET['edu'];?>
" />
					<input type="hidden" name="uptime" id="useruptime" <?php if ($_GET['uptime']=='') {?>disabled="disabled"<?php }?> value="<?php echo $_GET['uptime'];?>
" />
					<input type="hidden" name="report" id="userreport" <?php if ($_GET['report']=='') {?>disabled="disabled"<?php }?> value="<?php echo $_GET['report'];?>
" />
					<input type="hidden" name="exp" id="userexp" <?php if ($_GET['exp']=='') {?>disabled="disabled"<?php }?> value="<?php echo $_GET['exp'];?>
" />
					<input type="hidden" name="integrity" id="userintegrity" <?php if ($_GET['integrity']=='') {?>disabled="disabled"<?php }?> value="<?php echo $_GET['integrity'];?>
" />
					<input type="hidden" name="sex" id="usersex" <?php if ($_GET['sex']=='') {?>disabled="disabled"<?php }?> value="<?php echo $_GET['sex'];?>
" />
					<input type="hidden" name="tag" id="usertag" <?php if ($_GET['tag']=='') {?>disabled="disabled"<?php }?> value="<?php echo $_GET['tag'];?>
" />
					<input type="hidden" name="type" id="userrtype" <?php if ($_GET['type']=='') {?>disabled="disabled"<?php }?> value="<?php echo $_GET['type'];?>
" />
				</div>
				</div>
			</form>
		</div>
	</van-popup>
	<!--行业选择器-->
	<van-popup v-model="hyShow" round position="bottom">
	  <van-picker
	    show-toolbar
	    :columns="hyData"
	    :default-index="hyChoosed"
	    @cancel="hyShow = false"
	    @confirm="hyConfirm"
	  />
	</van-popup>
	<!--企业性质选择器-->
	<van-popup v-model="prShow" round position="bottom">
	  <van-picker
	    show-toolbar
	    :columns="prData"
	    :default-index="prChoosed"
	    @cancel="prShow = false"
	    @confirm="prConfirm"
	  />
	</van-popup>
	<!--企业规模选择器-->
	<van-popup v-model="munShow" round position="bottom">
	  <van-picker
	    show-toolbar
	    :columns="munData"
	    :default-index="munChoosed"
	    @cancel="munShow = false"
	    @confirm="munConfirm"
	  />
	</van-popup>
	<!--兼职类型选择器-->
	<van-popup v-model="partTypeShow" round position="bottom">
		<van-picker
				show-toolbar
				:columns="partTypeData"
				:default-index="partTypeChoosed"
				@cancel="partTypeShow = false"
				@confirm="partTypeConfirm"
		/>
	</van-popup>
	<!--兼职-结算周期选择器-->
	<van-popup v-model="partBillingCycleShow" round position="bottom">
		<van-picker
				show-toolbar
				:columns="partBillingCycleData"
				:default-index="partBillingCycleChoosed"
				@cancel="partBillingCycleShow = false"
				@confirm="partBillingCycleConfirm"
		/>
	</van-popup>
</div>
<?php echo '<script'; ?>
 type="text/javascript">
var cityChoosed =	parseInt('<?php echo $_smarty_tpl->tpl_vars['cityChoosed']->value;?>
');
// 开启分站并选择分站
<?php if (!$_smarty_tpl->tpl_vars['cityChoosed']->value&&$_smarty_tpl->tpl_vars['config']->value['sy_web_site']==1&&($_smarty_tpl->tpl_vars['config']->value['province']||$_smarty_tpl->tpl_vars['config']->value['cityid']||$_smarty_tpl->tpl_vars['config']->value['three_cityid'])) {?>
	<?php if ($_smarty_tpl->tpl_vars['config']->value['three_cityid']) {?>
		cityChoosed = parseInt('<?php echo $_smarty_tpl->tpl_vars['config']->value["three_cityid"];?>
');
	<?php } elseif ($_smarty_tpl->tpl_vars['config']->value['cityid']) {?>
		cityChoosed = parseInt('<?php echo $_smarty_tpl->tpl_vars['config']->value["cityid"];?>
');
	<?php } elseif ($_smarty_tpl->tpl_vars['config']->value['province']) {?>
		cityChoosed = parseInt('<?php echo $_smarty_tpl->tpl_vars['config']->value["province"];?>
');
	<?php }?>
<?php }?>

var jobChoosed	=	parseInt('<?php echo $_smarty_tpl->tpl_vars['jobChoosed']->value;?>
');

var searchUrlObj = {};
'<?php if ($_smarty_tpl->tpl_vars['searchUrlObj']->value) {?>'
searchUrlObj = JSON.parse('<?php echo $_smarty_tpl->tpl_vars['searchUrlObj']->value;?>
');
'<?php }?>'

//行业数据
var hy	=	parseInt('<?php echo $_GET['hy'];?>
');
var hyChoosed = 0;
var hyData = [];

//企业性质数据
var pr	=	parseInt('<?php echo $_GET['pr'];?>
');
var prChoosed = 0;
var prData = [];
//企业规模数据
var mun	=	parseInt('<?php echo $_GET['mun'];?>
');
var munChoosed = 0;
var munData = [];
// 兼职类型
var partType = parseInt('<?php echo $_GET['type'];?>
');
var partTypeChoosed = 0;
var partTypeData = [];
// 兼职-结算周期
var partBillingCycle = parseInt('<?php echo $_GET['cycle'];?>
');
var partBillingCycleChoosed = 0;
var partBillingCycleData = [];


var publictwo_vue =  new Vue({
		el: '#publictwo',
		data: {
			//城市筛选参数
	        areaValue:cityChoosed,
		    areaOptions:cityData,
		    areaShow:false,
		    //职能筛选参数
	        jobValue:jobChoosed,
		    jobOptions:jobData,
		    jobShow:false,
		    //职位更多筛选
		    jobmoreShow:false,
		    //简历更多筛选
		    resumemoreShow:false,
		    //行业
		    hyShow:false,
		    hyData:hyData,
		    hyChoosed:hyChoosed,
		    //企业性质
		    prShow:false,
		    prData:prData,
		    prChoosed:prChoosed,
		    //企业规模
		    munShow:false,
		    munData:munData,
		    munChoosed:munChoosed,
			//兼职类型
			partTypeShow: false,
			partTypeData: partTypeData,
			partTypeChoosed: partTypeData,
			//兼职-结算周期
			partBillingCycleShow: false,
			partBillingCycleData: partBillingCycleData,
			partBillingCycleChoosed: partBillingCycleData,
	    },
	    methods:{
	    	//筛选-地区
	    	onAreaFinish({selectedOptions}) {

		      	searchUrlObj.provinceid = searchUrlObj.cityid = searchUrlObj.threecityid = '';

		      	if(selectedOptions[0] && selectedOptions[0].value>0){
		      		searchUrlObj.provinceid = selectedOptions[0].value;
		      	}
		      	if(selectedOptions[1] && selectedOptions[1].value>0){
		      		searchUrlObj.cityid = selectedOptions[1].value;
		      	}
		      	if(selectedOptions[2] && selectedOptions[2].value>0){
		      		searchUrlObj.threecityid = selectedOptions[2].value;
		      	}
		      	this.searchUrl();
		    },
		    //筛选-职能
	    	onJobFinish({selectedOptions}) {
		      	
		      	searchUrlObj.job1 = searchUrlObj.job1son = searchUrlObj.jobpost = '';

		      	if(selectedOptions[0] && selectedOptions[0].value>0){
		      		searchUrlObj.job1 = selectedOptions[0].value;
		      	}
		      	if(selectedOptions[1] && selectedOptions[1].value>0){
		      		searchUrlObj.job1son = selectedOptions[1].value;
		      	}
		      	if(selectedOptions[2] && selectedOptions[2].value>0){
		      		searchUrlObj.jobpost = selectedOptions[2].value;
		      	}
		      	this.searchUrl();
		    },
		    hyConfirm:function(value,index){
		    	var that = this;
		    	searchUrlObj.hy = value.val;
		    	
		    	this.searchUrl();
		    },
		    prConfirm:function(value,index){
		    	var that = this;
		    	searchUrlObj.pr = value.val;
		    	
		    	this.searchUrl();
		    },
		    munConfirm:function(value,index){
		    	var that = this;
		    	searchUrlObj.mun = value.val;
		    	
		    	this.searchUrl();
		    },
			partTypeConfirm: function(value, index){
				var that = this;
				searchUrlObj.type = value.val;

				this.searchUrl();
			},
			partBillingCycleConfirm: function(value, index){
				var that = this;
				searchUrlObj.cycle = value.val;

				this.searchUrl();
			},
		    searchUrl(){
		    	var url = wapurl+'?';

		    	var url_arr = [];
				for(let i in searchUrlObj){
					if(i!='m'){
						url_arr.push(i+'='+searchUrlObj[i]);
					}
				}
				url += url_arr.join('&');
				location.href = url;
			}
		}
	});
function areashow(){
	$('#publictwo').removeClass('none');
	publictwo_vue.$data.areaShow = true;
}
function jobshow(){
	$('#publictwo').removeClass('none');
	publictwo_vue.$data.jobShow = true;
}
function jobmoreShow(){
	$('#publictwo').removeClass('none');
	publictwo_vue.$data.jobmoreShow = true;
}
function resumemoreShow(){
	$('#publictwo').removeClass('none');
	publictwo_vue.$data.resumemoreShow = true;
}
function hyshow(){
	//行业数据
	if(typeof hi !='undefined' && hi.length>0){
		for(let i in hi){
			hyData.push({text:hyname[hi[i]],val:hi[i]});
			if(hy == hi[i]){
				hyChoosed = i;
			}
		}
	}
	publictwo_vue.$data.hyData = hyData;
	publictwo_vue.$data.hyChoosed = hyChoosed;
	$('#publictwo').removeClass('none');
	publictwo_vue.$data.hyShow = true;
}
function prshow(){
	//企业性质数据
	if(typeof comd!='undefined' && comd['job_pr'].length>0){
		for(let i in comd['job_pr']){
			prData.push({text:comn[comd['job_pr'][i]],val:comd['job_pr'][i]});
			if(pr == comd['job_pr'][i]){
				prChoosed = i;
			}
		}
	}
	publictwo_vue.$data.prData = prData;
	publictwo_vue.$data.prChoosed = prChoosed;
	$('#publictwo').removeClass('none');
	publictwo_vue.$data.prShow = true;
}
function munshow(){
	//企业规模数据
	if(typeof comd!='undefined' && comd['job_mun'].length>0){
		for(let i in comd['job_mun']){
			munData.push({text:comn[comd['job_mun'][i]],val:comd['job_mun'][i]});
			if(mun == comd['job_mun'][i]){
				munChoosed = i;
			}
		}
	}
	publictwo_vue.$data.munData = munData;
	publictwo_vue.$data.munChoosed = munChoosed;
	$('#publictwo').removeClass('none');
	publictwo_vue.$data.munShow = true;
}
function partTypeShow(){
	//兼职类型数据
	if(typeof partd!='undefined' && partd['part_type'].length>0){
		for(let i in partd['part_type']){
			partTypeData.push({text:partn[partd['part_type'][i]],val:partd['part_type'][i]});
			if(partType == partd['part_type'][i]){
				partTypeChoosed = i;
			}
		}
	}
	publictwo_vue.$data.partTypeData = partTypeData;
	publictwo_vue.$data.partTypeChoosed = partTypeChoosed;
	$('#publictwo').removeClass('none');
	publictwo_vue.$data.partTypeShow = true;
}
function partBillingCycleShow(){
	//兼职类型数据
	if(typeof partd!='undefined' && partd['part_billing_cycle'].length>0){
		for(let i in partd['part_billing_cycle']){
			partBillingCycleData.push({text:partn[partd['part_billing_cycle'][i]],val:partd['part_billing_cycle'][i]});
			if(partBillingCycle == partd['part_billing_cycle'][i]){
				partBillingCycleChoosed = i;
			}
		}
	}
	publictwo_vue.$data.partBillingCycleData = partBillingCycleData;
	publictwo_vue.$data.partBillingCycleChoosed = partBillingCycleChoosed;
	$('#publictwo').removeClass('none');
	publictwo_vue.$data.partBillingCycleShow = true;
}
$(document).ready(function() {
	$("#job").submit(function(e) {
		var min = $("#minjob").val();
		var max = $("#maxjob").val();
		if(min && max && parseInt(max) < parseInt(min)) {
			$("#minjob").val(max);
			$("#maxjob").val(min);
		}
	});

	
});
function check_edu_li(id,type){
	//var comedu=$("#comedu").val();
	$('.eduCtrl').removeClass('conditional_screening_cur');
	$('#edu'+id).addClass('conditional_screening_cur');
	$('#comedu').removeAttr('disabled');
	$('#comedu').val(id);
}
function check_welfare_li(id,type){
	var comwelfare=$("#comwelfare").val();
	$('#welfare'+comwelfare).attr('class','');
	$('#welfare'+id).attr('class','conditional_screening_cur');
	$('#comwelfare').removeAttr('disabled');
	$('#comwelfare').val(id);
}
function check_exp_li(id,type){
	//var comexp = $("#comexp").val(); 
	$('.expCtrl').removeClass('conditional_screening_cur');
	$('#exp'+id).addClass('conditional_screening_cur');
	$('#comexp').removeAttr('disabled');
	$('#comexp').val(id);

}
function check_sex_li(id,type){
	//var comsex=$("#comsex").val();
	$('.sexCtrl').removeClass('conditional_screening_cur');
	$('#sex'+id).addClass('conditional_screening_cur');
	$('#comsex').removeAttr('disabled');
	$('#comsex').val(id); 
}
function check_uptime_li(id,type){
	var comuptime=$("#comuptime").val();
	$('#uptime'+comuptime).attr('class','');
	$('#uptime'+id).attr('class','conditional_screening_cur');
	if(id){
	$('#comuptime').removeAttr('disabled');
	}else{
	$('#comuptime').attr("disabled","disabled");
	}

	$('#comuptime').val(id);
}
function check_pr_li(id,type){
	var compr=$("#compr").val();
	$('#pr'+compr).attr('class','');
	$('#pr'+id).attr('class','conditional_screening_cur');
	if(id){
	$('#compr').removeAttr('disabled');
	}else{
	$('#compr').attr("disabled","disabled");
	}
	$('#compr').val(id);

}
function check_mun_li(id,type){
	var commun=$("#commun").val();
	$('#mun'+commun).attr('class','');
	$('#mun'+id).attr('class','conditional_screening_cur');
	if(id){
	$('#commun').removeAttr('disabled');
	}else{
	$('#commun').attr("disabled","disabled");
	}
	$('#commun').val(id);  

}
function check_userexp_li(id,type){
	var userexp=$("#userexp").val();
	$('#uexp'+userexp).attr('class','');
	$('#uexp'+id).attr('class','conditional_screening_cur');
	$('#userexp').removeAttr('disabled');
	$('#userexp').val(id);
}
function check_integrity_li(id,type){
	var userintegrity=$("#userintegrity").val();
	$('#integrity'+userintegrity).attr('class','');
	$('#integrity'+id).attr('class','conditional_screening_cur');
	$('#userintegrity').removeAttr('disabled');
	$('#userintegrity').val(id);
}
function check_usex_li(id,type){
	$('.sexCtrl').removeClass('conditional_screening_cur');
	$('#sex'+id).addClass('conditional_screening_cur');
	$('#usersex').removeAttr('disabled');
	$('#usersex').val(id);  
}
function check_uedu_li(id,type){
	var useredu=$("#useredu").val();
	$('#uedu'+useredu).attr('class','');
	$('#uedu'+id).attr('class','conditional_screening_cur');
	$('#useredu').removeAttr('disabled');
	$('#useredu').val(id);
}
function check_report_li(id,type){
	var userreport=$("#userreport").val();
	$('#report'+userreport).attr('class','');
	$('#report'+id).attr('class','conditional_screening_cur');
	$('#userreport').removeAttr('disabled');
	$('#userreport').val(id);
}
function check_useruptime_li(id,type){
	var useruptime=$("#useruptime").val();
	$('#uuptime'+useruptime).attr('class','');
	$('#uuptime'+id).attr('class','conditional_screening_cur');
	$('#useruptime').removeAttr('disabled');
	$('#useruptime').val(id);
}
function check_tag_li(id,type){
	var usertag=$("#usertag").val();
	$('#tag'+usertag).attr('class','');
	$('#tag'+id).attr('class','conditional_screening_cur');
	if(id){
		$('#usertag').removeAttr('disabled');
	}else{
		$('#usertag').attr("disabled","disabled");
	}
	$('#usertag').val(id);
}
function check_rtype_li(id,type){

	var userrtype=$("#userrtype").val();
	$('#rtype'+userrtype).attr('class','');
	$('#rtype'+id).attr('class','conditional_screening_cur');
	if(id){
		$('#userrtype').removeAttr('disabled');
	}else{
		$('#userrtype').attr("disabled","disabled");
	}
	$('#userrtype').val(id);
}
<?php echo '</script'; ?>
><?php }} ?>
