<?php

class index_controller extends special_controller{
	function index_action(){

		$this->seo("spe_index");
		$this->yun_tpl(array('index'));
	}

	//专题招聘详情
	function show_action(){
		$specialM	=	$this->MODEL('special');
		$info		=	$specialM->getSpecialOne(array("id"=>(int)$_GET['id'],"display"=>1));
        if (empty($info)){
            $this->ACT_msg($this->config['sy_weburl'], '没有找到该专题招聘');
        }
		$this->yunset("info",$info);

		if($this->uid && $this->usertype=='2'){
			$isapply	=	$specialM->getSpecialComOne(array("uid"=>$this->uid,"sid"=>(int)$_GET['id']));

			$this->yunset("isapply",$isapply);
		}
		if ($info['tpl'] == 'gl.htm'){
		    // 该模板需要所有参会企业uid，来查参会企业相关数据
		    $cuid = array();
		    $coms = $specialM->getSpecialComList(array('sid'=>(int)$_GET['id'], 'status'=> 1), array('field'=>'`uid`'));
		    foreach ($coms['list'] as $v){
		        $cuid[] = $v['uid'];
		    }
		    // 该模板需要的名企
		    $hotcom  =  $specialM->glFamous(array('sid'=>$info['id'], 'orderby'=>'sort,desc', 'limit'=>12));
		    $this->yunset('hotcom', $hotcom);
		    // 该模板所需的行业
		    $hy = $specialM->getSpecialHy($cuid);
		    $this->yunset($hy);
		}
		$this->data  =  array('spename'=>$info['title']);
		$this->seo("spe_show");

		$tpl		 =	@explode('.',$info['tpl']);

		$this->yun_tpl(array($tpl[0]));
	}
	//专题招聘报名
	function apply_action(){
		$data		=	array(
			'id'		=>	(int)$_POST['id'],
			'uid'		=>	$this->uid,
			'usertype'	=>	$this->usertype,
		);
		$specialM	=	$this->MODEL('special');
		$return		=	$specialM->addSpecialComInfo($data);
		if($return['url']){
			$this->layer_msg($return['msg'],$return['errcode'],0,$return['url']);
		}else{
			$this->layer_msg($return['msg'],$return['errcode'],0);
		}
	}
	// gl模板查询企业列表
	function getComList_action(){
	    
	    $res = $this->MODEL('special')->glComList($_POST['sid'], $_POST['hy'], $_POST['page'], $_POST['numb']);
	    
	    echo json_encode($res);
	}
	// gl模板查询职位列表
	function getJobList_action(){
	    
	    $res = $this->MODEL('special')->glJobList($_POST);
	    
	    echo json_encode($res);
	}
}
?>