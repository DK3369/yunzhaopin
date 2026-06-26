<?php


class wxapp_controller extends common
{

    public $comInfo     =   array();
    public $platform    =   '';
    public $plat        =   '';
    public $port        =   2;

    function __construct($tpl, $db, $def = '', $model = 'index', $m = '')
    {
        $_POST = $this->undefinedToEnpty($_POST);
        $this->common($tpl, $db, $def, 'wxapp');


        if (isset($_GET['h']) && ($_GET['h'] == 'user' || $_GET['h'] == 'com')) {
            // 
            $this->yzTokenNew(1, $_POST['uid'], $_POST['token']);
            
        }else if ($_POST['uid'] && $_POST['token'] && !in_array($_GET['c'], array('talentpool','invitesave'))){
            // 
            $this->yzTokenNew(0, $_POST['uid'], $_POST['token']);
        }
    }
    /**
     * @desc Render unified JSON response
     * @param string $error result code
     * @param string $msg result message
     * @param array $data payload
     * @param int $total
     */
    public function render_json($error = '', $msg = '', $data = array(), $total = 0)
    {

        $data   =   $this->nullToEnpty($data);

        if (is_string($msg) && $msg !== '' && preg_match('/[\x{4e00}-\x{9fff}]/u', $msg)) {
            $msg = yun_auto_t($msg);
        }
        if (function_exists('yun_auto_array') && is_array($data) && $data) {
            $data = yun_auto_array($data);
        }

        $result =   array(
            'error' =>  $error,
            'msg'   =>  isset($msg) ? preg_replace('/\([^\)]+?\)/x', "", str_replace(array("（", "）"), array("(", ")"), $msg)) : '',
            'data'  =>  isset($data) ? $data : array(),
            'total' =>  $total
        );
        header('content-type:application/json; charset=utf-8');
        echo json_encode($result);
        exit;
    }

    function yzToken($uid = '', $token = '')
    {
        if(empty($this->member)){
            // ，IDuid
            $this->yzTokenNew(0, $uid, $token);
        }
        return $this->member;
    }
    
