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
class index_controller extends adminCommon{

    /**
     * @desc后台首页
     */
    function index_action($set = ''){

        global $db_config;

        $this -> yunset('db_config', $db_config);

        if (!isset($_SESSION['auid'])) {

            $this->yuntpl(array('admin/login'));

            die;
        }

        // 权限配置
        $adminM  =  $this -> MODEL('admin');

        $this->adminPower  =  $adminM -> getPower(array('id'=>$this->adminShell['m_id']));

        $this -> yunset('power', $this->adminPower['power']);
        
        // 判断是否业务员 并且无管理权限 此处调整登录方法也需要相应调整
        if(in_array('216',$this->adminPower['power']) && !in_array('226',$this->adminPower['power'])){
			$this -> yunset('indexData', array(
                'nav_id' => 0,
                'one_menu_id' => 213,
                'two_menu_id' => 216,
                'name' => '工作台',
			    'path' => '/jobtai'
            ));
		}else{
			$this -> yunset('indexData', array(
                'nav_id' => 0,
                'one_menu_id' => 0,
                'two_menu_id' => 0,
                'name' => '首页',
                'path' => '/index'
            ));
		}

        $nav_user  =  array(
            'name'        =>  $this->adminPower['name'],
            'group_name'  =>  $this->adminPower['group_name']
        );

        //查询最近登录时间
        $logM  =  $this->MODEL('log');

        $adminLog  =  $logM -> getAdminLog(array('uid' => $_SESSION['auid'],'content' => '登录成功','orderby' => 'ctime'));

        // 导航配置
        $navM      =  $this -> MODEL('navigation');

        $navWhere = array(
            'display'=>array('<>',1),
            'id'=>array('in', pylode(',', $this->adminPower['power'])),
            'orderby'=>'sort,ASC'
        );
        $navList   =  $navM -> getAdminNavList($navWhere, array('utype'=>'power'));

        // 自定义快捷菜单
        $existCustomizeNavs = $navM->select_once('admin_customize_navs',array('auid' => $_SESSION['auid']));
        if ($existCustomizeNavs) {
            $navIdArr = array();
            $navIds = json_decode($existCustomizeNavs['nav_ids'], 1);
            foreach ($navIds as $navId) {
                in_array($navId, $this->adminPower['power']) && array_push($navIdArr, $navId);
            }
            $menu = $navM->select_all('admin_navigation',array('display'=>array('<>',1),'id'=>array('in', pylode(',', $navIdArr)),'orderby'=>'sort,ASC'),'id,keyid,name,classname,url,path');
            // 自定义菜单过滤后为空
            if (!$menu) {
                $menu = $navList['menu'];
            }
        } else {
            $menu = $navList['menu'];
        }

        $setarr    =  array(
            'one_menu'        =>  $navList['one_menu'],
            'two_menu'        =>  $navList['two_menu'],
            'navigation'      =>  $navList['navigation'],
            'menu'            =>  $menu,
            'nav_user'        =>  $nav_user,
            'admin_lasttime'  =>  $adminLog['ctime'] ? $adminLog['ctime'] : time()
        );
        $customizeIds = array();
        foreach ($menu as $m) {
            !in_array($m['id'], $customizeIds) && array_push($customizeIds, $m['id']);
        }
        $this->yunset('customize_ids', $customizeIds);

        $this -> yunset($setarr);

        // 获取默认页面
        if (is_array($navList['navigation'])) {

            foreach ($navList['navigation'] as $v) {

                $navigation_url_id[]  =  $v['id'];
            }

            $navigation_url = $this -> GET_web_default($navigation_url_id, $this->adminPower['power']);


        }

        $this -> yunset('navigation_url', $navigation_url);

        if ($set == '') {

            $this->yuntpl(array('admin/index'));

        }
    }
    /**
     * 登录
     */
    function login_action(){

        $_POST  =  $this -> post_trim($_POST);
        if (empty($_POST['username'])) {
            $this->render_json(1, '请填写用户名');
        }elseif (empty($_POST['password'])){
            $this->render_json(2, '请填写密码');
        }

        if (strpos($this -> config['code_web'], '后台登录') !== false) {

            if ($this -> config['code_kind'] > 2) {

                $check	=  verifytoken($this->config);
                // 极验验证
                if ($check['code'] != '200') {
                    $this->render_json(-1, $check['msg']);
                } else {
                    $this -> admin_get_user_login($_POST['username'], $_POST['password']);
                }

            } else {

                if (md5(strtolower($_POST['authcode'])) === $_SESSION['authcode']) {

                    unset($_SESSION['authcode']);

                    $this -> admin_get_user_login($_POST['username'], $_POST['password']);

                } else {

                    unset($_SESSION['authcode']);

                    $this->render_json(-1, '验证码错误');
                }
            }

        } else {

            $this -> admin_get_user_login($_POST['username'], $_POST['password']);
        }
    }
    function wxlogin_action()
    {

        $admin_wxloginid  =   isset($_COOKIE['admin_wxloginid']) ? $_COOKIE['admin_wxloginid'] : '';
        $WxM        =   $this->MODEL('weixin');
        $qrcode     =   $WxM->applyWxQrcode($admin_wxloginid,'admin_wxloginid');
        if(!$qrcode){
            $this->render_json(1, '二维码获取失败');
        }else{
            $this->render_json(0, 'ok', array('code_url' => $qrcode));
        }
    }
    function getwxloginstatus_action()
    {
        if ($_COOKIE['admin_wxloginid']) {

            $this -> admin_get_user_wxlogin();
            
        } else {
            $this->render_json(1, '');
        }
    }
    /**
     * @desc 首页-管理员退出
     */
    function logout_action(){

        $this -> adminlogout();

        $this -> layer_msg('您已成功退出！', 9, 0, 'index.php');
    }

    /**
     *@desc 首页-清除缓存
     */
    function del_cache_action(){
        $cache   =  $this->del_dir('../data/templates_c', 1);

        /* 生成四位JS，css识别码 */
        $configM  =  $this->MODEL('config');

        $configM -> cacheCode();

        $this -> web_config();

        if ($cache == true) {
            $this->admin_json(0, '清除缓存成功');
        } else {
            $this->render_json(1, '清除缓存失败，请检查是否有权限！');
        }
    }

