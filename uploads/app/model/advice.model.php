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
class advice_model extends model{
	
	/**
	 * 获取意见反馈列表
	 * $whereData 	查询条件
	 * $data 		自定义处理数组
	 */
	 
	public function getList($whereData,$data=array()){
        $field	=	empty($data['field']) ? '*' : $data['field'];
		$List		=	$this -> select_all('advice_question',$whereData, $field);
		if($List&&is_array($List)){
			foreach($List as $k=>$v){
			    if ($v['infotype'] == 1){
                    $List[$k]['infotype_n'] = '建议';
                }else if ($v['infotype'] == 2){
                    $List[$k]['infotype_n'] = '意见';
                }else if ($v['infotype'] == 3){
                    $List[$k]['infotype_n'] = '求助';
                }else if ($v['infotype'] == 4){
                    $List[$k]['infotype_n'] = '投诉';
                }else{
                    $List[$k]['infotype_n'] = '建议';
                }
                $List[$k]['ctime_n'] = date('Y-m-d', $v['ctime']);
				$List[$k]['content']=str_replace(array('<!--','-->'),array('&lt;!--','--&gt;'),$v['content']);
				if (mb_strlen($v['content'])>16){
                    $List[$k]['content_n'] = mb_substr($v['content'],0, 16).'...';
                }
			}
		}

		return	$List;
	}
	/**
	 * 获取单条意见反馈
	 * $whereData 	查询条件
	 * $data 		自定义处理数组
	 */
	function getInfo($whereData,$data=array()){
	    
	    $field  =	empty($data['field']) ? '*' : $data['field'];
	    if (!empty($whereData)) {
	        
	        $List  =  $this -> select_once('advice_question',$whereData, $field);
	        return $List;
	    }
	}

    /**
     *users:王旭
     *Data:2023/5/29
     *Time:16:03
     * 获取意见反馈数量
     * @param array $whereData
     * @return array|bool|string|void
     */
	function getAdviceNum($whereData = array()){
        $num = $this->select_num('advice_question', $whereData);
        return $num;
    }
	public function addInfo($data=array()){
		
		if($data['infotype']==''){
			return array('msg'=>'请选择意见类型','errcode'=>8);
		}elseif($data['username']==''){
			return array('msg'=>'请填写联系人姓名','errcode'=>8);
		}elseif($data['mobile']==''){
			return array('msg'=>'请填写联系手机','errcode'=>8);
		}elseif($data['content']==''){
			return array('msg'=>'请填写反馈内容','errcode'=>8);
		}

		require ('notice.model.php');
		$noticeM  =  new notice_model($this->db, $this->def);

		if($data['utype'] != 'wxapp' && $this->config['sy_advice_mobilecode']!=1){

			$result	  =	 $noticeM->jycheck($data['authcode'],'意见反馈');

			if(!empty($result)){

				return array('msg'=>$result['msg'],'errcode'=>'8');

			}
		}
		

		if($this->config['sy_msg_isopen'] && $this->config['sy_msg_login'] && $this->config['sy_advice_mobilecode']==1){	

			if($data['advice_code']==''){

				return array('msg'=>'请填写手机验证码','errcode'=>8);

			}else{

				$companywhere['check']		=		$data['mobile'];
				$companywhere['type']		=		2;
				$companywhere['orderby']	=		array('ctime,desc');
				
				include_once ('company.model.php');
				$CompanyM					=		new company_model($this->db, $this->def);	
				$cert_arr					=		$CompanyM->getCertInfo($companywhere);					
				if (is_array($cert_arr)) {
					$checkTime 				= 		$noticeM->checkTime($cert_arr['ctime']);
					if($checkTime){
						$res 				= 		$data['advice_code'] == $cert_arr['check2'] ? true : false;
						if($res == false){
							return array('msg'=>'短信验证码错误！','errcode'=>'8'); 
						}						
					}else {		
						return array('msg'=>'验证码验证超时，请重新点击发送验证码！','errcode'=>'8'); 			
					}					
				}else {		
					return array('msg'=>'验证码发送不成功，请重新点击发送短信验证码！','errcode'=>'8'); 			
				}
			}
		}
		
		$arr		=	array(
			'username'	=>	$data['username'],
			'ctime'		=>	time(),
			'infotype'	=>	$data['infotype'],
			'content'	=>	$data['content'],
			'mobile'	=>	$data['mobile']
		);
		$nid		=	$this -> insert_into("advice_question",$arr);
		
		if($data['utype']=='pc'){
			$url	=	Url('advice');
		}
		if($data['utype']=='wap'){
			$url	=	Url('wap',array('c'=>'advice'));
		}
		if($nid){
			return array('msg'=>'提交成功，感谢你的反馈！','errcode'=>9,'url'=>$url);
		}else{
			return array('msg'=>'提交失败，请重新填写！','errcode'=>8,'url'=>$url);
		}
		
	}
	/**
	 * 删除意见反馈
	 * $whereData 	查询条件
	 */
	public function delInfo($delId)
	{

	    $return['layertype']	=	0;
		
		if($delId){
		
			if(is_array($delId)){
				$delId	=	pylode(',', $delId);

				$return['layertype']	=	1;
			}
		 
			$return['id']		=	$this->delete_all('advice_question',array('id'=>array('in',$delId)),"");
			
	        $return['msg']		=	'意见反馈(ID:'.$delId.')';
			$return['errcode']	=	$return['id'] ? '9' :'8';
			$return['msg']		=	$return['id'] ? $return['msg'].'删除成功！' : $return['msg'].'删除失败！';
	    }else{
	        $return['msg']		=	'请选择您要删除的意见反馈';
	        $return['errcode']	=	'8';
	    }
	    return $return;
	}
	
	public function statusInfo($data=array() , $whereData=array()){
		if(!empty($data)){
			$nid      					=	$this->update_once('advice_question',$data,$whereData);
			
	        $return['msg']				=	'意见反馈(ID:'.$whereData['id'].')';
			$return['errcode']			=	$nid ? '9' :'8';
			$return['msg']				=	$nid ? $return['msg'].'处理成功！' : $return['msg'].'处理失败！';
	    }else{
	        $return['msg']      		=	'请选择您要处理的意见反馈';
	        $return['errcode']  		=	'8';
	    }
	    return $return;
	}
}
?>