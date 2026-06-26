<?php

class msg_model extends model{
	/**
     * @desc   引用log类，添加用户日志
     */
    private function addMemberLog($uid,$usertype,$content,$opera='',$type='') {
        
        require_once ('log.model.php');
        
        $LogM	=	new log_model($this->db, $this->def);
        
        return  $LogM -> addMemberLog($uid, $usertype, $content, $opera, $type);
        
    }

    /**
     * 求职咨询列表 查询数量
     * @param $whereData
     * @return int
     */
    public function getNum($whereData = array())
    {
        return intval($this->select_num('msg', $whereData));
    }

	/**
	 * 获取求职咨询列表
	 * $whereData 	查询条件
	 * $data 		自定义处理数组
	 */
	 
	public function getList($whereData,$data=array('utype' => 'admin')){
		
		$ListNew	=	array();
		
		$List		=	$this -> select_all('msg',$whereData);

		$rows		=	$this -> select_all('down_resume',array('comid'=>$whereData['job_uid']),'`uid`');
		
		$uids = array();
		foreach ($rows as $v) {
		    $uids[] = $v['uid'];
		}

		if(!empty( $List )){
			
			if(is_array($List)&&$List){
				
			    $uid =   array();
			    
				foreach($List as $v){
					
					$uid[]	=	$v['uid'];
				}
				$resume  =  $this -> select_all('resume_expect',array('uid' => array('in', pylode(',' , $uid)),'defaults' => 1),'`id`,`uid`');
				
				$resumeList  =  $this -> select_all('resume',array('uid'=>array('in', pylode(',' , $uid))));
				foreach($resumeList as $key=>$val){
					if(empty($val['name'])){
						$euids[]	=	$val['uid'];
					}
				}
				// 没有姓名的，显示用户名
				if (!empty($euids)){
				    $nameList  =  $this -> select_all('member',array('uid'=>array('in', pylode(',' , $euids))),'`uid`,`username`');
				    
				    foreach($resumeList as $key=>$val){
				        foreach($nameList as $k=>$v){
				            if($v['uid']==$val['uid']){
				                $resumeList[$key]['name']	=	$v['username'];
				            }
				        }
				    }
				}
				
				include_once('resume.model.php');
				$resumeM  =  new resume_model($this->db, $this->def);
				
				foreach($List as $key => $val){
				
					foreach ($resume as $va){
						
						if($val['uid'] == $va['uid']){
							
							$List[$key]['eid']	=	$va['id'];
							$List[$key]['wapexp_url'] = Url('wap',array('c'=>'resume','a'=>'show','id'=>$va['id']));
						}
					}
					
					foreach ($resumeList as $v){
					    if ($v['uid'] == $val['uid']){
					        // 后台列表或已下载的，直接显示姓名
					        if($data['utype'] == 'admin' || in_array($uid, $uids)){
					            
					            $List[$key]['username']  =   $v['name'];
					            
					        }else{
					            // 未下载的，对姓名进行处理
					            $nameArr = array('nametype' => $v['nametype'], 'name' => $v['name'], 'eid' => $v['def_job'], 'sex' => $v['sex']);
					            
					            $List[$key]['username']  =   $resumeM -> setUsernameShow($nameArr);
					        }
					    }
					}
                    $List[$key]['wapcom_url'] = Url('wap',array('c'=>'company','a'=>'show','id'=>$val['job_uid']));
					$List[$key]['content']	    =	strip_tags(trim($val['content']));
					$List[$key]['datetime_n']	=	date('Y-m-d H:i',$val['datetime']);
                    if ($val['reply_time'] > 0) {
                        $List[$key]['reply_time_n'] = date('Y-m-d H:i', $val['reply_time']);
                    }
                    $job_url = '';
                    if ($val['type'] == 1) {
                        $job_url_arr = array('c' => 'comapply', 'id' => $val['jobid']);
                        if ($data['utype'] == 'admin') {
                            $job_url_arr['look'] = 'admin';
                        }
                        $job_url = Url('job', $job_url_arr);
                    }
                    $com_url = '';
                    if ($val['type'] == 3) {
                    } else {
                        $com_url_arr = array('c' => 'show', 'id' => $val['job_uid']);
                        if ($data['utype'] == 'admin') {
                            $com_url_arr['look'] = 'admin';
                        }
                        $com_url = Url('company', $com_url_arr);
                    }
                    $List[$key]['job_url'] = $job_url;
                    $List[$key]['com_url'] = $com_url;
				}
			}
			$ListNew['list']	=	$List;
		}
		return	$ListNew;
	}
	/**
	 * 获取单条求职咨询
	 * $whereData 	查询条件
	 * $data 		自定义处理数组
	 */
	function getInfo($whereData,$data=array()){
	    
	    $field	=	empty($data['field']) ? '*' : $data['field'];
	    
	    if (!empty($whereData)) {
	        
	        $msg  =  $this -> select_once('msg',$whereData, $field);
	        
	        if($msg && is_array($msg)){
	            
	            $msg['content']         =   strip_tags($msg['content']);
	            
	            if (isset($msg['datetime'])){
	                $msg['datetime_n']	    =	date('Y-m-d H:i',$msg['datetime']);
	            }
	            if (isset($msg['reply_time'])){
	                $msg['reply_time_n']	=	date('Y-m-d H:i',$msg['reply_time']);
	            }
	            return $msg;
	        }
	    }
	}


