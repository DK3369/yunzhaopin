<?php

class redeem_model extends model
{
	/**
     * @desc   引用log类，添加用户日志   
     */
    private function addMemberLog($uid,$usertype,$content,$opera='',$type='') {
        require_once ('log.model.php');
        $LogM = new log_model($this->db, $this->def);
        return  $LogM -> addMemberLog($uid,$usertype,$content,$opera='',$type=''); 
    }
	//查询单条商品
    function getInfo($where=array(),$data=array()){
		
		$data['field']  =	empty($data['field']) ? '*' : $data['field'];
		
		$info	=	$this -> select_once('reward',$where,$data['field']);
		if(!empty($info)){
			if($info['pic']){
				$info['pic']	= checkpic($info['pic']);
			}
			if($info['content']){
				$content=htmlspecialchars_decode($info['content']);
				$info['content_n'] = $content;
				preg_match_all('/<img(.*?)src=("|\'|\s)?(.*?)(?="|\'|\s)/',$content,$res);
				if(!empty($res[3])){
					foreach($res[3] as $v){
						if(strpos($v,'http:')===false && strpos($v,'https:')===false){
							$ossv  			   = checkpic($v);
							$info['content_n'] = str_replace($v,$ossv,$content);
						}
					}
				}
			}
            $info['nid'] = $info['nid']?$info['nid']:'';
            $info['tnid'] = $info['tnid']?$info['tnid']:'';
		}
		return $info;
    }

	//查询商品列表
    function getList($whereData,$data=array()){
		
		$data['field']  =	empty($data['field']) ? '*' : $data['field'];
		
		$List			=	$this->select_all("reward",$whereData,$data['field']);
		
		if(!empty( $List )){
			
			foreach($List as $key=>$val){
				$nids[] 		= 	$val['nid'];
				$tnids[]	 	= 	$val['tnid'];
			}
			
			$classWhere['id']	=	array('>',0);
			$class				=	$this->select_all("redeem_class",$classWhere);
			
			$classname='';
			foreach($List as $k=>$v){
        		foreach($class as $val){
        			if($v['nid']==$val['id']){
        				$classname			=	$val['name'];
        			}
					
					if($v['tnid']==$val['id']){
						$classname			=	$List[$k]['classname'].'-'.$val['name'];
					}
					
					$List[$k]['classname']	=	$classname;
        		}
                $List[$k]['status_n'] = $v['status'] == 1 ? true : false ;
                $List[$k]['rec_n'] = $v['rec'] == 1 ? true : false ;
                $List[$k]['hot_n'] = $v['hot'] == 1 ? true : false ;
        	}
		}
		
		return $List;
	}
	
	function upInfo($data = array(),$whereData)
	{
		if(!empty($whereData)){
			$nid	=	$this -> update_once('reward',$data,$whereData);
		}
		return $nid;
	}

	function delReward($whereData,$data){
		
		if($data['type']=='one'){//单个删除
			
			$limit	=	'limit 1';
		}
		
		if($data['type']=='all'){//多个删除
		
			$limit	=	'';
		}
		
		$result	=	$this	->	delete_all('reward',$whereData,$limit);
		
		return	$result;
		
	}

	function addInfo($setData){

		if(!empty($setData)){
			
			$nid	=	$this -> insert_into('reward',$setData);
			
		}

		return $nid;
	}
	function getChangeList($whereData,$data=array()){
		$ListNew		=	array();
		$data['field']  =	empty($data['field']) ? '*' : $data['field'];
		$List			=	$this -> select_all('change',$whereData,$data['field']);
		
		if(!empty( $List )){
		    
		    $gid  =   array();
		    
			foreach($List as $key=>$val){
			
			    $gid[]   =   $val['gid'];

			}
			
			require_once ('redeem.model.php');
			$redeemM     =   new redeem_model($this->db, $this->def);
			$gift        =   $redeemM->getList(array('id'=>array('in',pylode(',', $gid))),array('field'=>'id,pic'));
			
			$dh = $sh = $wtg =0;
			foreach($List as $key=>$val){
			    $List[$key]['wapredeem_url'] = Url('wap',array('c'=>'redeem','a'=>'show','id'=>$val['gid']));
			    $List[$key]['ctime_n'] = date('Y-m-d h:i',$val['ctime']);
			    if($val['body']){
			        $List[$key]['address'] = mb_substr(trim($val['body']),5,-1);
                }else{
                    $List[$key]['address']="";
                }
                foreach ($gift as $v){
                    if($val['gid']==$v['id']){
                        $List[$key]['pic']	=	checkpic($v['pic'],$this->config['sy_imgsc_mr']);
                    }
                }
				
				if($data['utype']=='wap'){
				    
					
					if($val['status']==0){
						$sh   =   $sh + 1;
					}
					
					if($val['status']==2){
						$wtg  =   $wtg + 1;
					}
					if($val['status']==1){
						$dh   =   $dh + 1;
					}		
				}
			}
            $ListNew['dh']		=	$dh;
            $ListNew['wtg']		=	$wtg;
            $ListNew['sh']		=	$sh;
			$ListNew['list']	=	$List;
			
		}

		return	$ListNew;
	}
	
