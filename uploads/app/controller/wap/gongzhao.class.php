<?php


class gongzhao_controller extends common
{
    // 公招列表
    function index_action()
    {
        $this->yunset('backurl', Url('wap'));
        $this->yunset("headertitle", "公招");
        $this->seo("gongzhao_index");
        $this->yuntpl(array('wap/gongzhao'));
    }
    // 公招详情
    function show_action(){
        if ((int)$_GET['id']) {
            
            $id            = (int)$_GET['id'];
            $gongzhaoM = $this->MODEL('gongzhao');
            $row           = $gongzhaoM->getInfo(array('id' => $id));
            $this->yunset("row", $row);
            
            $data['gz_title'] = $row['title'];//公招名称
            $data['gz_desc']  = $this->GET_content_desc($row['description']);//描述
            $this->data       = $data;
            $this->seo("gongzhao");
            
            $this->yunset('backurl', Url('wap',array('c'=>'gongzhao')));
            $this->yunset("headertitle", "公招");
            $this->yuntpl(array('wap/gongzhaos'));
        }
    }
}

?>