	public function statusMsg($id, $upData = array())
    {

        $ids    =   @explode(',', trim($id));

        $return =   array('msg' => yun_at('model_00001'), 'errcode' =>  8);

        if (!empty($id)) {

            $idstr      =   pylode(',', $ids);

            $upData     =   array(

                'status'     =>  intval($upData['status']),
                'statusbody'=>  trim($upData['statusbody']),
            );
            if($upData['status'] == 1){
            	$upData['issys'] = 1;
            }

            $mbs   =   $this->getList(array('id' => array('in', $idstr)), array('field' => '`id`,`uid`,`job_uid`,`com_name`,`job_name`,`issys`'));


            $result     =   $this -> update_once('msg', $upData, array('id' => array('in', $idstr)));

            if ($result) {

                if($upData['status'] == 1 || $upData['status'] == 2){

                    $msg    =   array();
                    $commsg =	array();
                    $uids   =   array();
                    $job_uid=	array();
                    

                    foreach ($mbs["list"] as $v){

                        $uids[] =   $v['uid'];
                        if($v['issys']==0 && $upData['status'] == 1){
                        	$job_uid[] = $v['job_uid'];
                        }
                        
                    }

                    foreach ($mbs["list"] as $k => $v){

                    	$jobmsg = '';

                    	if($v['job_name']){
                    		$jobmsg = 'common_06535'.$v['job_name'];
                    	}
                        if ($upData['status'] == 2) {
                        	
                            $statusInfo         =   'common_06536'.$v['com_name'].$jobmsg;

                            if ($upData['statusbody']) {
                                $statusInfo     .=  'wap_00800'.$upData['statusbody'];
                            }

                            $msg[$v['uid']][]   =   $statusInfo;

                        }elseif ($upData['status'] == 1){

                            $msg[$v['uid']][]   =  'common_06537'.$v['com_name'].$jobmsg;
                        }
					}
                }
                
                $return['msg']      =   yun_at('model_00108').$idstr.yun_at('model_00109');
                    
				$return['errcode']  =  9;
                
			}else{

                $return['msg']      =  yun_at('model_00110').$idstr.yun_at('model_00111');
                $return['errcode']  =  8;
            }

        }else {

            $return['msg']          =   yun_at('common_00935');
            $return['errcode']      =   8;
        }

        return $return;
    }
	/**
	 * 添加求职咨询
	 * $data 		自定义数组
	 * 应用中使用到的添加求职咨询
	 */
	public function addMsg($data = array(),$code=''){
		$res                =   array();
		$res['errorcode']   =   8;
		$res['msg']         =   '';

		if(empty($data)){
			$res['msg']         =   yun_at('wap_00203');
			return $res;
		}
		if(empty($data['uid']) || empty($data['username'])){
			$res['msg']		=	yun_at('common_06042');
			return	$res;
		}
		if($data['usertype'] != 1){
			$res['msg']		=	yun_at('common_00785');
			return	$res;
		}

		//参数判断
		if(empty($data['content'])){
			$res['msg']			=	yun_at('common_06538');
			return	$res;
		}
		if($code == ''){
			if(empty($data['authcode'])){
				$res['msg']			=	yun_at('wap_js_00107');
				return	$res;
			}
			session_start();
			if(md5(strtolower($data['authcode'])) != $_SESSION['authcode'] || empty($_SESSION['authcode'])){
				$res['msg']			=	yun_at('model_00047');
				return	$res;
			}
		}
		
		$sql  =  array(
		    'uid'       =>  $data['uid'],
		    'datetime'  =>  time(),
		    'content'   =>  $data['content'],
		    'type'      =>  1,
		    'status'	=>	isset($data['status'])?$data['status']:$this->config['user_msg_status']
		);
		if($sql['status']=='1'){
			$sql['issys'] = 1;
		}else{
			$sql['issys'] = 0;
		}
		
		if (!empty($data['jobid'])){
		    
		    $job  =  $this->select_once('company_job', array('id'=>$data['jobid']),'`id`,`uid`,`name`,`com_name`');
		    
		    $sql['jobid']     =  $job['id'];
		    $sql['job_uid']   =  $job['uid'];
		    $sql['com_name']  =  $job['com_name'];
		    $sql['job_name']  =  $job['name'];
		    
		}else if (!empty($data['job_uid'])){
		    
		    $com  =  $this->select_once('company', array('uid'=>$data['job_uid']),'`uid`,`name`');
		    
		    $sql['job_uid']   =  $com['uid'];
		    $sql['com_name']  =  $com['name'];
		}
		
		//判断是否在黑名单
		include_once ('black.model.php');
        $blackM            		=   new black_model($this->db, $this->def);
		$black					=	$blackM -> getBlackNum(array(
		    'p_uid'				=>	$sql['uid'],
		    'c_uid'				=>	$sql['job_uid']
		));
		if(!empty($black)){
			$res['msg']			=	yun_at('common_00809');
			return	$res;			
		}
		
		$resume					=	$this->select_once("resume",array('uid'=>$sql['uid']),"`name`");
		
		if(!empty($resume)){
			
		    $sql['username']   =   $resume['name'];
			
		}
		
		$id						=	$this -> addInfo($sql);

		if(!empty($id)){

			if($sql['status']=='1'){
				include_once ('sysmsg.model.php');
				$sysmsgM    		=   new sysmsg_model($this->db, $this->def);
				$sysmsgM -> addInfo(array('uid' => $sql['job_uid'],'usertype'=>2,  'content' => yun_at('common_00792')));
			}

			$res['msg']			=	yun_at('common_00903');
			$res['errorcode']   =   9;
			return	$res;	
		}else{
			$res['msg']			=	yun_at('common_06539');
			return	$res;	
		}

	}