    /**
     * 后台地图和快捷导航 公用获取方法
     */
    function getMenu_action()
    {
        // 权限配置
        $adminM = $this->MODEL('admin');

        $this->adminPower  =  $adminM -> getPower(array('id'=>$this->adminShell['m_id']));

        // 导航配置
        $navM = $this->MODEL('navigation');

        $navWhere = array(
            'display' => array('<>', 1),
            'id'=>array('in', pylode(',', $this->adminPower['power'])),
            'orderby' => 'sort,ASC'
        );
        $navList = $navM->getAdminNavList($navWhere, array('utype' => 'power'));

        // 自定义快捷菜单
        $existCustomizeNavs = $navM->select_once('admin_customize_navs', array('auid' => $_SESSION['auid']));
        if ($existCustomizeNavs) {
            $navIdArr = array();
            $navIds = json_decode($existCustomizeNavs['nav_ids'], 1);
            foreach ($navIds as $navId) {
                in_array($navId, $this->adminPower['power']) && array_push($navIdArr, $navId);
            }
            $menu = $navM->select_all('admin_navigation', array('display' => array('<>', 1), 'id' => array('in', pylode(',', $navIdArr)), 'orderby' => 'sort,ASC'), 'id,keyid,name,classname,url,path');
            // 自定义菜单过滤后为空
            if (!$menu) {
                $menu = $navList['menu'];
            }
        } else {
            $menu = $navList['menu'];
        }

        $navigation = $navList['navigation'];
        $one_menu = $navList['one_menu'];
        $two_menu = $navList['two_menu'];

        if ($navigation) {
            foreach ($navigation as $key1 => $val1) {
                if (!empty($one_menu[$val1['id']])) {
                    $navigation[$key1]['children'] = $one_menu[$val1['id']];
                    foreach ($one_menu[$val1['id']] as $key2 => $val2) {
                        if (!empty($two_menu[$val2['id']])) {
                            $navigation[$key1]['children'][$key2]['children'] = $two_menu[$val2['id']];
                        }
                    }
                }
            }
        }

        $customizeIds = array();
        foreach ($menu as $m) {
            !in_array($m['id'], $customizeIds) && array_push($customizeIds, $m['id']);
        }

        $this->render_json(0, 'ok', compact('navigation', 'customizeIds'));
    }
    /**
     * 首页-设置快捷导航
     */
    function shortcut_menu_action(){
        if (empty($_POST['chk_value'])) {
            $this->render_json(1, '参数错误');
        }

        $navM = $this->MODEL('navigation');
        $return = $navM->setCustomizeNav($_POST['chk_value'], $_SESSION['auid']);
        if ($return) {
            $this->admin_json(0, '快捷导航设置成功');
        } else {
            $this->render_json(1, '快捷导航设置失败');
        }
    }
    /**
     * 首页-查询相关统计数据
     */
    function msgNum_action(){

        $MsgNum  =  $this -> MODEL('msgNum');
        echo $MsgNum->msgNum();
    }
    

    function wxbind_action(){
        $WxM=$this->MODEL('weixin');
        $qrcode = $WxM->applyWxQrcode($_COOKIE['wxadminbind'],'wxadminbind');
//        $qrcode = 'https://mp.weixin.qq.com/cgi-bin/showqrcode?ticket=gQG57zwAAAAAAAAAAS5odHRwOi8vd2VpeGluLnFxLmNvbS9xLzAyYW1YWWd5ci05MlUxc3gyTmhBY1MAAgShcHBkAwSAUQEA';
        if(!$qrcode){
            $this->render_json(1, '二维码获取失败');
        }else{
            $this->render_json(0, 'ok', array('code_url' => $qrcode));
        }
    }
    function getwxbindstatus_action(){
        if($_SESSION['auid']){
            $adminM  =  $this -> MODEL('admin');
            $uadmin = $adminM ->  getAdminUser(array('uid'=>$_SESSION['auid']));
            if($uadmin['wxid']!=''){
                $this->render_json(0, '已绑定', array('wxid' => $uadmin['wxid']));
            }else{
                $this->render_json(1, '暂未绑定');
            }
        }
    }
    // wangEditor上传图片
    function uploadfile_action(){
        $dir = $_GET['dir'] ? trim($_GET['dir']) : 'wangEditor';
        $data = array(
            'file' => $_FILES['file'],
            'path' => $dir
        );

        $uploadM = $this->MODEL('upload');
        $res = $uploadM->wangUpload($data);

        echo json_encode($res);
    }
    function wangeditorUpfile_action(){
        
        $errno = 1; $fileurl = $message ='';

        if (!empty($_FILES['file']) && $_FILES['file']['name']) {
            $upArr = array(
                'file' => $_FILES['file'],
                'dir' => 'wangEditor'
            );

            $uploadM = $this->MODEL('upload');

            $result = $uploadM->uploadDoc($upArr);
            
            if ($result['msg']) {
                $errno = 1;
                $message = $result['msg'];
            } else {
                $errno = 0;
                $message = 'SUCCESS';
                $fileurl = checkpic($result['docurl']);
            }
        }


        $data["errno"] = $errno;
        $data["message"] = $message;
        $data["data"] = array('url'=>$fileurl);
        
        echo json_encode($data,true);
    }
    //后台专用，layui上传图片公共方法
    function layui_upload_action()
    {
        if($_FILES['file']['tmp_name']){
            $data  =  array(
                'name'      =>  $_POST['name'],
                'path'      =>  $_POST['path'],
                'imgid'     =>  $_POST['imgid'],
                'file'      =>  $_FILES['file'],
                'auid'      =>  $_SESSION['auid']
            );
            if (isset($_POST['uid']) && isset($_POST['usertype'])){
                $data['uid']        =   $_POST['uid'];
                $data['usertype']   =   $_POST['usertype'];
            }
            $UploadM=$this->MODEL('upload');
            $return = $UploadM->layUpload($data);
            if (!empty($_POST['path']) && $_POST['path'] == 'logo'  && $return['code'] == 0){
                // 后台上传logo后，重新生成缓存
                $this->web_config();
            }
        }else{
            $return  =  array(
                'code'  =>  1,
                'msg'   =>  '请上传文件',
                'data'  =>  array()
            );
        }
        echo json_encode($return);
    }

