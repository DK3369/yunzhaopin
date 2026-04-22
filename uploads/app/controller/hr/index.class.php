<?php

class index_controller extends common{
	function index_action()
	{
		if($this->config['sy_hr_web']=="2")//后台已关闭
		{
			header("location:".Url('error'));
		}
		$this->seo("hrindex");
		$this->yun_tpl(array('index'));
	}
	function list_action()
	{
		if($this->config['sy_hr_web']=="2")//后台已关闭
		{
			header("location:".Url('error'));
		}
		
		$hrM	=	$this->MODEL("hr");
		$class	=	$hrM->getClassInfo(array("id"=>(int)$_GET['id']));
		$this->yunset("class",$class);
		
		$list	=	$hrM->getClassList();
		$this->yunset("list",$list);
		
		if(trim($_GET['keyword'])){
			$data['hr_class']	=	trim($_GET['keyword']);
		}else{
			$data['hr_class']	=	$class['name'];
		}
		$data['hr_desc']		=	$class['content'];
		$this->data				=	$data;
		$this->seo("hrlist");
		
		$this->yun_tpl(array('list'));
	}
	function ajax_action(){
		if($_POST['id']){
            $id		=	(int)$_POST['id'];
			$hrM	=	$this->MODEL("hr");
			$post	=	array(
				'downnum'	=>	array('+',1)
			);
			$hrM->upHrInfo(array("id"=>$id),array('post'=>$post));
			
			$row	=	$hrM->getInfo($id,array('field'=>'`url`'));
			
			echo checkpic($row['url']);die;
		}
	}
}
?>
