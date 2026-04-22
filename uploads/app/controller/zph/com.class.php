<?php


class com_controller extends zph_controller
{

    function index_action()
    {
        $this -> Zphpublic_action();

        $id     =   intval($_GET['id']);

        $zphM   =   $this -> MODEL('zph');
        
        $row    =   $zphM -> getInfo(array('id' => $id), array('pic' => 1));
        
        if (empty($row)) {
            $this -> ACT_msg(url("zph"), '没有找到该招聘会！');
        }
        
        $this -> yunset('row', $row);

        $where              =   array();
        $where['zid']       =   $id;
        $where['status']    =   1;
        $where['orderby']   =   array('sort,desc','ctime,asc');

        $urlarr             =   array();
        $urlarr['c']        =   $_GET['c'];
        $urlarr['id']       =   $id;
        $urlarr['page']     =   '{{page}}';
        
        $pageurl            =   Url('zph', $urlarr, '1');

        $pageM              =   $this -> MODEL('page');
        $pages              =   $pageM -> pageList('zhaopinhui_com', $where, $pageurl, $_GET['page']);

        if ($pages['total'] > 0) {
            
            $where['limit']     =   $pages['limit'];

            $List               =   $zphM -> getZphCompanyList($where);
			
            $this -> yunset('rows', $List);
        }
		
        $data['zph_title']      =   $row['title'];
        $data['zph_desc']       =   $this->GET_content_desc($row['body']);
        $this -> data           =   $data;
        $this -> seo('zph_com');
        $this -> yun_tpl(array('zphcom'));
    }
}
?>