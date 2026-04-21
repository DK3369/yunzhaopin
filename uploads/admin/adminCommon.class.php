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
 * phpyun后台公共代码： admin（pc后台）/wapadmin（手机站后台）/siteadmin（分站后台）
 */
class adminCommon extends common{

    public $adminShell = null;
    // 除了m=index和m=admin_right，其他的控制器可以直接调用，不用重复查询
    public $adminPower = null;
    // admin/siteadmin后台公共部分
    function admin(){
        // 登录接口不用验证权限
        if (empty($_GET['m']) && ($_GET['c'] == 'login'||$_GET['c'] == 'wxlogin'||$_GET['c'] == 'getwxloginstatus')){
            return true;
        }
        $this -> get_admin_user_shell(); // 权限

        // 凡是有POST参数过来的都验证token信息
        if ($this -> config['sy_iscsrf'] != '2') {

            if (! $_SESSION['pytoken']) {

                $_SESSION['pytoken']  =  substr(md5(uniqid() . $_SESSION['auid'] . $_SESSION['ausername'] . $_SESSION['ashell']), 8, 12);
            }
            if ($_POST) {
                if ($_POST['pytoken'] != $_SESSION['pytoken']) {

                    $this->render_json(-1, '来源地址非法');
                }
                unset($_POST['pytoken']);
            }
            $this -> yunset('pytoken', $_SESSION['pytoken']);
        }
        // 初始化请求地址
        global $adminDir;
        $this -> yunset('baseUrl', $this->config['sy_weburl'] . '/'.$adminDir.'/index.php');
        $this -> yunset('sy_weburl', $this->config['sy_weburl']);
    }

    function get_admin_user_shell(){

		$shellMsg = '暂无操作权限';

        // cache要判断C ajax不需要判断
        if ($_SESSION['auid'] && $_SESSION['ashell']) {

			//验证登录信息
            $this -> admin_get_user_shell($_SESSION['auid'], $_SESSION['ashell']);

            if (!isset($this->adminShell)) {

                $this -> adminlogout();

                $this -> ShellMsg('登录超时，请刷新后重新登录！');exit();
            }
            $m = isset($_GET['m']) ? $_GET['m'] : 'index';
            $c = isset($_GET['c']) ? $_GET['c'] : 'index';
            $a = isset($_GET['a']) ? $_GET['a'] : 'index';

            if (!preg_match("/^[0-9a-zA-Z\_]*$/",$m)){
                $m = 'index';
            }

            if ($m == 'index' || ($m == 'system' && $c == 'admin_nav') || $m == 'common') {
                $m = 'index';
            }

            if ($m != 'index') {
				//优先验证4级权限池（各类子模块权限）
                $moduleNav = $this -> get_shell_module($m,$c,$a);

				if($moduleNav['error'] == '2'){//子权限验证未通过

					$this -> ShellMsg($shellMsg);exit();

				}elseif($moduleNav['error'] == '1'){//子权限验证通过 直接放行 OR 继续父级权限的认证

					//return true;

				}
                $url      =  'index.php?m=' . $m;

                $c_array  =  array(
                    'cache',
                    'markcom',
                    'markuser',
                    'advertise',
                    'zhaopinhui',
                    'admin_user',
                    'wx',
                    'admin_siteadmin'
                );

                $wx_c_arr = array(
                    'Getpubtool',
                    'wxPubTempList',
                    'wxPubTemp',
                    'wxPubTempSave',
                    'wxPubTempDel',
                    'twTask',
                    'comtwTask',
                    'delTwTask',
                    'getTW',
                    'taskFinish'
                );

                $url  .=  '&c=' . $c .'&a=' . $a;
                $navM  =  $this -> MODEL('navigation');
                $info  =  $navM -> getAdminNav(array('url'=>$url));
                
                if (empty($info)) {
                    
                    $url  =  'index.php?m=' . $m .'&c=' . $c;
                }

                $nav  =  $this -> get_shell($url);
                if (! $nav) {

                    // $this->adminlogout();
                    $this -> ShellMsg($shellMsg);exit();
                }

                if (is_numeric($this->config['did'])) {

                    if ($m == 'admin_user') {

                        $where['url']   =  'index.php?m=admin_user&c=myuser';
                        $where['dids']  =  1;

                    } else {

                        $where['url']   =  $url;
                        $where['dids']  =  1;
                    }
                    $info  =  $navM -> getAdminNav($where);
                } else {
                    $info  =  $navM -> getAdminNav(array('url'=>$url));
                }
                if (! $info) {
                    $this -> ShellMsg($shellMsg);exit();
                }
            }
        } else {
            if ($_GET['m'] != '') {
                $this -> adminlogout();
                $this -> ShellMsg('登录超时，请刷新后重新登录！');exit();
            }elseif($_GET['c'] != 'login'){
                // 用作安装后第一次生成config文件
                if(!file_exists(PLUS_PATH."config.php")){
                    $this->web_config();
                }
                $this->yuntpl(array('admin/login'));
                
                die;
            }
        }
    }

