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
class tiny_controller extends wxapp_controller{
	function list_action(){//微简历列表
		$tinyM		=	$this->MODEL('tiny');
		$where['status']	=	'1';
		$keyword	=	$this->stringfilter($_POST['keyword']);
		$provinceid			=	(int)$_POST['provinceid'];
		$cityid				=	(int)$_POST['cityid'];
		$three_cityid		=	(int)$_POST['three_cityid'];
		$page		=	$_POST['page'];
		$sex		=	(int)$_POST['sex'];
		$edu		=	(int)$_POST['edu'];
		$limit		=	$_POST['limit'];
		$order		=	$_POST['order'];
		$nodata		=	$_POST['nodata'];
		$limit		=	!$limit?10:$limit;
		if($sex){//类别ID
			$where['sex']	=	$sex;
		}
		if($edu){//类别ID
			$where['edu']	=	$edu;
		}
		if($keyword){//关键字
			
			$where['PHPYUNBTWSTART_A']	=	'';
            $where['username']	=	array('like',$keyword);
            $where['job']		=	array('like',$keyword,'OR');
            
            $where['PHPYUNBTWEND_A']	=	'';
		}

		
		if($nodata){//排除没有值的字段
			$nodataarr	=	explode(",",$nodata);
			foreach($nodataarr as $v){
				$where[$v]	=	array('<>','');
			}
		}
		if($provinceid){//类别ID
			$where['provinceid']=	$provinceid;
		}
		if($cityid){//类别ID
			$where['cityid']	=	$cityid;
		}
		if($three_cityid){//类别ID
			$where['three_cityid']	=	$three_cityid;
		}
		// 处理分站查询条件
		if (!empty($_POST['did'])){
		    
		    $domain  =  $this->getDomain($_POST['did'], true);
		    
		    if (isset($domain['didcity'])){
		        
		        $data['didcity']    =  $domain['didcity'];
		        
		        if (!empty($_POST['provinceid'])){
		            // 分站下，再次选择城市，查询按选择的来
		            $where['provinceid']  =  $_POST['provinceid'];
		            $data['didcity']      =  $domain['city_name'][$_POST['provinceid']];
		        }elseif (!empty($domain['provinceid'])){
		            $where['provinceid']  =  $domain['provinceid'];
		        }
		        if (!empty($_POST['cityid'])){
		            // 分站下，再次选择城市，查询按选择的来
		            $where['cityid']  =  $_POST['cityid'];
		            $data['didcity']  =  $domain['city_name'][$_POST['cityid']];
		        }elseif (!empty($domain['cityid'])){
		            $where['cityid']  =  $domain['cityid'];
		        }
		        if (!empty($_POST['three_cityid'])){
		            // 分站下，再次选择城市，查询按选择的来
		            $where['three_cityid']  =  $_POST['three_cityid'];
		            $data['didcity']        =  $domain['city_name'][$_POST['three_cityid']];
		        }elseif (!empty($domain['three_cityid'])){
		            $where['three_cityid']  =  $domain['three_cityid'];
		        }
		        
		        $data['cityone']    =  $domain['cityone'];
		        $data['citytwo']    =  $domain['citytwo'];
		        $data['citythree']  =  $domain['citythree'];
		        $data['provinceid']    =  !empty($where['provinceid']) ? intval($where['provinceid']) : 0;
		        $data['cityid']        =  !empty($where['cityid']) ? intval($where['cityid']) : 0;
		        $data['three_cityid']  =  !empty($where['three_cityid']) ? intval($where['three_cityid']) : 0;
		    }
		}else{
		    // 没有已选择的城市，按后台设置的列表页区域默认设置来（后台-页面设置-列表页区域默认设置）
		    // 设置了一级城市，后面的搜索，不再展示其他一级城市
		    if (empty($_POST['provinceid']) && empty($_POST['cityid']) && empty($_POST['three_cityid']) || (!empty($_POST['provinceid']) && $_POST['provinceid'] == $this->config['sy_web_city_one'])){
		        
		        $list_cityid      = isset($where['cityid']) ? $where['cityid'] : 0;
		        $list_threecityid = isset($where['three_cityid']) ? $where['three_cityid'] : 0;
		        
		        $listback = $this->listCity($list_cityid, $list_threecityid);
		        if (!empty($listback)) {
		            if (isset($listback['provinceid'])){
		                $where['provinceid']  =  $listback['provinceid'];
		            }
		            if (isset($listback['cityid'])){
		                $where['cityid']  =  $listback['cityid'];
		            }
		            if (isset($listback['listcity'])){
		                $data['listcity']   =  $listback['listcity'];
		                $data['cityone']    =  $listback['cityone'];
		                $data['citytwo']    =  $listback['citytwo'];
		                $data['citythree']  =  $listback['citythree'];
		                
		                $data['provinceid']    =  !empty($where['provinceid']) ? intval($where['provinceid']) : 0;
		                $data['cityid']        =  $list_cityid;
		                $data['three_cityid']  =  $list_threecityid;
		            }
		        }
		    }
		}
		if($order){//排序
			$where['orderby']	=	$order.',desc';
		}else{
			$where['orderby']	=	'lastupdate,desc';
		}
		if($page){//分页
			$pagenav	=	($page-1)*$limit;
			$where['limit']		=	array($pagenav,$limit);
		}else{
			$where['limit']		=	array('',$limit);
		}
		$rows	=	$tinyM->getResumeTinyList($where);
		$List	=	$rows['list'];
		if($List && is_array($List)){
			$data['list']	=	count($List)?$List:array();
			
			$error	=	1;
		}else{
		    $error	=	2;
		}
		$this->render_json($error,'',$data);
	}
	function show_action(){//微简历内容
		$id		=	(int)$_POST['id'];
		if(!$id){
			$data['error']	=	3;
		}else{
			$tinyM	=	$this->MODEL('tiny');
			$row	=	$tinyM->getResumeTinyInfo(array('id'=>$id));
			$tinyM->upResumeTiny(array('hits'=>array('+',1)),array('id'=>$id));
			if(is_array($row)){
				$data['user_wjl_link']	=	$this->config['user_wjl_link'];
				
				if (!empty($row['production'])){
				    $row['production'] = $this->preghtml($row['production']);
				}
				
				$data['list']	=	count($row)?$row:array();
				
				$error	=	1;
			}else{
			    $error	=	2;
			}
		}
		$this->render_json($error,'',$data);
	}
    function sendmsg_action()
    {
        $moblie			=		$_POST['moblie'];

        $this->checkMcsdk($moblie);

        $noticeM 		= 		$this->MODEL('notice');

        $port	=	$this->plat == 'mini' ? '3' : '4';	// 短信发送端口$port : 3-小程序  4-APP
        $result	=	$noticeM->sendCode($moblie, 'code', $port, array(), 6, 120, 'msg');
        if($result['error']==1){
            $errcode	=	1;
            $msg = '发送成功';
        }else{
            $errcode	=	2;
            $msg		=	$result['msg'];
        }

        $this->render_json($errcode,$msg);
    }
	function add_action(){
	    $_POST	=	$this->post_trim($_POST);
	    $tinyM	= 	$this ->  MODEL('tiny');
		if($_POST['submit']){
			if(!$_POST['username'] || !$_POST['password'] || !$_POST['sex'] || !$_POST['exp'] || !$_POST['mobile'] || !$_POST['production'] || !$_POST['job']){
				$data['error']	=	3;
				$data['msg']	=	'请完善信息！';
				echo json_encode($data);die;
			}

			$post	=   array(
				'username' 		=>  $_POST['username'],
				'sex' 			=>  $_POST['sex'],
				'exp' 			=>  $_POST['exp'],
				'job' 			=>  $_POST['job'],
				'mobile' 		=>  $_POST['mobile'],
				'password'		=>	$_POST['password'],
				'provinceid' 	=>  $_POST['provinceid'],
				'cityid' 		=>  $_POST['cityid'],
				'three_cityid'	=>  $_POST['three_cityid'],
				'production' 	=>  $_POST['production'],
				'status' 		=>  $this->config['user_wjl'],
				'login_ip'		=>	fun_ip_get(),
				'time'			=>	time(),
				'lastupdate'	=>	time(),
			    'did'			=>	0
			);
			$addData	=	array(
				'id'				=>	(int)$_POST['id'],
				'post'				=>	$post,
                'moblie_code'				=>	$_POST['moblie_code'],
                'type'				=>	'wxapp'
			);
						
			$return	= 	$tinyM  ->  addResumeTinyInfo($addData,'wxapp');
			if($return['errcode']==9){
				$error	=	1;
			}else{
				$error	=	2;
			}
		}else{
			
			$data	=	$tinyM->getResumeTinyInfo(array('id'=>(int)$_POST['id']));
		}
        $data['ismoblie_code'] = $this->config['sy_msg_isopen'];
		$this -> render_json($error,$return['msg'],$data);

	}
	function pass_action(){
		$id	=	(int)$_POST['id'];
		if(!$_POST['password'] || !$id){
			$data['error']	=	3;
		}else{
			$tinyM	= 	$this ->  MODEL('tiny');
			$data	=	array(
				//'code'		=>  1,
				'id'		=>	(int)$_POST['id'],
				'password'	=>	$_POST['password'],
				'type'		=>	2
			);
			$return		=	$tinyM->setResumeTinyPassword($data);
			if($return['errcode']==9){
				$error	=	1;
			}else{
				$error	=	2;
			}
		}
		$this -> render_json($error,$return['msg']);
	}
	function del_action(){
		$id	=	(int)$_POST['id'];
		if(!$_POST['password'] || !$id){
			$data['error']	=	3;
		}else{
			$tinyM	= 	$this ->  MODEL('tiny');
			$data	=	array(
				//'code'		=>	1,
				'id'		=>	(int)$_POST['id'],
				'password'	=>	$_POST['password'],
				'type'		=>	3
			);
			$return		=	$tinyM->setResumeTinyPassword($data);
			
			if($return['errcode']==9){
				$rdata['error']	=	1;
			}else{
				$rdata['error']	=	2;
			}
			$rdata['msg']	=	$return['msg'];
		}
		$this -> render_json($rdata['error'],$rdata['msg']);
	}
	function editctime_action(){
		$id	=	(int)$_POST['id'];
		if(!$_POST['password'] || !$id){
			$data['error']	=	3;
		}else{
			$tinyM	= 	$this ->  MODEL('tiny');
			$data	=	array(
				//'code'		=>	1,
				'id'		=>	(int)$_POST['id'],
				'password'	=>	$_POST['password'],
				'type'		=>	1
			);
			$return		=	$tinyM->setResumeTinyPassword($data);
     
			if($return['errcode']==9){
				$nid	=	$tinyM->upResumeTiny(array('time'=>time()),array('id'=>$id));
				 
				if($nid){
					$rdata['error']	=	1;
					$rdata['msg']	=	'修改成功！';
				}else{
					$rdata['error']	=	2;
					$rdata['msg']	=	'修改失败！';
				}
			}else{
				$rdata['error']	=	2;
				$rdata['msg']	=	$return['msg'];
			}
		}
			$this -> render_json($rdata['error'],$rdata['msg']);
	}
	function isadd_action(){
		$tinyM	= 	$this ->  MODEL('tiny');
	    $data['sy_tiny']	=	$this->config['sy_tiny'];
	    $ip 	= 	fun_ip_get();
        $m_tiny_total	=	$tinyM->getResumeTinyNum(array('time'=>array('>',strtotime('today'))));
	    $m_tiny	=	$tinyM->getResumeTinyNum(array('login_ip'=>$ip,'time'=>array('>',strtotime('today'))));
	    $data['num'] = $this->config['sy_tiny'] - $m_tiny ;
        if(($this->config['sy_tiny_totalnum'] > $m_tiny_total) || $this->config['sy_tiny_totalnum'] == 0){
            if($this->config['sy_tiny']>$m_tiny||$this->config['sy_tiny']<1){
                $data['isadd']	=	true;
            }else{
                $data['isadd']	=	false;
            }
        }else{
            $data['isadd'] = false;
        }
	    $this->render_json(0, 'ok', $data);
	}
}
?>