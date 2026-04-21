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
class index_controller extends common{
	/**
	 * 网站公告列表/公告详情
	 */
	function index_action(){
		if((int)$_GET['id']){
			$id				=	(int)$_GET['id'];
			$announcementM	=	$this->MODEL('announcement');
            // 更新浏览次数
            $announcementM->upViewNum($id);// 先更新被浏览次数，再查公告信息，防止新公告首次被浏览时出现被浏览次数为0
			$gonggao		=	$announcementM->getInfo(array('id'=>$id));
			
			if($gonggao['id']==''){
				$this->ACT_msg($this->config['sy_weburl'],"没有找到该公告！");
			}
			//上一篇
			$where['id']				=	array('<', $id);
			$where['orderby']			=	array('id,desc');
			$where['PHPYUNBTWSTART_A'] 	= 	array();
			$where['startime'][]  		=  	array('<=',time(),'OR');
			$where['startime'][]  		=  	array('=',0,'OR');
			$where['startime'][] 		=  	array('isnull','','OR');
			$where['PHPYUNBTWEND_A'] 	= 	array();
			$where['PHPYUNBTWSTART_B'] 	= 	array();
			$where['endtime'][]  		=  	array('>',time(),'AND');
			$where['endtime'][] 		=  array('=',0,'OR');
			$where['endtime'][]  		=  array('isnull','','OR');
			$where['PHPYUNBTWEND_B'] 	= array();
			$annou_last=$announcementM->getInfo($where);
			if(!empty($annou_last)){
				$annou_last['url']=Url('announcement',array('id'=>$annou_last['id']));
			}
			$gonggao["last"]	=	$annou_last;
			//下一篇
			$nwhere['id']				=	array('>', $id);
			$nwhere['orderby']			=	array('id,asc');
			$nwhere['PHPYUNBTWSTART_A'] 	= 	array();
			$nwhere['startime'][]  		=  	array('<=',time(),'OR');
			$nwhere['startime'][]  		=  	array('=',0,'OR');
			$nwhere['startime'][] 		=  	array('isnull','','OR');
			$nwhere['PHPYUNBTWEND_A'] 	= 	array();
			$nwhere['PHPYUNBTWSTART_B'] 	= 	array();
			$nwhere['endtime'][]  		=  	array('>',time(),'AND');
			$nwhere['endtime'][] 		=  array('=',0,'OR');
			$nwhere['endtime'][]  		=  array('isnull','','OR');
			$nwhere['PHPYUNBTWEND_B'] 	= array();
			$annou_next=$announcementM->getInfo($nwhere);
			if(!empty($annou_next)){
				$annou_next['url']=Url('announcement',array('id'=>$annou_next['id']));
			}
			$gonggao["next"]	=	$annou_next;
			$this->yunset("Info",$gonggao);
			
			$data['gg_title']	=	$gonggao['title'];//新闻名称
			$description		=	$gonggao['description']?$gonggao['description']:$gonggao['content'];
			$data['gg_desc']	=	$this->GET_content_desc($gonggao['description']);//描述
			$this->data			=	$data;
	        $this->seo("announcement");
			
            $this->yun_tpl(array('show'));
		}else{
	        $this->seo("announcement_index");
			$this->yun_tpl(array('index'));
		}
	}
    function show_action(){

	    $this->seo("announcement");
        $this->yun_tpl(array('show'));
    }
}
?>