    /**
     * 获取手机号归属地
     *users:王旭
     *Data:2023/5/15
     *Time:9:35
     */
    function getMobileAddress_action(){
        if ($_POST['uid'] && $_POST['moblie']) {
            $moblieaddress = getMoblieAddress($_POST['moblie']);
            $userinfoM = $this->MODEL('userinfo');
            $userinfoM->upInfo(array('uid'=>$_POST['uid']),array('moblie_address'=>$moblieaddress));
            if ($moblieaddress){
                $msg = $moblieaddress;
            }elseif($this->config['sy_mobile'] == 2){
                $msg = '请先开启归属地查询';
            }else{
                $msg = '查询失败';
            }

            $this->render_json($moblieaddress ? 0 : 1, $msg);
        }
    }

    /**
     * 获取ip归属地
     *users:王旭
     *Data:2023/5/15
     *Time:9:36
     */
    function getIpAddress_action(){
        if ($_POST['uid'] && $_POST['ip']) {
            $ipaddress = getIpAddress($_POST['ip']);
            $userinfoM = $this->MODEL('userinfo');
            $userinfoM->upInfo(array('uid'=>$_POST['uid']),array('login_address'=>$ipaddress));
            if ($ipaddress){
                $msg = $ipaddress;
            }elseif($this->config['sy_ip'] == 2){
                $msg = '请先开启归属地查询';
            }else{
                $msg = '查询失败';
            }

            $this->render_json($ipaddress ? 0 : 1, $msg);
        }
    }