	/**
	 * 添加求职咨询
	 * $data 		自定义数组
	 */
	public function addInfo($data=array()){	
		
		return $this -> insert_into('msg', $data);		
	
	}

    /**
     * 删除求职咨询
     * @param $id
     * @param array $data
     * @return mixed
     */
	public function delInfo($id,$data=array()){
		
	    $limit                =  'limit 1';
	    
		$return['layertype']  =	 0;
	    
	    if (!empty($id)){
	        
	        if(is_array($id)){
	            
	            $id  =  pylode(',', $id);
	           
				$return['layertype']  =  1;
	            
				$limit                =  '';
	        }
	        if($data['uid']){
				$delWhere = array('uid'=>$data['uid'],'id'=>array('in',$id));
			}else{
				$delWhere = array('id'=>array('in',$id));
			}
			 
	        $nid      =  $this->delete_all('msg',$delWhere,$limit);
	        
	        if ($nid){
                if ($data['uid']) {

                    $this->addMemberLog($data['uid'], $data['usertype'], yun_at('api_wxapp_00001').$id.yun_at('api_wxapp_00030'), 18, 3);
                }

                $return['msg']      =   yun_at('model_00108').$id.yun_at('model_00112');
                $return['errcode']  =   9;
            } else {

                $return['msg']      =   yun_at('model_00108').$id.yun_at('model_00112');
                $return['errcode']  =   8;
            }
        } else {

            $return['msg']          =   yun_at('common_00815');
            $return['errcode']      =   8;
        }
        return $return;
    }
	
