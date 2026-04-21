<?php

/*
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

/**
 *  全局的权限验证 数据验证库
 */
class common{
	public $tpl='';
	public $db='';
	public $obj='';
	public $config = '';
	public $uid="";//前台会员中心UID
	public $data="";
	public $username="";//前台会员中心username
	public $usertype="";
	public $userdid='';
	public $dirname="";
	public $protocol="";
	public $protocol_wap="";
	public $cookie = '';

	//实例化
	function common($tpl,$db,$def='',$model='index',$m='') {
		global $config;
		$this->config = $config;
		$this->tpl=$tpl;
		$this->db=$db;
		$this->def=$def;
		$this->m=$m;
				
		require(APP_PATH.'app/public/action.class.php');
		require(APP_PATH.'app/model/cookie.model.php');
		$this->cookie = new cookie_model($this->db,$this->def,
			array('uid'=>$this->uid,'username'=>$this->username,'usertype'=>$this->usertype));

		$this->protocol = getprotocol($this->config['sy_weburl']);
		//当存在UID 的时候 判断UID是否符合当前权限验证，如不符合则清空COOKIE 防止注入
		
		if(!empty($_COOKIE['uid'])){
			
		    $shell=$this->GET_user_shell($_COOKIE['uid'],$_COOKIE['shell']);
		    
			if(!is_array($shell) || empty($shell)){
			    
                $this->cookie->unset_cookie();
				$this->uid='';
				$this->userdid='';
				$this->username='';
				$this->usertype='';
				$checkUrl = $this->protocol.$_SERVER['HTTP_HOST'].$_SERVER['REQUEST_URI'];
				header("location:".$checkUrl);
				die;
			}else{
				
			    $this->uid      =   !empty($shell['pid']) ? intval($shell['pid']) : intval($shell['uid']);

				$this->userdid  =   isset($_COOKIE['userdid']) ? intval($_COOKIE['userdid']) : '';
				$this->username =   $shell['username'];
				$this->usertype =   $_COOKIE['usertype'];
				
				$this->yunset('uid', intval($shell['uid']));
				$this->yunset('usertype', $_COOKIE['usertype']);
				$this->yunset('username', $shell['username']);

                $sy_only_price  =   @explode(',',$this->config['sy_only_price']);
                $this->yunset('sy_only_price', $sy_only_price);
			}
		}else{
		    
			$this->uid = '';
		}
		
		$this->obj= new model($this->db,$def,array('uid'=>$this->uid,'username'=>$this->username,'usertype'=>$this->usertype));
		$path['temstyle']=TPL_PATH;
		$path['style']=$this->config['sy_weburl']."/app/template/".$this->config['style'];
		$path['client']=$this->config['sy_weburl']."/about";
		$path['tplstyle']=TPL_PATH.$this->config['style'];
		$path['tpldir']=$this->tpl->template_dir;
		$path['ask_style']=$this->config['sy_weburl']."/app/template/ask";
		$path['askstyle']=TPL_PATH."ask";
		$path['adminstyle']=TPL_PATH."admin";
		$path['userstyle']=TPL_PATH."member/user";
		$path['user_style']=$this->config['sy_weburl']."/app/template/member/user";
		$path['comstyle']=TPL_PATH."member/com";
		$path['com_style']=$this->config['sy_weburl']."/app/template/member/com";
		$path['wapstyle']=TPL_PATH."wap";
		$path['wap_style']=$this->config['sy_weburl']."/app/template/wap";
		$path['admin_style']=$this->config['sy_weburl']."/app/template/admin";
		$path['plusstyle']=$this->config['sy_weburl']."/data/plus";
		
		$_SERVER['HTTP_REFERER'] = isset($_SERVER['HTTP_REFERER']) ? $_SERVER['HTTP_REFERER'] : '';
		
		if($_SERVER['PHP_SELF']!='/admin/index.php'){
			$path['cookie']=$_COOKIE;
		}
		$this->yunset($path);
		
		if(!($this->config['sy_wapdomain'])){

			if($this->config['sy_web_site']=='1' && $this->config['sy_indexdomain']){

				$this->config['sy_wapdomain'] = $this->config['sy_indexdomain'].'/'.$this->config['sy_wapdir'];
			}else{
				$this->config['sy_wapdomain'] = $this->config['sy_weburl'].'/'.$this->config['sy_wapdir'];
			}
			
			$this->yunset("config_wapdomain",$this->config['sy_wapdomain']);

		}else{
		    $this->protocol_wap =  $this->config['sy_wapssl']=='1' ? 'https://' : 'http://';
			if(strpos($this->config['sy_wapdomain'],'http://')===false && strpos($this->config['sy_wapdomain'],'https://')===false)
			{
			    $this->config['sy_wapdomain'] = $this->protocol_wap.$this->config['sy_wapdomain'];
			}

			$this->yunset("config_wapdomain",$this->config['sy_wapdomain']);
		}

		// oss关闭时，oss地址为服务器地址
		if (!isset($this->config['sy_oss']) || (isset($this->config['sy_oss']) && $this->config['sy_oss'] == 2)){
		    
		    $this->config['sy_ossurl'] = $this->config['sy_weburl'];
		}
		
		$this->yunset("config",$this->config);

		$this->$model();

		//自动更新登录日志
        if ($this->uid && $_COOKIE['amtype'] != '1' ) {
           
        	$this->upLoginLog($shell);
        }

		if(!file_exists(PLUS_PATH."config.php")){

			$this->web_config();
		}
		
		//wap底部导航的发布按钮所需数据
		if($model=='wap' || $model=='wap_member'){
			$this->member_fabudata();
		}
	}
	//底部导航的发布所需数据
	function member_fabudata(){

		if($this->uid){

			if($this->usertype=='1'){

				if (!empty($this->config['user_number'])){

					$resumeM    =  $this->MODEL('resume');

		            $num     =  $resumeM->getExpectNum(array('uid'=>$this->uid));

		            $maxnum  =  $this->config['user_number'] - $num;
		            
		            if($maxnum < 0){
		            	$maxnum='0';
		            }
		            if($num){
                        $num = 1;
                    }else{
                        $num = 0;
                    }
                    $this->yunset('resume_num',$num);
		        }else{
		            $maxnum  =  '';
		        }

		        $this->yunset('fabu_resume_maxnum',$maxnum);

			}
		}
		
		
	}
	//同步登录时间
    function upLoginLog($logininfo)
    {


        if ($logininfo['login_date'] < strtotime('today') && $this->usertype > 0) {

            $time   =   time();
            $ip     =   fun_ip_get();

            if (isMobileUser()) {
                $content    =   '手机';
                $port       =   2;
            } else {
                $content    =   'PC';
                $port       =   1;
            }

            $logindata  =   array(

                'uid'       =>  $this->uid,
                'usertype'  =>  $this->usertype,
                'content'   =>  $content . '端口延续登录'
            );
           
            $logM       =   $this->MODEL('log');
            $logM->addLoginlog($logindata, array('continue' => 1));
            
            $upLogin = array(
                'login_ip' => $ip,
                'login_date'=>$time
            );
            if ($logininfo['login_ip'] != $ip || $logininfo['login_address'] =='') {
                $ip_address = getIpAddress($ip);
                $upLogin['login_address'] = $ip_address;
            }
            $UserInfoM  =   $this->MODEL('userinfo');
            $UserInfoM->upInfo(array('uid' => $this->uid), $upLogin);
            
            //  同步登录时间
            $this->obj->update_once('company', array('login_date' => time()), array('uid' => $this->uid));
            $this->obj->update_once('resume', array('login_date' => time()), array('uid' => $this->uid));

            //登录自动简历刷新，在后台配置
            if ($this->config['resume_sx'] == 1 && $this->usertype == 1) {

                $resumeM    =   $this->MODEL('resume');
                $expect     =   $resumeM->getExpectByUid($this->uid, array('field' => '`id`'));
                if (!empty($expect)) {

                    $resumeM->upResumeInfo(array('uid' => $this->uid), array('rData' => array('lastupdate' => time()), 'port' => $port));
                }
            }
        }
    }

