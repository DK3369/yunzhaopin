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

class index_controller extends common{	
	//申请单个职位，TODO:仅前台
	function sq_job_action(){
		$JobM				=	$this -> MODEL('job');
		$data				=	array(
			'uid'			=>	$this -> uid,
			'usertype'		=>	$this -> usertype,
			'did'			=>	$this->userdid,
			'job_id'		=>	$_POST['jobid'],
			'eid'			=>	$_POST['eid'],
			'port'			=>	'1'
		);

		$res				=	$JobM -> applyJob($data);
		echo json_encode($res);die;
		//echo $res['errorcode'];die;
	}

	// 职位详情页面，，取消收藏操作
    public function cancelFavJob_action()
    {

        $jobM   =   $this->MODEL('job');

        $return =   $jobM->cancelFavJob(array('job_id' => $_POST['id'], 'uid' => $this->uid, 'usertype' => $this->usertype));

        echo json_encode($return);die;
	}

	//会员中心收藏，TODO:仅前台
    function favjobuser_action()
    {
        $data   =   array(
            'uid'       =>  $this->uid,
            'username'  =>  $this->username,
            'usertype'  =>  $this->usertype,
            'job_id'    =>  (int)$_POST['id'],
        );
        $jobM   =   $this->MODEL("job");
        $return =   $jobM->collectJob($data);
        echo json_encode($return);
        die;
    }

    // TODO:PC端邀请面试操作
    function indexajaxresume_action()
    {
        if ($_POST) {

            $_POST['uid']       =   $this->uid;
            $_POST['username']  =   $this->username;
            $_POST['usertype']  =   $this->usertype;
            $comtcM             =   $this->MODEL('comtc');

            $return             =   $comtcM->invite_resume($_POST);

            if ($return['status']) {
                $return['msgList']  =   $return['msgList']['pc'];
                echo json_encode($return);
                die();
            }
        }
    }

    // TODO:前台、企业会员中心 邀请面试套餐积分操作
    function sava_ajaxresume_action()
    {
        $jobM   =   $this->MODEL('job');

        $_POST  =   $this->post_trim($_POST);
		$_POST['port']	=	'1';
        $fidArr =   array(
            'fuid' => $this->uid,
            'fusername' => $this->username,
            'fusertype' => $this->usertype
        );

        $res = $jobM->addYqmsInfo(array_merge($fidArr, $_POST));
        echo json_encode($res);die();
    }
	
	//邀请面试发送邮件手机短信
	function msg_post($uid,$comid,$row=''){
		$userinfoM	=	$this->MODEL('userinfo');

		$com		=	$userinfoM->getUserInfo(array('uid'=>$comid),array('usertype'=>2,'field'=>'`uid`,`name`,`linkman`,`linktel`,`linkmail`'));

		$info		=	$userinfoM->getUserInfo(array('uid'=>$uid),array('usertype'=>1,'field'=>'`email`,`telphone` as `moblie`,`name`'));

		$data		=	array(
			'uid'		=>	$uid,
			'name'		=>	$info['name'],
			'cuid'		=>	$com['uid'],
			'cname'		=>	$com['name'],
			'type'		=>	"yqms",
			'company'	=>	$com['name'],
			'linkman'	=>	$com['linkman'],
			'comtel'	=>	$com['linktel'],
			'comemail'	=>	$com['linkmail'],
			'content'	=>	@str_replace("\n","<br/>",$row['content']),
			'jobname'	=>	$row['jobname'],
			'username'	=>	$row['username'],
		);
		if(checkMsgOpen($this -> config)){

			$data['moblie']	=	$info['moblie'];
		}
		if($this->config['sy_email_set']=="1"){

			$data['email']	=	$info['email'];
		}
		if($data['email'] || $data['moblie']){
			
			$notice	= 	$this->MODEL('notice');
			$notice->sendEmailType($data);
			$data['port']	=	'1';
			$notice->sendSMSType($data);
		}
	}
	private function _out($arr){
	    $arr['msg'] 		= 	$arr['msg'];
	    $arr['msgList']     = 	$arr['msgList']['pc'];
	    $arr['usertype']	=	$this->usertype;
		echo json_encode($arr);die;
	}
	/**
	 * 下载简历（查看联系方式）
	 * 2019-06-14
	 */
	function for_link_action(){
		$downReM	=	$this -> MODEL('downresume');
		$data		=	array(
			'eid'		=>	$_POST['eid'],
		    'uid'		=>	$this -> uid,
		    'usertype'	=>	$this -> usertype,
		);
		$downRes	=	$downReM -> downResume($data);
		$this -> _out($downRes);
	}
	
	//获取城市信息，TODO:前台、后台、会员中心
	function ajax_action(){

		include(PLUS_PATH."city.cache.php");

		if(is_array($_POST['str'])){

			$cityid	=	$_POST['str'][0];
		}else{
			$cityid	=	$_POST['str'];
		}
		$data		=	"<option value=''>--请选择--</option>";

		if(is_array($city_type[$cityid])){

			foreach($city_type[$cityid] as $v){

				$data.="<option value='$v'>".$city_name[$v]."</option>";
			}
		}
		echo $data;
	}
   
    //获取职位信息，TODO:前台、后台、会员中心
    function ajax_job_action(){

		include(PLUS_PATH."job.cache.php");

		if(is_array($_POST[str])){

			$jobid	=	$_POST[str][0];
		}else{
			$jobid	=	$_POST[str];
		}
		$data		=	"<option value=''>--请选择--</option>";

		if(is_array($job_type[$jobid])){

			foreach($job_type[$jobid] as $v){

				$data.="<option value='$v'>".$job_name[$v]."</option>";
			}
		}
		echo $data;
	}
	

	//职位列表，换一组，TODO:前台
    function exchange_action(){

		$where	=	array();
		
		if($this->config['sy_web_site']=="1"){

			if($this->config['province']>0 && $this->config['province']!=""){
				$where['provinceid']	=	$this->config['province'];
			}
			if($this->config['cityid']>0 && $this->config['cityid']!=""){
				$where['cityid']		=	$this->config['cityid'];
			}
			if($this->config['three_cityid']>0 && $this->config['three_cityid']!=""){
				$where['three_cityid']	=	$this->config['three_cityid'];
			}
			if($this->config['hyclass']>0 && $this->config['hyclass']!=""){
				$where['hy']			=	$this->config['hyclass'];
			}
		}
		$where['state']		=	1;
		$where['r_status']	=	array('<>',2);
		$where['status']	=	array('<>',1);
		$where['rec_time']	=	array('>',time());
		$where['orderby']	=	'lastupdate';
		$jobM=$this->MODEL('job');

		$num			=	$jobM->getJobNum($where);

		$_GET['page']	=	(int)$_GET['page'];

		if($num<=$_GET['page']*10){
			$page	=	1;
		}else{
			$page	=	intval($_GET['page'])+1;
		}
		if($num<=($_GET['page']*10-10)){
			$pnum	=	0;
		}else{
			$pnum	=	$_GET['page']*10-10;
		}
		$where['limit']	= array($pnum,10);
		
		$rows	=	$jobM->getList($where,array('field'=>'`id`,`name`,`uid`,`com_name`,`minsalary`,`maxsalary`,`rec_time`,`exp`,`edu`,`cityid`,`three_cityid`','isurl'=>'yes'));
		$rows	=	$rows['list'];
		if($rows && is_array($rows)){
			
			$html	=	"<input type=\"hidden\" value='".$page."' id='exchangep'/>";
			foreach($rows as $key=>$val){
				if($val['rec_time']>time()){

					$val['name']	=	"<font color='red'>".$val['name']."</font>";
				}
				$html	.=	"<li> <a href=\"".$val['joburl']."\" class=\"job_recommendation_jobname\">".$val['name']."</a><a href=\"".$val['comurl']."\" class=\"job_recommendation_Comname\">".$val['com_name']."</a><div class=\"job_recommendation_msg\"><span><em class=\"job_right_box_list_c\">".$val['job_salary']."</em></span></div></li>";
			}
		}
		echo $html;die;
	}
	
	//获取地图配置信息，TODO:前台、后台、会员中心
    function mapconfig_action(){
    	$arr	=	array(
			'map_x'					=>	$this->config['map_x'],
			'map_y'					=>	$this->config['map_y'],
			'map_rating'			=>	$this->config['map_rating'],
			'map_control'			=>	$this->config['map_control'],
			'map_control_anchor'	=>	$this->config['map_control_anchor'],
			'map_control_type'		=>	$this->config['map_control_type'],
			'map_control_xb'		=>	$this->config['map_control_xb'],
			'map_control_scale'		=>	$this->config['map_control_scale'],
		);
    	echo json_encode($arr);
    }
    