	function getChangeInfo($whereData, $data = array()){
		
		if($whereData){
			$data['field']  =	empty($data['field']) ? '*' : $data['field'];
		
			$List	=	$this -> select_once('change',$whereData,$data['field']);
		}

		return $List;
	
	}
	function AddChange($data = array()){
		$num	=	(int)$data['num'];
		$id		=	(int)$data['id'];
		
		$info		=	$this->select_once('member',array("uid"=>$data['uid']),'`password`,`salt`');
		
		require_once ('statis.model.php');
		$statisM 	= 	new statis_model($this->db, $this->def);
		$statis		=	$statisM->getInfo($data['uid'],array("usertype"=>$data['usertype'],"field"=>"`integral`"));
		
		$gift		=	$this->getInfo(array("id"=>(int)$data['id']));
		
		$nums		=	$this->select_num('change',array("gid"=>$gift['id'],"uid"=>$data['uid']));
		
		$integral	=	$gift['integral']*$num;
		
		if(!$data['uid'] && !$data['username']){
			$return['msg']		=	yun_at('wap_js_00154');
			$return['errcode']		=	'8';
		}elseif(!$data['linkman'] || !$data['linktel'] ){
			$return['msg']		=	yun_at('common_00718');
			$return['errcode']		=	'8';
		}elseif($data['linktel']&&CheckMobile($data['linktel'])==false){
			$return['msg']		=	yun_at('wap_00306');
			$return['errcode']		=	'8';
		}elseif(!$data['password']){
			$return['msg']		=	yun_at('wap_01273');
			$return['errcode']		=	'8';
		}elseif(!passCheck($data['password'],$info['salt'],$info['password'])){
			$return['msg']		=	yun_at('model_00066');
			$return['errcode']		=	'8';
		}elseif($num<1){
			$return['msg']		=	yun_at('common_01154');
			$return['errcode']		=	'8';
		}elseif($num>$gift['stock']){
			$return['msg']		=	yun_at('common_01250');
			$return['errcode']		=	'8';
		}elseif($gift['restriction']!='0' && $nums+$num>$gift['restriction']){
			$return['msg']		=	yun_at('common_01251');
			$return['errcode']		=	'8';
		}elseif($statis['integral']<$integral){
			$return['msg']		=	yun_at('wap_js_00157').$this->config['integral_pricename'].'wap_js_00156';
			$return['errcode']		=	'8';
		}else{
			require_once ('integral.model.php');
			$integralM = new integral_model($this->db, $this->def);
			//积分操作记录
			$integralM->company_invtal($data['uid'],$data['usertype'],$integral,false,"".$this->config['integral_pricename'].'admin_yunying_00117',true,2,'integral',24);
      if($data['bodyt']){
         $data['body']=$data['bodyt'];
      }else{
        $data['body']=yun_at('wap_js_00155').$data['provinceid'].' '.$data['cityid'].' '.$data['three_cityid'];
        if($data['address']){
          $data['body'].=' '.$data['address'];
        }
        if($data['other']){
          $data['body'].=yun_at('wap_01697').$data['other'];
        }
      }


			$post=array(
				'uid'		=>	$data['uid'],
				'username'	=>	$data['username'],
				'usertype'	=>	$data['usertype'],
				'name'		=>	$gift['name'],
				'gid'		=>	$gift['id'],
				'linkman'	=>	$data['linkman'],
				'linktel'	=>	$data['linktel'],
				'body'		=>	$data['body'],
				'integral'	=>	$integral,
				'num'		=>	$num,
				'ctime'		=>	time()
			);
			$this->insert_into('change',$post);
			
			$this->update_once('reward',array('num'=>array('+',$num),'stock'=>array('-',$num)),array("id"=>$data['id']));

            $this->addMemberLog($data['uid'], $data['usertype'], $this->config['integral_pricename'].'兑换：兑换商品（ID：'.$gift['id'].'）' , 17, 1);

			$return['msg']=yun_at('model_00051');
			$return['errcode']='9';
			if($data['utype']=='pc'){
				$return['url']=Url('redeem',array('c'=>'show','id'=>$id));
			}
			if($data['utype']=='wap'){
				$return['url']=Url('wap',array('c'=>'redeem','a'=>'show','id'=>$id));
			}
		}
		return $return;
	}

