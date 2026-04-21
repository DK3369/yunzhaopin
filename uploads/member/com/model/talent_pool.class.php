<?php
/* *
* $Author ：PHPYUN开发团队
*
* 官网: http://www.phpyun.com
*
* 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
*
* 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
*/
class talent_pool_controller extends company
{
	function index_action(){
		$this->public_action();
		$ResumeM					=  $this -> MODEL('resume');
		$where['cuid']				=  $this -> uid;
		
		if(trim($_GET['keyword'])){
            $nwhere['PHPYUNBTWSTARTB'] = '';
            $nwhere['name'] = array('like',trim($_GET['keyword']));
            $nwhere['uname']	    =	array('like', trim($_GET['keyword']), 'OR');
            $nwhere['PHPYUNBTWENDB']  = '';
			$resume					=  $ResumeM -> getSimpleList($nwhere,array('field'=>'uid'));
			
			if ($resume){
			    $uid    =   array();
				foreach($resume as $v){
					if($v['uid'] && !in_array($v['uid'],$uid)){
						$uid[]		=  $v['uid'];
					}
				}
				$where['uid']		=  array('in',pylode(',', $uid));
			}
			$urlarr['keyword']		=	trim($_GET['keyword']);
		}
		$urlarr['c']	=	$_GET['c'];
		$urlarr['page']	=	'{{page}}';
	    $pageurl		=	Url('member',$urlarr);

	    $pageM			=	$this   ->  MODEL('page');
	    $pages			=	$pageM  ->  pageList('talent_pool',$where,$pageurl,$_GET['page']);
	    
	    if($pages['total'] > 0){
	        $where['orderby']		=	'id';
	        $where['limit']			=	$pages['limit'];
	        $List   =  $ResumeM -> getTalentList($where,array('uid'=>$this->uid,'isdown'=>1,'utype'=>'pc'));
	    }

        //获取状态栏选项
        $CacheM =   $this->MODEL('cache');
        $cache  =   $CacheM->GetCache(array('com'));
        $this->yunset($cache);

 		//邀请面试选择职位
		$this->yqmsInfo();
		$this->yunset("rows",$List);
		$this->company_satic();
		$this->com_tpl('talent_pool');
	}

	function del_action(){
		if ($_GET['id']){
	        $id   =  intval($_GET['id']);
	    }elseif ($_POST['delid']){
	        $id   =  $_POST['delid'];
	    }
	    $ResumeM    =  $this->MODEL('resume');
	    $return   =  $ResumeM -> delTalentPool($id,array('uid'=>$this->uid,'usertype'=>$this->usertype));
	    $this -> layer_msg($return['msg'],$return['errcode'],$return['layertype'],$_SERVER['HTTP_REFERER']);
	}
}
?>