    function mapconfigdiffdomains_action(){
      $arr	=	array(
		'map_x'					=>	$this->config['map_x'],
		'map_y'					=>	$this->config['map_y'],
		'map_rating'			=>	$this->config['map_rating'],
		'map_control'			=>	$this->config['map_control'],
		'map_control_anchor'	=>	$this->config['map_control_anchor'],
		'map_control_type'		=>	$this->config['map_control_type'],
		'map_control_xb'		=>	$this->config['map_control_xb'],
		'map_control_scale'		=>	$this->config['map_control_scale'],
	  );
    	echo 'diffdomains('.json_encode($arr).')';
    }

	//下载简历，TODO:前台
    function resume_word_action()
    {

        $resumeM=   $this->MODEL('resume');
        $downM  =   $this->MODEL('downresume');
        $getId  =   (int)$_GET['id'];

        if (isset($_GET['s']) && isset($_GET['d'])) {
            if (isset($_GET['f']) && $_GET['f'] == 1) {
                $downInfo = $resumeM->getFreeDownResumeInfo(array('uid' => $_GET['d'], 'eid' => $getId));
            } else {
                $downInfo = $resumeM->getDownResumeInfo(array('uid' => $_GET['d'], 'eid' => $getId));
            }
            $str = md5($_GET['d'] . $downInfo['downtime']);
            if ($str == $_GET['s']) {
                $uid = $_GET['d'];
                $usertype = 2;
            }
        } else {
            $uid = $this->uid;
            $usertype = $this->usertype;
        }

        $isDown =   $downM->isDown(array('eid' => $getId, 'comid' => $uid, 'usertype' => $usertype));
        $expect =   $resumeM->getInfoByEid(array('eid' => $getId, 'uid' => $uid, 'usertype' => $usertype));

        if (!empty($expect) || !empty($isDown)) {

            global $phpyun;
            $this->yunset('Info', $expect);

            $wordStr    =   $phpyun->fetch(TPL_PATH.'resume/wordresume.htm', time());
            $this->startword($expect['name'], $wordStr);
        }
    }
    
    //下载简历，TODO:前台
	function startword($wordname,$html){
        ob_start();
        header("Content-Type:  application/msword");
        header("Content-Disposition:  attachment;  filename=".iconv('utf-8','gbk',$wordname).".doc"); //指定文件名称后缀
        header("Pragma:  no-cache");
        header("Expires:  0");
        echo $html;
    }
	
	 
	//首页显示职位类别
	function show_leftjob_action(){
		$cache		=	$this->MODEL('cache')->GetCache(array('job'));
		$this->yunset($cache);
        global $config;
		$job_index	=	$cache['job_index'];
		$job_type	=	$cache['job_type'];
		$job_name	=	$cache['job_name'];
		// 类别数量，不同的模板可能有不同
		$num        =   isset($_POST['num']) ? $_POST['num'] : 11;
		
		$html	    =	'<ul>';
		if(is_array($job_index)){
			foreach($job_index as $j=>$v){

				$jobclassurl	=	searchListRewrite(array('job1'=>$v),$config);
                if($j<$num){

                    $html.='<li class="lst'.$j.' " onmouseover="show_job(\''.$j.'\',\'0\');" onmouseout="hide_job(\''.$j.'\');"> <b></b> <a class="link" href="'.$jobclassurl.'" title="'.$job_name[$v].'">'.$job_name[$v].'</a> <i></i><div class="lstCon"><div class="lstConClass">';

                    if(is_array($job_type[$v])){
                        $html .= '';
                        foreach($job_type[$v] as $vv){
                            $jobclassurls	=	searchListRewrite(array('type'=>'job1_son','job1'=>$v,'job1_son'=>$vv),$config);
                            if(is_array($job_type[$vv])){
                                $html.=' <dl><dt> <a  href="'.$jobclassurls.'" title="'.$job_name[$vv].'">'.$job_name[$vv].'</a> </dt><dd> ';
                                foreach($job_type[$vv] as $vvv){

                                    $jobclassurlss	=	searchListRewrite(array('type'=>'job_post','job1'=>$v,'job1_son'=>$vv,'job_post'=>$vvv),$config);

                                    $html.=' <a  href="'.$jobclassurlss.'" title="'.$job_name[$vvv].'">'.$job_name[$vvv].' </a>';
                                }
                                $html.=' </dd><dd style="display:block;clear:both;float:inherit;width:100%;font-size:0;line-height:0;"></dd></dl>';
                            }else{
                                $html.='<a  href="'.$jobclassurls.'" title="'.$job_name[$vv].'">'.$job_name[$vv].'</a> ';
                            }

                        }

                    }
                    $html.=' </div> </div></li>';
                }
			}
		}
		$html.=' </ul>';

		echo $html;die;
	}

	//购买简历模板
	function pay_action(){
		$resumeM	=	$this->MODEL('resume');
		$statisM	=	$this->MODEL('statis');
		$tplM		=	$this->MODEL('tpl');
		$IntegralM	=	$this->MODEL('integral');

		$id   		=   intval($_GET['id']);
		$eid  		=   intval($_GET['eid']);

		$expect		=	$resumeM->getExpect(array('id'=>$eid,'uid'=>$this->uid),array('field'=>'`id`'));
		if($expect['id']==''){
			$this->layer_msg('简历不存在！',8,0,Url("resume"));
		}

		$statis		=	$statisM->getInfo($this->uid,array('usertype'=>$this->usertype,'field'=>'`tpl`,`paytpls`,`integral`'));

		$info		=	$tplM->getResumetpl(array('id'=>$id),array('field'=>'`id`,`price`'));

		$paytpls	=	array();

		if($statis['paytpls']){

			$paytpls	=	@explode(',',$statis['paytpls']); 

			if(in_array($info['id'],$paytpls)){

				$this->layer_msg('请勿重复购买！',8,0,"index.php?c=resumetpl");
			}
		}
		if($info['price']>$statis['integral']){

			$this->layer_msg($this->config['integral_pricename'].'不足，请先充值！',8,0);
		}else{
			$nid	=	$IntegralM->company_invtal($this->uid,1,$info['price'],false,"购买简历模板",true,2,'integral',15);
			if($nid){
				$paytpls[]	=	$info['id'];

				$statisM->upInfo(array('tpl'=>$info['id'],'paytpls'=>pylode(',',$paytpls)),array('uid'=>$this->uid,'usertype'=>1));

				$this->layer_msg('购买成功！',9,0,Url('resume',array('c'=>'show','id'=>$expect['id'],'see'=>'member','look'=>'admin')));
			}else{
			    $this->layer_msg('购买失败！',8,0,Url('resume',array('c'=>'show','id'=>$expect['id'],'see'=>'member','look'=>'admin')));
			}
		} 
	}
	//使用简历模板
	function settpl_action(){
		$resumeM	=	$this->MODEL('resume');
		$statisM	=	$this->MODEL('statis');

		$id		=	intval($_GET['id']);
		$eid	=	intval($_GET['eid']);
		
		$expect	=	$resumeM->getExpect(array('id'=>$eid,'uid'=>$this->uid),array('field'=>'`id`'));
		if($expect['id']==''){
			$this->layer_msg('简历不存在！',8,0,Url("resume"));
		}
		$statis		=	$statisM->getInfo($this->uid,array('usertype'=>$this->usertype,'field'=>'`tpl`,`paytpls`,`integral`'));
		
		$paytpls	=	array();
		if($statis['paytpls']){
			$paytpls	=	@explode(',',$statis['paytpls']);  
		}
		$statisM->upInfo(array('tpl'=>$id),array('uid'=>$this->uid,'usertype'=>1));
		
		$this->layer_msg('操作成功！',9,0,Url('resume',array('c'=>'show','id'=>$eid,'see'=>'member','look'=>'admin')));
	}
	
    
    //获取城市列表，TODO:前台
	function getcity_subscribe_action(){

		include(PLUS_PATH."city.cache.php");

		if(is_array($city_type[$_POST['id']])){

			$data		=	'<ul class="post_read_text_box_list">';

			foreach($city_type[$_POST['id']] as $v){

				$data	.=	"<li><a href=\"javascript:check_input('".$v."','".$city_name[$v]."','".$_POST['type']."');\">".$city_name[$v]."</a></li>";
			}
			$data		.=	'</ul>';
		}
		echo $data;die;
	}
   
    //获取职位类别，TODO:前台
	function getjob_subscribe_action(){

		include(PLUS_PATH."job.cache.php");

		if(is_array($job_type[$_POST['id']])){

			$data		=	'<ul class="post_read_text_box_list">';

			foreach($job_type[$_POST['id']] as $v){

				$data	.=	"<li><a href=\"javascript:check_input('".$v."','".$job_name[$v]."','".$_POST['type']."');\">".$job_name[$v]."</a></li>";
			}
			$data		.=	'</ul>';
		}
		echo $data;die;
	}
    
