<?php

/**
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

class index_controller extends common
{
    /**
     * 首页
     */
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
        //热门职位查询
        $categoryM	=	$this -> MODEL('category');
        $hotclass = $categoryM->getHotJobClass(array('rec'=>1,'orderby'=>'sort,desc'),'`id`,`keyid`,`name`');
        
        $this->yunset('hotclass', $hotclass);
        //首页弹框广告记录
        $bannerFlag   =   $_COOKIE['wap_bannerFlag'];
        if (!$bannerFlag) {
            $this->cookie->setcookie('wap_bannerFlag', 1, time() + 3600);
        }
        $this->yunset("bannerFlag", $bannerFlag);
        
        $this->yuntpl(array('wap/index'));
         
    }

    /**
     * 退出登录
     */
    function loginout_action()
    {
        $this->cookie->unset_cookie();
        $this->wapheader('');
    }

    // 关于我们
    function about_action()
    {

        $descM      =   $this->MODEL('description');
        $content    =   $descM->getDes(array('name' => '关于我们'), array('field' => 'content'));
        $this->yunset('content', $content);
        if ($_GET['fr'] == 'wxapp') {
            $this->yunset('wxapp', 1);
        }
        $this->yunset('headertitle', '关于我们');
        $this->yunset('title', '关于我们');
        $this->yuntpl(array('wap/about'));
    }

    // 联系我们
    function contact_action()
    {
        $descM      =   $this->MODEL('description');
        $content    =   $descM->getDes(array('name' => '联系我们'), array('field' => 'content'));
        $this->yunset('content', $content);
        if ($_GET['fr'] == 'wxapp') {
            $this->yunset('wxapp', 1);
        }
        $this->yunset('headertitle', '联系我们');
        $this->yunset('title', '联系我们');
        $this->yuntpl(array('wap/about'));
    }

    //下载app
    function appDown_action()
    {
        if (preg_match("/(iphone|ipod|ipad)/i", strtolower($_SERVER['HTTP_USER_AGENT']))){
            // 苹果
            include(DATA_PATH . 'api/wxapp/app.config.php');
            if (is_weixin()){
                // 微信内用二维码
                $down = array(
                    'qrcode' => $this->config['sy_ossurl'] .'/' .$this->config['sy_iosu_qcode']
                );
            }else{
                // 其他浏览器用链接
                if (!empty($appconfig['iosurl'])) {
                    // 苹果微信内不支持链接直接打开appstore
                    $down['url'] = $appconfig['iosurl'];
                }
            }
            $down['version'] = $appconfig['iosversion'];
        }else{
            // 安卓
            include(DATA_PATH . 'api/wxapp/app.config.php');
            if (is_weixin()){
                // 微信内用二维码
                $down = array(
                    'qrcode' => $this->config['sy_ossurl'] .'/' .$this->config['sy_androidu_qcode']
                );
            }else{
                // 其他浏览器用链接
                include(DATA_PATH . 'api/wxapp/app.config.php');
                if (!empty($appconfig['androidurl'])) {
                    $down['url'] = $appconfig['androidurl'];
                }
            }
            $down['version'] = $appconfig['androidversion'];
        }
        $this->yunset('down', $down);
        $this->yunset('headertitle', '下载APP');
        $this->yuntpl(array('wap/appdown'));
    }

    // 隐私政策
    function privacy_action()
    {
        $descM      =   $this->MODEL('description');
        $content    =   $descM->getDes(array('name' => '隐私政策'), array('field' => 'content'));
        $this->yunset('content', $content);
        if ($_GET['fr'] == 'wxapp') {
            $this->yunset('wxapp', 1);
        }
        $this->yunset('headertitle', '隐私政策');
        $this->yunset('title', '隐私政策');
        $this->yuntpl(array('wap/about'));
    }

    // 用户协议
    function protocol_action()
    {
        $descM      =   $this->MODEL('description');
        $content    =   $descM->getDes(array('name' => '注册协议'), array('field' => 'content'));
        $this->yunset('content', $content);
        if ($_GET['fr'] == 'wxapp') {
            $this->yunset('wxapp', 1);
        }
        $this->yunset('headertitle', '服务协议');
        $this->yunset('title', '服务协议');
        $this->yuntpl(array('wap/about'));
    }

     
    // 首页名企
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
                // 职位更新时间(职位修改时，会更新名企表lastupdate字段)
                $hcwhere['orderby']  =  'lastupdate,DESC';
            }elseif($this->config['hotcom_top'] == 2){
                // 随机
                $hcwhere['orderby']  =  'rand()';
            }else{
                // 后台手动设置
                $hcwhere['orderby']  =  'sort';
            };
            //  查询基础条件是通过$hcom，传过去直接复用
            $hotcom = $companyM->getHotJobList($hcwhere, array('utype' => 'wxapp', 'field' => '`uid`,`hot_pic`','hcom'=>$hcom));
        }else{
            $hotcom = array();
        }
        echo json_encode($hotcom);
    }
    function getCityDomain_action(){

        $siteM = $this->MODEL('site');
        
        $return = $siteM->getCityDomain(array('x'=>$_POST['x'],'y'=>$_POST['y'],'did'=>$this->config["did"]));
        
        echo json_encode($return);exit();
    }
}

?>