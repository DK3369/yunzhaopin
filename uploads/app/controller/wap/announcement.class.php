<?php

class announcement_controller extends common{
    /**
     * 网站公告列表/公告详情
     */
	function index_action(){
		if((int)$_GET['id']){
			
			$id				=	(int)$_GET['id'];
			$announcementM	=	$this->MODEL('announcement');
            // 更新浏览次数
            $announcementM->upViewNum($id);// 先更新被浏览次数，再查公告信息，防止新公告首次被浏览时出现被浏览次数为0
			$row			=	$announcementM->getInfo(array('id'=>$id));
			$this->yunset("row",$row);
			
			$data['gg_title']	=	$row['title'];//公告名称
			$data['gg_desc']	=	$this->GET_content_desc($row['description']);//描述
			$this->data			=	$data;
			$this->seo("announcement");

			$this->yunset("headertitle","网站公告");
			$this->yuntpl(array('wap/announcements'));
		}else{
			$this->yunset("headertitle","网站公告");
	        $this->seo("announcement_index");
			$this->yuntpl(array('wap/announcement'));
		}
		
	}	
}
?>