    //获取薪资类别，TODO:前台
	function getsalary_subscribe_action(){
		if($_POST['type']==1){

			include(PLUS_PATH."com.cache.php");

			if(is_array($comdata['job_salary'])){

				$data		=	'<ul class="post_read_text_box_list">';

				foreach($comdata['job_salary'] as $v){

					$data	.=	"<li><a href=\"javascript:check_input('".$v."','".$comclass_name[$v]."','salary');\">".$comclass_name[$v]."</a></li>";
				}
				$data		.=	'</ul>';
			}
		}else{
			include(PLUS_PATH."user.cache.php");

			if(is_array($userdata['user_salary'])){

				$data		=	'<ul class="post_read_text_box_list">';

				foreach($userdata['user_salary'] as $v){

					$data	.=	"<li><a href=\"javascript:check_input('".$v."','".$userclass_name[$v]."','salary');\">".$userclass_name[$v]."</a></li>";
				}
				$data		.=	'</ul>';
			}
		}
		echo $data;die;
	}
	
	//发送短信验证码，TODO:前台
    function regcode_action(){
		
        $this->regcode(1); // PC发送短信
	}

    //发送短信验证码，PC快速投递
    function regcodeks_action(){
        $this->regcode(7); // PC快速投递
    }

    function regcode($port = 1){
		
        $noticeM 	= 	$this->MODEL('notice');
		
		$result		=		$noticeM->jycheck($_POST['code'],'注册会员');
		
		if(!empty($result)){
		    
			$this -> layer_msg($result['msg'], 9, 0, '', 2, $result['error']);
			
		}
		
		$moblie 	= 	trim($_POST['moblie']);
		
		$result	 	= 	$noticeM -> sendCode($moblie, 'regcode', $port);
		
		echo json_encode($result);exit();
	}
	
	//加入人才库，TODO:前台、WAP
    function talent_pool_action(){
		$data=array(
			'eid'		=>	$_POST['eid'],
			'cuid'		=>	$this->uid,
			'usertype'	=>	$this->usertype,
			'uid'		=>	(int)$_POST['uid'],
			'remark'	=>	$_POST['remark'],
			'ctime'		=>	time(),
		);
		$ResumeM	=	$this->MODEL('resume');
	    $return		=	$ResumeM -> addTalent($data);
		echo json_encode($return);die;
	}
	function atn_action(){

		$data	=	array(
			'id'			=>	(int)$_POST['id'],
			'uid'			=>	$this->uid,
			'usertype'		=>	$this->usertype,
			'username'		=>	$this->username,
			'sc_usertype'	=>	(int)$_POST['type']
		);
		$atnM	=	$this->MODEL('atn');
		$return	=	$atnM->addAtnLt($data);

		echo json_encode($return);die;
	}
 
	/* 关注企业，TODO:前台列表 */
	function atncompany_action(){
		$data	=	array(
			'id'			=>	(int)$_POST['id'],
			'tid'			=>	(int)$_POST['tid'],
			'uid'			=>	$this->uid,
			'usertype'		=>	$this->usertype,
		    'username'		=>	$this->username,
		    'utype'			=>	'teacher',
		    'sc_usertype'	=>	2
		);
		 
		$atnM	=	$this->MODEL('atn');
		$return	=	$atnM->addAtnLt($data);
		echo json_encode($return);die;
	}

    /**
     * 获取登陆头信息-用户名红色
     */
	function RedLoginHead_action(){

		if($this->uid!=""&&$this->username!=""){
			if($_COOKIE['remind_num']>0){
				$html   ='<div class="header_Remind header_Remind_hover"> <em class="header_Remind_em "><i class="header_Remind_msg"></i></em><div class="header_Remind_list" style="display:none;">';
				if($this->usertype==1){
                    $html.='<div class="header_Remind_list_a"><a href="'.$this->config['sy_weburl'].'/member/index.php?c=msg">邀请面试</a><span class="header_Remind_list_r fr">('.$_COOKIE['userid_msg'].')</span></div><div class="header_Remind_list_a"><a href="'.$this->config['sy_weburl'].'/member/index.php?c=sysnews">系统消息</a><span class="header_Remind_list_r fr">('.$_COOKIE['sysmsg1'].')</span></div><div class="header_Remind_list_a"><a href="'.$this->config['sy_weburl'].'/member/index.php?c=commsg">企业回复咨询</a><span class="header_Remind_list_r fr">('.$_COOKIE['usermsg'].')</span></div>';
				}elseif($this->usertype==2){
				    $html.='<div class="header_Remind_list_a"><a href="'.$this->config['sy_weburl'].'/member/index.php?c=hr"class="fl">应聘简历</a><span class="header_Remind_list_r fr">('.$_COOKIE['userid_job'].')</span></div><div class="header_Remind_list_a"><a href="'.$this->config['sy_weburl'].'/member/index.php?c=sysnews" class="fl"> 系统消息</a><span class="header_Remind_list_r fr">('.$_COOKIE['sysmsg2'].')</span></div><div class="header_Remind_list_a"><a href="'.$this->config['sy_weburl'].'/member/index.php?c=msg"class="fl">求职咨询</a><span class="header_Remind_list_r fr">('.$_COOKIE['commsg'].')</span></div>';
				}
				$html.='</div> </div>';
			}
			$UserInfo   = $this->MODEL('userinfo') ;
			$username   = $UserInfo ->getUserInfo(array('uid'=>$this->uid),array('usertype'=>$this->usertype));
            if($this->usertype==1){

                $countname = $username['name'];

            }elseif($this->usertype==2){

                $countname = $username['name'];

            }

			if($countname){
                $html2= "<span class=\"sss\">您好：</span><a href=\"".$this->config['sy_weburl']."/member\" ><font color=\"red\">".$countname."</font></a>！<a href=\"".$this->config['sy_weburl']."/member\" >进入用户中心>></a> <a href=\"javascript:void(0)\" onclick=\"logout(\'".$this->config['sy_weburl']."/index.php?c=logout\');\">[安全退出]</a>";

            }else{
                $html2= "<span class=\"sss\">您好：</span><a href=\"".$this->config['sy_weburl']."/member\" ><font color=\"red\">".$this->username."</font></a>！<a href=\"".$this->config['sy_weburl']."/member\" >进入用户中心>></a> <a href=\"javascript:void(0)\" onclick=\"logout(\'".$this->config['sy_weburl']."/index.php?c=logout\');\">[安全退出]</a>";

            }

			$html.='<div class="yun_header_af fr">'.$html2.'</div>';

		}else{
			$login_url 			= 	Url("login",array(),"1");


			if($this->config['reg_moblie']){
				$reg_url 		= 	Url("register",array("usertype"=>"1",'type'=>2),"1");
				$reg_com_url	= 	Url("register",array("usertype"=>"2",'type'=>2),"1");
			}
			else if($this->config['reg_email']){
				$reg_url 		= 	Url("register",array("usertype"=>"1",'type'=>3),"1");
				$reg_com_url 	= 	Url("register",array("usertype"=>"2",'type'=>3),"1");
			}
			else{
				$reg_url 		= 	Url("register",array("usertype"=>"1",'type'=>1),"1");
				$reg_com_url 	= 	Url("register",array("usertype"=>"2",'type'=>1),"1");
			}
			$style 	= 	$this->config['sy_weburl']."/app/template/".$this->config['style'];

            $kjlogin = '';
            $html='<div class=" fr"><div class="yun_topLogin_cont"><div class="yun_topLogin"><a class="" href="'.Url("login").'">用户登录</a></div><div class="yun_topLogin yun_topreg"> <a class="" href="'.Url("register").'">免费注册</a></div></div></div>';

            if($this->config['sy_qqlogin']=='1'||$this->config['sy_sinalogin']=='1'||$this->config['wx_author']=='1'){

                if($_GET['type']=='index'){

                    if($this->config['sy_qqlogin']=='1'){
                        $kjlogin.='<li><img src="'.$this->config['sy_weburl'].'/app/template/'.$this->config['style'].'/images/yun_qq.png" class="png" ><a href="'.$this->config['sy_weburl'].'/qqlogin.php'.'">QQ登录</a></li>';
                    }
                    if($this->config['sy_sinalogin']=='1'){
                        $kjlogin.='<li><img src="'.$this->config['sy_weburl'].'/app/template/'.$this->config['style'].'/images/yun_sina.png" class="png" ><a href="'.Url("sinaconnect",array(),"1").'">新浪登录</a></li>';
                    }
                    if($this->config['wx_author']=='1'){
                        $kjlogin.='<li><img src="'.$this->config['sy_weburl'].'/app/template/'.$this->config['style'].'/images/yun_wx.png" class="png" ><a href="'.Url("wxconnect",array(),"1").'">微信登录</a></li>';
                    }
                }else{
                    $flogin='<div class="fastlogin fr">';
                    if($this->config['sy_qqlogin']=='1'){
                        $flogin.='<span style="width:80px;"><img src="'.$this->config['sy_weburl'].'/app/template/'.$this->config['style'].'/images/yun_qq.png" class="png" > <a href="'.$this->config['sy_weburl'].'/qqlogin.php'.'">QQ登录</a></span>';
                    }
                    if($this->config['sy_sinalogin']=='1'){
                        $flogin.='<span><img src="'.$this->config['sy_weburl'].'/app/template/'.$this->config['style'].'/images/yun_sina.png" class="png"> <a href="'.Url("sinaconnect",array(),"1").'">新浪</a></span>';
                    }
                    if($this->config['wx_author']=='1'){
                        $flogin.='<span><img src="'.$this->config['sy_weburl'].'/app/template/'.$this->config['style'].'/images/yun_wx.png" class="png"> <a href="'.Url("wxconnect",array(),"1").'">微信</a></span>';
                    }
                    $flogin.='</div>';
                    $html.=$flogin;
                }
        

				$html = str_replace("{kjlogin}",$kjlogin,$html);
			}
		}
		if(isset($_GET['type']) && $_GET['type'] == 'ajax'){
			echo str_replace("\'", "'", $html);
		}else{
			echo "document.write('".$html."');";
		}
	}