	//页面不存在跳转
	function DoException(){
		$this->ACT_msg("index.php","您请求的页面不存在");
	}
	//后台模板
	function yuntpl($tplarr=array()){
		if(is_array($tplarr) && $tplarr!=''){
			foreach($tplarr as $v){
				if($v!=''){
				    $this->tpl->display($v.".htm");
					
				}
			}
			exit();
			
		}else{
			echo "模版不能为空！";die;
		}
	}
	//前台模板
	function yun_tpl($tplarr=array()){
		if(is_array($tplarr) && $tplarr!=''){
			foreach($tplarr as $v){
				
				$this->tpl->display($this->config['style']."/".$this->m."/".$v.".htm");
				
			}
			exit();
		}else{
			echo "模版不能为空！";die;
		}
	}
	//UC资料修改
    function uc_edit_pw($post, $old = "1", $url)
    {

        $old_info   =   $this->obj->select_once('member', array('uid' => $post['uid']), "`name_repeat`,`username`");
        if ($post['password'] != "") {

            if ($this->config['sy_uc_type'] == "uc_center" && $old_info['name_repeat'] != "1") {

                $this->obj->uc_open();

                $ucresult   =   uc_user_edit($old_info['username'], $post['oldpw'], $post['password'], $post['email'], $old);
                if ($ucresult == -1) {

                    $msg = '旧密码不正确';
                } elseif ($ucresult == -4) {

                    $msg = 'Email 格式有误';
                } elseif ($ucresult == -5) {

                    $msg = 'Email 不允许注册';
                } elseif ($ucresult == -6) {

                    $msg = '该 Email 已经被注册';
                }
                if ($msg != "") {
                    $this->ACT_msg($url, $msg);
                }
            }

            $salt   =   substr(uniqid(rand()), -6);
            $pass   =   passCheck($post['password'], $salt);
            $upData =   array('password' => $pass, 'salt' => $salt);

        }
        if (is_array($post)) {
            foreach ($post as $k => $v) {
                if ($k != "password" && $k != "oldpw") {
                    $upData[$k]  =   $v;
                }
            }
            $upData['username'] =   $old_info['username'];
        }
        return $this->obj->update_once('member', $upData,  array('uid' => $post['uid']));
    }
	
	/**
     * 向smarty模版赋值
	 * Summary of yunset
     * @param mixed $name 单个变量变量名：如“webname”，或包含多个变量的数据：如“array('webname'=>'phpyun人才系统')”
	 * @param mixed $value
	 */
    function yunset($name, $value = null)
    {
        if (is_array($name)) {
            foreach ($name as $k => $v) {

                $this->tpl->assign($k, $v);
            }
        } else {

            $this->tpl->assign($name, $value);
        }
    }

