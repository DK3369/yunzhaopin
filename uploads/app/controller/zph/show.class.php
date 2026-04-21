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
class show_controller extends zph_controller
{

    function index_action()
    {
        $this   ->  Zphpublic_action();
        $id     =   intval($_GET['id']);
        
        $zphM   =   $this->MODEL('zph');
        $row    =   $zphM->getInfo(array('id' => $id), array('banner' => 1));
        
        if (empty($row)) {
            $this -> ACT_msg(url('zph'), '没有找到该招聘会！');
        }
        
        $this->yunset('Info', $row);

        $rows   =   $zphM -> getZphPicList(array('zid' => $id));
        $this->yunset('Image_info', $rows);

        //参会企业第一页
        $zclimit            =   20;
        $zcwhere            =   array();
        $zcwhere['zid']     =   $id;
        $zcwhere['status']  =   1;
        $zcwhere['orderby'] =   array('sort,desc','ctime,asc');

        $zcnum              =   $zphM->getZphComNum($zcwhere);

        $zcwhere['limit']   =   $zclimit;
        $zcList             =   $zphM -> getZphCompanyList($zcwhere);
        $this->yunset('zcrows',$zcList);
        
        $zcpage             =   intval(ceil($zcnum/$zclimit));
        $this->yunset('zcpage',$zcpage);
        //参会企业第一页 end

        //参会职位第一页
        
        $zjwhere            =   array();
        $zjwhere['zid']     =   $id;
        $zjwhere['status']  =   1;
        $zjwhere['orderby'] =   array('sort,desc','ctime,asc');

        $zjwhereData        =   array('zwhereData'=>$zjwhere);

        $zjlimit            =   40;
        $jwhere['state']    =   1;
        $jwhere['status']   =   0;
        $jwhere['r_status'] =   1;
        $jwhere['limit']    =   $zjlimit;
        $jwhere['orderby']  =   'lastupdate,desc';

        $zjwhereData['jwhereData'] = $jwhere;

        $zjList             =   $zphM -> getZphJobList($zjwhereData);
        
        $this->yunset('zjrows',$zjList['list']);
        
        $zjpage             =   intval(ceil($zjList['zjnum']/$zjlimit));
        $this->yunset('zjpage',$zjpage);
        //参会职位第一页 end
		
        $data['zph_title']  =   $row['title'];
        $data['zph_desc']   =   $this -> GET_content_desc($row['body']); // 描述
        $this -> data       =   $data;

        $this -> seo('zph_show');
        $this -> yun_tpl(array('zphshow'));
    }
}
?>