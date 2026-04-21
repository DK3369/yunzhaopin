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
class info_controller extends com_controller{

    /**
     * 获取企业信息
     */
    function getinfo_action()
    {

        $companyM  =  $this->MODEL('company');
        $resumeM  =  $this->MODEL('resume');
        $info      =  $companyM->getInfo($this->member['uid'],array('edit'=>'1','logo'=>'1','utype'=>'user','setname'=>'1'));
        if ($info){
            $temp = $info['not_disturb'] ? explode('-', $info['not_disturb']) : '';
            $info['start_not_disturb'] = isset($temp[0]) ? trim($temp[0]): '';
            $info['end_not_disturb'] = isset($temp[1]) ? trim($temp[1]) : '';
        }

        $comcert   =  $companyM->getCertInfo(array('uid'=>$this->member['uid'],'type'=>3));

        $info['cert_pic']    =  $comcert['check'];
        // 首先根据企业表的认证状态来判断
        if ($info['yyzz_status'] == 1){
            $info['cert_status'] = 1;
        }else{
            $info['cert_status'] =  isset($comcert['status']) ? $comcert['status'] : -1;
        }
        if(!empty($info['content'])){
            $info['content_t']  =  strip_tags($info['content']);
        }

        $cacheM    =  $this->MODEL('cache');
        $cache     =  $cacheM->GetCache('com');
        //福利缓存数据
        if(is_array($cache['comdata']['job_welfare'])){
            foreach($cache['comdata']['job_welfare'] as $k=>$v){
                $welfareData[]	=	$cache['comclass_name'][$v];
				$welfaresy[]	=	array('name'=>$cache['comclass_name'][$v],'ischecked'=>0);
            }
        }
        $welfareArr	 =	array_unique(array_merge($info['arraywelfare'],$welfareData));

        $welfareAll  =  $w  =  array();

        foreach($welfareArr as $k=>$v){
            $w['name']  =  $v;
            if(in_array($v,$info['arraywelfare'])){
                $w['ischecked']	 =	1;
            }else{
                $w['ischecked']	 =	0;
            }
            $welfareAll[]  =  $w;
        }
		if($welfareAll){
			$info['welfareAll']  =  $welfareAll;
        }else{
			$info['welfareAll']  =  $welfaresy;
		}

		$usernum	=	$resumeM->getResumeNum(array('uid'=>$this->member['uid']));
		$info['usernum']		=	$usernum;
		$info['setPosition']=  $this->config['com_enforce_setposition'];

		include(CONFIG_PATH.'db.data.php');

		$info['app_push']   =  $this->member['app_push'];
		$info['usertype'] 	= 	$this->member['usertype'];
        
		// 第三方绑定相关参数
		$userInfoM  =  $this->MODEL('userinfo');
		$member     =  $userInfoM->getInfo(array('uid'=>$this->member['uid']),array('field'=>'`qqid`,`qqunionid`,`wxid`,`wxopenid`,`unionid`,`sinaid`,`maguid`,`qfyuid`'));
		
		$bind  =  array(
		    'qqbind'    =>  0,
		    'wxbind'    =>  0,
		    'sinabind'  =>  0
		);
		if (isset($this->config['sy_qqdt']) && $this->config['sy_qqdt'] == 1 && !empty($member['qqunionid'])){
		    $bind['qqbind']  =  1;
		}elseif (!empty($member['qqid'])){
		    $bind['qqbind']  =  1;
		}
		if ($_POST['provider'] == 'h5'){
		    if (!empty($member['wxid']) || !empty($member['unionid'])){
		        $bind['wxbind']  =  1;
		    }else{
		        if (!empty($member['wxopenid'])){
		            $bind['wxbind']  =  2;
		        }
		    }
		}
		if (!empty($member['sinaid'])){
		    $bind['sinabind']  =  1;
		}
		$info = array_merge($info, $bind);

        $this->render_json(0, 'ok', $info);
    }