    function company()
    {
        $this->tpl->is_fun();

        $company        =   $this->obj->select_once('company', array('uid' => $_GET['id']), "`name`,`content`");
        $data['company_name']       =   $company['name'];
        $data['company_name_desc']  =   $company['content'];
        if ($_GET['nid']) {

            $news       =   $this->obj->select_once('company_news', array('id' => $_GET['nid']),"`title`");
            $data['company_news']   =   $news['title'];
        }
        if ($_GET['pid']) {
            $product    =   $this->obj->select_once('company_product', array('id' => $_GET['pid']), "`title`");
            $data['company_product']=   $product['title'];
        }
        $this->data = $data;
    }
	
	// TODO: 没有APP后台，待删除
	function appadmin(){
		$this->get_appadmin_source();
	}
	
	//前台公共部分
	function index(){
		$UA = strtoupper($_SERVER['HTTP_USER_AGENT']);
		if(strpos($UA, 'WINDOWS NT') === true){
			
			header("location:".Url('index',array('c'=>'wap')));
			exit();
		}
		$this->tpl->is_fun();
		if(empty($_GET['keyword'])){
		    $_GET['keyword']  =  '';
		}
		$qq=@explode(",",$this->config['sy_qq']);
		$this->yunset("qq",$qq);

	}
	
	function wap(){

		if($this->config['sy_wap_web']=='2'){
			header('location:'.$this->config['sy_weburl']);
		}
		
		if(is_weixin()){
		    
		    //取微信分享参数
		    if($this->config['wx_appid'] && $this->config['wx_appsecret']){
		        $signPackage = getWxJsSdk();
		        $this->yunset('signPackage',$signPackage);
		    }
		    $this->yunset('isweixin',1);
		   
		}
		$this->tpl->is_fun();
		include PLUS_PATH.'/tplmoblie.cache.php';
		if($tplmoblie['wapdiy']==1){
			$this->yunset('tplmoblie',$tplmoblie);
		}
		
		$this->moreMenu();
		
	}
  
	function wap_member(){
		if($this->config['sy_wap_web']=='2'){
			header('location:'.$this->config['sy_weburl']);
		}
		$UA = strtoupper($_SERVER['HTTP_USER_AGENT']);
		if($this->config['sy_pc_jump_wap']!='1' && strpos($UA, 'WINDOWS NT') !== false){
			header('location:'.Url('index',array('c'=>'wap')));
		}

		if(!$this->uid || !$this->username){
		    
		    $this->cookie->unset_cookie();
		    
			if(is_weixin()){
                // 外部链接访问，会员中心的，进入微信授权页面，已绑定的，直接登录，进到会员中心首页
			    if (!$_SERVER['HTTP_REFERER']){
			        
			        header('location:'.Url('wap',array('c'=>'wxoauth')));die;
			    }
			}
			
			header('location:'.Url('wap',array('c'=>'login')));die;
		}else{

			$shell=$this->GET_user_shell($this->uid,$_COOKIE['shell']);

			if(!is_array($shell)){

				$this->cookie->unset_cookie();
			    header('location:'.Url('wap',array('c'=>'login')));die;
			}else{
			    if(is_weixin()){
			        if(in_array($_GET['c'],array('integral','finance','job'))){
			            //取微信分享参数
			            if($this->config['wx_appid'] && $this->config['wx_appsecret']){
			                
			                $signPackage = getWxJsSdk();
			                
			                $this->yunset('signPackage',$signPackage);
			            }	
			        }
			        $this->yunset('isweixin',1);
			    }
				$this->yunset('uid',$this->uid);
				$this->yunset('username',$this->username);
			}
			$this->MemberLock($this->uid,array('utype'=>'wap'));
		}
	}

    function member()
    {
        $this->tpl->is_fun();
        if (!$this->uid) {
            $login = Url('login');
            $this->ACT_msg($login, '请先登录');
        }
        $this->yunset("uid", $this->uid);
        $this->yunset("username", $this->username);
        $this->yunset("useremail", $_COOKIE['email']);
        $this->yunset("is_member", 1);
    }
    function wxapp(){
        
    }

    function datav(){

    }
	//生成缓存-------------开始
	function web_config(){
		$configM	=	$this->MODEL('config');
		
		$List		=	$configM->getList(array('name'=>array('<>','')));
		
		$config		=	$List['list'];
		
		if(is_array($config)){
			foreach($config as $v){
				if($v['name']=='sy_member_icon_arr' || $v['name']=='sy_member_iconv_arr'){
				    $configarr[$v['name']]	=	empty($v['config']) ? '' : unserialize($v['config']);
				}else{
					$configarr[$v['name']]	=	$v['config'];
				}
				
			}
		}
		if(!empty($configarr)){
			made_web(PLUS_PATH.'config.php',ArrayToString($configarr),'config');
		}
		if(!file_exists(PLUS_PATH.'pimg_cache.php')){
			global $config;
			$config	=	$configarr;
			$adM	=	$this->MODEL('ad');
			$adM -> model_ad_arr();
		}
		if(!file_exists(PLUS_PATH.'dbstruct.cache.php')){
			include_once(LIB_PATH."cache.class.php");
			$cacheclass= new cache(PLUS_PATH,$this->obj);
			$cacheclass->database_cache("dbstruct.cache.php");
		}
	}

    function header_desc($title = "", $keyword = "", $description = "")
    {
        $this->yunset("title", $title);
        $this->yunset("keywords", $keyword);
        $this->yunset("description", $description);
    }

