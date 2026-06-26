<?php

class special_controller extends common{
	function index_action(){
		$this->yunset("headertitle",yun_at('wap_com_00310'));
		$this->seo("spe_index");
		$this->yuntpl(array('wap/spe_index'));
	}
	function show_action(){
		$specialM	=	$this->MODEL('special');
		$info		=	$specialM->getSpecialOne(array("id"=>(int)$_GET['id'],"display"=>1));
        if(empty($info)){
            $this -> ACT_msg_wap($_SERVER['HTTP_REFERER'], yun_at('wap_00520'));
        }
		$this->yunset("info",$info);

        if($info['etime']<time()){
            $this->yunset("isover",1);
        }
		if($this->uid && $this->usertype=='2'){
			$isapply	=	$specialM->getSpecialComOne(array("uid"=>$this->uid,"sid"=>(int)$_GET['id']));

			$this->yunset("isapply",$isapply);
		}

		$this->data	=	array('spename'=>$info['title']);
		$this->seo("spe_show");

		$this->yunset("headertitle",yun_at('wap_00521'));

        if ($info['tpl'] == 'gl.htm'){
            // uid，
            $cuid = array();
            $coms = $specialM->getSpecialComList(array('sid'=>(int)$_GET['id'], 'status'=> 1), array('field'=>'`uid`'));
            foreach ($coms['list'] as $v){
                $cuid[] = $v['uid'];
            }
            // 
            $hotcom  =  $specialM->glFamous(array('sid'=>$info['id'], 'orderby'=>'sort', 'limit'=>12));
            $this->yunset('hotcom', $hotcom);
            // 
            $hy = $specialM->getSpecialHy($cuid);
            $this->yunset($hy);
            
            $this->yuntpl(array('wap/spe_gl'));
        }else{
            $this->yuntpl(array('wap/spe_show'));
        }
	}
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
	// gl
	function getComList_action(){
	    
	    $res = $this->MODEL('special')->glComList($_POST['sid'], $_POST['hy'], $_POST['page'], $_POST['numb']);
	    
	    echo yun_json_encode($res);
	}
	// gl
	function getJobList_action(){
	    
	    $res = $this->MODEL('special')->glJobList($_POST);
	    
	    echo yun_json_encode($res);
	}
	
    function hotjobclass_action(){
        $categoryM	=	$this -> MODEL('category');
        $List = $categoryM->getHotJobClass(array('rec'=>1),'*');

        echo yun_json_encode($List);
    }
}
?>