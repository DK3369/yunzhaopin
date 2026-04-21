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
class shop_class_controller extends adminCommon{
	function index_action(){
		$redeemM			=	$this->MODEL('redeem');
		$where['keyid']		=	'0';
		$where['orderby']	=	'sort,asc';
		$position			=	$redeemM->GetRewardClass($where);
		$list = $position['list'];
        $this->render_json(0, 'ok', compact('list'));
	}
	
	function save_action(){
		$redeemM			=	$this->MODEL('redeem');
	    $_POST				=	$this->post_trim($_POST);	
	    $position			=	explode('-',$_POST['name']);
		foreach ($position as $val){
			if($val){
				$name[]=$val;
			}
		}
		$where['name']		=	array('in',@implode(',', $name));
		$redeem_class		=	$redeemM->GetRewardClass($where);
		
		if(empty($redeem_class['list'])){

            foreach ($name as $key=>$val){
				
                if($_POST['ctype']=='1'){//一级分类
                    $value['name']	=	$val;
                }else{
					$value['name']	=	$val;
					$value['keyid']	=	intval($_POST['nid']);
                }
				
				$add	=	$redeemM->addRedeemClassInfo($value);
            }
			$this->cache_action();
			$add		?	$msg	=	2	:	$msg	=	3;
			if ($add){
                $tit = '商品类别添加成功';
            }else{
			    $tit = '添加失败';
            }
			$this->MODEL('log')->addAdminLog("商品类别(ID:".$add.")添加成功");
		}else{
			$msg	=	1;
			$tit = '类别已存在';
		}
        $this->render_json($msg ==2 ? 0 : 1, $tit);
	}
	
	function up_action(){
		$redeemM					=	$this->MODEL('redeem');
		if((int)$_POST['id']){
			$oneWhere['id']			=	(int)$_POST['id'];
			$onejob					=	$redeemM->getRedeemClassInfo($oneWhere);
			
			$twoWhere['keyid']		=	(int)$_POST['id'];
			$twoWhere['orderby']	=	'sort,asc';
            $twojob					=	$redeemM->GetRewardClass($twoWhere);
            $list = array_merge(array($onejob),$twojob['list']);
            
		}
		$data['list'] = !empty($list)?$list:array();
		
		$this->render_json(0, 'ok', $data);
	}

	function del_action(){
		$redeemM	=	$this->MODEL('redeem');
		if(is_array($_POST['del'])){
			$where['id']	=	array('in',pylode(',',$_POST['del']));
			$where['keyid']	=	array('in',pylode(',',$_POST['del']),'OR');
			$del			=	$redeemM->delRedeemClass($where,array('type'=>'all'));
			$delid			=	pylode(',',$_POST['del']);
		}else{
			$where['id']	=	(int)$_POST['del'];
			$where['keyid']	=	array('=',(int)$_POST['del'],'OR');
			$del			=	$redeemM->delRedeemClass($where,array('type'=>'one'));
			$delid			=	(int)$_POST['del'];
		}
		
		if(!$delid){
            $this->render_json(1, '请选择要删除的内容');
		}
		$this->cache_action();
        $this->render_json(isset($del)?0:1, isset($del)?'商品类别删除成功':'删除失败');
		
	}
	
	function ajax_action(){
		$redeemM	=	$this->MODEL('redeem');
		if(isset($_POST['sort'])){
			$sValue['sort']	=	$_POST['sort'];
			$sWhere['id']	=	$_POST['id'];
			$up				=	$redeemM->upRedeemClassInfo($sWhere,$sValue);
			$this->MODEL('log')->addAdminLog("商品类别(ID:".$_POST['id'].")排序修改成功");
		}
		
		if($_POST['name']){
			$nValue['name']	=	$_POST['name'];
			$nWhere['id']	=	$_POST['id'];
			$up				=	$redeemM->upRedeemClassInfo($nWhere,$nValue);
			$this->MODEL('log')->addAdminLog("商品类别(ID:".$_POST['id'].")名称修改成功");
		}
		$this->cache_action();
        $this->render_json(0, '修改成功');
	}
	
	function cache_action()	{
		include(LIB_PATH."cache.class.php");
		$cacheclass	= 	new cache(PLUS_PATH,$this->obj);
		$makecache	=	$cacheclass->redeem_cache("redeem.cache.php");
	}
}

?>