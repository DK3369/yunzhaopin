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
class info_controller extends user_controller{
    
	/*wxapp基本信息页面显示、创建简历页面显示*/
	function getinfo_action()
	{
		$resumeM	=	$this->MODEL('resume');
		$resume		=	$resumeM -> getResumeInfo(array('uid'=>$this->member['uid']),array('logo'=>1,'setname'=>1));

		$data		=	count($resume) ? $resume : array();
		$data['resumename'] = !empty($this->config['sy_resumename_num']) ? $this->config['sy_resumename_num'] : 0;
		$this->render_json(0,'',$data);
	}
	/*wxapp保存基本信息*/
	function saveinfo_action()
	{
		$resumeM  	=  	$this -> MODEL('resume'); 

    	$resume 	= 	$resumeM->getResumeInfo(array('uid'=>$this->member['uid']));

		$mData	=	array(
			'moblie'		=>	$_POST['telphone'],
			'email'			=>	$_POST['email'],
		);
		$rData	=	array(
			'name'			=>	$_POST['name'],
			'sex'			=>	$_POST['sex'],
			'birthday'		=>	$_POST['birthday'],
			'edu'			=>	$_POST['edu'],
			'exp'			=>	$_POST['exp'],
			'telphone'		=>	$_POST['telphone'],
			'marriage'		=>	$_POST['marriage'],
			'height'		=>	$_POST['height'],
			'weight'		=>	$_POST['weight'],
			'nationality'	=>	$_POST['nationality'],
			'domicile'		=>	$_POST['domicile'],
			'address'		=>	$_POST['address'],
			'homepage'		=>	$_POST['homepage'],
			'living'		=>	$_POST['living'],
			'email'			=>	$_POST['email'],
			'phototype'     =>  $_POST['phototype'],
			'nametype'      =>  $_POST['nametype'],
			'wxid'			=>	$_POST['wxid'],
			'ewmFromWx' 	=> 	$_FILES['photos'],//二维码
		    'lastupdate'	=>  time()
		);
		if(resumeTimeState($this->config['user_revise_state']) == '0'){
            $rData['state'] = 0 ;
        }
        if(!$resume['photo'] || ($resume['defphoto']==2 && $rData['sex']!=$resume['sex'])){
			$logo = $resumeM->deflogo($rData);
			if($logo!=''){
				$rData['photo'] 		= $logo;
    			$rData['defphoto'] 		= 2;
    			$rData['photo_status'] 	= 0;
			}
    		
    	}
		if($this->member['usertype'] == 0){
			$userinfoM    =   $this->MODEL("userinfo");
			$userinfoM -> activUser($this->member['uid'],1);
		}
		$resumeM  =  $this->MODEL('resume');
		$return   =  $resumeM -> upResumeInfo(array('uid'=>$this->member['uid']),array('mData'=>$mData,'rData'=>$rData,'utype'=>'user','source'=>$_POST['source'], 'port' => $port));
		$data['error']	=	$return['errcode']==9 ? 1 : 2;
		$this -> render_json($data['error'],$return['msg'],'');
	}
	/*wxapp保存头像*/
	function savephoto_action()
	{
		
		$resumeM  		=	$this		-> MODEL('resume');


		$upData = array('utype'=>'user','photo'=>'','source'=>$_POST['source'],'preview'=>1);
		if (!empty($_POST['source']) && $_POST['source'] == 'wap'){
		    $upData['base'] = $_POST['uimage'];
        }else{
            $upData['photo'] = $_FILES['photos'];
        }

		$return   		=   $resumeM	-> upPhoto(array('uid'=>$this->member['uid']), $upData);

		$data['error']	=	$return['errcode']==9	?	1	:	2;
		
		$this -> render_json($data['error'],$return['msg'],$return['picurl']);

	}
	
}