	function getChangeNum($whereData, $data = array()){
		
		if($whereData){
			$data['field']  =	empty($data['field']) ? '*' : $data['field'];
		
			$List	=	$this -> select_num('change',$whereData,$data['field']);
		}

		return $List;
	
	}

	function addChangeInfo($setData){

		if(!empty($setData)){
			
			$nid	=	$this -> insert_into('change',$setData);
			
		}

		return $nid;
	
	}

	function upChangeInfo($whereData, $data = array()){

		if(!empty($whereData)){
			
			$nid	=	$this -> update_once('change',$data,$whereData);
			
		}

		return $nid;
	
	}
	
	 function delChange($whereData,$data=array())
	{
		$limit = "";//多个删除
		if(!is_array($whereData['id'])){
		    $limit  =  'limit 1';
		}
		
		/**$data['member']	:会员中心执行删除，member为user表示个人会员，com表示企业会员
		*  $data['uid']	  	:用户uid
		*  $data['usertype']:用户usertype
		*  $data['id']	  	:change表id
		*/

		if($data['member']){
			if($data['uid']==''
				||($data['member']=='com'&&$data['usertype']!='2')
				||($data['member']=='user'&&$data['usertype']!='1')){

				$result['msg']	=	yun_at('common_02124');
				$result['cod']	=	8;
			}else{
				$rows	=	$this	->	getChangeInfo(array('uid'=>$data['uid'],'id'=>$data['id']));
				if($rows['id']){
					require_once('integral.model.php');
					$IntegralM	=	new integral_model($this->db,$this->def);
					
					$this	->	update_once('reward',array('num'=>array('-',$rows['num']),'stock'=>array('+',$rows['num'])),array('id'=>$rows['gid']));

					$IntegralM	->	company_invtal($data['uid'],$data['usertype'],$rows['integral'],true,'common_06573',true,2,'integral',24);
					$this		->	delete_all('change',array('uid'=>$data['uid'],'id'=>$data['id']),$limit);
				}
                $this->addMemberLog($data['uid'], $data['usertype'], $this->config['integral_pricename'] . 'common_06574', 17, 3);//会员日志
				$result['msg']	=	yun_at('wap_01290');
				$result['cod']	=	9;
			}
		}else{
			$result	=	$this -> delete_all('change',$whereData,$limit);
		}
		
		return	$result;
		
	}
	//************************end************************
	

	//*********************redeem_class******************
	//查询商品分类
    function GetRewardClass($whereData,$data=array()){
		$ListNew		=	array();
		$data['field']  =	empty($data['field']) ? '*' : $data['field'];
		$List			=	$this -> select_all('redeem_class',$whereData,$data['field']);
		
		if(!empty( $List )){
			
			$ListNew['list']	=	$List;
		}

		return	$ListNew;
		
    }
	
	function getRedeemClassInfo($whereData, $data = array()){
		
		if($whereData){
			$data['field']  =	empty($data['field']) ? '*' : $data['field'];
		
			$List	=	$this -> select_once('redeem_class',$whereData,$data['field']);
		}

		return $List;
	
	}

	function addRedeemClassInfo($setData){

		if(!empty($setData)){
			
			$nid	=	$this -> insert_into('redeem_class',$setData);
			
		}

		return $nid;
	
	}

	function upRedeemClassInfo($whereData, $data = array()){

		if(!empty($whereData)){
			
			$nid	=	$this -> update_once('redeem_class',$data,$whereData);
			
		}

		return $nid;
	
	}
	
	
	function delRedeemClass($whereData,$data){
		
		if($data['type']=='one'){//单个删除
			
			$limit		=	'limit 1';
			
		}
		
		if($data['type']=='all'){//多个删除
		
			$limit		=	'';
			
		}
		
		$result			=	$this	->	delete_all('redeem_class',$whereData,$limit);
		
		return	$result;
		
	}
	//************************end************************
}
?>