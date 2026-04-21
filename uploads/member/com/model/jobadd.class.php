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
class jobadd_controller extends company
{

    function index_action()
    {
        
        $company    =   $this -> comInfo['info'];
        if(!$company['name'] || ! $company['provinceid'] || (!$company['linktel'] && ! $company['linkphone'])){
            $this->ACT_msg('index.php?c=info', '请先完善基本资料！');
        }
        $this->yunset('company', $company);
        $this->yunset('companycert', $this->comInfo['cert'] ? $this->comInfo['cert'] : array());
        $this->yunset("job_ms_rating", !empty($this->config['job_ms_rating']) ? @explode(',',$this->config['job_ms_rating']) : array());
        if ($company['r_status'] != 1) {
            $jobState = 0;
        } else if ($this->config['com_free_status'] == 1 && $this->comInfo['cert']['status'] == 1) {
            $jobState = 1;
        } else if ($this->config['job_ms_rating'] && in_array($company['rating'], $this->config['job_ms_rating'])) {
            $jobState = 1;
        } else {
            $jobState = $this->config['com_job_status'];
        }
        $this->yunset('jobState', $jobState);
        // 身份认证信息，强制邮箱、手机、企业资质认证配置--start
        $msg        =   array();
        
        $isallow_addjob =   '1';
        
        $url        =  'index.php?c=binding';
        
        if ($this->config['com_enforce_emailcert'] == '1') {
            if ($company['email_status'] != '1') {
                
                $isallow_addjob =   '0';
                $msg[]          =   '邮箱认证';
            }
        }
        if ($this->config['com_enforce_mobilecert'] == '1') {
            if ($company['moblie_status'] != '1') {
                
                $isallow_addjob =   '0';
                $msg[]          =   '手机认证';
            }
        }

        if ($this->config['com_enforce_licensecert'] == '1') {
            $comM         =   $this->MODEL('company');
            $cert =   $comM -> getCertInfo(array('uid'=>$this->uid,'type'=>3), array('field' => '`uid`,`status`'));
            
            if ($company['yyzz_status'] != '1' && (empty($cert) || $cert['status'] == 2)) {
                
                $isallow_addjob =   '0';
                $msg[]          =   '企业资质认证';
            }
        }
        
        if ($isallow_addjob == '0') {

            $this -> ACT_msg($url, '请先完成'.implode('、', $msg).'！');
        }
        
        if ($this->config['com_enforce_setposition'] == '1') {
            if (empty($company['x']) || empty($company['y'])) {
                
                $this->ACT_msg('index.php?c=map', '请先完成地图设置！');
            }
        }

        if ($this->config['com_gzgzh'] == '1') {

            $userInfoM  =   $this->MODEL('userinfo');
            $uInfo      =   $userInfoM->getInfo(array('uid' => $this->uid), array('field' => '`subscribe`,`wxid`'));

            $isSubscribe=   1;
            if (!empty($uInfo['wxid'])) {
                if ($uInfo['subscribe'] != 1) {
                    $wxM        =   $this->MODEL('weixin');
                    $wxUserInfo =   $wxM->getWxUser($uInfo['wxid']);
                    $this->obj->update_once('member', array('subscribe' => $wxUserInfo['subscribe']), array('uid' => $this->uid));
                    if ($wxUserInfo == 0) {
                        $isSubscribe    =   0;
                    }
                }
            } else {
                $isSubscribe    =   0;
            }
            if ($isSubscribe == 0) {
                $this->cookie->SetCookie('gzh', '', (strtotime('today') - 86400));
                $this->ACT_msg('index.php', '请先关注公众号！');
            }
        }

        $statics    =   $this -> company_satic();
        
        if ($statics['addjobnum'] == 0) { // 会员过期
            
            $this->ACT_msg('index.php?c=right', '你的会员已到期！', 8);
        }

        $CacheArr   =   $this->MODEL('cache')->GetCache(array('hy', 'job', 'city', 'com', 'circle','user'));
        $this->yunset($CacheArr);
        if(empty($CacheArr['city_type'])){
            $this   ->  yunset('cionly',1);
        }
        if(empty($CacheArr['job_type'])){
            $this   ->  yunset('jionly',1);
        }
        $row            =   array();
        $row['hy']      =   $company['hy'];
        $row['sdate']   =   time();
        $row['number']  =   $CacheArr['comdata']['job_number'][0];
        $row['type']    =   $CacheArr['comdata']['job_type'][0];
        $row['exp']     =   $CacheArr['comdata']['job_exp'][0];
        $row['report']  =   $CacheArr['comdata']['job_report'][0];
        $row['age']     =   $CacheArr['comdata']['job_age'][0];
        $row['edu']     =   $CacheArr['comdata']['job_edu'][0];
        $row['marriage']=   $CacheArr['comdata']['job_marriage'][0];
        $row['arraywelfare'] = $company['arraywelfare']; // 添加职位，带企业添加的福利待遇
        $this->yunset('row', $row);

        $jobnum         =   $this->MODEL('job')->getJobNum(array('uid' => $this->uid, 'status' => 0, 'state' => 1));
        $this->yunset('jobnum', $jobnum);

        $this->public_action();
        $this->com_tpl('jobadd');
    }

