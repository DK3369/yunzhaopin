<?php

class link_model extends model{
	function get_cache(){
		include(LIB_PATH."cache.class.php");
		$cacheM		= 	new cache(PLUS_PATH,$this);
		$makecache	=	$cacheM->link_cache("link.cache.php");
	}
	/**
	* @desc   获取友情链接列表
	* @param  $whereData:查询条件
	* @param  $data:自定义处理数组 
	*/
	public function getList($whereData,$data=array()) {        
       
		$select  =   $data['field'] ? $data['field'] : '*';     
      
		$List    =   $this -> select_all('admin_link',$whereData,$select);
        require_once('cache.model.php');
        $cacheM = new cache_model($this->db,$this->def);
        $domain  =	$cacheM -> GetCache('domain',$Options=array('needreturn'=>true,'needassign'=>true,'needall'=>true));
		if(!empty($List)){
			foreach($List as $k => $v){
				if($v['pic']){
					$List[$k]['pic']	=	checkpic($v['pic']);
				}
                $List[$k]['ctime_n'] = date('Y-m-d', $v['link_time']);
				if ($v['link_type'] == 1){
                    $List[$k]['link_type_n'] = yun_at('admin_01013');
                }else{
                    $List[$k]['link_type_n'] = yun_at('admin_00100');
                }
                foreach ($domain['Dname'] as $dk=>$dv){
                    if ($v['did'] == $dk){
                        $List[$k]['did_n'] = $dv;
                    }
                }
			}
		}
		
	   return $List;  
    
	}
	/**
	* @desc   获取工具箱详情
	*/
	public function getInfo($where=array(),$data	=	array()){
		
		$select   =   $data['field'] ? $data['field'] : '*';	
			
		$Info	  =	 $this -> select_once('admin_link',$where, $select);
		
		if($Info['pic']){
			$Info['pic_n']	=	checkpic($Info['pic']);
		}
		
		return $Info;
	}
	/**
	* @desc   审核友情链接
	*/
	 function setLinkStatus($id,$data=array())
    {
        if($id){
			
			$return['id']	=	$this->update_once("admin_link",array('link_state'=>$data['status'],'statusbody'=>$data['statusbody']),array('id'=>$id));
			
			if($return['id']){
				
				$this->get_cache();
				
				$return['msg']		=	yun_at('common_06516');
				$return['errcode']	=	9;
			}else{
				$return['msg']		=	yun_at('common_06517');
				$return['errcode']	=	8;
			}
		}else{
			$return['msg']		=	yun_at('common_06518');
			$return['errcode']	=	8;
		}
		return $return;
    }
    function addInfo($data=array())
    {
		$id		=	$data['id'];
		$post	=	$data['post'];
		if($data['utype']=='index'){
			session_start();
			if(md5(strtolower($data['authcode'])) != $_SESSION['authcode'] || empty($_SESSION['authcode'])){
				unset($_SESSION['authcode']);
				$return['msg']		=	yun_at('wap_js_00109');
				$return['errcode']	=	8;
				return	$return;
			}
			
		}
		if($data['utype']=='admin'){
			if(preg_match("/[^\d-., ]/",$post['link_sorting'])){
				$return['msg']		=	yun_at('common_00811');
				$return['errcode']	=	8;
			}
		}
		if($post['sorting']==""){
			$post['sorting']	=	"0";
		}
		if($post['phototype']==""){
			$post['phototype']	=	"0";
		}
		if($return['msg']==''){
			if($id){
				$return['id']		=	$this -> update_once("admin_link",$post,array('id'=>$id));
				$msg	=	'wap_js_00073';
			}else{
				
				$post['link_time']	=	time();
				$return['id']		=	$this -> insert_into("admin_link",$post);
				$msg	=	'wap_js_00091';
			}
			if($return['id']){
				$this->get_cache();
				if($data['utype']=='index'){
					$return['msg']	=	yun_at('common_01157');
				}
				if($data['utype']=='admin'){
					
					$return['msg']	=	yun_at('model_00211') . $return['id'] . yun_at('model_00130') . $msg . 'wap_js_00104';
				}
				$return['errcode']	=	9;
			}else{
				$return['msg']		=	$msg.'wap_js_00103';
				$return['errcode']	=	8;
			}
		}

		return	$return;
    }
	/**
	* @desc   删除友情链接
	*/
	public function delInfo($id,$data=array()){
		
		if(empty($id)){
           
			return	array(
              
				'errcode' 	=> 	8,
				'msg' 		=> 	yun_at('member_com_00084'),
				'layertype'	=>	0
            );
        
		}else{
			
			if(is_array($id)){
				
				$ids	=	pylode(',',$id);
				$return['layertype']	=	1;
			
			}else{
				$ids	=	$id;
				$return['layertype']	=	0;
			}
			 
			
			$return['id']	=	$this -> delete_all('admin_link',array('id' => array('in',$ids)),'');
			$this->get_cache();
			$return['msg']		=	yun_at('model_00211') . $ids . yun_at('model_00130');
			$return['errcode']	=	$return['id'] ? '9' :'8';
			$return['msg']		=	$return['id'] ? $return['msg'].'admin_user_00187' : $return['msg'].'admin_user_00186';
		}
		return	$return;
	}
	public function setLinkSite($data=array()){
		
		if($data['id']){
			$id 	= 	pylode(',',$data['id']);
			if($id){
				require_once ('site.model.php');
				$siteM 		= 	new site_model($this->db, $this->def);
				$Table 		= 	array('admin_link');
				$siteM->updDid($Table,array('id'=>array('in',$id)),array('did'=>$data['did']));
				
				$this->get_cache();
				
				$return['msg']		=	yun_at('model_00211') . $data['uid'] . yun_at('model_00212');
				$return['errcode']	=	9;
			}else{
				$return['msg']		=	yun_at('admin_user_00030');
				$return['errcode']	=	8;
			}
		}else{
			$return['msg']			=	yun_at('common_01236');
			$return['errcode']		=	8;
		}
		return $return;
	}
	/**
	* @desc 友情链接数目
	*/
	function getLinkNum($where = array()){
		return $this->select_num('admin_link', $where);
	}
}
?>