    //TODO:已移动到当前目录的model.php中，待修改完成后删除
	//调用分页,$table表名,$where条件，$pageurl分页链接,$limit条数,$rowsname模板接收变量,$pagenavname分页模板接收变量
    function get_page($table, $where = "", $pageurl = "", $limit = 20, $field = "*", $rowsname = "rows", $pagenavname = "pagenav")
    {

        $rows       =   array();
        $page       =   $_GET['page'] < 1 ? 1 : $_GET['page'];
        $ststrsql   =   ($page - 1) * $limit;
        $num        =   $this->obj->DB_select_num($table, $where);
        $this->yunset("total", $num);
        if ($num > $limit) {
            $pages      =   ceil($num / $limit);
            $pagenav    =   Page($page, $num, $limit, $pageurl, $notpl = false, $this->tpl, $pagenavname);
            $this->yunset("pages", $pages);
        }
        $rows   =   $this->obj->DB_select_all($table, "$where limit $ststrsql,$limit", $field);
        $this->yunset($rowsname, $rows);
        return $rows;
    }
	
	//验证手机是否存在
    function FetchMoblie($moblie)
    {
        $UserInfoM  =   $this->MODEL('userinfo');
        return $UserInfoM->GetMemberOne(array('moblie' => $moblie), array('field' => "`uid`,`username`"));
    }

    function logout($result = true)
    {

        //  添加referer限制请求来源
        if (stripos($_SERVER['HTTP_REFERER'], $this->config['sy_weburl']) === false) {

            exit();
        }
        if ($this->config['sy_uc_type'] == "uc_center") {

            $this->obj->uc_open();
            $logout =   uc_user_synlogout();
            $this->cookie->unset_cookie();
            echo $logout;
        } elseif ($this->config["sy_pw_type"]) {

            include(APP_PATH . "/api/pw_api/pw_client_class_phpapp.php");
            $username   =   $_SESSION["username"];
            $pw         =   new PwClientAPI($username, "", "");
            $pw->logout();  //PW同步退出
            $this->cookie->unset_cookie();

        } else {

            $this->cookie->unset_cookie();
        }
        session_start();
        unset($_SESSION['qq']);
        unset($_SESSION['wx']);
        unset($_SESSION['sina']);
        //PW同步退出
        //$this->ACT_msg($this->config['sy_weburl'],"您已成功退出！");
        if ($result) {
            echo 1;
            die;
        }
    }

    function del_dir($dir_adds = '', $del_def = 0)
    {

        $result =   false;
        if (!is_dir($dir_adds)) {
            return false;
        }
        $handle =   opendir($dir_adds);
        while (($file = readdir($handle)) !== false) {
            if ($file != '.' && $file != '..') {
                $dir    =   $dir_adds.DIRECTORY_SEPARATOR.$file;
                is_dir($dir) ? $this->del_dir($dir) : @unlink($dir);
            }
        }
        closedir($handle);
        if ($del_def == 0) {
            $result =   @rmdir($dir_adds) ? true : false;
        } else {
            $result =   true;
        }
        return $result;
    }

	function seo($ident,$title='',$keyword='',$description='',$settpl = true, $iswxapp = false)
	{
	 
		include PLUS_PATH."seo.cache.php";
	 
		$seo=$seo[$ident];
		if(is_array($seo)){
			foreach($seo as $k=>$v){
				if($this->config['did']!="" && $this->config['did']==$v['did']){
					$fzseo=$v;
				}else{
					$seo=$v;
				}
			}
			if($fzseo){
				$seo=$fzseo;
			}
		}
		$data=$this->data;
		//获取对应标签
		if(is_array($seo)){
		    $cityname = !empty($this->config['cityname'])?$this->config['cityname']:$this->config["sy_indexcity"];
			if($title){
				$this->config['sy_webname'] = $title;
				$seo['title'] = $title;
			}
			if($keyword){
				$this->config['sy_webkeyword'] = $keyword;
				$seo['keywords'] = $keyword;
			}
			if($description){
				$this->config['sy_webmeta'] = $description;
				$seo['description'] = $description;
			}
			foreach($seo as $key=>$value){
				$seo[$key] = str_replace("{webname}",$this->config['sy_webname'],$seo[$key]);
				$seo[$key] = str_replace("{webkeyword}",$this->config['sy_webkeyword'],$seo[$key]);
				$seo[$key] = str_replace("{webdesc}",$this->config['sy_webmeta'],$seo[$key]);
				$seo[$key] = str_replace("{weburl}",$this->config['sy_weburl'],$seo[$key]);
				$seo[$key] = str_replace("{city}",$cityname,$seo[$key]);

				if(is_array($data)){
					foreach($data as $k=>$v){
						$seo[$key] = str_replace("{".$k."}",$v,$seo[$key]);//替换
					}
				}
				if (!empty($seo[$key])){
				    
				    if(!@strpos('{seacrh_class}',$seo[$key])){
				        $rdata=$this->get_seacrh_class($ident,$key);
				        if($rdata && !empty($rdata) && (count($rdata)==1&&$rdata[0])){
				            $seo[$key] = str_replace("{seacrh_class}",$rdata,$seo[$key]);
				        }else{
				            $seo[$key] = str_replace("{seacrh_class}",'',$seo[$key]);
				        }
				    }
				    if(!@strpos('{search_job}',$seo[$key])){
				        $rdata=$this->get_search_job($ident,$key);
				        if($rdata && !empty($rdata) && (count($rdata)==1&&$rdata[0])){
				            $seo[$key] = str_replace("{search_job}",$rdata,$seo[$key]);
				        }else{
				            $seo[$key] = str_replace("{search_job}",'',$seo[$key]);
				        }
				    }
				    if(!@strpos('{search_city}',$seo[$key])){
				        $rdata=$this->get_search_city($ident,$key);
				        if($rdata && !empty($rdata) && (count($rdata)==1&&$rdata[0])){
				            $seo[$key] = str_replace("{search_city}",$rdata,$seo[$key]);
				        }else{
				            $seo[$key] = str_replace("{search_city}",'',$seo[$key]);
				        }
				    }
				}
				$seo[$key]=str_replace('  ',' ',$seo[$key]);
				$seo[$key]=str_replace(array("-  -","- -"),'-',$seo[$key]);
				$seo[$key]=str_replace(array("-  -","- -"),'-',$seo[$key]);

				if($key == 'share_pic'){
					$seo[$key]	=	checkpic($seo[$key]);
				}
			}
		}
		$desc = mb_substr(str_replace("	","",str_replace("\r","",str_replace("\n","",strip_tags($seo['description'])))),0,200,'utf-8');
		
		if($settpl){
		    $this->yunset('title',$seo['title']);
		    $this->yunset('keywords',$seo['keywords']);
		    $this->yunset('description',$desc);
		}else{
		    $seo['description']  =  $desc;
		    unset($seo['did']);
		    unset($seo['php_url']);
		    unset($seo['rewrite_url']);
		    return $seo;
		}
	}