    /**
     * @param string $uid
     * @param string $token
     * @return array
     */
    function yzTokenNew($isMember = 0, $uid = '', $token = '')
    {
        if (!empty($this->uid) && !empty($this->usertype)){
            // wap
            $this->member  =  array(
                'uid'       =>  $this->uid,
                'username'  =>  $this->username,
                'usertype'  =>  $this->usertype,
                'did'       =>  $this->userdid,
                'app_push'  =>  ''
            );
            $this->wxappMember(array('uid'=>$this->uid), $isMember);
            
        }elseif (!empty($uid) && !empty($token)){
            // 
            $this->wxappMember(array('uid'=>$uid,'token'=>$token), $isMember);
            
        }elseif($_GET['c']!='advice'){
            $this -> render_json(1002, yun_at('wap_00376'));
        }
    }
    // 
    private function wxappMember($param = array(), $isMember = 0){
        
        $field = '`uid`,`username`,`usertype`,`password`,`salt`,`pid`,`did`,`status`,`login_date`,`subscribe`,`wxid`,`login_ip`,`login_address`';
        // APP，
        if (isset($this->config['sy_push_open']) && $this->config['sy_push_open'] == 1){
            $field .= ',`app_push`';
        }
        $userInfoM  =  $this->MODEL('userinfo');
        $member     =  $userInfoM->getInfo(array('uid'=>$param['uid']),array('field'=>$field));
        $user  =  array(
            'uid'       =>  $member['uid'],
            'username'  =>  $member['username'],
            'usertype'  =>  $member['usertype'],
            'did'       =>  $member['did'],
            'status'    =>  $member['status'],
            'wxid'      =>  $member['wxid'],
            'subscribe' =>  $member['subscribe'],
            'gzhtoken'  =>  $this->generateToken('gzh', $member['uid'], $member['password'])
        );
        if (empty($this->member)){
            $this->member = $user;
        }else{
            $this->member['status']   = $user['status'];
            $this->member['subscribe']= $user['subscribe'];
            $this->member['wxid']     = $user['wxid'];
            $this->member['gzhtoken'] = $user['gzhtoken'];
        }
        if ($member){
            if ($member['status'] == 2){
                
                $logoutM  =  $this->MODEL('logout');
                $logout	  =	 $logoutM->getInfo(array('uid'=>$param['uid'],'status'=>1));
                
                if (!empty($logout)){
                    $this -> render_json(1002, yun_at('wap_01861'));
                }else{
                    $this -> render_json(1002, yun_at('wap_01862'));
                }
            }
            // token。tokenwap，common
            if (!empty($param['token'])){
                $mdtoken  =  md5($member['username'].$member['password'].$member['salt'].$member['usertype']);
                if($param['token'] != $mdtoken){
                    
                    $this -> render_json(1002, yun_at('wap_01863'));
                }
            }
            // ，
            if ($isMember == 1){
                if (($member['usertype'] == 2 && $_GET['h']=='com') || ($member['usertype'] == 1 && $_GET['h']=='user')) {
                    
                    if ($member['usertype'] == 2) {
                        
                        $comM = $this->MODEL('company');
                        
                        $this->comInfo = $comM->getInfo($user['uid'], array('logo' => 1, 'utype' => 'user'));
                        if (!empty($this->config['com_package_open']) && empty($this->comInfo['package'])) {
                            $packageOpenArr = explode(',', $this->config['com_package_open']);
                            if (in_array($this->comInfo['rating'], $packageOpenArr) || ($this->comInfo['vipetime'] > 0 && $this->comInfo['vipetime'] < time() && in_array('999', $packageOpenArr))) {

                                $this->comInfo['noPermission'] = 1;
                            }
                        }
                        $this->comInfo = !empty($this->comInfo) ? $this->comInfo : array();

                        if (empty($this->comInfo)) {
                            
                            $userInfoM->activUser($user['uid'], 2);
                        }
                    } elseif ($member['usertype'] == 1) {
                        
                        $resumeM = $this->MODEL('resume');
                        
                        $resume = $resumeM->getResumeInfo(array('uid' => $member['uid']), array('field' => '`uid`'));
                        
                        if (empty($resume)) {
                            
                            $userInfoM->activUser($member['uid'], 1);
                        }
                    }
                }else{
                    $this -> render_json(1003, yun_at('wap_01864'));
                }
            }
            // ，。，
            if ($member['login_date'] < strtotime('today')){
                $needlog = true;
                $get_m = isset($_GET['m']) ? $_GET['m'] : '';
                $get_c = isset($_GET['c']) ? $_GET['c'] : '';
                $get_h = isset($_GET['h']) ? $_GET['h'] : '';
                
                if (in_array($get_m, array('chat','version','public'))
                    || (!empty($get_h) && $get_m != 'index')
                    || ($get_m == 'index' && $get_c != 'index')
                    || ($get_m == 'job' && $get_c == 'jobShowOther')
                    || ($get_m == 'company' && $get_c == 'comShowOther')
                    || ($get_m == 'company' && $get_c == 'getBusinessInfo')){
                        $needlog = false;
                }
                if ($member['usertype'] > 0 && $needlog){
                    // ，
                    $time  =  time();
                    $ip    =  fun_ip_get();
                    
                    $logindata  =  array(
                        'uid'      => $user['uid'],
                        'usertype' => $user['usertype'],
                        'content'  => yun_auto_t('wap端口延续登录')
                    );
                    $logM = $this -> MODEL('log');
                    $logM->addLoginlog($logindata, array('continue' => 1));
                    $upLogin = array(
                        'login_ip' => $ip,
                        'login_date'=>$time
                    );
                    if ($member['login_ip'] != $ip || $member['login_address'] =='') {
                        $ip_address = getIpAddress($ip);
                        $upLogin['login_address'] = $ip_address;
                    }
                    $userInfoM->upInfo(array('uid' => $user['uid']),$upLogin);
                    // 、
                    if ($member['usertype'] == 1){
                        
                        $rData    = array('login_date' => $time);
                        $resumeM  =  $this -> MODEL('resume');
                        // ，
                        if ($this->config['resume_sx'] == 1) {
                            $expect   =  $resumeM->getExpectByUid($member['uid'], array('field' => '`id`'));
                            if (!empty($expect)) {
                                $rData['lastupdate'] = $time;
                            }
                        }
                        $resumeM->upResumeInfo(array('uid' => $member['uid']), array('rData' => $rData, 'port' => $this->port));
                    }elseif ($member['usertype'] == 2){
                        
                        if (!isset($comM)){
                            $comM = $this->MODEL('company');
                        }
                        $comM->upInfo($member['uid'], array('login_date' => $time));
                    }
                }
            }
        }else{
            $this -> render_json(1002, yun_at('wap_01865'));
        }
    }

