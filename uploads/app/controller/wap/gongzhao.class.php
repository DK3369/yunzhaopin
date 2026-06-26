<?php


class gongzhao_controller extends common
{
    // 
    function index_action()
    {
        $this->yunset('backurl', Url('wap'));
        $this->yunset("headertitle", yun_at('default_00134'));
        $this->seo("gongzhao_index");
        $this->yuntpl(array('wap/gongzhao'));
    }
    // 
    function show_action(){
        if ((int)$_GET['id']) {
            
            $id            = (int)$_GET['id'];
            $gongzhaoM = $this->MODEL('gongzhao');
            $row           = $gongzhaoM->getInfo(array('id' => $id));
            $this->yunset("row", $row);
            
            $data['gz_title'] = $row['title'];// title
            $data['gz_desc']  = $this->GET_content_desc($row['description']);// description
            $this->data       = $data;
            $this->seo("gongzhao");
            
            $this->yunset('backurl', Url('wap',array('c'=>'gongzhao')));
            $this->yunset("headertitle", yun_at('default_00134'));
            $this->yuntpl(array('wap/gongzhaos'));
        }
    }
}

?>