	//获取搜索类别
	function get_seacrh_class($ident,$type='title')
	{
	    $cacheM  =  $this->MODEL('cache');
	    $cache   =  $cacheM->GetCache(array('hy'));
		
	    $industry_name  =  $cache['industry_name'];
	    
		if($ident=='com_search' || $ident=='part'){
		    $comcache   =  $cacheM->GetCache(array('com'));
		    $comdata    =  $comcache['comdata'];
		    $sex        =  $comcache['com_sex'];
		    $comclass_name  =  $comcache['comclass_name'];
		}
		if($ident=='user_search'){
		    $usercache  =  $cacheM->GetCache(array('user'));
		    $userdata   =  $usercache['userdata'];
		    $sex        =  $usercache['user_sex'];
		    $userclass_name  =  $usercache['userclass_name'];
		}
		
		$uptime  =  array(1=>'今天',3=>'最近3天',7=>'最近7天',30=>'最近一个月','90'=>'最近三个月');
		$data    =  array();

		foreach($_GET as $key=>$v){
			switch($key){
				case 'hy':
				    $data[]=$industry_name[$v];
				    break;
				case 'rec':
				    $data[]='推荐';
				    break;
				case 'urgent':
    				$data[]='紧急';
    				break;
				case 'pic':
    				$data[]='照片';
    				break;
				default:
				if(!in_array($key,array('idcard','work','cert'))){
					if($comdata['job_'.$key]&&$comclass_name[$v]){
						$data[]=$comclass_name[$v];
					}else if($key=='salary'){
						$data[]=$v;
					}else if($key=='sex'&&$sex[$v]){
					    $data[]=$sex[$v];
					}else if($key=='uptime'&&$uptime[$v]){
						$data[]=$uptime[$v];
					}else if(($userdata['user_'.$key]||$key=='exp')&&$userclass_name[$v]){
						$data[]=$userclass_name[$v];
					} 
				}
				
				break;
			}
		}
		foreach($data as $key=>$val){
			if($val){
				$alldata[]=$val;
			}
		}
		if(is_array($alldata)){
			if($type=='title'){
				$data = implode('-',$alldata);
			}else{
				$data = implode(',',$alldata);
			}
		}
		return $data;
	}

	//获取搜索职能
    function get_search_job($ident, $type = 'title')
    {
        $cacheM = $this->MODEL('cache');
        $cache = $cacheM->GetCache(array('job'));

        $job_name = $cache['job_name'];

        $data = array();

        $getArr = $_GET;
        //对职能、城市进行处理，有子级时，显示子级，不显示父级、祖父级
        foreach ($getArr as $key => $v) {
            if ($key == 'job_post' && $v != '') {
                unset($getArr['job1_son']);
                unset($getArr['job1']);
            } elseif ($key == 'job1_son' && $v != '') {
                unset($getArr['job1']);
            }
        }
        foreach ($getArr as $key => $v) {
            switch ($key) {
                case 'job1':
                case 'job1_son':
                case 'job_post':
                    $data[] = $job_name[$v];
                    break;
            }
        }
        foreach ($data as $key => $val) {
            if ($val) {
                $alldata[] = $val;
            }
        }
        if (is_array($alldata)) {
            if ($type == 'title') {
                $data = implode('-', $alldata);
            } else {
                $data = implode(',', $alldata);
            }
        }
        return $data;
    }

	//获取搜索城市
    function get_search_city($ident, $type = 'title')
    {
        $cacheM     =   $this->MODEL('cache');
        $cache      =   $cacheM->GetCache(array('city'));
        $city_name  =   $cache['city_name'];

        $data       =   array();
        $getArr     =   $_GET;

        //对职能、城市进行处理，有子级时，显示子级，不显示父级、祖父级
        foreach ($getArr as $key => $v) {
            if ($key == 'three_cityid' && $v != '') {

                unset($getArr['cityid']);
                unset($getArr['provinceid']);
            } elseif ($key == 'cityid' && $v != '') {

                unset($getArr['provinceid']);
            }
        }
        foreach ($getArr as $key => $v) {
            switch ($key) {
                case 'provinceid':
                case 'cityid':
                case 'three_cityid':
                    $data[] = $city_name[$v];
                    break;
            }
        }
        foreach ($data as $key => $val) {
            if ($val) {
                $alldata[] = $val;
            }
        }
        if (is_array($alldata)) {
            if ($type == 'title') {

                $data   =   implode('-', $alldata);
            } else {
                $data   =   implode(',', $alldata);
            }
        }
        return $data;
    }