    function homeData_action(){

        $data = $sysinfo = array();

        global $db_config;
        // 获取当前版本，并比较，确认最新的升级文件是否允已运行
        $versionM = $this->MODEL('version');
        $version = $versionM->getVerion();
         
        $pyu = $versionM->phpyunVersionCompare($version);
        
        $base       =       base64_encode($db_config['coding'] . '|phpyun|' . $this->config['sy_webname'] . '|phpyun|' . $this->config['sy_weburl'] . '|phpyun|' . $this->config['sy_webemail'] . '|phpyun|' . $version);
        
        if (is_dir('../admin')) {
            
            $dirname[]      =       'admin';
        } else {
            // 生成admin是否更改
            $admindir       =   str_replace('/index.php', '', $_SERVER['PHP_SELF']);
            $admindir_arr   =   explode('/', $admindir);
            $newadmindir    =   $admindir_arr[count($admindir_arr) - 1];
            
            include (PLUS_PATH . '/admindir.php');
            if ($admindir != $newadmindir) {
                $cont       =       "<?php";
                $cont       .=       "\r\n";
                $cont       .=      "\$admindir='" . $newadmindir . "';";
                $cont       .=      "?>";
                
                $fp          =   @fopen(PLUS_PATH . '/admindir.php', 'w+');
                $filetouid   =   @fwrite($fp, $cont);
                @fclose($fp);
            }
        }
        if (is_dir('../install')){
            $dirname[] = 'install';
        }
        
        $adminM     =     $this -> MODEL('admin');
        
        $nav_user   =   $adminM -> getPower(array('id'=>$this->adminShell['m_id']));
        
        $mruser     =   $this->adminShell['username'] == 'admin' && $this->adminShell['password'] == $adminM->makePass('admin') ? 1 : 0;
        $indextip_show = false;
        if (($dirname || $mruser==1 || $pyu) && in_array('161',$nav_user['power'])){
            $indextip_show = true;
        }

        $sysinfo['version']    =    $version;
        $sysinfo['soft']       =    $_SERVER['SERVER_SOFTWARE'];
        $sysinfo['kongjian']   =    round(@disk_free_space('.') / (1024 * 1024), 2);
        $sysinfo['phpbanben']  =    PHP_VERSION;
        $sysinfo['banben']     =    $this->db->mysql_server(1);
        $sysinfo['yonghu']     =    @get_current_user();
        $sysinfo['server']     =    $_SERVER['SERVER_NAME'];

        $topinfo['indextip_show']   =   $indextip_show;
        $topinfo['mruser']          =   $mruser;
        $topinfo['msg_setting']     =   in_array('147',$nav_user['power'])?1:0;
        $topinfo['pyu']             =   $pyu;
        $topinfo['dirname']         =   !empty($dirname)?@implode(',', $dirname):'';
        $topinfo['updateUrl']       =   $this->config['sy_weburl'].'/update/index.php';


        $data['index_lookstatistc'] =   $this->adminShell['index_lookstatistc'];
        $data['base']               =   $base;
        $data['topinfo']            =   $topinfo;
        $data['sysinfo']            =   $sysinfo;
        
        $this->render_json(0,'',$data);
    }
    function ajax_right_action(){
        if ($this->adminShell['index_lookstatistc'] != 2) {
            $this->render_json(1);
        }
        

        //短信数
        $msgnum = 0;
        if (!empty($this->config['sy_msg_appkey']) && !empty($this->config['sy_msg_appsecret'])) {

            //短信检测
            $url = 'https://u.phpyun.com/feature';
            $url .= '?appKey=' . $this->config['sy_msg_appkey'] . '&appSecret=' . $this->config['sy_msg_appsecret'];

            if (extension_loaded('curl')) {

                $return = CurlGet($url);
            } else if (function_exists('file_get_contents')) {

                $return = file_get_contents($url);
            }

            if ($return) {
                $msgInfo = json_decode($return, true);
                if ($msgInfo['code'] == '200') {
                    $msgnum = $msgInfo['num'];
                }
            }
        }

        $data['msgnum'] = $msgnum;
        
        $this->render_json(0,'',$data);

    }
    // 后台首页：会员统计信息：今日新增会员总数、新增个人会员数、新增简历、新增企业、新增职位、
    // 待审核个人会员、企业、职位、简历
    // 后台首页：收益统计：总收益、
    function ajax_statis_action(){
        // 拦截没有首页查看统计权限的请求
        if ($this->adminShell['index_lookstatistc'] != 2) {
            $this->render_json(1);
        }
        $UserinfoM  =   $this->MODEL('userinfo');
        $ResumeM    =   $this->MODEL('resume');
        $JobM       =   $this->MODEL('job');
        $CorderM    =   $this->MODEL('companyorder');
        
        $downresumeM=   $this->MODEL('downresume');
        
        $res = array();
        if ($_POST['type'] == 'month') {
            $thismonth  =   strtotime(date('Y-m'));
            if ($_POST['area'] == 'member') {
                // 本月新增非个人、企业会员数（其他会员）
                $ommwhere['usertype'][]  =   array("<>",1);
                $ommwhere['usertype'][]  =   array("<>",2,'and');
                $ommwhere['reg_date']    =   array(">=",$thismonth);
                $ommemberNum              =   $UserinfoM->getMemberNum($ommwhere);
                // 本月新增会员数
                $where['reg_date']      =   array(">=",$thismonth);
                $monthMemberNum         =   $UserinfoM->getMemberNum($where);
                //当月企业会员数
                $oldcomwhere['reg_date']    =   array(">=",$thismonth);
                $oldcomwhere['usertype']    =   2;
                $oldcomwhere['pid']         =   0;
                $oldcompanyNum              =   $UserinfoM->getMemberNum($oldcomwhere);
                //本月个人会员数
                $olduserwhere['reg_date']   = array(">=",$thismonth);
                $olduserwhere['usertype']   =   1;
                $olduserNum                 =   $UserinfoM->getMemberNum($olduserwhere);
                $res['monthMemberNum'] = $monthMemberNum;// 本月新增会员数
                $res['ommemberNum'] = $ommemberNum;// 本月新增其他会员数
                $res['userNumMon'] = $olduserNum;// 本月新增个人会员数
                $res['companyNumMon'] = $oldcompanyNum;// 本月新增企业会员数
           } else if ($_POST['area'] == 'money') {
                // 本月总收益
                $owhere['order_state']  =   2;
                $owhere['order_time']   =   array('>=',$thismonth);
                $owhere['type']         =   array('<>', 6);
                $monthmoneyTotal =   $CorderM->getInfo($owhere,array('field'=>'sum(order_price) as `total`'));
                $monthMoneyTotal = $monthmoneyTotal['total'];
                // 本月会员套餐 收益
                $vwhere['order_state']  =   2;
                $vwhere['order_time']   =   array('>=',$thismonth);
                $vwhere['type']         =   1;
                $monthMoneyVip   =   $CorderM->getInfo($vwhere,array('field'=>'sum(order_price) as `total`'));
                $monthMoneyVip   =   $monthMoneyVip['total'];
                // 本月增值服务 收益
                $swhere['order_state']  =   2;
                $swhere['order_time']   =   array('>=',$thismonth);
                $swhere['type']         =   5;
                $monthMoneyService   =   $CorderM->getInfo($swhere,array('field'=>'sum(order_price) as `total`'));
                $monthMoneyService   =   $monthMoneyService['total'];
                // 本月其他收益（除去会员套餐和增值服务，都归为其他收益）
                $mIwhere['order_state']      =   2;
                $mIwhere['order_time']       =   array('>=',$thismonth);
                $mIwhere['PHPYUNBTWSTART_A'] =   '';
                $mIwhere['type'][]           =   array('<>', 1, 'AND');
                $mIwhere['type'][]           =   array('<>', 5, 'AND');
                $mIwhere['type'][]           =   array('<>', 6, 'AND');
                $mIwhere['PHPYUNBTWEND_A']   =   '';
                $monthMoneyIntegral  =   $CorderM->getInfo($mIwhere,array('field'=>'sum(order_price) as `total`'));
                $monthMoneyIntegral  =   $monthMoneyIntegral['total'];
                $res['monthMoneyTotal'] = !empty($monthMoneyTotal) ? str_replace('.00', '', $monthMoneyTotal) : '0';//本月收益
                $res['monthMoneyVip'] = !empty($monthMoneyVip) ? str_replace('.00', '', $monthMoneyVip) : '0';//本月会员套餐
                $res['monthMoneyIntegral'] = !empty($monthMoneyIntegral) ? str_replace('.00', '', $monthMoneyIntegral) : '0';//本月其他服务收益
                $res['monthMoneyService'] = !empty($monthMoneyService) ? str_replace('.00', '', $monthMoneyService) : '0';//本月增值服务收益
            }
        } else {
            $today  =   strtotime('today');
            // 昨日时间条件
            $date[] =   array('>=', strtotime('yesterday'));
            $date[] =   array('<', $today);

            // 今日新增会员数
            $where['reg_date']  =   array(">=",$today);
            $memberNum          =   $UserinfoM->getMemberNum($where);

            // 今日新增非个人、企业会员数（其他会员）
            $omtwhere['usertype'][]  =   array("<>",1);
            $omtwhere['usertype'][]  =   array("<>",2,'and');
            $omtwhere['reg_date']    =   array(">=",$today);
            $otmemberNum             =   $UserinfoM->getMemberNum($omtwhere);
            // 新增个人会员数
            $userwhere['reg_date']  =   array(">=",$today);
            $userwhere['usertype']  =   1;
            $userNum                =   $UserinfoM->getMemberNum($userwhere);
            // 新增企业会员数
            $comwhere['reg_date']   =   array(">=",$today);
            $comwhere['usertype']   =   2;
            $comwhere['pid']        =   0;
            $companyNum             =   $UserinfoM->getMemberNum($comwhere);
            //今日简历数
            $expectwhere['ctime']   =   array(">=",$today);
            $resumeNum              =   $ResumeM->getExpectNum($expectwhere);
            //昨日简历数
            $oldexpectwhere['ctime']    =   $date;
            $oldresumeNum               =   $ResumeM->getExpectNum($oldexpectwhere);
            // 今日拨号
            $telwhere['ctime']  =   array(">=",$today);
            $tellognum = $JobM->getTelLogNum($telwhere);
            //昨日拨号
            $telwhere['ctime']  =   $date;
            $oldtellognum = $JobM->getTelLogNum($telwhere);

            //简历投递数
            $useridjobwhere['datetime'] = array(">=",$today);
            $useridjobNum               =   $ResumeM->getUseridJobNum($useridjobwhere);
            //昨日简历投递数
            $olduseridjobwhere['datetime'] = $date;
            $olduseridjobNum                =   $ResumeM->getUseridJobNum($olduseridjobwhere);
            //今日职位数
            $jobwhere['sdate']      =   array(">=",$today);
            $jobNum                 =   $JobM->getJobNum($jobwhere);
            //昨日职位数
            $oldjobwhere['sdate']       =   $date;
            $oldjobNum                  =   $JobM->getJobNum($oldjobwhere);
            //简历下载数
            $downresumewhere['downtime'] = array(">=",$today);
            $downresumeNum = $downresumeM->getDownNum($downresumewhere);
            //昨日简历下载数
            $olddownresumewhere['downtime'] = $date;
            $olddownresumeNum = $downresumeM->getDownNum($olddownresumewhere);
            //今日免费下载数
            $freedownresumewhere['downtime'] = array(">=",$today);
            $freedownnum = $downresumeM->getFreeDownNum($freedownresumewhere);

            // 总收益
            $owhere['order_state']  =   2;
            $owhere['order_time']   =   array('>=',$today);
            $owhere['type']         =   array('<>', 6);
            $moneyTotal =   $CorderM->getInfo($owhere,array('field'=>'sum(order_price) as `total`'));
            $moneyTotal = $moneyTotal['total'];
            // 会员套餐 收益
            $vwhere['order_state']  =   2;
            $vwhere['order_time']   =   array('>=',$today);
            $vwhere['type']         =   1;
            $moneyVip   =   $CorderM->getInfo($vwhere,array('field'=>'sum(order_price) as `total`'));
            $moneyVip   =   $moneyVip['total'];
            // 增值服务 收益
            $swhere['order_state']  =   2;
            $swhere['order_time']   =   array('>=',$today);
            $swhere['type']         =   5;
            $moneyService   =   $CorderM->getInfo($swhere,array('field'=>'sum(order_price) as `total`'));
            $moneyService   =   $moneyService['total'];
            // 其他收益（除去会员套餐和增值服务，都归为其他收益）
            $Iwhere['order_state']      =   2;
            $Iwhere['order_time']       =   array('>=',$today);
            $Iwhere['PHPYUNBTWSTART_A'] =   '';
            $Iwhere['type'][]           =   array('<>', 1, 'AND');
            $Iwhere['type'][]           =   array('<>', 5, 'AND');
            $Iwhere['type'][]           =   array('<>', 6, 'AND');
            $Iwhere['PHPYUNBTWEND_A']   =   '';
            $moneyIntegral  =   $CorderM->getInfo($Iwhere,array('field'=>'sum(order_price) as `total`'));
            $moneyIntegral  =   $moneyIntegral['total'];

            $res = array(
                'memberNum'                  =>         $memberNum,
                'otmemberNum'                =>         $otmemberNum,
                'userNum'                    =>         $userNum,
                'companyNum'                 =>         $companyNum,
                'resumeNum'                  =>         $resumeNum,
                'useridjobNum'               =>         $useridjobNum,
                'olduseridjobNum'            =>         $olduseridjobNum,
                'oldresumeNum'               =>         $oldresumeNum,
                'jobNum'                     =>         $jobNum,
                'oldjobNum'                  =>         $oldjobNum,
                'moneyTotal'                 =>         !empty($moneyTotal) ? str_replace('.00', '', $moneyTotal) : '0',
                'moneyVip'                   =>         !empty($moneyVip) ? str_replace('.00', '', $moneyVip) : '0',
                'moneyIntegral'              =>         !empty($moneyIntegral) ? str_replace('.00', '', $moneyIntegral) : '0',
                'moneyService'               =>         !empty($moneyService) ? str_replace('.00', '', $moneyService) : '0',
                
                'downresumeNum'              =>         $downresumeNum,
                'olddownresumeNum'           =>         $olddownresumeNum,
                'tellognum'                  =>         $tellognum,
                'oldtellognum'               =>         $oldtellognum,
                'freedownnum'                =>         $freedownnum,
                // 月数据默认值
                'monthMoneyTotal'            =>         0,
                'monthMoneyVip'              =>         0,
                'monthMoneyIntegral'         =>         0,
                'monthMoneyService'          =>         0,
                'monthMemberNum'             =>         0,
                'ommemberNum'                =>         0,
                'userNumMon'                 =>         0,
                'companyNumMon'              =>         0,
            );

            //短信数
            $res['msgnum'] = 0;

            if (!empty($this->config['sy_msg_appkey']) && !empty($this->config['sy_msg_appsecret'])) {

                //短信检测
                $url = 'https://u.phpyun.com/feature';
                $url .= '?appKey=' . $this->config['sy_msg_appkey'] . '&appSecret=' . $this->config['sy_msg_appsecret'];

                if (extension_loaded('curl')) {

                    $return = CurlGet($url);
                } else if (function_exists('file_get_contents')) {

                    $return = file_get_contents($url);
                }

                if ($return) {
                    $msgInfo = json_decode($return, true);
                    if ($msgInfo['code'] == '200') {
                        $res['msgnum'] = $msgInfo['num'];
                    }
                }
            }
        }
        
        $this->render_json(0,'',$res);
    }
    // 月数据统计
    function monthStatis_action(){
        // 拦截没有首页查看统计权限的请求
        if ($this->adminShell['index_lookstatistc'] != 2) {
            $this->render_json(1);
            exit();
        }
        $UserinfoM  =   $this->MODEL('userinfo');
        $ResumeM    =   $this->MODEL('resume');
        $JobM       =   $this->MODEL('job');
        $AdM        =   $this->MODEL('ad');
        
        $downresumeM=   $this->MODEL('downresume');
        
        $date[] =   array('>=', strtotime(date('Y-m-d') .' -1day'));
        $date[] =   array('<', strtotime(date('Y-m-d')));
        if (!empty($_POST['sdate']) && !empty($_POST['edate'])) {
            $sdate = strtotime($_POST['sdate']);
            $edate = strtotime($_POST['edate']. ' 23:59:59');
            $tw = array(
                array('>=', $sdate),
                array('<=', $edate)
            );
            
        } else {
            $month = strtotime(date('Y-m')); // 月初第一天
            $tw = array('>=', $month);
            
        }
        
        $mrewhere['ctime']  =   $tw;
        $resumeNumMon       =   $ResumeM->getExpectNum($mrewhere);
        
        $mjobwhere['sdate'] =   $tw;
        $jobNumMon          =   $JobM->getJobNum($mjobwhere);
        
        $mcomwhere['usertype']  =   2;
        $mcomwhere['reg_date']  =   $tw;
        $companyNumMon          =   $UserinfoM->getMemberNum($mcomwhere);
        
        $muserwhere['usertype'] =   1;
        $muserwhere['reg_date'] =   $tw;
        $userNumMon             =   $UserinfoM->getMemberNum($muserwhere);
        
        $madwhere['addtime']    =   $tw;
        $ggNumMon               =   $AdM->getAdClickNum($madwhere);
        
        $mujobwhere['datetime'] = $tw;
        $userjobNumMon = $JobM->getSqJobNum($mujobwhere);
        
        $myqmswhere['datetime'] = $tw;
        $yqmsNumMon = $JobM->getYqmsNum($myqmswhere);
        
        $mdownresumewhere['downtime'] = $tw;
        $downreusmeNumMon = $downresumeM->getDownNum($mdownresumewhere);
        // 新增 将免费的简历下载 统计到数据中
        $freedownresumeM = $downresumeM->getFreeDownNum($mdownresumewhere);
        $downreusmeNumMon = $freedownresumeM+$downreusmeNumMon;
        
        
        $wxwhere['wxid']  = array('<>','');
        $wxwhere['usertype']  = 1;
        $wxbduserNumMon = $UserinfoM->getMemberNum($wxwhere);
        $wxwhere['usertype']  = 2;
        $wxbdcomNumMon = $UserinfoM->getMemberNum($wxwhere);
        $wxwhere['wxbindtime']  = $tw;
        $wxbdNumMon = $UserinfoM->getMemberNum($wxwhere);

        $userallnum = $UserinfoM->getMemberNum(array('usertype'=>1));
        $comallnum = $UserinfoM->getMemberNum(array('usertype'=>2));

        $userwx_percent = ceil(100*$wxbduserNumMon/$userallnum);
        $comwx_percent = ceil(100*$wxbdcomNumMon/$comallnum);

        $data = array(
            'resumeNumMon'               =>         $resumeNumMon,
            'jobNumMon'                  =>         $jobNumMon,
            'companyNumMon'              =>         $companyNumMon,
            'userNumMon'                 =>         $userNumMon,
            'ggNumMon'                   =>         $ggNumMon,
            'userjobNumMon'              =>         $userjobNumMon,
            'yqmsNumMon'                 =>         $yqmsNumMon,
            'downreusmeNumMon'           =>         $downreusmeNumMon,
            'wxbdNumMon'                 =>         $wxbdNumMon,
            'wxbduserNumMon'             =>         $wxbduserNumMon,
            'wxbdcomNumMon'              =>         $wxbdcomNumMon,
            'userwx_percent'             =>         $userwx_percent,
            'comwx_percent'              =>         $comwx_percent,

        );
        $this->render_json(0,'',$data);
    }
    
