<?php

class announcement_controller extends common{
    
	function index_action(){
		if((int)$_GET['id']){
			
			$id				=	(int)$_GET['id'];
			$announcementM	=	$this->MODEL('announcement');
            // 
            $announcementM->upViewNum($id);// bump view count before load
			$row			=	$announcementM->getInfo(array('id'=>$id));
			$this->yunset("row",$row);
			
			$data['gg_title']	=	$row['title'];// title
			$data['gg_desc']	=	$this->GET_content_desc($row['description']);// description
			$this->data			=	$data;
			$this->seo("announcement");

			$this->yunset("headertitle",yun_at('default_00104'));
			$this->yuntpl(array('wap/announcements'));
		}else{
			$this->yunset("headertitle",yun_at('default_00104'));
	        $this->seo("announcement_index");
			$this->yuntpl(array('wap/announcement'));
		}
		
	}	
}
?>