<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 18:19:32
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/default/header.htm" */ ?>
<?php /*%%SmartyHeaderCode:8113251369e8a0b405ec74-34223040%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    'dfeb0041b8d5e9835a3545d937f781b07a9bbf13' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/default/header.htm',
      1 => 1700725930,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '8113251369e8a0b405ec74-34223040',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'config' => 0,
    'maplist' => 0,
    'navlist_app' => 0,
    'navlist' => 0,
    'style' => 0,
    'nlist' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8a0b40664d9_49638671',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8a0b40664d9_49638671')) {function content_69e8a0b40664d9_49638671($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><div class="yun_new_top">
  <div class="yun_new_cont">
    <div class="yun_new_left">
      <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_web_site']=='1') {?>
	  <span class="fl">
      <span class="yun_new_left_city" id="substation_city_id"></span>
	  <input type="hidden" id="indexdir" value="<?php echo $_GET['indexdir'];?>
">
	  </span>
      <?php }?>
电话：<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_freewebtel'];?>

</div>
    <div class="yun_new_right" id = "login_head_div">
      <div class="yun_topNav fr">
        <a class="yun_navMore" href="javascript:;">网站导航</a>
        <div class="yun_webMoredown none">
          <div class="yun_top_nav_box">
            <ul class="yun_top_nav_box_l">
				<?php  $_smarty_tpl->tpl_vars['maplist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['maplist']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
global $db,$db_config,$config;$paramer=array("key"=>"“key“","item"=>"“maplist“","nocache"=>"")
;
		include(PLUS_PATH."/navmap.cache.php");$Navlist=array();
		if(is_array($navmap)){
			$ParamerArr = GetSmarty($paramer,$_GET,$_smarty_tpl);
			$paramer = $ParamerArr[arr];
			$Purl =  $ParamerArr[purl];
		}
		//默认调用头部导航
		$Navlist =$navmap[0];
		if(is_array($navmap)){
			foreach($navmap as $k=>$v){
				foreach($Navlist as $key=>$val){
					if($k==$val[id]){
						foreach($v as $kk=>$value){
							if($value[type]=='1'){
								if($config[sy_seo_rewrite]=="1" && $value[furl]!=''){
									$v[$kk][url] = $config[sy_weburl]."/".$value[furl];
								}else{
									$v[$kk][url] = navUrl(array("url"=>$value['url']));
								}
							}
						}
						$Navlist[$key]['list'][]=$v;
					}
				}
			}
			foreach($Navlist as $key=>$value){
				if($value[type]=='1'){
					if($config[sy_seo_rewrite]=="1" && $value[furl]!=''){
						$Navlist[$key][url] = $config[sy_weburl]."/".$value[furl];
					}else{
						$Navlist[$key][url] = navUrl(array("url"=>$value['url']));
					}
				}
			}
		}$Navlist = $Navlist; if (!is_array($Navlist) && !is_object($Navlist)) { settype($Navlist, 'array');}
foreach ($Navlist as $_smarty_tpl->tpl_vars['maplist']->key => $_smarty_tpl->tpl_vars['maplist']->value) {
$_smarty_tpl->tpl_vars['maplist']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['maplist']->key;
?>
				<li><a href="<?php echo $_smarty_tpl->tpl_vars['maplist']->value['url'];?>
" <?php if ($_smarty_tpl->tpl_vars['maplist']->value['eject']) {?> target="_blank"<?php }?>><?php echo $_smarty_tpl->tpl_vars['maplist']->value['name'];?>
 </a> </li> 
				<?php } ?>
			</ul> 
			<ul class="yun_top_nav_box_wx">
			    <?php  $_smarty_tpl->tpl_vars['navlist_app'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['navlist_app']->_loop = false;
global $db,$db_config,$config;include(PLUS_PATH."/menu.cache.php");$Navlist=array();
		if(is_array($menu_name)){
            $paramer=array("item"=>"“navlist_app“","hovclass"=>"“nav_list_hover“","nid"=>"11","nocache"=>"")
;
			$ParamerArr = GetSmarty($paramer,$_GET,$_smarty_tpl);
			$paramer = $ParamerArr[arr];
			$Purl =  $ParamerArr[purl];

			foreach($menu_name[12] as $key=>$val){
				
				if($val['type']=='2'){
					if($config['sy_seo_rewrite']=="1" && $val[furl]!=''){
						$menu_name[12][$key]['url'] = $val[furl];
					}else{
						$menu_name[12][$key]['url'] = $val['url'];
					}
					$menu_name[12][$key][navclass] = navcalss($val,$paramer[hovclass]);
				}
			}
			foreach($menu_name[1] as $key=>$value){
				if($value['url']=="/"){
					$value['url']="";
				}
				if($value['type']=='1'){
					if($config['sy_seo_rewrite']=="1" && $value['furl']!=''){
						$menu_name[1][$key]['url'] = $config[sy_weburl]."/".$value['furl'];
					}else{
						$menu_name[1][$key]['url'] = navUrl(array("url"=>$value['url']));
					}
					$menu_name[1][$key][navclass] = navcalss($value,$paramer[hovclass]);
				}
			}
			foreach($menu_name[2] as $key=>$value){
                if($value['url']=="/"){
					$value['url']="";
				}
				if($value['type']=='1'){
					if($config['sy_seo_rewrite']=="1" && $value['furl']!=''){
						$menu_name[2][$key]['url'] = $config[sy_weburl]."/".$value['furl'];
					}else{
						$menu_name[2][$key]['url'] = navUrl(array("url"=>$value['url']));
					}
					$menu_name[2][$key][navclass] = navcalss($value,$paramer[hovclass]);
				}
			}
            $isCurrentFind=false;
			foreach($menu_name[4] as $key=>$value){
                if($value['url']=="/"){
					$value['url']="";
				}
				if($value['type']=='1' && $value['furl']!=''){
					if($config['sy_seo_rewrite']=="1"){
						$menu_name[4][$key]['url'] = $config[sy_weburl]."/".$value['furl'];
					}else{
						$menu_name[4][$key]['url'] = navUrl(array("url"=>$value['url']));
					}
				}
                if($isCurrentFind==false){
				    $menu_name[4][$key][navclass] = navcalss($value,$paramer[hovclass]);
                }
                if($menu_name[4][$key][navclass]){
                    $isCurrentFind=true;
                }
			}
			foreach($menu_name[5] as $key=>$value){
                if($value['url']=="/"){
					$value['url']="";
				}
				if($value['type']=='1'){
					if($config['sy_seo_rewrite']=="1" && $value['furl']!=''){
						$menu_name[5][$key]['url'] = $config[sy_weburl]."/".$value['furl'];
					}else{
						$menu_name[5][$key]['url'] = navUrl(array("url"=>$value['url']));
					}
					$menu_name[5][$key][navclass] = navcalss($value,$paramer[hovclass]);
				}
			}
		}
		//默认调用头部导航
		if($paramer[nid]){
			$Navlist =$menu_name[$paramer[nid]];
		}else{
			$Navlist =$menu_name[1];
		}$Navlist = $Navlist; if (!is_array($Navlist) && !is_object($Navlist)) { settype($Navlist, 'array');}
foreach ($Navlist as $_smarty_tpl->tpl_vars['navlist_app']->key => $_smarty_tpl->tpl_vars['navlist_app']->value) {
$_smarty_tpl->tpl_vars['navlist_app']->_loop = true;
?>
				<li> 
					<a class="move_0<?php echo $_smarty_tpl->tpl_vars['navlist_app']->value['sort'];?>
" <?php if ($_smarty_tpl->tpl_vars['navlist_app']->value['eject']) {?> target="_blank"<?php }?> href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/<?php echo $_smarty_tpl->tpl_vars['navlist_app']->value['url'];?>
"><?php echo $_smarty_tpl->tpl_vars['navlist_app']->value['name'];?>
</a>
                </li>
               <?php } ?>
            </ul>

          </div>
        </div>
      </div>

      <span class="yun_new_right_we"><?php echo $_smarty_tpl->tpl_vars['config']->value['sy_webname'];?>
欢迎您！</span>
	  
	   
      <a href="<?php echo smarty_function_url(array('m'=>'index','c'=>'wap'),$_smarty_tpl);?>
" class="yun_new_right_wap">手机端</a>
 <span id="login_head_id"></span>
     
    </div>
  </div>
</div>
<!--top end-->
<div class="hp_head hp_head_box">
  <div class="w1200">
    <div class="hp_head_ft fl">
      <div class="phpyun_logo fl"><a href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
"
          title="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_webname'];?>
最新招聘求职信息"><img
            src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_ossurl'];?>
/<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_logo'];?>
" alt="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_webname'];?>
" /></a>
      </div>

    </div>
    <div class="yun_header_nav_box">
      <ul>
        <?php  $_smarty_tpl->tpl_vars['navlist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['navlist']->_loop = false;
global $db,$db_config,$config;include(PLUS_PATH."/menu.cache.php");$Navlist=array();
		if(is_array($menu_name)){
            $paramer=array("item"=>"“navlist“","hovclass"=>"“nav_list_hover“","nid"=>"1","nocache"=>"")
;
			$ParamerArr = GetSmarty($paramer,$_GET,$_smarty_tpl);
			$paramer = $ParamerArr[arr];
			$Purl =  $ParamerArr[purl];

			foreach($menu_name[12] as $key=>$val){
				
				if($val['type']=='2'){
					if($config['sy_seo_rewrite']=="1" && $val[furl]!=''){
						$menu_name[12][$key]['url'] = $val[furl];
					}else{
						$menu_name[12][$key]['url'] = $val['url'];
					}
					$menu_name[12][$key][navclass] = navcalss($val,$paramer[hovclass]);
				}
			}
			foreach($menu_name[1] as $key=>$value){
				if($value['url']=="/"){
					$value['url']="";
				}
				if($value['type']=='1'){
					if($config['sy_seo_rewrite']=="1" && $value['furl']!=''){
						$menu_name[1][$key]['url'] = $config[sy_weburl]."/".$value['furl'];
					}else{
						$menu_name[1][$key]['url'] = navUrl(array("url"=>$value['url']));
					}
					$menu_name[1][$key][navclass] = navcalss($value,$paramer[hovclass]);
				}
			}
			foreach($menu_name[2] as $key=>$value){
                if($value['url']=="/"){
					$value['url']="";
				}
				if($value['type']=='1'){
					if($config['sy_seo_rewrite']=="1" && $value['furl']!=''){
						$menu_name[2][$key]['url'] = $config[sy_weburl]."/".$value['furl'];
					}else{
						$menu_name[2][$key]['url'] = navUrl(array("url"=>$value['url']));
					}
					$menu_name[2][$key][navclass] = navcalss($value,$paramer[hovclass]);
				}
			}
            $isCurrentFind=false;
			foreach($menu_name[4] as $key=>$value){
                if($value['url']=="/"){
					$value['url']="";
				}
				if($value['type']=='1' && $value['furl']!=''){
					if($config['sy_seo_rewrite']=="1"){
						$menu_name[4][$key]['url'] = $config[sy_weburl]."/".$value['furl'];
					}else{
						$menu_name[4][$key]['url'] = navUrl(array("url"=>$value['url']));
					}
				}
                if($isCurrentFind==false){
				    $menu_name[4][$key][navclass] = navcalss($value,$paramer[hovclass]);
                }
                if($menu_name[4][$key][navclass]){
                    $isCurrentFind=true;
                }
			}
			foreach($menu_name[5] as $key=>$value){
                if($value['url']=="/"){
					$value['url']="";
				}
				if($value['type']=='1'){
					if($config['sy_seo_rewrite']=="1" && $value['furl']!=''){
						$menu_name[5][$key]['url'] = $config[sy_weburl]."/".$value['furl'];
					}else{
						$menu_name[5][$key]['url'] = navUrl(array("url"=>$value['url']));
					}
					$menu_name[5][$key][navclass] = navcalss($value,$paramer[hovclass]);
				}
			}
		}
		//默认调用头部导航
		if($paramer[nid]){
			$Navlist =$menu_name[$paramer[nid]];
		}else{
			$Navlist =$menu_name[1];
		}$Navlist = $Navlist; if (!is_array($Navlist) && !is_object($Navlist)) { settype($Navlist, 'array');}
foreach ($Navlist as $_smarty_tpl->tpl_vars['navlist']->key => $_smarty_tpl->tpl_vars['navlist']->value) {
$_smarty_tpl->tpl_vars['navlist']->_loop = true;
?>
        <li class="<?php echo $_smarty_tpl->tpl_vars['navlist']->value['navclass'];?>
"> 
			<a href="<?php echo $_smarty_tpl->tpl_vars['navlist']->value['url'];?>
" <?php if ($_smarty_tpl->tpl_vars['navlist']->value['eject']) {?> target="_blank" <?php }?> class="png"> <?php echo $_smarty_tpl->tpl_vars['navlist']->value['name'];?>
 </a>
			<?php if ($_smarty_tpl->tpl_vars['navlist']->value['model']=="new") {?>
			<div class="nav_icon nav_icon_new"> <img src="<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/images/new.gif"> </div>
			<?php } elseif ($_smarty_tpl->tpl_vars['navlist']->value['model']=="hot") {?>
			<div class="nav_icon nav_icon_hot"> <img src="<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/images/hotn.gif"> </div>
			<?php }?>
			<i class="yun_new_headernav_list_line"></i>
        </li>
        <?php } ?>
      </ul>
    </div>
  </div>
</div>

<?php if ($_smarty_tpl->tpl_vars['config']->value['sy_header_fix']=='1') {?>
<!--滚动展示内容-->
<div class="header_fixed yun_bg_color none" id="header_fix">
  <div class="header_fixed_cont">
    <ul class="header_fixed_list">
      <?php  $_smarty_tpl->tpl_vars['nlist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['nlist']->_loop = false;
global $db,$db_config,$config;include(PLUS_PATH."/menu.cache.php");$Navlist=array();
		if(is_array($menu_name)){
            $paramer=array("item"=>"“nlist“","hovclass"=>"“header_fixed_list_cur“","nid"=>"1","nocache"=>"")
;
			$ParamerArr = GetSmarty($paramer,$_GET,$_smarty_tpl);
			$paramer = $ParamerArr[arr];
			$Purl =  $ParamerArr[purl];

			foreach($menu_name[12] as $key=>$val){
				
				if($val['type']=='2'){
					if($config['sy_seo_rewrite']=="1" && $val[furl]!=''){
						$menu_name[12][$key]['url'] = $val[furl];
					}else{
						$menu_name[12][$key]['url'] = $val['url'];
					}
					$menu_name[12][$key][navclass] = navcalss($val,$paramer[hovclass]);
				}
			}
			foreach($menu_name[1] as $key=>$value){
				if($value['url']=="/"){
					$value['url']="";
				}
				if($value['type']=='1'){
					if($config['sy_seo_rewrite']=="1" && $value['furl']!=''){
						$menu_name[1][$key]['url'] = $config[sy_weburl]."/".$value['furl'];
					}else{
						$menu_name[1][$key]['url'] = navUrl(array("url"=>$value['url']));
					}
					$menu_name[1][$key][navclass] = navcalss($value,$paramer[hovclass]);
				}
			}
			foreach($menu_name[2] as $key=>$value){
                if($value['url']=="/"){
					$value['url']="";
				}
				if($value['type']=='1'){
					if($config['sy_seo_rewrite']=="1" && $value['furl']!=''){
						$menu_name[2][$key]['url'] = $config[sy_weburl]."/".$value['furl'];
					}else{
						$menu_name[2][$key]['url'] = navUrl(array("url"=>$value['url']));
					}
					$menu_name[2][$key][navclass] = navcalss($value,$paramer[hovclass]);
				}
			}
            $isCurrentFind=false;
			foreach($menu_name[4] as $key=>$value){
                if($value['url']=="/"){
					$value['url']="";
				}
				if($value['type']=='1' && $value['furl']!=''){
					if($config['sy_seo_rewrite']=="1"){
						$menu_name[4][$key]['url'] = $config[sy_weburl]."/".$value['furl'];
					}else{
						$menu_name[4][$key]['url'] = navUrl(array("url"=>$value['url']));
					}
				}
                if($isCurrentFind==false){
				    $menu_name[4][$key][navclass] = navcalss($value,$paramer[hovclass]);
                }
                if($menu_name[4][$key][navclass]){
                    $isCurrentFind=true;
                }
			}
			foreach($menu_name[5] as $key=>$value){
                if($value['url']=="/"){
					$value['url']="";
				}
				if($value['type']=='1'){
					if($config['sy_seo_rewrite']=="1" && $value['furl']!=''){
						$menu_name[5][$key]['url'] = $config[sy_weburl]."/".$value['furl'];
					}else{
						$menu_name[5][$key]['url'] = navUrl(array("url"=>$value['url']));
					}
					$menu_name[5][$key][navclass] = navcalss($value,$paramer[hovclass]);
				}
			}
		}
		//默认调用头部导航
		if($paramer[nid]){
			$Navlist =$menu_name[$paramer[nid]];
		}else{
			$Navlist =$menu_name[1];
		}$Navlist = $Navlist; if (!is_array($Navlist) && !is_object($Navlist)) { settype($Navlist, 'array');}
foreach ($Navlist as $_smarty_tpl->tpl_vars['nlist']->key => $_smarty_tpl->tpl_vars['nlist']->value) {
$_smarty_tpl->tpl_vars['nlist']->_loop = true;
?>
        <li class="<?php echo $_smarty_tpl->tpl_vars['nlist']->value['navclass'];?>
">
          <a href="<?php echo $_smarty_tpl->tpl_vars['nlist']->value['url'];?>
" <?php if ($_smarty_tpl->tpl_vars['nlist']->value['eject']) {?> target="_blank" <?php }?>><?php echo $_smarty_tpl->tpl_vars['nlist']->value['name'];?>
 </a> 
        </li> 
      <?php } ?> 
    </ul>
</div>
<div class="header_fixed_close"><a href="javascript:;" onclick="$('#header_fix').remove();" rel="nofollow">关闭</a></div>
</div>
<!--滚动展示内容 end-->
<?php }?><?php }} ?>