    function addkeywords($type, $keyword)
    {
        $info   =   $this->obj->select_once('hot_key', array('key_name' => $keyword, 'type' => $type));

        if (is_array($info)) {

            $this->obj->update_once('hot_key', array('num' => array('+', 1)), array('key_name' => $keyword, 'type' => $type));
        } else {

            $this->obj->insert_into('hot_key', array('key_name' => $keyword, 'num' => 1, 'type' => $type, 'check' => 0));
        }
    }
	
	/**
     * 系统消息提醒
     */
    function automsg($content, $uid)
    {

        $msgM    =   $this->MODEL('sysmsg');

        $msgM->addInfo(array('uid' => $uid, 'content' => $content));
    }

    /**
     * @param $data
     * @return mixed
     */
    function post_trim($data)
    {
        foreach ($data as $d_k => $d_v) {
            if (is_array($d_v)) {

                $data[$d_k] =   $this->post_trim($d_v);
            } else {

                $data[$d_k] =   trim($data[$d_k]);
            }
        }
        return $data;
    }

    function get_moblie()
    {
        if ($this->config['sy_wap_web'] == "2") {

            header('location:' . $this->config['sy_weburl']);
        }

        $UA =   strtoupper($_SERVER['HTTP_USER_AGENT']);
        if ($this->config['sy_pc_jump_wap'] != '1' && strpos($UA, 'WINDOWS NT') !== false) {

            header('location:' . Url('index', array('c' => 'wap')));
        }
    }

    /**
     * @desc 订阅通知
     * @param $id
     * @param $type
     */
    //  TODO 订阅已调整为计划任务推送，采集职位/简历 不做订阅信息查询推送，待删除
    function send_dingyue($id, $type)
    {

        $notice =   $this->MODEL('notice');
        if ($type == "2") {

            $job    =   $this->obj->select_once('company_job', array('id' => $id), '`name`,`hy`,`uid`');

            if ($job['hy'] > 0) {

                $user   =   $this->obj->select_all('resume', array('hy_dy' => array('findin', $job['hy'])), '`email_dy`,`msg_dy`,`email`,`telphone`,`uid`,`name`');

                if (is_array($user) && $user) {

                    foreach ($user as $v) {

                        $data['uid']        = $v['uid'];
                        $data['name']       = $v['name'];
                        $data['type']       =   "userdy";
                        $data['jobname']    =   $job['name'];

                        if ($v['email_dy'] == "1") {

                            $data['email']  =   $v['email'];
                            $notice->sendEmailType($data);
                        }

                        if ($v['msg_dy'] == "1") {

                            $data['moblie'] =   $v['telphone'];
                            $data['port']   =   '1';
                            $notice->sendSMSType($data);
                        }
                    }
                }
            }
        } else {

            $expect =   $this->obj->select_once('resume_expect', array('id' => $id), '`hy`,`name`');
            $user   =   $this->obj->select_all('company', array('hy_dy' => array('findin', $expect['hy'])), '`email_dy`,`msg_dy`,`uid`,`email`,`linktel`,`name`');

            if (is_array($user) && $user) {

                foreach ($user as $v) {

                    $data['uid']        =   $v['uid'];
                    $data['name']       =   $v['name'];
                    $data['type']       =   "comdy";
                    $data['resumename'] =   $expect['name'];

                    if ($v['email_dy'] == "1") {

                        $data['email']  =   $v['email'];
                        $notice->sendEmailType($data);
                    }

                    if ($v['msg_dy'] == "1") {

                        $data['moblie'] =   $v['linktel'];
                        $data['port']   =   '1';
                        $notice->sendSMSType($data);
                    }
                }
            }
        }
    }

    /**
     * @desc post提交，在原页面显示提示框
     *
     * @param string $msg   提示消息
     * @param int $st       图标编号
     * @param string $url   跳转地址
     * @param int $tm       跳转时间
     * @param string $type  1：后台操作
     */
    function ACT_layer_msg($msg = "操作已成功！", $st = 9, $url = '', $tm = 2, $type = '0')
    {
        //解决ie9 $.get $.post 回调函数的返回值为undefine
        header("Content-Type: text/html; charset=UTF-8");

        echo '<meta charset="utf-8"/>';
        if (is_array($msg)) {
            $Html = '';
            foreach ($msg as $k => $v) {
                $Html .= '<div id="' . $k . '">' . $v . '</div>';
            }
            echo $Html;
            die;
        }
        if ($st == 9 && $type == '1') {
            $this->MODEL('log')->addAdminLog($msg);
        }
        $msg    =   preg_replace('/\([^\)]+?\)/x', "", str_replace(array("（", "）"), array("(", ")"), $msg));
        echo '<input id="layer_url" type="hidden" value="' . $url . '"><input id="layer_msg" type="hidden" value="' . $msg . '"><input id="layer_time" type="hidden" value="' . $tm . '"><input id="layer_st" type="hidden" value="' . $st . '">';
        exit();
    }