	/**
     * 修改求职咨询回复
	*/
	public function eidtMsg($id,$data=array()){
	    if(!empty($id)){
            $data['status'] = 1;
            $nid = $this -> update_once('msg',$data,array('id'=>$id));
            if($nid){
                $return['msg'] = yun_at('model_00113') . $id . yun_at('model_00114');
                $return['errcode'] = 9;
            }else{
                $return['msg'] = yun_at('common_06540');
                $return['errcode'] = 8;
            }
        }else{
	        $return['msg'] = yun_at('common_01161');
	        $return['errcode'] = 8;
        }
        return $return;
    }
	
	/**
	 * 删除求职咨询
	*/
	public function upInfo($whereData, $data = array()){
		
		if(!empty($whereData)) {
			if($data['usertype']=='1'){
				//个人会员提醒出来，未读状态改成已读状态
				$nid  =  $this -> update_once('msg', array('user_remind_status'=>$data['user_remind_status']),$whereData);
			}else{
			    // 企业删除是软删除
				$PostData	=	array(
					
					'del_status'	=>	'1',
				
				);
				
				$nid  =  $this -> update_once('msg', $PostData, array('id'=>$whereData['id'],'job_uid'=>$whereData['job_uid']));
			}
		}
		
		return $nid;
	
	}
	
	/**
	 * 回复状态
	*/
	public function upReplyInfo($whereData, $data = array()){
		
		if(!empty($whereData)) {
			
			$PostData	=	array(
				
				'reply'					=>$data['reply'],
				
				'reply_time'			=>$data['reply_time'],
				
				'user_remind_status'	=>$data['user_remind_status'],
			
			);
			
			$nid  =  $this -> update_once('msg', $PostData, array('id'=>$whereData['id'],'job_uid'=>$whereData['job_uid']));
		
		}
		
		return $nid;
	
	}
	
	public function delMsg($id, $data = array()){
		
		if(!empty($id)){
            
            if(is_array($id)){
                
                $ids    =	$id;
                
                $return['layertype']	=	1;
                
            }else{
                
                $ids    =   @explode(',', $id);
				
				$return['layertype']	=	0;
                
            }
            
            $id            		=   pylode(',', $ids);

            $return['id']		=	$this -> delete_all('msg',array('id' => array('in',$id),'job_uid'=>$data['job_uid']),'');
            
            $return['msg']		=	yun_at('wap_com_00408');
            
            $return['errcode']	=	$return['id'] ? '9' :'8';
           
		    $return['msg']		=	$return['id'] ? $return['msg'].'admin_user_00187' : $return['msg'].'admin_user_00186';
       
	    }else{
            
			$return['msg']		=	yun_at('common_00921');
            
			$return['errcode']	=	8;
        
		}
        
        return	$return;
	}
	/**
     * @desc    获取职位咨询数目
     */
	function getMsgNum($WhereData = array()){
		
		$Msgnum	=	$this->select_num('msg', $WhereData);
		
		return $Msgnum;
	}
	
}
?>