    function edit_action()
    {

        $jobM       =   $this->MODEL('job');
        $statis     =   $this->company_satic();
        $this->yunset('statis', $statis);

        $id     =   intval($_GET['id']);
        
        $row    =   $jobM->getInfo(array('id' => $id,'uid'=>$this->uid),array('add'=>'yes'));

        if (empty($row)) {
            
            $this->ACT_msg('index.php?c=jobadd', '职位参数错误！');
        }

        $company = $this -> comInfo['info'];
        if ($company['linktel'] == '' && $company['linkphone']) {
            $company['linktel'] = $company['linkphone'];
        }
        $this->yunset('company', $company);
        $this->yunset('companycert', $this->comInfo['cert'] ? $this->comInfo['cert'] : array());
        $this->yunset("job_ms_rating", !empty($this->config['job_ms_rating']) ? @explode(',',$this->config['job_ms_rating']) : array());
        if ($company['r_status'] != 1) {
            $jobState = 0;
        } else if ($this->config['com_free_status'] == 1 && $this->comInfo['cert']['status'] == 1) {
            $jobState = 1;
        } else if ($this->config['job_ms_rating'] && in_array($company['rating'], $this->config['job_ms_rating'])) {
            $jobState = 1;
        } else {
            $jobState = $this->config['com_job_status'];
        }
        $this->yunset('jobState', $jobState);
        $CacheArr   =   $this->MODEL('cache') -> GetCache(array('hy', 'job', 'city', 'com', 'user'));
        $this->yunset($CacheArr);
        if(empty($CacheArr['city_type'])){
            $this   ->  yunset('cionly',1);
        }
        if(empty($CacheArr['job_type'])){
            $this   ->  yunset('jionly',1);
        }
        if ($row['autotime'] > time()) {
            $row['autodate']    =   date('Y-m-d', $row['autotime']);
        }
        $row['description']     =   str_replace(array('ti<x>tle','“','”'), array('title',' ',' '), $row['description']);
        $fuli           =  explode(',', $row['welfare']);
        $row['arraywelfare'] = array_filter($fuli);
        $this->yunset('row', $row);



        $this->yunset('referer', $_SERVER['HTTP_REFERER']);// 获取上级页面

        $this->public_action();
        $this->com_tpl('jobadd');
    }