    /**
     * @desc 消息返回
     *
     * @param $msg          信息提示
     * @param string $st    图标编号9 成功 8 失败
     * @param string $type  1-批量 0-单条
     * @param string $url   跳转地址
     * @param string $tm    时间
     * @param string $error
     */
    function layer_msg($msg, $st = '9', $type = '0', $url = '', $tm = '2', $error = '0')
    {
        if ($type == '1') {

            $this->ACT_layer_msg($msg, $st, $url, $tm, $type);
        } else {
            if ($st == 9) {

                $this->MODEL('log')->addAdminLog($msg);
            }
            $msg                =   preg_replace('/\([^\)]+?\)/x', "", str_replace(array("（", "）"), array("(", ")"), $msg));
            $layer_msg['msg']   =   $msg;
            $layer_msg['tm']    =   $tm;
            $layer_msg['st']    =   $st;
            $layer_msg['url']   =   $url;
            $layer_msg['error'] =   $error;
            $msg = json_encode($layer_msg);

            //解决ie9 $.get $.post 回调函数的返回值为undefine
            header("Content-Type: text/html; charset=UTF-8");
            echo $msg;
            die;
        }
    }

    function wapheader($url)
    {
        $url    =   $this->config['sy_wapdomain'] . "/" . $url;
        header('Location: '.$url);
        exit();
    }

    /**
     * 特殊字符过滤
     * @param $string
     * @return
     */
    function stringfilter($string)
    {
        $str    =   trim($string);

        $regex  =   "/\\$|\'|\\\|/";
        $str    =   preg_replace($regex, "", $str);
        return $str;
    }

    public function MODEL($ModelName = null, $ModelPath = null)
    {
        require_once(APP_PATH.'app/public/action.class.php');
        if ($ModelName) {
            if ($ModelPath) {
                if (file_exists($ModelPath . '/' . $ModelName . '.class.php')) {

                    require_once($ModelPath . '/' . $ModelName . '.class.php');
                } else {

                    return null;
                }
            } else {

                $ModelPath      =   APP_PATH.'app/model/';
                $ModelFileName  =   $ModelName.'.model.php';
                if (file_exists($ModelPath.$ModelFileName)) {

                    require_once($ModelPath.$ModelFileName);
                } else {

                    return null;
                }
            }
            $ModelName  =   $ModelName.'_model';
        } else {

            $ModelName  =   'model';
        }
        $Model  =   new $ModelName($this->db, $this->def, array('uid' => $this->uid, 'username' => $this->username, 'usertype' => $this->usertype), $this->tpl);
        return $Model;
    }
	
	//获取描述
    function GET_content_desc($cont)
    {
        return mb_substr(strip_tags($cont), 0, 200, "utf-8");
    }

    /**
     * 自动消息提示框
     * @param string $url   跳转地址
     * @param string $msg   提示消息
     * @param int $st       1成功2失败
     * @param int $tm       跳转时间
     */
    function ACT_msg($url = '', $msg = "操作已成功！", $st = 8, $tm = 3)
    {
        // url = 1，浏览器直接后退
        if ($url == '') {
            $url = $this->config['sy_weburl'];
        }
        $this->yunset(array('msg' => $msg, 'url' => $url));
        if (isMobileUser()) {

            $this->yuntpl(array('wap/msg'));
        } else {

            $this->yuntpl(array('member/public/msg'));
        }
        exit();
    }

    /**
     * wap自动消息提示框
     * @param string $url   跳转地址
     * @param null $msg     提示消息
     * @param int $st       1成功 2失败
     * @param int $tm       跳转时间
     */
    function ACT_msg_wap($url = '', $msg = null, $st = 1, $tm = 5)
    {
        // url = 1，浏览器直接后退
        if ($url == '') {
            $url = $this->config['sy_weburl'];
        }

        if ($msg == null) {
            $msg = '操作已成功！';
        }

        $this->yunset(array('msg' => $msg, 'url' => $url, 'tm' => $tm));
        $this->yuntpl(array('wap/msg'));

        exit();
    }
    /**
     * 用户权限
     *
     * @param $uid
     * @param $shell
     * @return bool|null
     */
    function GET_user_shell($uid, $shell)
    {

        if (!preg_match("/^\d*$/", $uid)) {
            return false;
        }

        if (!preg_match("/^\d*$/", $_COOKIE['usertype'])) {
            return false;
        }

        $SQL    =   "SELECT `uid`,`username`,`usertype`,`password`,`salt`,`pid`,`status`,`login_date`,`login_ip`,`login_address` FROM `".$this->def."member` WHERE `uid`='$uid'  limit 1";

        $query  =   $this->db->query($SQL);

        $us     =   is_array($row = $this->db->fetch_array($query));

        //验证各项COOKIE信息是否对应
        if ($row['status'] != '2') {

            $shell  =   $us ? $shell == md5($row['username'].$row['password'].$row['salt']) : FALSE;
        } else {

            $shell  =   FALSE;
        }

        if ($shell) {

            if ($row['usertype'] != $_COOKIE['usertype'] && $row['usertype'] != 0 && isset($_COOKIE['usertype'])) {
                // 当前用户登录的身份与数据库中身份不同，需判断是否为后台操作的，非后台操作需清除登录状态，重新登录。
                session_start();

                if (isset($_SESSION['auid'])) {

                    $shell  =   TRUE;
                } else {

                    $shell  =   FALSE;
                }
            }

            if ($row['usertype'] == 0 && !isset($_COOKIE['usertype'])) {
                $shell  =   TRUE;
            }
        }
        return $shell ? $row : NULL;
    }

