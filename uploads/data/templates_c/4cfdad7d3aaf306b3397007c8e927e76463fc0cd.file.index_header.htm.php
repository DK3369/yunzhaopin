<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 13:00:14
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/default/index_header.htm" */ ?>
<?php /*%%SmartyHeaderCode:89460396569e855de098c20-42698513%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '4cfdad7d3aaf306b3397007c8e927e76463fc0cd' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/default/index_header.htm',
      1 => 1706496289,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '89460396569e855de098c20-42698513',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'config' => 0,
    'maplist' => 0,
    'navlist_app' => 0,
    'keylist' => 0,
    'usertype' => 0,
    'navlist' => 0,
    'style' => 0,
    'nlist' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e855de0a8853_20919811',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e855de0a8853_20919811')) {function content_69e855de0a8853_20919811($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
if (!is_callable('smarty_function_listurl')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.listurl.php';
?><?php echo '<script'; ?>
>
var weburl="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
",
	user_sqintegrity="<?php echo $_smarty_tpl->tpl_vars['config']->value['user_sqintegrity'];?>
",
	integral_pricename='<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
',
	pricename='<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
',
	code_web='<?php echo $_smarty_tpl->tpl_vars['config']->value['code_web'];?>
',
	code_kind='<?php echo $_smarty_tpl->tpl_vars['config']->value['code_kind'];?>
';
<?php echo '</script'; ?>
>
<div class="yun_new_top">
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
<!--header 6.0-->
<div class="yunheader_60"> 
   <div class="w1200">
 <div class="yunheader_60logo fl"><a href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
"
          title="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_webname'];?>
最新招聘求职信息"><img
            src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_ossurl'];?>
/<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_logo'];?>
" alt="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_webname'];?>
" /></a>
      </div>




			<div class="hp_head_search fl">
			<div class="hp_head_searchbor">
				<form action="<?php if (!$_smarty_tpl->tpl_vars['config']->value['sy_jobdir']) {?>index.php<?php } else {
echo smarty_function_url(array('m'=>'job'),$_smarty_tpl);
}?>"
				 method="get" onsubmit="return search_keyword(this);" id="index_search_form">
					<input type="hidden" <?php if (!$_smarty_tpl->tpl_vars['config']->value['sy_jobdir']||$_smarty_tpl->tpl_vars['config']->value['sy_seo_rewrite']!='1') {?>name="m"<?php }?> value="job" id="m" />
					<input type="hidden" name="c" value="search" id="search" />
	
					<div class="hp_head_search_job fl">
						<span class="hp_head_search_job_b" id="search_name">职位</span>
						<div class="index_header_seach_find_list yunHeaderSearch_list_box none">
	
							<a href="javascript:void(0)" onclick="top_search('job', '找工作', '<?php echo smarty_function_url(array('m'=>'job'),$_smarty_tpl);?>
', '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_job_web'];?>
', '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_jobdir'];?>
'); $('#search').attr('name', 'c');">找工作</a>
	
							<a href="javascript:void(0)" onclick="top_search('resume', '找人才', '<?php echo smarty_function_url(array('m'=>'resume'),$_smarty_tpl);?>
', '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_resume_web'];?>
', '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_resumedir'];?>
'); $('#search').attr('name', 'c');">找人才</a>
	
							<a href="javascript:void(0)" onclick="top_search('tiny', '普工简历', '<?php echo smarty_function_url(array('m'=>'tiny'),$_smarty_tpl);?>
', '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_tiny_web'];?>
', '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_tinydir'];?>
'); $('#search').attr('name', '');">普工简历</a>
	
							<a href="javascript:void(0)" onclick="top_search('article', '新闻', '<?php echo smarty_function_url(array('m'=>'article'),$_smarty_tpl);?>
', '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_article_web'];?>
', '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_articledir'];?>
');" class="none">新闻</a>
	
							<a href="javascript:void(0)" onclick="top_search('once', '店铺招聘', '<?php echo smarty_function_url(array('m'=>'once'),$_smarty_tpl);?>
', '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_once_web'];?>
', '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_oncedir'];?>
'); $('#search').attr('name', '');">店铺招聘</a>
						</div>
					</div>
	
					
						<input class="hp_head_search_text fl" type="text" name="keyword" value="<?php echo $_GET['keyword'];?>
"
						 placeholder="请输入要搜索的关键字，如 会计,仓管,设计师等">
						<input class="hp_head_search_sr fl" type="submit" value="搜索" style=" float:right" />
					
				</form>
				</div>
				<div class="clear"></div>
				<div class="hp_head_search_bom">
					<div class="hp_head_search_bom_left">
						<span style="color: #a4a1a1;">热搜职位：</span>
						<?php  $_smarty_tpl->tpl_vars['keylist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['keylist']->_loop = false;
global $config;$paramer=array("limit"=>"6","recom"=>"1","type"=>"3","item"=>"“keylist“","nocache"=>"")
;$list=array();
        
        $ParamerArr = GetSmarty($paramer,$_GET,$_smarty_tpl);
		$paramer = $ParamerArr[arr];
		//是否推荐
		if($paramer[recom]){
			$tuijian = 1;
		}
		//类别
		if($paramer[type]){
			$type = $paramer[type];
		}
		//查询条数
		if($paramer[limit]){
			$limit=$paramer[limit];
		}else{
			$limit=5;
		}
		include PLUS_PATH."/keyword.cache.php";
		if($paramer[iswap]){
			$wap = "/wap";
		}else{
			$index =1;
		}
		if(is_array($keyword)){
			if($paramer[iswap]){
				$i=0;
				foreach($keyword as $k=>$v){
					if($tuijian && $v[tuijian]!=1){
						continue;
					}
					if($type && $v[type]!=$type){
						continue;
					}

					$i++;
					if($v[type]=="1"){
						$v[url] = Url("wap",array("c"=>"once","keyword"=>$v['key_name']));
						$v[type_name]='店铺招聘';
					}elseif($v['type']=="13"){
						$v['url'] = Url("wap",array("c"=>"tiny","keyword"=>$v['key_name']));
						$v['type_name']='普工简历';
					}elseif($v[type]=="3"){
						$v[url] = Url("wap",array("c"=>"job","keyword"=>$v['key_name']));
						$v[type_name]='职位';
					}elseif($v['type']=="4"){
						$v['url'] = Url("wap",array("c"=>"company","keyword"=>$v['key_name']));
						$v['type_name']='公司';
					}elseif($v['type']=="5"){
						$v['url'] = Url("wap",array("c"=>"resume","keyword"=>$v['key_name']));
						$v['type_name']='人才';
					}
					$v['key_title']=$v['key_name'];
					if($v['color']){
						$v['key_name']="<font color='".$v['color']."'>".$v['key_name']."</font>";
					}
					$list[] = $v;
					if($i==$limit){
						break;
					}
				}
			}else{
				$i=0;
				foreach($keyword as $k=>$v){
					if($tuijian && $v['tuijian']!=1){
						continue;
					}
					if($type && $v['type']!=$type){
						continue;
					}
					$i++;
					if($v['type']=="1"){
						$v['url'] = Url("once",array("keyword"=>$v['key_name']));
						$v['type_name']='店铺招聘';
					}elseif($v['type']=="2"){
						$v['url'] = Url("part",array("keyword"=>$v['key_name']));
						$v['type_name']='兼职';
					}elseif($v['type']=="13"){
						$v['url'] = Url("tiny",array("keyword"=>$v['key_name']));
						$v['type_name']='普工简历';
					}elseif($v['type']=="3"){
						$v['url'] = Url("job",array("c"=>"search","keyword"=>$v['key_name']));
						$v['type_name']='职位';
					}elseif($v['type']=="4"){
						$v['url'] = Url("company",array("keyword"=>$v['key_name']));
						$v['type_name']='公司';
					}elseif($v['type']=="5"){
						$v['url'] = Url("resume",array("c"=>"search","keyword"=>$v['key_name']));
						$v['type_name']='人才';
					}else if($v['type']=="12"){
						$v['url'] = Url("ask",array("c"=>"search","keyword"=>$v['key_name']));
						$v['type_name']='问答';
					}
					$v['key_title']=$v['key_name'];
					if($v['color']){
						$v['key_name']="<font color='".$v['color']."'>".$v['key_name']."</font>";
					}

					$list[] = $v;
					if($i==$limit){
						break;
					}
				}
			}
		}$list = $list; if (!is_array($list) && !is_object($list)) { settype($list, 'array');}
foreach ($list as $_smarty_tpl->tpl_vars['keylist']->key => $_smarty_tpl->tpl_vars['keylist']->value) {
$_smarty_tpl->tpl_vars['keylist']->_loop = true;
?>
						<a href="<?php echo smarty_function_listurl(array('type'=>'keyword','v'=>$_smarty_tpl->tpl_vars['keylist']->value['key_title']),$_smarty_tpl);?>
" class="" title="<?php echo $_smarty_tpl->tpl_vars['keylist']->value['key_title'];?>
"><?php echo $_smarty_tpl->tpl_vars['keylist']->value['key_name'];?>
</a>
						<?php } ?>
					</div>
					<div class="yun_new_header_search_more moreOptions">
						<div class="">
							<a href="<?php echo smarty_function_url(array('m'=>'job'),$_smarty_tpl);?>
">职位遍历 </a>
							<a href="<?php echo smarty_function_url(array('m'=>'map'),$_smarty_tpl);?>
">地图搜索</a>
							<?php if ($_smarty_tpl->tpl_vars['usertype']->value==1) {?>
							<a href="<?php echo smarty_function_url(array('m'=>'member','c'=>'finder','act'=>'edit'),$_smarty_tpl);?>
">高级搜索</a>
							<?php } else { ?>
							<a href="<?php echo smarty_function_url(array('m'=>'job'),$_smarty_tpl);?>
">高级搜索</a>
							<?php }?>
						</div>
					</div>
				</div>
				
			</div>
		 <div class="yunheader_60wx ">	 <div class="yunheader_60wx_box "><img src="<?php if ($_smarty_tpl->tpl_vars['config']->value['sy_wx_qcode']) {
echo $_smarty_tpl->tpl_vars['config']->value['sy_ossurl'];?>
/<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_wx_qcode'];
}?>"  ></div>	</div>

	
    <div class="yunheader_60nav">
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
