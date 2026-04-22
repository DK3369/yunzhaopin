<?php

class index_controller extends common{
	
    // 公招列表
	function index_action(){
	    
        $this->seo("gongzhao_index");
		$this->yun_tpl(array('index'));
	}
	// 公招详情
    function show_action(){
        
        if((int)$_GET['id']){
            $id				=	(int)$_GET['id'];
            $gongzhaoM	=	$this->MODEL('gongzhao');
            $gonggao		=	$gongzhaoM->getInfo(array('id'=>$id));
            
            if($gonggao['id']==''){
                $this->ACT_msg($this->config['sy_weburl'],"没有找到该公招！");
            }
            //上一篇
            $annou_last=$gongzhaoM->getInfo(array('id'=>array('<',$id),'orderby'=>'id,desc'));
            if(!empty($annou_last)){
                $annou_last['url']=Url('gongzhao',array('c'=>'show','id'=>$annou_last['id']));
            }
            $gonggao["last"]	=	$annou_last;
            //下一篇
            $annou_next=$gongzhaoM->getInfo(array('id'=>array('>',$id),'orderby'=>'id,asc'));
            if(!empty($annou_next)){
                $annou_next['url']=Url('gongzhao',array('c'=>'show','id'=>$annou_next['id']));
            }
            $gonggao["next"]	=	$annou_next;
            $this->yunset("Info",$gonggao);
            
            $data['gz_title']	=	$gonggao['title'];//新闻名称
            $description		=	$gonggao['description']?$gonggao['description']:$gonggao['content'];
            $data['gz_desc']	=	$this->GET_content_desc($gonggao['description']);//描述
            $this->data			=	$data;
            $this->seo("gongzhao");
            
            $this->yun_tpl(array('show'));
        }
    }
}
?>
