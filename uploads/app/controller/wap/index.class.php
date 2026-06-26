<?php



class index_controller extends common
{
    
    function index_action()
    {
        
        if ($this->uid && $this->usertype == '1') {

            $resumeM    =   $this->MODEL('resume');

            $expect     =   $resumeM->getExpectByUid($this->uid, array('field' => 'id,status'));
            if (!empty($expect)) {

                $user_resume    =   $resumeM->getUserResumeInfo(array('uid' => $this->uid, 'eid' => $expect['id']), array('field' => '`skill`,`work`,`project`,`edu`,`training`'));
                $resume_yhnum   =   0;
                foreach ($user_resume as $rk => $rv) {
                    if ($rv == 0) {
                        $resume_yhnum++;
                    }
                }
                $this->yunset('resume_yhnum', $resume_yhnum);
            }
            $this->yunset('expect', $expect);
        }

        if ($this->config['sy_web_city_one']) {
            $_GET['provinceid'] = 	$this->config['sy_web_city_one'];
        }
        if ($this->config['sy_web_city_two']) {
            $_GET['cityid'] 	= 	$this->config['sy_web_city_two'];
        }

        $this->get_moblie();

        if ($this->config["did"]) {

            $this->seo("index", $this->config['sy_webtitle'], $this->config['sy_webkeyword'], $this->config['sy_webmeta']);
        } else {

            $this->seo('index');
        }
        $this->yunset('indexnav', 1);

        $annM = $this->MODEL('announcement');
        $annum = $annM->getNum();
        $this->yunset('annum', $annum);
        // 
        $categoryM	=	$this -> MODEL('category');
        $hotclass = $categoryM->getHotJobClass(array('rec'=>1,'orderby'=>'sort,desc'),'`id`,`keyid`,`name`');
        
        $this->yunset('hotclass', $hotclass);
        // 
        $bannerFlag   =   $_COOKIE['wap_bannerFlag'];
        if (!$bannerFlag) {
            $this->cookie->setcookie('wap_bannerFlag', 1, time() + 3600);
        }
        $this->yunset("bannerFlag", $bannerFlag);
        
        $this->yuntpl(array('wap/index'));
         
    }

    
    function loginout_action()
    {
        $this->cookie->unset_cookie();
        $this->wapheader('');
    }

    
    function langpack_action()
    {
        $keys = null;
        if (!empty($_GET['keys'])) {
            $keys = array_filter(array_map('trim', explode(',', $_GET['keys'])));
        }

        header('Content-Type: application/json; charset=utf-8');
        echo yun_i18n_langpack_json($keys);
        exit;
    }

    // 
    function about_action()
    {

        $descM      =   $this->MODEL('description');
        $content    =   $descM->getDes(array('name' => yun_at('wap_00218')), array('field' => 'content'));
        $this->yunset('content', $content);
        if ($_GET['fr'] == 'wxapp') {
            $this->yunset('wxapp', 1);
        }
        $this->yunset('headertitle', yun_at('wap_00218'));
        $this->yunset('title', yun_at('wap_00218'));
        $this->yuntpl(array('wap/about'));
    }

    // 
    function contact_action()
    {
        $descM      =   $this->MODEL('description');
        $content    =   $descM->getDes(array('name' => yun_at('wap_00220')), array('field' => 'content'));
        $this->yunset('content', $content);
        if ($_GET['fr'] == 'wxapp') {
            $this->yunset('wxapp', 1);
        }
        $this->yunset('headertitle', yun_at('wap_00220'));
        $this->yunset('title', yun_at('wap_00220'));
        $this->yuntpl(array('wap/about'));
    }