    /**
     * 获取登陆头信息
     */
	function ReadIndexHeader_action(){
		if($this->uid == "" && $this->username == ""){
			$loginUrl	= 	Url("login",array(),"1");
			$regUrl 	= 	Url("register",array(),"1");

			$html   =   '<div class="header_fixed_login"><div class="header_fixed_login_l" style="display:block"><a href="'.$loginUrl.'" style="color:#fff"><span class="header_fixed_login_dl" did="login" tyle="background:none;">登录</span></a>|<a href="'.$regUrl.'" style="color:#fff"><span class="header_fixed_login_dl" did="register" style="background:none;">注册</span></a></div></div>';
		}else{
			 $murl 	= 	$this->config['sy_weburl'].'/member/index.php';	
			 		
			$html   =   "<div class=\"header_fixed_login\"><div class=\"header_fixed_login_after\"><div class=\"header_fixed_login_after_cont\"><span class=\"header_fixed_login_after_name\">".$this->username."</span><div class=\"header_fixed_reg_box header_fixed_reg_box_ye none\"><a href=\"".$murl."\" class=\"header_fixed_login_a\">进入会员中心</a><a href=\"javascript:void(0)\" class=\"header_fixed_login_a\" onclick=\"logout(\'".$this->config['sy_weburl']."/index.php?c=logout\');\">[安全退出]</a></div></div></div></div>";
		}
		if(isset($_GET['type']) && $_GET['type'] == 'ajax'){
			echo str_replace("\'", "'", $html);
		}else{
			echo "document.write('".$html."');";
		}
	}
	
	// 分站信息
	function Site_action(){
		$html	=	'';
		if($this->config['sy_web_site'] == 1){
			session_start();
			if($this->config['cityname']){
        		$cityname = $this->config['cityname'];
      		}else{
				$cityname = $this->config['sy_indexcity'];
      		}
			$site_url	= Url('index', array('c' => 'site'), 1);
			$html 		= "<span class=\"hp_head_ft_city_x\">".$cityname."站</span><span class=\"hp_head_ft_city_qh\">【<a href=\"".$site_url."\">切换城市</a>】</span>";
		}
		if(isset($_GET['type']) && $_GET['type'] == 'ajax'){
			echo $html;
		}else{			
			echo "document.write('".$html."');";
		}
	}

	// 分站城市信息
	function SiteCity_action(){
		unset($_SESSION['province']);unset($_SESSION['cityid']);unset($_SESSION['three_cityid']);unset($_SESSION['cityname']);unset($_SESSION['newsite']);unset($_SESSION['host']);unset($_SESSION['did']);unset($_SESSION['hyclass']);unset($_SESSION['fz_type']);

		if($_POST[cityid]=="nat"){
			if($this->config['sy_indexdomain']){
				$_SESSION['host'] = $this->config['sy_indexdomain'];
			}else{
				$_SESSION['host'] = $this->config['sy_weburl'];
			}
			echo $_SESSION['host'];die;
		}
		if((int)$_POST['cityid']>0){
			if(file_exists(PLUS_PATH."/domain_cache.php")){
				include(PLUS_PATH."/domain_cache.php");
				if(is_array($site_domain)){
					foreach($site_domain as $key=>$value){
						if($value['province']==$_POST['cityid'] || $value['cityid']==$_POST['cityid'] || $value['three_cityid']==$_POST['cityid']){

							$_SESSION['host'] 			= 	$value['host'];
						}
						if($value['province']==$_POST['cityid']){

							$_SESSION['province'] 		= 	$value['province'];

						}elseif($value['three_cityid']==$_POST['cityid']){

							$_SESSION['three_cityid'] 	= 	$value['three_cityid'];

						}else{
							$_SESSION['cityid'] 		= 	$_POST['cityid'];
						}
					}
				}
			}
			if($_SESSION['host'] && $this->protocol.$_SESSION['host']==$this->config['sy_weburl'] ){
				$_SESSION[newsite]	=	"new";
			}
			$_SESSION['host'] = $_SESSION['host']!=""?$this->protocol.$_SESSION['host']:$this->config['sy_weburl'];

			$_SESSION['cityname'] 	= 	$_POST['cityname'];

			echo $_SESSION['host'];die;
		}else{
			$this->ACT_layer_msg("传递了非法参数！",8,$_SERVER['HTTP_REFERER']);
		}
	}

	// 分站行业信息
	function SiteHy_action(){
		unset($_SESSION['cityid']);unset($_SESSION['three_cityid']);unset($_SESSION['cityname']);unset($_SESSION['hyclass']);unset($_SESSION['fz_type']);

		if($_POST['hyid']=="0"){
			$_SESSION['host'] = $this->config['sy_indexdomain'];
			echo $_SESSION['host'];die;
		}
		unset($_SESSION['newsite']);
		unset($_SESSION['host']);
		unset($_SESSION['did']);
		if((int)$_POST['hyid']>0){
			if(file_exists(PLUS_PATH."/domain_cache.php")){
				include(PLUS_PATH."/domain_cache.php");
				if(is_array($site_domain)){
					foreach($site_domain as $key=>$value){
						if($value['hy']==$_POST['hyid']){
							$_SESSION['host'] = $value['host'];
						}
					}
				}
			}
			if($_SESSION['host'] && $this->protocol.$_SESSION['host']==$this->config['sy_weburl'] ){
				$_SESSION['newsite']="new";
			}
			$_SESSION['host'] = $_SESSION['host']!=""?$this->protocol.$_SESSION['host']:$this->config['sy_weburl'];
			$_SESSION['hyclass'] = $_POST['hyid'];
			echo $_SESSION['host'];die;
		}else{
			$this->ACT_layer_msg("传递了非法参数！",8,$_SERVER['HTTP_REFERER']);
		}
	}
	//认领
	function claim_action(){
		if((int)$_GET['uid']){
			$UserinfoM	=	$this->MODEL('userinfo');
			$companyM	=	$this->MODEL('company');
			$notice 	= 	$this->MODEL('notice');

			$row		=	$UserinfoM->getInfo(array("uid"=>(int)$_GET['uid']),array("field"=>"`source`,`email`,`moblie`,`claim`")); 

			if($row['source']=="6" && $row['email']!=""){

				if($row['claim']=="1"){
					$this->layer_msg('该用户已被认领！',8,0); 

				}

				$cert	=	$companyM->getCertInfo(array('uid' => intval($_GET['uid']),'type'=>6));
				 
				 
				if(empty($cert)){
					$salt 	= 	substr(uniqid(rand()), -6);
					$value	=	array(
						'check'		=>	$row['email'],
						'check2'	=>	$salt,
						'uid'		=>	(int)$_GET['uid'],
						'type'		=>	6,
						'ctime'		=>	time(),
					);
					$companyM->addCertInfo($value);

				}else{
					$salt 	= 	$cert['check2'];
				}
				$info	=	$companyM->getInfo((int)$_GET['uid'],array('field'=>'name'));

				$data			=	array();
				$data['uid']	=	(int)$_GET['uid'];
				$data['name']	=	$info['name'];
				$data['email']	=	$row['email'];
				$data['mobile']	=	$row['moblie'];
				$data['type']	=	"claim";

				$url	=	Url("claim",array('uid'=>(int)$_GET['uid'],'code'=>$salt),"1");

				$data['url']	=	"<a href='".$url."'>".$url."</a> ,如果您不能在邮箱中直接打开，请复制该链接到浏览器地址栏中直接打开：".$url;
				
				$notice->sendEmailType($data);

				if($data['mobile']){
					$data['url']	=	"<a href='".$url."'>".$url."</a>";
					$data['port']	=	'1';
					$notice->sendSMSType($data);
				}
				$email		=	@explode('@',$row['email']);
				$newemail	=	substr($email[0],0,3).'****@'.end($email); 

				$this->layer_msg('<div class="rl_box"><div class="rl_yx_p">已发送到您的邮箱：</div><div class="rl_yx">'.$newemail.'，</div><div class="">请登录您的邮箱重置帐号密码！</div><div class="">如换邮箱请联系客服电话：</div><div class="rl_tel">'. $this->config['sy_freewebtel'] .'</div></div>',9,0); 

			}else{
				$this->layer_msg('该用户不符合认领条件！',8,0); 
			}
		}
	}
	//报名招聘会条件判断
	function ajaxZph_action(){
		$data	=	array(
			'usertype'	=>	$this->usertype,
		    'uid'		=>	$this->uid,
			'jobid'		=>	$_POST['jobid'],
		    'did'		=>	$this->config['did'],
		    'id'		=>	intval($_POST['id']),
			'zid'		=>	intval($_POST['zid']),
		);
		$zphM	=	$this->MODEL('zph');
		$arr	=	$zphM->ajaxZph($data);
		
	    echo json_encode($arr);die;
	}
	