    // 用户退出
    function adminlogout(){
        unset($_SESSION['authcode']);
        unset($_SESSION['auid']);
        unset($_SESSION['ausername']);
        unset($_SESSION['ashell']);
        unset($_SESSION['md']);
        unset($_SESSION['tooken']);
        unset($_SESSION['xsstooken']);

    }
    // wapadmin用户退出
    function wapadminlogout(){
        unset($_SESSION['authcode']);
        unset($_SESSION['wuid']);
        unset($_SESSION['wusername']);
        unset($_SESSION['wshell']);
        unset($_SESSION['md']);
        unset($_SESSION['tooken']);
        unset($_SESSION['xsstooken']);

    }
	function ShellMsg($msg = '您暂无操作权限！'){

	    header('HTTP/1.1 777 Not Authorization');
		if($_POST){
		    $this->render_json(777, $msg);
		}else{
			echo $msg;
		}
		exit();
	}
    /**
     *@desc  后台用户登录
     */
    function admin_get_user_login($username, $password){

        global $config;

        $username  =  str_replace(' ', '', $username);

        $adminM    =  $this -> MODEL('admin');

        if ($config['sy_web_site'] == '1') {

            $where['username']        =  $username;
            $where['PHPYUNBTWSTART']  =  '';
            $where['did']             =  $config['did'];
            $where['isdid']           =  array('=',1,'OR');
            $where['PHPYUNBTWEND']    =  '';
            $where['status']          =  1;

            $user  =  $adminM -> getAdminUser($where, array());
        } else {

            $user  =  $adminM -> getAdminUser(array('username' => $username,'status' => 1), array());
        }
        //判断是否有登录时间控制
        if($user['control_login']){
            $timeArr = explode(" - ",$user['control_login']);
            $currtime = time();
            $stime = strtotime(date('Y-m-d ',time()).$timeArr[0]) ;
            $etime = strtotime(date('Y-m-d ',time()).$timeArr[1]);
            if($stime>$currtime  || $etime<$currtime){
                $this->render_json(-1, '不在规定登录时间内，无法登录');
            }
        }
        $verify  =  $adminM -> verifyPass($password, $user['password']);

		$logM      =  $this -> MODEL('log');

        if ($verify) {

            $_SESSION['auid']       =  $user['uid'];
            $_SESSION['ausername']  =  $user['username'];
            $_SESSION['xsstooken']  =  sha1($config['sy_safekey']);
            $_SESSION['ashell']     =  md5($user['username'] . $user['password'] . $this -> md);



            $adminLog  =  $logM -> getAdminLog(array('uid' => $_SESSION['auid'],'content' => '登录成功','orderby' => 'ctime'));

            $time      =  time();

            if ($adminLog) {

                $this -> cookie -> setCookie('lasttime', $adminLog['ctime'], $time + 80000);
            } else {

                $this -> cookie -> setCookie('lasttime', $time, $time + 80000);
            }

            $this -> cookie -> setCookie('ashell', md5($user['username'] . $user['password'] . $this -> md), $time + 80000);

            $adminM -> upAdminUser(array('lasttime' => $time), array('uid' => $user['uid']));

            $group = $adminM -> getPower(array('id'=>$user['m_id']));
            $path = in_array('216', $group['power']) && !in_array('226', $group['power']) ? '/jobtai' : '/index'; // 前端登录成功后默认跳转的页面

            $this->admin_json(0, '登录成功', compact('path'));
        } else {

			$logM -> addAdminLog("管理员：".$username.' 登录失败，登录密码：'.$password, '', '', '',array('auid'=>$user['uid'],'ausername'=>$username));

			$this->render_json(-1, '用户名或密码不正确');
        }
    }