    // app
    function appDown_action()
    {
        if (preg_match("/(iphone|ipod|ipad)/i", strtolower($_SERVER['HTTP_USER_AGENT']))){
            // 
            include(DATA_PATH . 'api/wxapp/app.config.php');
            if (is_weixin()){
                // 
                $down = array(
                    'qrcode' => $this->config['sy_ossurl'] .'/' .$this->config['sy_iosu_qcode']
                );
            }else{
                // 
                if (!empty($appconfig['iosurl'])) {
                    // appstore
                    $down['url'] = $appconfig['iosurl'];
                }
            }
            $down['version'] = $appconfig['iosversion'];
        }else{
            // 
            include(DATA_PATH . 'api/wxapp/app.config.php');
            if (is_weixin()){
                // 
                $down = array(
                    'qrcode' => $this->config['sy_ossurl'] .'/' .$this->config['sy_androidu_qcode']
                );
            }else{
                // 
                include(DATA_PATH . 'api/wxapp/app.config.php');
                if (!empty($appconfig['androidurl'])) {
                    $down['url'] = $appconfig['androidurl'];
                }
            }
            $down['version'] = $appconfig['androidversion'];
        }
        $this->yunset('down', $down);
        $this->yunset('headertitle', yun_at('wap_00217'));
        $this->yuntpl(array('wap/appdown'));
    }

    // 
    function privacy_action()
    {
        $descM      =   $this->MODEL('description');
        $content    =   $descM->getDes(array('name' => yun_at('wap_00313')), array('field' => 'content'));
        $this->yunset('content', $content);
        if ($_GET['fr'] == 'wxapp') {
            $this->yunset('wxapp', 1);
        }
        $this->yunset('headertitle', yun_at('wap_00313'));
        $this->yunset('title', yun_at('wap_00313'));
        $this->yuntpl(array('wap/about'));
    }

    // 
    function protocol_action()
    {
        $descM      =   $this->MODEL('description');
        $content    =   $descM->getDes(array('name' => yun_at('wap_00219')), array('field' => 'content'));
        $this->yunset('content', $content);
        if ($_GET['fr'] == 'wxapp') {
            $this->yunset('wxapp', 1);
        }
        $this->yunset('headertitle', yun_at('wap_00498'));
        $this->yunset('title', yun_at('wap_00498'));
        $this->yuntpl(array('wap/about'));
    }

     
    // 
    function getmq_action(){
        
        $time = time();
        $companyM   =   $this->MODEL('company');
        
        if ($this->config['sy_web_site']=='1') {
            
            if (!empty($this->config['provinceid'])) {
                $hotcomwhere['provinceid']  =   $this->config['provinceid'];
            }
            if (!empty($this->config['cityid'])) {
                $hotcomwhere['cityid']      =   $this->config['cityid'];
            }
            if (!empty($this->config['three_cityid'])) {
                $hotcomwhere['three_cityid']=   $this->config['three_cityid'];
            }
            if (!empty($this->config['hyclass'])) {
                $hotcomwhere['hy']          =   $this->config['hyclass'];
            }
        }
        
        $hotcomwhere['hottime']     =   array('>', $time);
        $hotcomwhere['r_status']    =   1;
        
        $hcom                       =   $companyM->getChCompanyList($hotcomwhere, array('field' => '`uid`,`name`,`shortname`'));
        
        if (!empty($hcom)) {
            foreach ($hcom as $v) {
                $hcuid[]    =   $v['uid'];
            }
            $hcwhere['uid'] =   array('in', pylode(',', $hcuid));
            $hcwhere['time_start']  =   array('<', $time);
            $hcwhere['time_end']    =   array('>', $time);
            $hcwhere['limit']       =   $_POST['limit'];
            if($this->config['hotcom_top'] == 1){
                // (，lastupdate)
                $hcwhere['orderby']  =  'lastupdate,DESC';
            }elseif($this->config['hotcom_top'] == 2){
                // 
                $hcwhere['orderby']  =  'rand()';
            }else{
                // 
                $hcwhere['orderby']  =  'sort';
            };
            // $hcom，
            $hotcom = $companyM->getHotJobList($hcwhere, array('utype' => 'wxapp', 'field' => '`uid`,`hot_pic`','hcom'=>$hcom));
        }else{
            $hotcom = array();
        }
        echo yun_json_encode($hotcom);
    }
    function getCityDomain_action(){

        $siteM = $this->MODEL('site');
        
        $return = $siteM->getCityDomain(array('x'=>$_POST['x'],'y'=>$_POST['y'],'did'=>$this->config["did"]));
        
        echo yun_json_encode($return);exit();
    }
}

?>