	//报名招聘会条件判断
	function ajaxComjob_action(){
	    
	    if ($_POST['zph']){
	        
	        $zphM  =  $this->MODEL('zph');
	        $comrow   =  $zphM->getZphComInfo(array('uid'=>$this->uid,'zid'=>$_POST['id']));
	        
	    }
	    if (!empty($comrow)){
	        
	        $data['status']	= 2;
	        
	        if($comrow['status']==0){

	        	$data['msg']	= "您已报名,请等待审核！";

	        }else if($comrow['status']==1){

	        	$data['msg']	= "您已报名了，请不要重复报名！";

	        }else if($comrow['status']==2){
	        	
	        	$data['msg']	= "您已报名,且审核未通过！";
	        }
	        
	    }else{
	        
	        $where	=	array(
	            'uid'		=>	$this->uid,
	            'state'		=>	1,
	            'status'	=>	0,
	            'r_status'	=>	array('<>',2),
	        );
	        if($this->config['did']){
	            $where['did']=$this->config['did'];
	        }
	        $jobM	=	$this->MODEL('job');
	        $arr	=	$jobM->getList($where, array('field'=>'`id`,`name`'));
	        $list	=	$arr['list'];
	        
	        if(!empty($list)){
	            $html = '';
	            
	            foreach($list as $v){
	                
	                $html			.=	'<div class="layui-input-inline job_select">';
	                $html			.=	'	<input name="checkbox_job" lay-filter="oneChoose" value="'.$v[id].'" id="status_'.$v[id].'" type="checkbox" lay-skin="primary" title="'.$v[name].'" class="check_all"/>';
	                //$html			.=	'		<label for="status_'.$v[id].'">'.$v[name].'</label>';
	                $html			.=	'</div>';
	                
	            }
	            $data['html']	= $html;
	            
	            $data['status']	= 1;
	            
	        }else{
	            
	            $data['status']	= 2;
	            
	            $data['msg']	= "您还没有发布职位，请先发布职位！";
	        }
	    }
		
	    echo json_encode($data);die;
	}
	//快速申请职位入口
	function temporaryresume_action(){
		$userinfoM	=	$this->MODEL("userinfo");
		$_POST 		= 	$this->post_trim($_POST);
        $tdResArr = $userinfoM->fastToudi($_POST);
        $url = $tdResArr['url'] ? $tdResArr['url'] : '';
        $this->ACT_layer_msg($tdResArr['msg'],$tdResArr['errcode'],$url);
	}

	// 底部统计
	function footertj_action(){
		//查询企业总数
		$companyM	= 	$this->MODEL("company");
		$jobM		=	$this->MODEL("job");
		$resumeM	=	$this->MODEL("resume");

		$comnum = $companyM->getCompanyNum();
		//查询职位总数
		$jobnum = $jobM->getJobNum();
		//查询简历总数
		$expectnum = $resumeM->getExpectNum();

		$html='<a href="javascript:void(0);" onclick="$(\'.tip_bottom\').hide();" class="tip_bottom_close png"></a> 
      			<div class="tip_bottom_ewm"><div class="tip_bottom_ewm_bg">
      			<img src="'.$this->config['sy_ossurl'].'/'.$this->config['sy_wx_qcode'].'"></div>
				
			<div class="tip_bottom_ewm_p">手机也能找工作</div></div>
				
      			<div class="tip_bottom_leftbox">
        		<h2><span class="tip_bottom_fast">海量职位 让求职更简单</h2>
        	<div class="tip_bottom_num "><span>'.$comnum.'</span>+企业的共同选择</div>
        		<div class="tip_bottom_num "><span>'.$jobnum.'</span>+高薪职位任您挑选</div></div>';
		if(!$this->uid){
			$html.='<div class="tip_bottom_member">
        		<a href="'.Url('login').'" class="tip_bottom_login">立即登录</a> 
        		<a href="'.Url('register').'" class="tip_bottom_reg" >快速注册<i class="tip_bottom_reg_icon"></i></a> </div>';
		}
		echo $html;
	}
	
	//登录框窗体html返回
	function DefaultLoginIndex_action(){

		$resumM		=	$this->MODEL('resume');
		$statisM	=	$this->MODEL('statis');
		$companyM	=	$this->MODEL('company');
		$jobM		=	$this->MODEL('job');

		$downM		=	$this->MODEL('downresume');
		$entrustM	=	$this->MODEL('entrust');
		$lookresumeM	=	$this -> MODEL('lookresume');
		$MsgM			=	$this -> MODEL('msg');
		if($this->usertype=='1' && $this->uid){
			
			$member	=	$statisM->getInfo($this->uid,array('usertype'=>1));
			$reume	=	$resumM->getResumeInfo(array('uid'=>$this->uid),array('field'=>'`name`,`photo`,`sex`','logo'=>1));
            $member['name']	=	$reume['name'];
            $member['photo']	=	$reume['photo'];
			$this->yunset("member",$member);

			//面试通知数
			$yqnum		=	$jobM -> getYqmsNum(array('uid'=>$this->uid,'isdel'=>9));
			$this -> yunset("yqnum",$yqnum); 
			
			//谁看了我的简历
			$lookNum	=	$lookresumeM -> getLookNum(array('uid'=>$this->uid,'status'=>array('<>',1)));
			$this -> yunset("lookNum",$lookNum);
		}else if($this->usertype=='2' && $this->uid){
			
			$company				=	$companyM->getInfo($this->uid,array('field'=>'`name`,`logo`','logo'=>1));
			
			$company['sq_job']		= 	$jobM->getSqJobNum(array('com_id'=>$this->uid,'isdel'=>9));
			$company['job']			=	$jobM->getJobNum(array('uid'=>$this->uid,'status'=>0,'state'=>1));
			$company['status2']		=	$resumM->getTalentNum(array('uid'=>$this->uid));
			$company['msgnum']		=	$MsgM -> getMsgNum(array('job_uid'=>$this->uid,'status'=>1,'del_status'=>0));
			$company['look_jobnum']	=	$jobM -> getLookJobNum(array('com_id'=>$this->uid,'com_status'=>0), array('usertype' => $this->usertype));
			$this->yunset("company",$company);
			
			$statis     =   $statisM->vipOver($this->uid, 2);
			
			$this->yunset('addjobnum', $statis['addjobnum']);
			
		} 

		$this->yun_tpl(array('login'));
	}
	
	//显示分站，TODO:后台
	function selsite_action(){
	    if($_POST['keyword']){
	        if ($_POST['keyword'] == '主站' || $_POST['keyword'] == '总站') {
                echo "<option></option><option  value='0'>" . $_POST['keyword'] . "</option>";
            } else {
                $siteM = $this->MODEL('site');
                $where['title']	= array('like',$_POST['keyword']);
                $Site = $siteM->getList($where,array('field'=>'`id`,`title`'));
                if(is_array($Site) && !empty($Site)){
                    $siteHtml = '';
                    foreach($Site as $value){
                        $siteHtml .= '<option></option><option  value='.$value['id'].'>'.$value['title'].'</option>';
                    }
                    echo $siteHtml;
                }else{
                    echo 1;
                }
            }
	    }else{
			echo 0;
		}
	}
	
	//显示业务员
	function selcrm_action(){
	    
	    if($_POST['keyword']){
	        
	        //提取顾问信息
	        $adminM		=   $this->MODEL('admin');

			$whereNew	=	array(
				
				'is_crm'			=>	'1',
				'PHPYUNBTWSTART_A'	=>	'',
				'name'				=>	array('like',trim($_POST['keyword']),'OR'),
				'username'			=>	array('like',trim($_POST['keyword']),'OR'),
				'PHPYUNBTWEND_A'	=>	''
			);
			$gwInfo	 =	 $adminM -> getList($whereNew);
	        
			if(is_array($gwInfo) && !empty($gwInfo)){
	            $guwen  =   '';
	            foreach($gwInfo as $value){
 	                
	                $guwen .='<option value="'.$value['uid'].'">'.$value['name'].'</option>';
	            }
	            echo $guwen;
	        }else{
	            echo 1;
	        }
	    }else{
			echo 0;
		}
	}
	