    function admin_get_user_wxlogin(){

        global $config;

        $adminM    =  $this -> MODEL('admin');

        $weixinM    =  $this -> MODEL('weixin');

        $logM      =  $this -> MODEL('log');

        $wxqrcode = $weixinM->getWxQrcode(array('wxloginid' =>$_COOKIE['admin_wxloginid'], 'status' => 1));
        if(empty($wxqrcode)){
            $this->render_json(1, '');
        }
        if($wxqrcode['wxid']){
            if ($config['sy_web_site'] == '1') {

                $where['wxid']            =  $wxqrcode['wxid'];
                $where['PHPYUNBTWSTART']  =  '';
                $where['did']             =  $config['did'];
                $where['isdid']           =  array('=',1,'OR');
                $where['PHPYUNBTWEND']    =  '';
                $where['status']          =  1;

                $user  =  $adminM -> getAdminUser($where, array());
            } else {

                $user  =  $adminM -> getAdminUser(array('wxid' => $wxqrcode['wxid'],'status' => 1), array());
            }
        }

        if (!empty($user)) {

            //判断是否有登录时间控制
            if($user['control_login']){
                $timeArr = explode(" - ",$user['control_login']);
                $currtime = time();
                $stime = strtotime(date('Y-m-d ',time()).$timeArr[0]) ;
                $etime = strtotime(date('Y-m-d ',time()).$timeArr[1]);
                if($stime>$currtime  || $etime<$currtime){
                    $this->render_json(-1, '不在规定登录时间内，无法登录');
                }
            }

            $_SESSION['auid']       =  $user['uid'];
            $_SESSION['ausername']  =  $user['username'];
            $_SESSION['xsstooken']  =  sha1($config['sy_safekey']);
            $_SESSION['ashell']     =  md5($user['username'] . $user['password'] . $this -> md);

            $adminLog  =  $logM -> getAdminLog(array('uid' => $_SESSION['auid'],'content' => '登录成功','orderby' => 'ctime'));

            $time      =  time();

            if ($adminLog) {

                $this -> cookie -> setCookie('lasttime', $adminLog['ctime'], $time + 80000);
            } else {

                $this -> cookie -> setCookie('lasttime', $time, $time + 80000);
            }

            $this -> cookie -> setCookie('ashell', md5($user['username'] . $user['password'] . $this -> md), $time + 80000);

            $adminM -> upAdminUser(array('lasttime' => $time), array('uid' => $user['uid']));

            $group = $adminM -> getPower(array('id'=>$user['m_id']));
            $path = in_array('216', $group['power']) && !in_array('226', $group['power']) ? '/jobtai' : '/index'; // 前端登录成功后默认跳转的页面

            $this->admin_json(0, '登录成功', compact('path'));
        } else {

            $logM -> addAdminLog('管理员微信扫码登录失败');

            $this->render_json(-1, '管理员微信扫码登录失败');
        }
    }
    /**
     * 管理员用户登录权限验证
     */
    function admin_get_user_shell($uid, $shell){

        global $config;

        if (! preg_match("/^\d*$/", $uid)) {
            return false;
        }

        $adminM    =  $this -> MODEL('admin');

        if ($config['sy_web_site'] == '1') {

            $where['uid']             =  $uid;
            $where['PHPYUNBTWSTART']  =  '';
            $where['did']             =  $config['did'];
            $where['isdid']           =  array('=',1,'OR');
            $where['PHPYUNBTWEND']    =  '';

            $user  =  $adminM -> getAdminUser($where);
        } else {

            $user  =  $adminM -> getAdminUser(array('uid' => $uid));
        }

        $ps = !empty($user) ? $shell == md5($user['username'] . $user['password'] . $this->md) : FALSE;

        if ($ps){
			//活动期间 保持SESSION 活跃度
			if(!$_SESSION['rftime'] || $_SESSION['rftime']<(time()-300)){
				$_SESSION['rftime'] = time();
			}
            $this->adminShell = $user;
        }
    }

    /**
     * 检查访问地址
     */
    function check_token(){
        if ($this->config['sy_iscsrf'] != '2') {

            if ($_SESSION['pytoken'] != $_GET['pytoken'] || ! $_SESSION['pytoken']) {

                unset($_SESSION['pytoken']);

                $this -> ACT_layer_msg('来源地址非法！', 8, 'index.php');

                exit();
            }
        }
    }