    /**
     * 补全闭合标签
     *
     * @param $html
     * @return string
     */
    function CloseTags($html)
    {
        $html       =   preg_replace('/<[^>]*$/', '', $html);
        preg_match_all('#<([a-z]+)(?: .*)?(?<![/|/ ])>#iU', $html, $result);

        $opentags   =   $result[1];
        preg_match_all('#</([a-z]+)>#iU', $html, $result);

        $closetags  =   $result[1];
        $len_opened =   count($opentags);
        if (count($closetags) == $len_opened) {
            return $html;
        }
        $opentags   =   array_reverse($opentags);

        $sc         =   array('br', 'input', 'img', 'hr', 'meta', 'link');
        for ($i = 0; $i < $len_opened; $i++) {

            $ot     =   strtolower($opentags[$i]);

            if (!in_array($opentags[$i], $closetags) && !in_array($ot, $sc)) {

                $html .= '</' . $opentags[$i] . '>';
            } else {

                unset($closetags[array_search($opentags[$i], $closetags)]);
            }
        }
        return $html;
    }

	/**
	 * 检查上传图片路径，该路径是否有图片，防止恶意上传
     *
	 * @param string $post 上传路径
	 * @param string $url  现有路径
	 * @return string
	 */
	//  TODO：暂时无调用，待删除
    function checksrc($post, $url = '')
    {

        if (strpos($post, $this->config['sy_weburl']) !== false) {

            $post   =   str_replace($this->config['sy_weburl'], '.', $post);
        }

        $post   =   str_replace(array('./data', '../data'), 'data', $post);

        if ((strpos($post, 'data/logo/') !== false || strpos($post, 'data/upload/special/') !== false) && file_exists(APP_PATH.$post)) {

            $picUrl =   $post;
        } else {

            $src    =   substr($post, strpos($post, 'data/upload/'));

            if (file_exists(APP_PATH.$src) && $post != $url) {

                $picUrl =   './'.$post;

                if ($url != '') {
                    $url = str_replace(array('./', '../'), '', $url);
                }
            } else {
                $picUrl = $url;
            }
        }
        return $picUrl;
    }

    /**
     * @desc 检查账号是否被锁定
     * @param $uid
     * @param array $data
     */
    function MemberLock($uid, $data = array())
    {

        $userInfoM  =   $this->MODEL('userinfo');
        $uInfo      =   $userInfoM->getInfo(array('uid' => $uid), array('field' => '`status`'));

        if ($uInfo['status'] == 2) {

            $this->cookie->unset_cookie();

            if ($data['utype'] == 'wap') {

                $data['msg'] = '账号已被锁，请联系管理员';
                $data['url'] = Url('wap', array('c' => 'login'));
                $this->yunset("layer", $data);
            }else{

                $this->ACT_msg(Url('login'), "账号已被锁，请联系管理员");
            }
        }
    }

    /**
     * 更多栏目导航
     */
    function moreMenu()
    {

        include PLUS_PATH.'menu.cache.php';

        $navList    =   isset($menu_name) && $menu_name[26] ? $menu_name[26] : array();

        if (!empty($navList)) {

            foreach ($navList as $k => $v) {

                $navList[$k]['pic_n'] = checkpic($v['pic']);

                if ($v['type'] == '2') {

                    $navList[$k]['url_n'] = $v['url'];
                } else {

                    if (strpos($v['url'], '&') !== false && strpos($v['url'], '=') !== false) {

                        $url = explode('&', $v['url']);
                        $sonUrl = array();
                        foreach ($url as $key => $value) {

                            $urls = explode('=', $value);
                            $sonUrl[$urls[0]] = $urls[1];
                        }
                        $navList[$k]['url_n'] = Url('wap', $sonUrl);
                        unset($sonUrl);
                    } else {

                        $navList[$k]['url_n'] = Url('wap', array('c' => $v['url']));
                    }
                }
            }
            $this->yunset('navlist', $navList);
        }
    }
    function salaryArr($data = false,$type=0)
    {
        
        if ($data){
            $salary  =  array(
                1  =>  array('minsalary'=>2000, 'maxsalary'=>0),
                2  =>  array('minsalary'=>4000, 'maxsalary'=>0),
                3  =>  array('minsalary'=>6000, 'maxsalary'=>0),
                4  =>  array('minsalary'=>8000, 'maxsalary'=>0),
                5  =>  array('minsalary'=>10000, 'maxsalary'=>0)
            );
        }else{
            if($type==1){
                $salary  =  array(
                    array('id'=>'0' ,'name'=>'全部', 'min'=>0),
                    array('id'=>'1' ,'name'=>'2000以上','min'=>2000),
                    array('id'=>'2' ,'name'=>'4000以上','min'=>4000),
                    array('id'=>'3' ,'name'=>'6000以上','min'=>6000),
                    array('id'=>'4' ,'name'=>'8000以上','min'=>8000),
                    array('id'=>'5' ,'name'=>'10000以上','min'=>10000)
                );
            }else{
                $salary  =  array(
                    'id'    =>  array(0,1,2,3,4,5,),
                    'name'  =>  array('全部','2000以上','4000以上','6000以上','8000以上', '10000以上')
                );
            }
        }
        return $salary;
    }
    function toLoginPage()
    {
        if ($this->config['sy_list_login'] == 1 && !$this->uid) {

            if (!empty($this->config['sy_list_agent'])) {

                $userAgent = explode("\n", $this->config['sy_list_agent']);

                $needLogin = true;

                foreach ($userAgent as $key => $value) {
                    if (stripos($_SERVER['HTTP_USER_AGENT'], trim($value)) !== false) {

                        $needLogin = false;
                    }
                }

                if ($needLogin) {
                    header('Location:' . Url('login'));
                    exit();
                }
            } else {

                header('Location:' . Url('login'));
                exit();
            }
        }
    }

}
?>