    function save_action(){


        if ($_POST) {

            $company        =   $this->comInfo['info'];

            $rstaus         =   $company['r_status'];

            $description    =   str_replace(array('&amp;', 'background-color:#ffffff', 'background-color:#fff', 'white-space:nowrap;'), array('&', 'background-color:', 'background-color:', 'white-space:'), $_POST['description']);

            if ($_POST['job_post']) {

                $cacheArr   =   $this->MODEL('cache')->GetCache('job');

                if(empty($cacheArr['job_type'])){

                    $_POST['job1']      =   $_POST['job_post'];
                    $_POST['job1_son']  =   '';
                    $_POST['job_post']  =   '';
                }else{

                    $categoryM  =   $this->MODEL('category');
            
                    $row1       =   $categoryM->getJobClass(array('id' => intval($_POST['job_post'])), '`keyid`');
                    $row2       =   $categoryM->getJobClass(array('id' => $row1['keyid']), '`keyid`');

                    if ($row2['keyid'] == '0') {
                        
                        $_POST['job1_son']  =   $_POST['job_post'];
                        $_POST['job1']      =   $row1['keyid'];
                        $_POST['job_post']  =   '';
                    } else {
                        
                        $_POST['job1_son']  =   $row1['keyid'];
                        $_POST['job1']      =   $row2['keyid'];
                    }
                }
            }

            if ($_POST['link_id'] == -1){

                $provinceid     =   $this->comInfo['provinceid'];
                $cityid         =   $this->comInfo['cityid'];
                $three_cityid   =   $this->comInfo['three_cityid'];
                $x              =   $this->comInfo['x'];
                $y              =   $this->comInfo['y'];
            }else if ((int)$_POST['link_id'] > 0){

                $addressM       =   $this->MODEL('address');
                $addresInfo     =   $addressM->getAddressInfo(array('id' => $_POST['link_id'], 'uid' => $this->uid));
                $provinceid     =   $addresInfo['provinceid'];
                $cityid         =   $addresInfo['cityid'];
                $three_cityid   =   $addresInfo['three_cityid'];
                $x              =   $addresInfo['x'];
                $y              =   $addresInfo['y'];
            }
			
            $post   = array(
            
                'job1'          =>  intval($_POST['job1']),
                'job1_son'      =>  intval($_POST['job1_son']),
                'job_post'      =>  intval($_POST['job_post']),

                'provinceid'    =>  intval($provinceid),
                'cityid'        =>  intval($cityid),
                'three_cityid'  =>  intval($three_cityid),

                'x'             =>  $x,
                'y'             =>  $y,

                'minsalary'     =>  intval($_POST['salary_type']) == 1 ? 0 : intval($_POST['minsalary']),
                'maxsalary'     =>  intval($_POST['salary_type']) == 1 ? 0 : intval($_POST['maxsalary']),

                'description'   =>  $description,

                'is_link'       =>  $_POST['is_link'] ? $_POST['is_link'] : 1,
                'link_id'       =>  (int)$_POST['link_id'] > 0 ? $_POST['link_id'] : '',

                'is_message'    =>  $_POST['is_message'] ?  $_POST['is_message'] : 1,
                'is_email'      =>  (int)$_POST['is_email'] == 2 ?  3 : 1,

                'r_status'      =>  $rstaus,
                'hy'            =>  intval($_POST['hy']),
                'number'        =>  intval($_POST['number']),
                'exp'           =>  intval($_POST['exp']),
                'report'        =>  intval($_POST['report']),
                'age'           =>  intval($_POST['age']),
                'sex'           =>  intval($_POST['sex']),
                'edu'           =>  intval($_POST['edu']),
                'is_graduate'   =>  intval($_POST['is_graduate']),
                'marriage'      =>  intval($_POST['marriage']),
                'welfare'       =>  @implode(',', $_POST['welfare']),
                'lang'          =>  trim(pylode(',', $_POST['lang'])),

                'exp_req'       =>  trim($_POST['exp_req']),
				'edu_req'       =>  trim($_POST['edu_req']),
				'sex_req'       =>  trim($_POST['sex_req']),
				'minage_req'    =>  trim($_POST['minage_req']),
				'maxage_req'    =>  trim($_POST['maxage_req']),

                'zp_num'        => intval($_POST['zp_num']),
                'zp_minage'     => intval($_POST['zp_minage']),
                'zp_maxage'     => intval($_POST['zp_maxage'])



            );
            if($this->config['joblock']!=1 || empty($_POST['id'])){
				$post['name']	=	$_POST['name'];
			}
			 
            $data   =   array(
                'post'      => $post,
                'id'        => intval($_POST['id']),
                'uid'       => $this->uid,
                'usertype'  => $this->usertype,
                'did'       => $this->userdid,

                'link_man'      => intval($_POST['islink']) == 2 ? $_POST['link_man'] : '',
                'link_moblie'   => intval($_POST['islink']) == 2 ? $_POST['link_moblie'] : '',
                'email'         => intval($_POST['islink']) == 2 ? $_POST['email'] : '',
                'link_address'  => intval($_POST['islink']) == 2 ? $_POST['link_address'] : '',
                'x'             => intval($_POST['islink']) == 2 ? $_POST['map_xval'] : '',
                'y'             => intval($_POST['islink']) == 2 ? $_POST['map_yval'] : '',
                
                'tblink'    => $_POST['tblink'],
                'jobcopy'   => $_POST['jobcopy']
            );

            $this->cookie->SetCookie('delay', '', time() - 60);

            $jobM   =   $this->MODEL('job');
            
            $return =   $jobM->addJobInfo($data);

            if ($return['errcode'] == 9) {
                
                $this->ACT_layer_msg($return['msg'], $return['errcode'], $return['id']);
            } elseif ($return['url']) {
                
                $this->ACT_layer_msg($return['msg'], $return['errcode'], $return['url']);
            } else {
                
                $this->ACT_layer_msg($return['msg'], $return['errcode']);
            }
        }
    }

    function getJobNum_action()
    {
        if ($_POST['uid']) {

            $statis = $this->company_satic();

            if ($statis) {
                echo $statis['addjobnum'];
                die();
            }
        }
    }
    
    /**
     *  @desc 发布职位条件查询 
     */
    function jobCheck_action()
    {
        
        $jobM   =   $this->MODEL('job');
        $statisM=   $this->MODEL('statis');
        
        $uid    =   $this->uid;
        $statis =   $statisM->getInfo($uid, array('usertype' => 2, 'field' => '`integral`,`job_num`,`rating_type`,`vip_etime`'));

        $result =   $jobM->getAddJobNeedInfo($uid, 1);
              
        $return =   array(
            
            'msgList'   =>  $result['pc'],
            'integral'  =>  (int)$statis['integral'],
            'job_num'  =>  (int)$statis['job_num']
        );

        echo json_encode($return);
        
        die();
        
    }
}
?>