    /**
     * 后台顶部导航条默认页面
     * @param $id
     * @param $power
     */
    function GET_web_default($id, $power){

        if ($this->config['sy_web_site'] == '1' && $this->config['did']) {

            $where['dids'] = 1;
        }

        $navM  =  $this -> MODEL('navigation');

        $web   =  $navM -> getAdminNavList(array('keyid'=>array('in',pylode(',', $id)),'orderby'=>'sort,asc'));

        if (is_array($web['list'])) {

            foreach ($web['list'] as $v) {

                if (@in_array($v['id'], $power)) {

                    $arr[]           =  $v['id'];

                    $arr2[$v['id']]  =  $v['keyid'];
                }
            }

            $where['keyid']    =  array('in',pylode(',', $arr));
            $where['orderby']  =  'sort,asc';

            $webaa  =  $navM -> getAdminNavList($where);

            if (is_array($webaa)) {

                foreach ($webaa['list'] as $va) {

                    if (@in_array($va['id'], $power)) {

                        $value[$arr2[$va['keyid']]]  =  $va['url'];
                    }
                }
            }
        }

        return $value;
    }
	/**
     * 验证当前登录管理员，权限组操作模块权限认证
     * @param string $web  要检测的链接url
     * @param string $type
     * @return boolean
     */
    function get_shell_module($m,$c,$a){

		$navM    =  $this -> MODEL('navigation');
        //首先检测当前链接是否在操作模块权限池中
		$nav     =  $navM -> getAdminNav(array('url'=>'index.php?m='.$m.'&c='.$c.'&a='.$a,'level'=>4));

		//不在权限池就无需验证
		if(!empty($nav)){
			// index.php里已经查了一遍权限，为了防止直接请求具体方法的，保留做容错处理
			if (empty($this->adminPower['power'])){
			    $adminM  =  $this->MODEL('admin');
			    $this->adminPower  =  $adminM -> getPower(array('id'=>$this->adminShell['m_id']));
			}

			if ($this->adminPower['power'] && in_array($nav['id'], $this->adminPower['power'])){

				$return['error'] = 1;
				$return['power'] = $this->adminPower;
			} else {
				$return['error'] = 2;
			}
		}else{

			$return['error'] = 3;

		}

        return $return;
    }
    /**
     * 验证当前登录管理员，管理员组导航菜单权限检测
     * @param string $web  要检测的链接url
     * @param string $type
     * @return boolean
     */
    function get_shell($web, $type = ''){
        // $web=str_replace('&','&amp;',$web);

        // get_shell_module里面，4级权限没查到的，不会查总权限，这里需要在查一遍
        if (empty($this->adminPower['power'])){
            $adminM  =  $this->MODEL('admin');
            $this->adminPower  =  $adminM -> getPower(array('id'=>$this->adminShell['m_id']));
        }

        //判断当前管理员类型有权限
        if ($this->adminPower['power'] && $this->adminPower['group_name']){

            $navM    =  $this -> MODEL('navigation');

            $nav     =  $navM -> getAdminNav(array('url'=>$web));

            $return  =  @in_array($nav['id'], $this->adminPower['power']) ? true : false;
        } else {
            $return  =  false;
        }
        return $return;
    }
    /**
     * 获取页面名称
     * @param string $id
     * @return string
     */
    function GET_web_check($id){

        $navM  =  $this -> MODEL('navigation');

        $nav   =  $navM -> getAdminNav(array('id'=>$id));

        if (is_array($nav)) {

            $value  =  $this -> GET_web_check($nav['keyid']);

            $value  .=  $nav['name'] . ' - ';
        }
        return $value;
    }
    /**
     * 获取微信/公众号绑定状态
     * @param string $id
     * @return string
     */
    function wxBindState($wxBind){

        if(!empty($wxBind['wxid'])){
            if(!empty($wxBind['unionid'])){
                $wxGzhmsg = '公众号已绑定，微信开放平台已绑定';
            }else{
                $wxGzhmsg = '公众号已绑定';
            }
        }else{
            $wxGzhmsg= '公众号未绑定';
        }
        $wxBindmsg = $wxGzhmsg;
        return $wxBindmsg;
    }
    /**
     * @desc 需记录管理员日志的操作，
     */
    public function admin_json($error = 0, $msg = NULL, $data = array())
    {
        if (isset($msg) && $msg != ''){

            $this->MODEL('log')->addAdminLog($msg);
            // 过滤掉消息中括号以及括号内字符。如：“操作成功（ID:2）”  处理后变成“操作成功”
            $msg = preg_replace('/\([^\)]+?\)/x', "", str_replace(array("（", "）"), array("(", ")"), $msg));
        }else{
            $msg = '';
        }

        $result =   array(
            'error' =>  $error,
            'msg'   =>  $msg
        );
        ($data) && ($result['data'] = $data);
        header('content-type:application/json; charset=utf-8');
        echo json_encode($result);
        exit;
    }
    /**
     * @desc    一般性数据查询和简单提示(例如缺少参数，条件不符等)，将接口返回数据以统一格式的JSON输出
     *
     * @param string $error 执行结果标识码
     * @param string $msg   执行结果描述信息
     * @param array $data   执行结果输出的数据
     * @param int $total
     */
    public function render_json($error = 0, $msg = '', $data = array())
    {
        $msg =  isset($msg) ? preg_replace('/\([^\)]+?\)/x', "", str_replace(array("（", "）"), array("(", ")"), $msg)) : '';
        $result =   array(
            'error' =>  $error,
            'msg'   =>  $msg,
            'data'  =>  isset($data) ? $data : array()
        );

        header('content-type:application/json; charset=utf-8');
        echo json_encode($result);
        exit;
    }

}
?>