	//公共二维码跳转
	function pubqrcode_action(){
	    
	    $wapUrl = Url('wap');
	    ob_clean();
	    
	    $toc         = isset($_GET['toc']) ? $_GET['toc'] : '';
	    $toa         = isset($_GET['toa']) ? $_GET['toa'] : '';
	    $totype      = isset($_GET['totype']) ? $_GET['totype'] : '';
	    $twarr       = array('job','resume','company','article','announcement','jobtel','parttel','comtel','part','register','gongzhao','zph');
	    $sy_ewm_type = isset($this -> config['sy_ewm_type']) ? $this -> config['sy_ewm_type'] : '';
	    
	    if(in_array($sy_ewm_type, array('weixin','xcx')) && in_array($toc, $twarr) && $totype != 'wxpubtool'){
	        // 使用场景码的功能
	        $WxM	=	$this -> MODEL('weixin');
	        
	        $qrcode =	$WxM->pubWxQrcode($toc,$_GET['toid'],$sy_ewm_type);
	        if($qrcode){
	            
	            $imgStr	=	CurlGet($qrcode);
	            
	            header("Content-Type:image/png");
	            
	            echo $imgStr;
	            
	        }
	        
	    }else{
	        if( isset($_GET['toid']) && $_GET['toid'] != ''){
	            $wapUrl = Url('wap',array('c'=>$toc,'a'=>$toa,'id'=>(int)$_GET['toid']));
	        }
	        if($toc == 'register'){
	            $wapUrl = Url('wap',array('c'=>$toc,'uid'=>(int)$_GET['touid']));
	        }
	        include_once LIB_PATH."yunqrcode.class.php";
	        YunQrcode::generatePng2($wapUrl,4);
	    }
	}

	// wap公共二维码跳转
	function wappubqrcode_action(){
		$wapUrl = Url('wap',array('type'=>1));

		if( isset($_GET['toid']) && $_GET['toid'] != ''){

		    $wapUrl = Url('wap',array('c'=>$_GET['toc'],'a'=>$_GET['toa'],'id'=>(int)$_GET['toid']));
		}
		include_once LIB_PATH."yunqrcode.class.php";
		YunQrcode::generatePng2($wapUrl,4);
	}
	// 查询职位描述模板
	function getcontent_action(){
		$categoryM	=	$this->MODEL('category');
		$ids		=	@explode(',',$_POST['ids']);		
		$rows		=	$categoryM->getJobClassList(array('id'=>array('in',pylode(',',$ids),'content'=>array('<>',''),'orderby'=>'sort,asc')));
		if($rows&&is_array($rows)){
			$content		=	array();
			foreach($rows as $k=>$val){
				if(!empty($val['content'])){
					$content[]	=	$val['id']."###".$val['name']; 
				}
			}
			echo @implode('@@@@',$content);die;
		}
	} 
	// 职位描述模板-内容返回
	function setexample_action(){
		$categoryM	=	$this->MODEL('category');
		$row		=	$categoryM->getJobClass(array('id'=>intval($_POST['id'])),'`content`');
		if($row['content']){
			echo $row['content'];die;
		}
	}

	// WAP跳转（资讯静态页面使用）
	function wjump_action(){
		if (isMobileUser()){
			if($_GET['url']){
				$getType 		= 	explode(';',$_GET['url']);

				foreach($getType as $value){

					$typeSon	= 	explode(',',$value);
					$getArr[$typeSon[0]]	= 	$typeSon[1];
				}
			}
			if(!empty($getArr)){
				echo 'document.write("<script>window.location.href=\''.Url('wap',$getArr).'\';</script>")';
			}else{
				echo 'document.write("<script>window.location.href=\''.Url('wap').'\';</script>")';
			}	
		}
	}
	
	function redeem_city_action(){
		include(PLUS_PATH."city.cache.php");
		if(is_array($city_type[$_POST['id']])){
		    $data   =   '';
			foreach($city_type[$_POST['id']] as $v){
				if($_POST['type']=="province"){
					$data.='<li><a href="javascript:;" onclick="redeem_city(\''.$v.'\',\'city\',\''.$city_name[$v].'\',\'three_city\');">'.$city_name[$v].'</a></li>';
				}else{
					$data.='<li><a href="javascript:;" onclick="redeems(\''.$v.'\',\'three_city\',\''.$city_name[$v].'\');">'.$city_name[$v].'</a></li>';
				}
			}
			echo $data;
		}
	}
	
	function ajax_once_city_action(){
		if($_POST['ptype']=='city'){
			include(PLUS_PATH."city.cache.php");
			if(is_array($city_type[$_POST['id']])){
				$data   =   '<ul class="once_citylist">';
				foreach($city_type[$_POST['id']] as $v){
					if($_POST['gettype']=="citys"){
						$data.='<li><a href="javascript:;" onclick="select_once_city(\''.$v.'\',\'citys\',\''.$city_name[$v].'\',\'three_city\',\'city\');">'.$city_name[$v].'</a></li>';
					}else{
						$data.='<li><a href="javascript:;" onclick="selects_once(\''.$v.'\',\'three_city\',\''.$city_name[$v].'\');">'.$city_name[$v].'</a></li>';
					}
				}
				$data.='</ul>';
			}
			echo $data;die;
		}
	}

	//两次推荐职位、简历的时间间隔判断
	function ajax_recommend_interval_action(){

		$recommendM	=	$this->MODEL('recommend');
		if($_POST['uid'] && $_POST['uid'] == $this->uid){

			if(isset($this->config['sy_recommend_day_num']) && $this->config['sy_recommend_day_num'] > 0){

				$num = $recommendM->getRecommendNum(array('uid'=>$this->uid));

				if($num >= $this->config['sy_recommend_day_num']){

					$data['msg'] = "每天最多推荐{$this->config['sy_recommend_day_num']}次职位/简历！";
					$data['status'] = 1;
					echo json_encode($data);
					exit;
				}
			}else{
				$data['msg'] = "推荐功能已关闭！";
				$data['status'] = 1;
				echo json_encode($data);
				exit;
			}

			if(isset($this->config['sy_recommend_interval']) && $this->config['sy_recommend_interval'] > 0){

				$row = $recommendM->getInfo(array('uid'=>$this->uid,'orderby'=>'addtime'));
				
				if(isset($row['addtime']) && (time() - $row['addtime']) < $this->config['sy_recommend_interval']){

					$needTime 	= 	$this->config['sy_recommend_interval'] - (time() - $row['addtime']);

 					if($needTime>60){
						$h = floor(($needTime % (3600*24)) / 3600);
						$m = floor((($needTime % (3600*24)) % 3600) / 60);
						$s = floor((($needTime % (3600*24)) % 3600 % 60));
						if($h!='0'){
							$needTime	=	$h.'时';
						}else if($m!='0'){
							$needTime	=	$m.'分';
						} 
					}else{
						$needTime		=	$needTime.'秒';
					}
					
					$recs	= 	$this->config['sy_recommend_interval'];
					if($recs>60){
						$h = floor(($recs % (3600*24)) / 3600);
						$m = floor((($recs % (3600*24)) % 3600) / 60);
						$s = floor((($recs % (3600*24)) % 3600 % 60));
						if($h!='0'){
							$recs	=	$h.'时';
						}else if($m!='0'){
							$recs	=	$m.'分';
						} 
					}else{
						$recs		=	$recs.'秒';
					}
					$data['msg'] 	= 	"推荐职位/简历间隔不得少于{$recs}，请{$needTime}后操作！";
					$data['status']	= 	2;
 					echo json_encode($data);exit;
				}
			}
			echo json_encode( array('status' => 0));exit;
		}
	}

	//返回$_POST[cityid]的下级城市<option>代码给layui表单select使用
	function get_city_option_action(){
				
		include(PLUS_PATH."city.cache.php");

		$html = '<option value="">请选择</option>';
		if(!isset($_POST['cityid']) || !isset($city_type[$_POST['cityid']])
			|| count($city_type[$_POST['cityid']]) < 1){
			echo $html;
			exit;
		}
		
		foreach($city_type[$_POST['cityid']] as $cid){
			$cname = isset($city_name[$cid]) && $city_name[$cid] ? $city_name[$cid] : '';
			if($cname != ''){
				$html .= "<option value='{$cid}'>{$cname}</option>";
			}
		}	
		echo $html;
		exit;
	}
	
	 
	