    // 网站统计
    function getweb_action(){
        $this->tjl("member","login_log", array('reg_date','ctime'), array('个人注册','个人登录','上月个人注册','上月个人登录'), array('type'=>1));
    }
    function comtj_action(){
        $this->tjl("member","login_log", array('reg_date','ctime'), array('企业注册','企业登录','上月企业注册','上月企业登录'), array('type'=>2));
    }
    function resumetj_action(){
        $this->tj("resume_expect", array('ctime','r_time'), array('简历新增','简历刷新','上月简历新增','上月简历刷新'));
    }
    function jobtj_action(){
        $this->tj("company_job", array('sdate','r_time'), array('职位新增','职位刷新','上月职位新增','上月职位刷新'));
    }
    function ujobtj_action(){
        $this->tj("userid_job", array('datetime'), array('简历投递统计','上月简历投递统计'));
    }
    function yqmstj_action(){
        $this->tj("userid_msg", array('datetime'), array('邀请面试统计','上月邀请面试统计'));
    }
    function downresumetj_action(){
        $this->tj("down_resume", array('downtime'), array('简历下载统计','上月简历下载统计'));
    }
    
    function adtj_action(){
        $this->tj("adclick", array('addtime'), array('广告点击统计','上月广告点击统计'));
    }
    function wxbdtj_action(){
        $this->tjl("member","member", array('wxbindtime','wxbindtime'), array('个人微信绑定','企业微信绑定','上月个人微信绑定','上月企业微信绑定'), array('type'=>1,'type2'=>2,'where'=>array('wxid'=>array('<>',''))));
    }
    function tjl($tablename1, $tablename2, $field = array(), $name = array(), $data = array())
    {
        // 拦截没有首页查看统计权限的请求 
        if ($this->adminShell['index_lookstatistc'] != 2) {
            $list = array();
            $name = array();
        } else {
            $TimeDate = $this->day();
            $sdate = $TimeDate['sdate'];
            $edate = $TimeDate['edate'];
            $days = $TimeDate['days'];
            $initM = empty($_POST['days']);
            $startTime = strtotime($sdate);
            $endTime = strtotime($edate . ' 23:59:59');

            $data['type2'] = !empty($data['type2'])?$data['type2']:$data['type'];

            $tjldata['where'] = !empty($data['where'])?$data['where']:$data['where'];

            if ($field[0]) {
                $list[] = $this->tjlData($tablename1, $data['type'], $field[0], $days, $startTime, $endTime, $initM, $tjldata);
            }
            if ($field[1]) {
                $list[] = $this->tjlData($tablename2, $data['type2'], $field[1], $days, $startTime, $endTime, $initM, $tjldata);
            }
            if ($initM) {
                $lastEndtTime = $startTime - 1;
                isset($field[0]) && $list[] = $this->tjlData($tablename1, $data['type'], $field[0], $days, strtotime(date('Y-m', $lastEndtTime)), $lastEndtTime, true, $tjldata);
                isset($field[1]) && $list[] = $this->tjlData($tablename2, $data['type2'], $field[1], $days, strtotime(date('Y-m', $lastEndtTime)), $lastEndtTime, true, $tjldata);
            } else {
                unset($name[2]);
                unset($name[3]);
            }
        }
        $data['list'] = $list;
        $data['name'] = $name;
        $this->render_json(0,'',$data);
    }
    function tj($tablename, $field = array(), $name = array(), $where = '')
    {
        // 拦截没有首页查看统计权限的请求
        if ($this->adminShell['index_lookstatistc'] != 2) {
            $list = array();
            $name = array();
        } else {
            $TimeDate = $this->day();

            $sdate = $TimeDate['sdate'];
            $edate = $TimeDate['edate'];
            $days = $TimeDate['days'];
            $initM = empty($_POST['days']);
            $startTime = strtotime($sdate);
            $endTime = strtotime($edate . ' 23:59:59');
            $freeDown = [];
            $freeDownTable ='';
            if ($field[0]) {
                $list[] = $this->tjData($tablename, $field[0], $days, $startTime, $endTime, $initM);
                if ($tablename == 'down_resume'){
                    $freeDownTable = 'freedown_resume';
                    $arr = $this->tjData($freeDownTable, $field[0], $days, $startTime, $endTime, $initM);
                    $freeDown[$TimeDate['sdate']] = $arr['list']?$arr['list']:array();
                }
            }
            if ($field[1]) {
                $ntablename = $tablename;
                if ($tablename == 'resume_expect') {
                    $ntablename = 'resume_refresh_log';
                }
                if ($tablename == 'company_job') {
                    $ntablename = 'job_refresh_log';
                    $loginWhere['type'] = 1;
                }

                $list[] = $this->tjData($ntablename, $field[1], $days, $startTime, $endTime, $initM);
            }
            if ($initM) {
                $lastEndtTime = $startTime - 1;
                isset($field[0]) && $list[] = $this->tjData($tablename, $field[0], $days, strtotime(date('Y-m', $lastEndtTime)), $lastEndtTime);
                if (isset($field[0]) && $freeDownTable){
                    $arr = $this->tjData($freeDownTable, $field[0], $days,strtotime(date('Y-m', $lastEndtTime)), $lastEndtTime);
                    $freeDown[date('Y-m', $lastEndtTime)] = $arr['list']?$arr['list']:array();
                }
                isset($field[1]) && $list[] = $this->tjData($ntablename, $field[1], $days, strtotime(date('Y-m', $lastEndtTime)), $lastEndtTime);
            }
            if ($tablename == 'down_resume'){
                $list = $this->setDownResumeData($list,$freeDown);
            }
            $lnum = count($list);
            $nnum = count($name);
            if ($lnum < $nnum) {
                for ($i = $lnum; $i < $nnum; $i++) {
                    unset($name[$i]);
                }
            }
        }
        $data['list'] = $list;
        $data['name'] = $name;
        $this->render_json(0,'',$data);
        
    }
    function tjlData($tablename, $usertype, $field, $days, $sdate, $edate, $initM = true,$data=array())
    {
        $TongjiM = $this->MODEL('tongji');
        $where = !empty($data['where'])?$data['where']:array();
        $where['usertype'] = $usertype;
        $field == 'reg_date' && $where['pid'] = 0;
        $where[$field][] = array(">=", $sdate);
        $where[$field][] = array("<=", $edate);

        $where['groupby'] = 'td';
        $where['orderby'] = array('td,desc');
        $statistics = $TongjiM->tjtotal($tablename, $where, array('field' => 'FROM_UNIXTIME('.$field.',"%Y%m%d") as td, count(*) as cnt'));

        $list = $dateList = array();

        if (is_array($statistics)) {
            $allNum = 0;

            foreach ($statistics as $key => $value) {
                $allNum += $value['cnt'];
                $dateList[$value['td']] = $value;
            }

            if ($days > 0) {

                if ($initM) { // 每月1号开始
                    $d = (int)date('d', $edate);

                    if ($d == 1 || $d == '01') {// 月初第一天
                        $y = date('Y', $edate);
                        $m = date('m', $edate);
                        // 前一天数据单独单独处理
                        $days = 0;
                        $lastday = strtotime('-1 day');
                        $ly = date('Y', $lastday);
                        $lm = date('m', $lastday);
                        $ld = (int)date('d', $lastday);
                        $londay = $ly . $lm . $ld;
                        $ltd = "{$lm}-{$ld}";
                        $ldate = "{$ly}-{$lm}-{$ld}";
                        if (!empty($dateList[$londay])) {
                            $dateList[$londay]['td'] = $ltd;
                            $dateList[$londay]['date'] = $ldate;

                            $list[$londay] = $dateList[$londay];
                        } else {
                            $list[$londay] = array('td' => $ltd, 'cnt' => 0, 'date' => $ldate);
                        }
                    } else {
                        $y = date('Y', $sdate);
                        $m = date('m', $sdate);
                    }

                    // for ($i = 1; $i <= $d && $i <= $days + 1; $i++) {
                    for ($i = 1; $i <= $days + 1; $i++) {
                        $ds = $i < 10 ? '0' . $i : $i;
                        $onday = $y . $m . $ds;
                        $td = "{$m}-{$ds}";
                        $date = "{$y}-{$m}-{$ds}";

                        if (!empty($dateList[$onday])) {
                            $dateList[$onday]['td'] = $td;
                            $dateList[$onday]['date'] = $date;

                            $list[$onday] = $dateList[$onday];
                        } else {
                            $list[$onday] = array('td' => $td, 'cnt' => 0, 'date' => $date);
                        }
                    }
                } else { // 开放区间
                    for ($i = 0; $i <= $days; $i ++) {
                        $onday  =   date("Ymd", strtotime(' -' . $i . 'day', $edate));
                        $td     =   date('m-d', strtotime(' -' . $i . 'day', $edate));
                        $date   =   date('Y-m-d', strtotime(' -' . $i . 'day', $edate));

                        if (! empty($dateList[$onday])) {
                            $dateList[$onday]['td']     =   $td;
                            $dateList[$onday]['date']   =   $date;
                            $list[$onday]           =   $dateList[$onday];
                        } else {
                            $list[$onday]           =   array('td' => $td,'cnt' => 0,'date' => $date);
                        }
                    }
                }
            }
        }
        ksort($list);

        return array('count' => $allNum, 'list' => $list);
    }
    function tjData($tablename, $field, $days, $sdate, $edate, $initM = true)
    {
        $TongjiM = $this->MODEL('tongji');
        $where[$field][]    =   array(">=", $sdate);
        $where[$field][]    =   array("<=", $edate);
        $where['groupby']   =   'td';
        $where['orderby']   =   array('td,desc');

        $statistics = $TongjiM->tjtotal($tablename, $where, array('field' => 'FROM_UNIXTIME(' . $field . ',"%Y%m%d") as td,count(*) as cnt'));

        $list = $dateList = array();

        if (is_array($statistics)) {
            $allNum = 0;

            foreach ($statistics as $key => $value) {
                $allNum += $value['cnt'];
                $dateList[$value['td']] = $value;
            }

            if ($days > 0) {
                if ($initM) { // 每月1号开始
                    $d = (int)date('d', $edate);

                    if ($d == 1 || $d == '01') {// 月初第一天
                        $y = date('Y', $edate);
                        $m = date('m', $edate);
                        // 前一天数据单独单独处理
                        $days = 0;
                        $lastday = strtotime('-1 day');
                        $ly = date('Y', $lastday);
                        $lm = date('m', $lastday);
                        $ld = (int)date('d', $lastday);
                        $londay = $ly . $lm . $ld;
                        $ltd = "{$lm}-{$ld}";
                        $ldate = "{$ly}-{$lm}-{$ld}";
                        if (!empty($dateList[$londay])) {
                            $dateList[$londay]['td'] = $ltd;
                            $dateList[$londay]['date'] = $ldate;

                            $list[$londay] = $dateList[$londay];
                        } else {
                            $list[$londay] = array('td' => $ltd, 'cnt' => 0, 'date' => $ldate);
                        }
                    } else {
                        $y = date('Y', $sdate);
                        $m = date('m', $sdate);
                    }

                    // for ($i = 1; $i <= $d && $i <= $days + 1; $i++) {
                    for ($i = 1; $i <= $days + 1; $i++) {
                        $ds = $i < 10 ? '0' . $i : $i;
                        $onday = $y . $m . $ds;
                        $td = "{$m}-{$ds}";
                        $date = "{$y}-{$m}-{$d}";

                        if (!empty($dateList[$onday])) {
                            $dateList[$onday]['td'] = $td;
                            $dateList[$onday]['date'] = $date;

                            $list[$onday] = $dateList[$onday];
                        } else {
                            $list[$onday] = array('td' => $td, 'cnt' => 0, 'date' => $date);
                        }
                    }
                } else { // 开放区间
                    for ($i = 0; $i <= $days; $i ++) {
                        $onday  =   date("Ymd", strtotime(' -' . $i . 'day', $edate));
                        $td     =   date('m-d', strtotime(' -' . $i . 'day', $edate));
                        $date   =   date('Y-m-d', strtotime(' -' . $i . 'day', $edate));

                        if (! empty($dateList[$onday])) {
                            $dateList[$onday]['td']     =   $td;
                            $dateList[$onday]['date']   =   $date;
                            $list[$onday]           =   $dateList[$onday];
                        } else {
                            $list[$onday] = array('td' => $td,'cnt' => 0,'date' => $date);
                        }
                    }
                }
            }
        }
        ksort($list);

        return array('count' => $allNum, 'list' => $list);
    }
    // 时间获取
    function day(){

        if ((int) $_POST['days'] > 0) {

            $days   =   (int) $_POST['days'];
            $sdate  =   date('Y-m-d', (time() - $days * 86400));
            $edate  =   date('Y-m-d');

        } elseif ($_POST['sdate']) {

            $sdate  =   $_POST['sdate'];
            $days   =   ceil(abs(time() - strtotime($sdate)) / 86400);

            if ($_POST['edate']) {

                $edate = strtotime($_POST['edate'])>time()?date('Y-m-d'):$_POST['edate'];

                $days   =   ceil(abs(strtotime($edate) - strtotime($sdate)) / 86400);
            }

            $days   =   $days == 0 ? 1 : $days;

        } else {
            
            $days   =   intval(date('d')) - 1;
            
            $days   =   $days == 0 ? 1 : $days;
            
            $sdate  =   date('Y-m-d', (time() - $days * 86400));
            $edate  =   date('Y-m-d');
        }
        return array('sdate' => $sdate,'days' => $days,'edate' => $edate);
    }
    public function setDownResumeData($resume,$freeResume){
        foreach ($resume as $key =>$value){
            $data = $value['list'];
            foreach ($data as $kd => &$vd){
                $keydate = date('Y-m',strtotime($vd['date']));
                $freeData = $freeResume[$keydate];
                $vd['cnt']= $vd['cnt'] + $freeData[$kd]['cnt'];
            }
            $resume[$key]['count'] = array_sum(array_map(function($val){return $val['cnt'];}, $data));
            $resume[$key]['list'] = $data;
        }
        return $resume;
    }
}
?>