	function saveinfo_action()
	{
		$companyM 	=   $this->MODEL('company');

		if(isset($this->comInfo['r_status'])){
		    $rstaus  =  $this->comInfo['r_status'];
		}else{
		    $rstaus  =  $this->config['com_status'];
		}

		$mData     =  array(
			'moblie'        =>  $_POST['linktel'],
			'email'         =>  $_POST['linkmail']
		);
        $source = $_POST['source'];
        if (!empty($_POST['source']) && $_POST['source'] == 'wap') {
            $source = 2;
        }
		$setData	=	array(
			'name' 			=> 	$_POST['name'],
			'shortname' 	=> 	$_POST['shortname'],
			'hy' 			=> 	$_POST['hy'],
			'pr' 			=> 	$_POST['pr'],
			'mun' 			=> 	$_POST['mun'],
			'provinceid' 	=>	$_POST['provinceid'],
			'cityid' 		=> 	$_POST['cityid'],
			'three_cityid' 	=> 	$_POST['three_cityid'],
			'address' 		=>	$_POST['address'],
		    'x' 			=>	$_POST['x'],
		    'y' 			=>	$_POST['y'],
			'linkman'		=> 	$_POST['linkman'],
			'linktel'		=>	$_POST['linktel'],
			'linkphone' 	=> 	$_POST['linkphone'],
			'linkmail' 		=> 	$_POST['linkmail'],
			'sdate' 		=> 	$_POST['sdate'],
			'moneytype' 	=> 	$_POST['moneytype'],
			'money' 		=>	$_POST['money'],
			'linkjob' 		=> 	$_POST['linkjob'],
			'linkqq' 		=> 	$_POST['linkqq'],
			'website' 		=> 	$_POST['website'],
			'busstops' 		=> 	$_POST['busstops'],
			'infostatus' 	=> 	$_POST['infostatus'],
			'welfare' 		=> 	$_POST['welfare'],
		    'photos' 		=> 	$_FILES['photos'],
			'preview'       =>  $_POST['preview'],
			'lastupdate'	=>  time(),
		    'r_status' 		=> 	$rstaus,
			'content'		=> 	str_replace(array('&amp;','background-color:#ffffff','background-color:#fff','white-space:nowrap;'),array('&','background-color:','background-color:','white-space:'),$_POST['content'])
		);
        if ($_POST['notStartDisturb'] && $_POST['notEndDisturb'] && strtotime($_POST['notStartDisturb']) && strtotime($_POST['notEndDisturb'])){
            $setData['not_disturb'] = $_POST['notStartDisturb'].' - '.$_POST['notEndDisturb'];
        }
		$userinfoM    =   $this->MODEL("userinfo");
		if(empty($this -> comInfo['uid'])){

		    $userinfoM -> activUser($this->member['uid'],2);
		}

		$return =  $companyM -> setCompany(array('uid'=>$this->member['uid']),array('mData'=>$mData,'comData'=>$setData,'utype'=>'user','wap'=>1,'source'=>$source));

		if($return['errcode'] == 9){
			$error	=  1;
		}else{
		    $error	=  2;
		}
		$msg  =  preg_replace('/\([^\)]+?\)/x','',$return['msg']);
		$this->render_json($error, $msg);
	}

	function savephoto_action()
	{
		$companyM	=	$this->MODEL('company');

		if(empty($this->member['uid'])){
		    $error	=	3;
		    $msg	=	'参数不正确';
		}else{
		    if(empty($this -> comInfo['uid'])){
		        $userinfoM  =  $this->MODEL("userinfo");
		        $userinfoM -> activUser($this->member['uid'],2);
		    }

		    $data['utype'] = 'user';
            $data['source'] = $_POST['source'];
		    if($_POST['source'] == 'wap'){
                $data['source'] = 2;
                $data['base'] = $_POST['uimage'];
            }else{
                $data['photo'] = $_FILES['photos'];
            }

		    $return	=  $companyM -> upLogo(array('uid'=>$this->member['uid']), $data);
			$msg    =  preg_replace('/\([^\)]+?\)/x','',$return['msg']);
		}
		$this->render_json(1, $msg);
	}

    function show_action()
    {
        $companyM	=	$this->MODEL('company');

        if(empty($this->member['uid'])){
            $this->render_json(3, '参数不正确');
        }else{
            $where['uid']	=	$this->member['uid'];
            !empty($_GET['pageSize']) && $where['limit'] = $_GET['pageSize'];
            $where['orderby']		=	'sort,desc';

            $List					=	$companyM->getCompanyShowList($where,array('field'=>'`title`,`id`,`picurl`'));

            $this->render_json(0, 'ok', array('list' => $List));
        }
    }

    function showsave_action()
    {
        $companyM	=	$this->MODEL('company');

        if(empty($this->member['uid'])){
            $error	=	3;
            $msg	=	'参数不正确';
        }else{
            if($_POST['wappic']==1){
                $data['base'] = $_POST['uimage'];
            }else {
                $data['file'] = $_FILES['pic'];
            }
            $data['uid'] = $this->member['uid'];

            $data['r_status'] = $this->comInfo['info']['r_status'];

            $return = $companyM->setShow($data);
            if ($return['errcode'] == '9') {
                $error = 1;
            } else {
                $error = 2;
            }
            $msg = preg_replace('/\([^\)]+?\)/x', '', $return['msg']);
        }

        $this->render_json($error, $msg, array('id' => !empty($return['id']) ? $return['id'] : ''));
    }

    function showdel_action(){
        $companyM	=	$this->MODEL('company');
        $logM		=	$this->MODEL('log');

        if(empty($this->member['uid']) || empty($_POST['id'])){
            $error	=	3;
            $msg	=	'参数不正确';
        }else{
            $oid	=	$companyM->delCompanyShow(is_array($_POST['id']) ? $_POST['id'] : explode(',', $_POST['id']),array('uid'=>$this->member['uid']));

            if($oid){
                $logM->member_log("删除企业环境展示",16,3, $this->member['uid'], $this->member['usertype']);//会员日志

                $error	=	1;
                $msg	=	'删除成功';
            }else{
                $error	=	2;
                $msg	=	'删除失败';
            }
        }
        $this->render_json($error, $msg);
    }
	function delcomqcode_action(){
        if(empty($this->member['uid'])){
            $error	=	3;
            $msg	=	'参数不正确';
        }else{
            $comM      =   $this->MODEL('company');
            $return = $comM->upInfo($this->member['uid'],array(),array('comqcode'=>''));
            if($return){
                $error	=	1;
                $msg	=	'删除成功';
            }else{
                $error	=	2;
                $msg	=	'删除失败';
            }
        }
        $this->render_json($error, $msg);
    }
}
?>