	//工作类别
	function get_job_option_action(){
		
		include(PLUS_PATH."job.cache.php");
		$html = '<option value="">请选择</option>';
		if(!isset($_POST['job1_son']) || !isset($job_type[$_POST['job1_son']])
			|| count($job_type[$_POST['job1_son']]) < 1){
			echo $html;
			exit;
		}
		foreach($job_type[$_POST['job1_son']] as $job1_son){
			$job1_sonname = isset($job_name[$job1_son]) && $job_name[$job1_son] ? $job_name[$job1_son] : '';
			if($job1_sonname != ''){
				$html .= "<option value='{$job1_son}'>{$job1_sonname}</option>";
			}
		}	
		echo $html;
		exit;
		
	}
	
	//天眼查工商数据获取
	function getbusiness_action(){

		if($_POST['name']){
			$noticeM	= 	$this->MODEL('notice');
			$reurn 		= 	$noticeM->getBusinessInfo($_POST['name']);
			
			if(!empty($reurn['content'])){
				$comGsInfo	= 	$reurn['content'];

				echo json_encode($comGsInfo);
			}
		}
	}
	
	//layui上传文件公共方法
	function layui_upload_action()
	{
	    if($_FILES['file']['tmp_name']){

            $data  =  array(
                'name'      =>  $_POST['name'],
                'path'      =>  $_POST['path'],
                'imgid'     =>  $_POST['imgid'],
                'file'      =>  $_FILES['file']
            );

            if ($_POST['notoken']){ //  后台企业列表直接上传LOGO
                $data['notoken']    =   1;
                session_start();
                if (!isset($_SESSION['auid'])){

                    echo json_encode(array('code' => 1, 'msg' => '操作异常，请重试！' ));
                }else{

                    $data['uid']        =   $_POST['uid'];
                    $data['usertype']   =   $_POST['usertype'];
                }
            }else{
                $data['uid']        =   $this->uid;
                $data['usertype']   =   $this->usertype;
            }

	        $UploadM=$this->MODEL('upload');

	        $return = $UploadM->layUpload($data);
	        
	        if (!empty($_POST['name']) && $return['code'] == 0){
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
     * 职位详情页查询信息
     */
	function msgNum_action()
	{
	    
	    $MsgNumM  =  $this->MODEL('msgNum');
	    $msg      =  $MsgNumM->getmsgNum($this->uid,$this->usertype);
	    echo json_encode($msg);
	}

    // 企业每日最大操作次数检查
    public function ajax_day_action_check_action()
    {
        $type   =   isset($_POST['type']) ? $_POST['type'] : '';
        
        // 解决ie9 $.get $.post 回调函数的返回值为undefine
        header("Content-Type: text/html; charset=UTF-8");
        $comM   =   $this->MODEL('company');
        $com_id =   $this->uid;
        $result =   $comM -> comVipDayActionCheck($type, $com_id);
       
        echo json_encode($result);
        die();
    }
	
	/**
	 * @desc   购买弹出框：购买增值包金额数据查询
	 * 19-07-23
	 */
	function getPackPrice_action(){
	    
	    if(empty($_POST['packid'])){
	        echo 0;
	    }
	    $ratingM      =	  $this -> MODEL('rating');
		$statisM      =	  $this -> MODEL('statis');
		
		if ($this->usertype == 2) {
		    
    	    $packinfo =   $ratingM -> getComSerDetailInfo($_POST['packid'], array('field' => '`id`,`service_price`,`type`'));
    	    $pack     =   $ratingM -> getComServiceInfo(array('id'=> $packinfo['type']), array('field' => '`name`'));
		}
		
	    $statis	      =	  $statisM -> getInfo($this->uid, array('usertype' => $this->usertype));
	    
	    $online       =   intval($this->config['com_integral_online']);
	    $pro          =   intval($this->config['integral_proportion']);
        if (!empty($statis)) {
            $rating     =   $statis['rating'];
            $discount   =   $ratingM->getInfo(array('id' => $rating));
        }

        $data           =   array();

        $data['tid']    =   $packinfo['id'];
        $data['name']   =   $pack['name'];
	    
	    if ($online == 3 && !in_array('pack', explode(',', $this->config['sy_only_price']))) {
	        
	        if(!empty($discount['service_discount'])){
	            
	            $data['yh_price']         =   $packinfo['service_price'] * $discount['service_discount'] * 0.01 * $pro;
	            $data['service_price']    =   $packinfo['service_price'] * $pro;
	            $data['price']            =   $data['yh_price'] > $statis['integral'] ? $packinfo['service_price'] * $discount['service_discount'] * 0.01 : $packinfo['service_price'] * $discount['service_discount'] * 0.01 * $pro;
	            $data['style']            =   $data['yh_price'] > $statis['integral'] ? 3 : 2;
	            echo json_encode($data);
	        }else{
	            
	            $data['service_price']     =   $packinfo['service_price'] * $pro;
	            $data['price']             =   $data['service_price'] > $statis['integral'] ? $packinfo['service_price'] : $packinfo['service_price'] * $pro;
	            $data['style']             =   $data['service_price'] > $statis['integral'] ? 3 : 2;
	            echo json_encode($data);
	        }
	    }else {
	        $data['style']     =   1;
	        if(!empty($discount['service_discount'])){
	            
	            $data['yh_price']          =   $packinfo['service_price'] * $discount['service_discount'] * 0.01;
	            $data['service_price']     =   $packinfo['service_price'];
	            $data['price']             =   $data['yh_price'];
	            echo json_encode($data);
	        }else{
	            
	            $data['service_price']     =   $packinfo['service_price'];
	            $data['price']             =   $data['service_price'];
	            echo json_encode($data);
	        }
	    }
	}
	/**
	 * @desc   购买弹出框：购买会员套餐金额数据查询
	 * 19-07-23
	 */
	function getVipPrice_action(){
	    
	    if(empty($_POST['id'])){
	        echo 0;
	    }
		$ratingM   =   $this -> MODEL('rating');
		$statisM   =   $this -> MODEL('statis');
		
		$packinfo  =   $ratingM -> getInfo(array('id' => $_POST['id']), array('field'=>'`id`,`name`,`service_price`,`yh_price`,`time_start`,`time_end`'));
		
		$statis    =   $statisM -> getInfo($this->uid, array('usertype' => $this->usertype, array('field'=>'`integral`')));
		
	    $online    =   intval($this->config['com_integral_online']);
	    $pro       =   intval($this->config['integral_proportion']);

	    $data      	   =   array();
	    
	    $data['id']	   =   $packinfo['id'];
	    $data['name']  =   $packinfo['name'];
	    
	    if ($online == 3 && !in_array('vip', explode(',', $this->config['sy_only_price']))) { // 积分消费
	        
	        if($packinfo['time_start'] < time() && $packinfo['time_end'] > time()){
	            
	            $data['yh_price']          =   $packinfo['yh_price'] * $pro;
	            $data['service_price']     =   $packinfo['service_price'] * $pro;
	            $data['price']             =   $data['yh_price'] > $statis['integral'] ? $packinfo['yh_price'] : $packinfo['yh_price'] * $pro;
	            $data['style']             =   $data['yh_price'] > $statis['integral'] ? 3 : 2;
	            echo json_encode($data);
	        }else{
	            
	            $data['service_price']     =   $packinfo['service_price'] * $pro;
	            $data['price']             =   $data['service_price'] > $statis['integral'] ? $packinfo['service_price'] : $packinfo['service_price'] * $pro;
	            $data['style']             =   $data['service_price'] > $statis['integral'] ? 3 : 2;
	            echo json_encode($data);
	        }
	    }else{
	        
	        $data['style']     =   1;
	        if($packinfo['time_start'] < time() && $packinfo['time_end'] > time()){
	            
	            $data['yh_price']          =   $packinfo['yh_price'];
	            $data['service_price']     =   $packinfo['service_price'];
	            $data['price']             =   $packinfo['yh_price'];
	            echo json_encode($data);
	        }else{
	            
	            $data['service_price']     =   $packinfo['service_price'];
	            $data['price']             =   $packinfo['service_price'];
	            echo json_encode($data);
	        }
	    }
	}
	/**
	 * @desc   购买弹出框：提交订单
	 * 19-07-23
	 */
	function addOrder_action(){
		
	    $_POST				=	$this -> post_trim($_POST);
	    
	    if (empty($_POST)) {
	        echo json_encode(array('error' => 1, 'msg' => '参数错误，请重试！'));die();
	    }
	    $data				=	$_POST;
	    $data['uid']		=   $this -> uid;
	    $data['username']	=   $this -> username;
	    $data['usertype']	=   $this -> usertype;
	    $data['did']		=   $this -> userdid;
	    
	    $compayM            =   $this->MODEL('compay');
	    $return				=	$compayM->orderBuy($data);
	    echo json_encode($return);
	}
	/**
	 * @desc   购买弹出框：积分购买
	 * 19-07-23
	 */
	function integralBuy_action()
	{
	    $_POST             =   $this -> post_trim($_POST);
	    
	    if(empty($_POST)){
	        
	        echo json_encode(array('error'=>1,'msg'=>'参数错误，请重试！'));die;
	    }
	    
	    $data				=	$_POST;
	    $data['uid']		=	$this	->	uid;
	    $data['username']	=	$this	->	username;
	    $data['usertype']	=	$this	->	usertype;
	    
	    $jfdkM				=	$this	->	MODEL('jfdk');
	    $return				=	$jfdkM	->	dkBuy($data);
	    echo json_encode($return);
    }
	//视频面试详情-职位详情
	function getJobInfo_action(){
		
		$jobM   =   $this -> MODEL('job');
		
		$job	=	$jobM -> getInfo(array('id'=>$_POST['jobid']));
		
		$html	=	"";
		
		$html	.=	"<div class='job_popup_hd'>";
		$html	.=	"<div class='job_popup_hd_name'>".$job['name']."</div>";	
		$html	.=	"<div class='job_popup_hd_dy'>";			
		$html	.=	"<span class='gz'>".$job['job_salary']."</span>";			
		if($job['job_city_two']){
			$html	.=	$job['job_city_one']."-".$job['job_city_two']."<span class='xian'></span>";		
		}
		if($job['job_exp']){
			$html	.=	$job['job_exp']."<span class='xian'></span>";		
		}
		if($job['job_edu']){
			$html	.=	$job['job_edu']."<span class='xian'></span>";		
		}
		$html	.=	"</div>";	
		$html	.=	"<div class='job_popup_hd_fuli'>";		
		$html	.=	"<div class='bq1'>";	
		if($job['urgent_time']>time()){
			$html	.=	"<span class='jjzp_span'>紧急招聘</span>";	
		}
		if($job['rec_time']>time()){
			$html	.=	"<span class='zztj_span'>站长推荐</span>";	
		}
		if(!empty($job['welfare'])){
			
			foreach($job['arraywelfare'] as $key=>$val){
				if(!empty($val)){
					$html	.=	"<span>".$val."</span>";	
				}
			}
		}
		$html	.=	"</div></div></div>";	
		
		$html	.=	"<div class='job_popup_md'><div class='job_popup_ftit'>职位要求</div><ul class='job_popup_yaoqiu'>";	
		if(!empty($job['job_number'])){
				
			$html	.=	"<li><label>招聘人数：</label>".$job['job_number']."</li>";	
		}
		if(!empty($job['job_report'])){
				
			$html	.=	"<li><label>到岗时间：</label>".$job['job_report']."</li>";	
		}
		if(!empty($job['job_age'])){
				
			$html	.=	"<li><label>年龄要求：</label>".$job['job_age']."</li>";	
		}
		if(!empty($job['job_sex'])){
			
			$html	.=	"<li><label>性别要求：</label>".$job['job_sex']."</li>";	
		}
		if(!empty($job['job_marriage'])){
				
			$html	.=	"<li><label>婚况要求：</label>".$job['job_marriage']."</li>";	
		}
		if(!empty($job['job_lang'])){
				
			$html	.=	"<li><label>语言要求：</label>".$job['job_lang']."</li>";	
		}
		$html	.=	"</ul></div>";	
			
		$html	.=	"<div class='job_popup_bm'><div class='job_popup_ftit'>职位描述</div>";	
		$html	.=	"<div>".$job['description']."</div>";	
		$html	.=	"</div>";	
		
		echo $html;die;
	}
	 
	 
 
	// 公招海报
	function getgongzhaoHb_action()
    {
        $whbM   =   $this->MODEL('whb');
        if (!$_GET['hb']) {
            $whb=   $whbM->getWhb(array('type' => 4, 'isopen' => 1, 'orderby' => 'sort,desc'));
            $_GET['hb'] = $whb['id'];
        }
        $data   =   array(
            'hb'    =>  $_GET['hb'] ? $_GET['hb'] : 1,
            'id'    =>  $_GET['id']
        );
        echo $whbM->getGongzhaoHb($data);
    }
	// 获取职位海报
    function getJobHb_action()
    {
        $whbM   =   $this->MODEL('whb');
        if (!$_GET['hb']) {
            $whb=   $whbM->getWhb(array('type' => 1, 'isopen' => 1, 'orderby' => 'sort,desc'));
            $_GET['hb'] = $whb['id'];
        }
        $data   =   array(
//            'hb'    =>  $_GET['hb'] ? $_GET['hb'] : 1,
            'id'    =>  $_GET['id']
        );
        if($_GET['hb']){

            $data['hb'] =   $_GET['hb'];
        }
        echo $whbM->getJobHb($data);
    }

    // 获取企业海报
    function getComHb_action()
    {
        $whbM   =   $this->MODEL('whb');

        if (!$_GET['hb'] && !$_GET['style']) {
            $whb=   $whbM->getWhb(array('type' => 2, 'isopen' => 1, 'orderby' => 'sort,desc'));
            $_GET['hb'] = $whb['id'];
        }

        $data   =   array(
            'uid'   =>  $_GET['uid']
        );

        if($_GET['hb']){

            $data['hb'] =   $_GET['hb'];
        }

        if($_GET['style']){

            $data['style'] =   $_GET['style'];
        }
        if ($_GET['jobids']) {

            $data['jobids'] =   $_GET['jobids'];
        }
        echo $whbM->getComHb($data);
    }

    // 获取邀请注册海报模板列表
    function getInviteRegHbList_action()
    {
        $whbM   =   $this->MODEL('whb');
        $list   =   $whbM->getWhbList(array('type' => 3, 'isopen' => 1, 'orderby' => 'sort,desc'), array('field' => 'id'));

        echo json_encode(['list' => $list]);die;
    }

    // 获取邀请注册海报
    function getInviteRegHb_action()
    {
        $whbM   =   $this->MODEL('whb');
        if (!$_GET['hb']) {
            $whb=   $whbM->getWhb(array('type' => 3, 'isopen' => 1, 'orderby' => 'sort,desc'));
            $_GET['hb'] = $whb['id'];
        }
        $data   =   array(
            'uid'   =>  $this->uid,
            'hb'    =>  $_GET['hb'] ? $_GET['hb'] : 1
        );
        echo $whbM->getInviteRegHb($data);
    }
   
    //获取LOGO（后台生成LOGO或预览LOGO调用）
    function getLogoHb_action()
    {

        $whbM = $this->MODEL('whb');
        $data = array('text' => $_GET['name'], 'hb' => $_GET['hb']);
        if ($_GET['out']){
            $data['out']    =   1;
        }
        echo $whbM->getLogoHb($data);
    }


    //微信发布工具搜索职位
    function getWxpubJob_action(){
    	$jobM	=	$this->MODEL('job');

		$result    =   array();

        if (!empty($_GET['keyword'])) {
            
            $where['state']		=	1;
			$where['status']	=	0;
			$where['r_status']	=	1;


			$_GET['keyword']	=	trim($_GET['keyword']);

			if(is_numeric($_GET['keyword'])){
				$where['id']	=	$_GET['keyword'];
			}else{

				$where['PHPYUNBTWSTART']	=	'';
				$where['name']				=	array('like', $_GET['keyword']);
				$where['com_name']			=	array('like', $_GET['keyword'],'OR');
				$where['PHPYUNBTWEND']	    =	'';

			}

            $jobArr    =   $jobM -> getList($where, array('field' => '`id`,`name`,`com_name`'));
            
            if (!empty($jobArr['list'])) {
                
                $jobList   =   $jobArr['list'];
                foreach ($jobList as $jk => $jv){
                    $result[$jk]['name']    =   $jv['name'];
                    $result[$jk]['value']   =   $jv['id'];
                    $result[$jk]['upname']   =   $jv['com_name'];
                 }
            }
        }
        echo json_encode($result);die;
    }
    //微信发布工具搜索企业
    function getComBySearch_action(){
		$result    =   array();

        if (!empty($_GET['keyword'])) {
            
            $where['r_status']     =   1;

            $where['name']  =   array('like', trim($_GET['keyword']));
        
            $where['limit']     =   10;
            
            $comM      =   $this->MODEL('company');
            
            $comArr    =   $comM -> getList($where, array('field' => '`uid`,`name`'));
            
            if (!empty($comArr)) {
                
                $comList   =   $comArr['list'];
                foreach ($comList as $k => $v){
                    $result[$k]['name']    =   $v['name'];
                    $result[$k]['value']   =   $v['uid'];
                }
            }
        }
        echo json_encode($result);die;
	}
    /**
     * 检测用户名重复性
     */
    function checkUsername_action(){

        $userinfoM	=	$this -> MODEL('userinfo');

        $result     =   $userinfoM -> addMemberCheck(array('username'=>trim($_POST['username'])));

        echo json_encode($result);die;
    }
    // 生成HR年度报告海报
    function lastYearReport_action(){
        if($_GET['uid']){
            $hrlog = $this->MODEL('hrlog');
            echo $hrlog->getLastYearReportHb(array('comid'=>$_GET['uid']));
        }
    }
}
?>