    function getBdOpenid($code){
        include(dirname(dirname(dirname(__FILE__))).'/data/api/baidu/baidu_data.php');
        $appKey = $baiduData['sy_bdlogin_appKey'];
        $sk = $baiduData['sy_bdlogin_appSecret'];
        $token_url = 'https://spapi.baidu.com/oauth/jscode2sessionkey?code='.$code.'&client_id='.$appKey.'&sk='.$sk;
        if(function_exists('curl_init')) {

            $result  =  CurlGet($token_url);
            $user    = json_decode($result,true);

            $user['appid']  =  $appKey;
            return $user;
        }else{
            $this->render_json(1005, yun_auto_t('不支持curl'));
        }
    }

	
	function fktype()
	{
	    $fktype  =  array(
	        'goumai' => yun_at('member_user_00285'),
	        'fuhao'  => '￥',
	        'fkjg'   => yun_at('wap_00563'),
	        'wxsrc'  => $this->config['sy_weburl'].'/api/wxapp/static/image/wxzf.png',
	        'alsrc'  => $this->config['sy_weburl'].'/api/wxapp/static/image/zfb.png',
	    );

	    if($this->config['alipay']=='1' &&  $this->config['alipaytype']=='1'){
	        $fktype['fkal']  =  yun_at('wap_user_00319');
	    }
	    return $fktype;
	}
	function preghtml($str){
	    $return  =  strip_tags($str,'<div> <p> <img> <br>');
	    $return = preg_replace("/<div[^>]*?>(.*?)<\/div>/is","<div>$1</div>",$return);
	    $return = preg_replace("/<p[^>]*?>(.*?)<\/p>/is","<p>$1</p>",$return);

	    return $return;
	}
	/**
	 * @param array $arr
	 * @return array
	 */
	function undefinedToEnpty($arr = array()){
	    
	    if (!empty($arr)){
	        
	        foreach ($arr as $k=>$v){
	            
	            if (is_array($v)){
	                
	                $arr[$k]  =  $this->undefinedToEnpty($v);
	                
	            }elseif ($v == 'undefined'){
	                
	                $arr[$k] = '';
	            }
	        }
	    }
	    return $arr;
	}
	/**
	 * @param array $arr
	 * @return array
	 */
	function nullToEnpty($arr = array()){
	    
	    if (!empty($arr)){
	        
	        foreach ($arr as $k=>$v){
	            
	            if (is_null($v)){
	                
	                $arr[$k] = '';
	                
	            }elseif (is_array($v)){
	                
	                $arr[$k]  =  $this->nullToEnpty($v);
	            }
	        }
	    }
	    return $arr;
	}
	/**
	 * @param mixed $did
	 */
	function getDomain($did, $needCache = FALSE){
	    
	    $fz_type = 0;
	    
	    include(PLUS_PATH.'domain_cache.php');
	    include(PLUS_PATH.'cityparent.cache.php');
	    foreach ($site_domain as $v){
	        if($v['id'] == $did){
	            if ($v['fz_type'] == 1){
	                
	                $fz_type  =  1;
	                
	                if(!empty($v['province'])){
	                    $return['provinceid'] = $v['province'];
	                }
	                if(!empty($v['cityid'])){
	                    $return['cityid']     = $v['cityid'];
	                    $return['provinceid'] = $city_parent[$return['cityid']];
	                }
	                if(!empty($v['three_cityid'])){
	                    $return['three_cityid'] = $v['three_cityid'];
	                    $return['cityid']       = $city_parent[$return['three_cityid']];
	                    $return['provinceid']   = $city_parent[$return['cityid']];
	                }
	            }elseif ($v['fz_type'] == '2'){
	                
	                $fz_type  =  2;
	                
	                if ($v['hy']){
	                    $return['hyclass']  =  $v['hy'];
	                }
	            }
	        }
	    }
	    if ($needCache){
	        
	        if ($fz_type == 1){
	            // ，
	            $cacheM		=  $this->MODEL('cache');
	            $cacheList	=  $cacheM->GetCache('city');
	            $city_index	=  $cacheList['city_index'];
	            $city_type	=  $cacheList['city_type'];
	            $city_name	=  $cacheList['city_name'];
	            
	            $didcity    =  $city_name[$return['provinceid']];
	            $cityone[]  =  array('value'=>$return['provinceid'],'label'=>$city_name[$return['provinceid']]);
	            
	            if (!empty($return['cityid'])){
	                // 2
	                $didcity    =  $city_name[$return['cityid']];
	                
	                $citytwo[0][]  =  array('value'=>$return['cityid'],'label'=>$city_name[$return['cityid']]);
	                
	            }elseif(!empty($return['provinceid']) && empty($return['cityid'])){
	                // 1，2
	                foreach ($city_type[$return['provinceid']] as $v){
	                    
	                    $citytwo[0][]  =  array('value'=>$v,'label'=>$city_name[$v]);
	                }
	            }
	            if (!empty($return['three_cityid'])){
	                // 3
	                $didcity      =  $city_name[$return['three_cityid']];
	                
	                $citythree[0][0][]  =  array('value'=>$return['three_cityid'],'label'=>$city_name[$return['three_cityid']]);
	                
	            }elseif(!empty($return['cityid']) && empty($return['three_cityid'])){
	                // 2，3
	                foreach ($city_type[$return['cityid']] as $v){
	                    
	                    $citythree[0][0][]  =  array('value'=>$v,'label'=>$city_name[$v]);
	                }
	            }
	            
	            $return['didcity']    =  $didcity;
	            $return['cityone']    =  !empty($cityone) ? $cityone : array();
	            $return['citytwo']    =  !empty($citytwo) ? $citytwo : array();
	            $return['citythree']  =  !empty($citythree) ? $citythree : array();
	            $return['city_name']  =  $city_name;
	            
	        } elseif ($fz_type == 2){
	            
	            $cacheM		=  $this->MODEL('cache');
	            $cacheList	=  $cacheM->GetCache('hy');
	            $industry_name  =  $cacheList['industry_name'];
	            
	            if ($return['hyclass']){
	                
	                $return['didhy']  =  $industry_name[$return['hyclass']];
	                
	                $return['hydata']  =  array(
	                    'id'    =>  array($return['hyclass']),
	                    'name'  =>  array($return['didhy'])
	                );
	            }
	        } 
	    }
	    
	    return $return;
	}
	function listCity($search_cityid = '', $search_threecityid = ''){
	    
	    $return = array();
	    
	    if (!empty($this->config['sy_web_city_one'])) {
	        $return['provinceid']  =  $this->config['sy_web_city_one'];
	    }
	    if (!empty($this->config['sy_web_city_two'])) {
	        $return['cityid']  =  $this->config['sy_web_city_two'];
	    }
	    // 
	    if (!empty($search_cityid)){
	        $return['cityid']  =  $search_cityid;
	    }
	    // 
	    if (!empty($search_threecityid)){
	        $return['three_cityid']  =  $search_threecityid;
	    }
	    if (!empty($return)){
	        $cacheM		=  $this->MODEL('cache');
	        $cacheList	=  $cacheM->GetCache(array('city','cityfs'));
	        $city_index	=  $cacheList['city_index'];
	        $city_type	=  $cacheList['city_type'];
	        $city_name	=  $cacheList['city_name'];
	        $city_three =  $cacheList['city_three'];
	        
	        $listcity   =  $city_name[$return['provinceid']];
	        $cityone[]  =  array('value'=>$return['provinceid'],'label'=>$city_name[$return['provinceid']]);
	        $citytwo    =  $citythree  =  array();
	        
	        
	        if(!empty($this->config['sy_web_city_one']) && empty($this->config['sy_web_city_two'])){
	            // 
	            $provinceid        =  $this->config['sy_web_city_one'];
	            $citytwo[0][]      =  array('value'=>0,'label'=>yun_at('wap_js_00075'));// Column 2: all
	            $citythreetwoArr[$provinceid][]	=  array(array());// level-1 all placeholder
	            foreach ($city_type[$provinceid] as $v){
	                
	                $citytwo[0][]  =  array('value'=>$v,'label'=>$city_name[$v]);
	                if (is_array($city_type[$v]) && !empty($city_three)){
	                    $citythreeArr  =  array();
	                    $citythreeArr[] =  array('value'=>0,'label'=>yun_at('wap_js_00075'));
	                    foreach ($city_type[$v] as $ka=>$va){
	                        $citythreeArr[]  =	array('value'=>$va,'label'=>$city_name[$va]);
	                    }
	                    $citythreetwoArr[$provinceid][]   =	$citythreeArr;
	                }
	            }
	            if (!empty($city_three)){
	                $citythree	=  array_values($citythreetwoArr);
	            }
	        }
	        
	        if (!empty($this->config['sy_web_city_two'])) {
	            // 
	            $cityid        =  $this->config['sy_web_city_two'];
	            $citytwo[0][]  =  array('value'=>$cityid,'label'=>$city_name[$cityid]);
	            if (!empty($city_three)){
	                // ，3
	                $citythree[0][0][]  =  array('value'=>0,'label'=>yun_at('wap_js_00075'));// Column 3: all
	                foreach ($city_type[$cityid] as $v){
	                    
	                    $citythree[0][0][]  =  array('value'=>$v,'label'=>$city_name[$v]);
	                }
	            }
            }
            if (!empty($return['cityid'])) {
                $listcity  =  $city_name[$return['cityid']];
            }
            if (!empty($return['three_cityid'])) {
                $listcity  =  $city_name[$return['three_cityid']];
            }
	        $return['listcity']   =  $listcity;
	        $return['cityone']    =  !empty($cityone) ? $cityone : array();
	        $return['citytwo']    =  !empty($citytwo) ? $citytwo : array();
	        $return['citythree']  =  !empty($citythree) ? $citythree : array();
	    }
	    return $return;
	}
	function checkMcsdk($moblie = '')
	{
	    if(empty($moblie)){
	        $this->render_json(-1, yun_at('wap_01866'));
	    }
	    $mcsdk = $_SERVER['HTTP_MCSDK'];
	    if (empty($mcsdk)){
	        $this -> render_json(-1, yun_at('wap_01867'));
	    }else{
	        $phone = '';
	        if (isset($_SERVER['HTTP_TIMEOFFSET'])){
	            // ，
	            $time = $this->bytimezone($_SERVER['HTTP_TIMEOFFSET']);
	            $day = date('j', $time);
	        }else{
	            $day = date('j');
	        }
	        
	        if ($this->plat == 'mini'){
	            
	            $openssl  = new OpensslCrypt($this->xcxKey, $this->xcxPy);
	            $decrypt  = $openssl->miniDecrypt($mcsdk);
	            $phone = str_replace($this->xcxShell.$day,'',$decrypt);
	            
	        }elseif ($this->plat == 'app'){
	            
	            $openssl  = new OpensslCrypt($this->appKey, $this->appPy);
	            $decrypt  = $openssl->miniDecrypt($mcsdk);
	            $phone = str_replace($this->appShell.$day,'',$decrypt);
	            
	        }
	        if (!empty($moblie) && $phone != $moblie){
	            $this -> render_json(-1, yun_at('wap_01868'));
	        }
	    }
	}
	public function randomArr($data, $random){
	    if ($random && count($data) > $random) {
	        $temp = [];
	        $random_keys = array_rand($data, $random);
	        
	        if($random == 1) {
	            $temp[] = $data[$random_keys];
	        } else {
	            foreach ($data as $key => $value) {
	                if (in_array($key, $random_keys)) {
	                    $temp[$key] = $value;
	                }
	            }
	        }
	        $data = $temp;
	    }
	    
	    return  $data;
	}
	// 
	public function ad_wxapp($param = array(), $ad_label = null, $randomNum = 0){
	    
	    $did  = !empty($_POST['did']) ? $_POST['did'] : 0;
	    
	    if (!empty($param['class_id'])){
	        $adid = (int)$param['class_id'];
	    }elseif (!empty($param['name'])){
	        // 
	        $flagArray = [
	            'job_show'    => 512,
	            'resume'      => 505,
	            'job'         => 504,
	            'popup_ad'    => 502,
	            'com_center'  => 501,
	            'user_center' => 500
	        ];
	        $adid = $flagArray[$param['name']];
	    }
	    $adpics = array();
	    
	    if (!empty($adid)){
	        if (!isset($ad_label)){
	            include PLUS_PATH.'pimg_cache.php';
	        }
	        $time  =  time();
	        foreach ($ad_label[$adid] as $k=>$v){
	            // 、、、app
	            if($v['type']=='pic' && $v['start']<$time && $v['end']>$time && ($v['did'] == -1 || $v['did'] == $did) && !($this->plat == 'app' && stripos($v['appurl'],'jiqiren') > -1)){
	                $appad = array('pic_url'=>$v['pic'],'src'=>$v['pic']);
	                if (!empty($v['appurl'])){
	                    // ，/，
	                    $appad['appurl']  =  stripos($v['appurl'],'/') == 0 ? $v['appurl'] : '/'.$v['appurl'];
	                }else{
	                    $appad['appurl']  =  '';
	                }
	                
	                $adpics[]  =  $appad;
	            }
	        }
	        
	        // 
	        if ($adpics && $randomNum) {
	            $adpics = $this->randomArr($adpics, $randomNum);
	        }
	    }
	    return $adpics;
	}
	function decryptRequest($str = '')
	{
	    $res = '';
	    if (isset($_SERVER['HTTP_TIMEOFFSET'])){
	        // ，
	        $time = $this->bytimezone($_SERVER['HTTP_TIMEOFFSET']);
	        $day = date('j', $time);
	    }else{
	        $day = date('j');
	    }
	    
	    if ($this->plat == 'mini'){
	        
	        $openssl  = new OpensslCrypt($this->xcxKey, $this->xcxPy);
	        $decrypt  = $openssl->miniDecrypt($str);
	        $res = str_replace($this->xcxShell.$day,'',$decrypt);
	        
	    }elseif ($this->plat == 'app'){
	        
	        $openssl  = new OpensslCrypt($this->appKey, $this->appPy);
	        $decrypt  = $openssl->miniDecrypt($str);
	        $res = str_replace($this->appShell.$day,'',$decrypt);
	        
	    }
	    return $res;
	}
	// （）
	private function generateToken($type, $uid, $password = '')
	{
	    // ，tokenSaltpassword，token
	    $password = substr($password, 0, 8);
	    
	    $this->tokenSalt = $this->config['sy_safekey'];
	    
	    return yunEncrypt("{$type}|{$uid}|{$password}", $this->tokenSalt);
	}
	function bytimezone($lcoaloffset){
	    // ，，。jstimeoffset，。
	    // jstimeoffset（php）。
	    $timeoffset = $lcoaloffset * -1 * 60;
	    // 
	    $serverdate = new DateTime();
	    $serveroffset = $serverdate->getOffset();
	    if($timeoffset > 0){
	        // 
	        $time = time() + ($timeoffset - abs($serveroffset));
	    }else{
	        // 
	        if($serveroffset > 0){
	            // 
	            $time = time() - (abs($timeoffset) + $serveroffset);
	        }else{
	            // 
	            $time = time() + (abs($timeoffset) + abs($serveroffset));
	        }
	    }
	    return $time;
	}
}
?>
