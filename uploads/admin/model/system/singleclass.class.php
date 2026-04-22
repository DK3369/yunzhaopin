<?php

class singleclass_controller extends adminCommon
{
    
    function index_action()
    {
        $descM                  =   $this   ->  MODEL('description');

        $page                   =   !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize               =   !empty($this->config['sy_listnum']) ? intval($this->config['sy_listnum']) : 10;
        
        $whereData['limit']     =   array(($page - 1) * $pageSize, $pageSize);
        $whereData['orderby']   =   'sort,desc';
        $list                   =   $descM  ->  getDesClassList($whereData);

        $data['list']           =   $list;
        $data['total']          =   $descM ->  getDesClassNum(array('id'=>array('>',0)));
        $data['perPage']        =   $pageSize;
        $this->render_json(0, '', $data);
    }

    //添加
    function add_action()
    {
        
        $_POST          =   $this   ->  post_trim($_POST);
        $data['name']   =   @explode('-',$_POST['name']);
        $descM          =   $this   ->  MODEL('description');
        $error         =   $descM  ->  addDesClass($data);
        $this   ->  cache_action();
        $this->render_json($error, '');
    }

    //删除
    function del_action()
    {
        $descM                  =   $this   ->  MODEL('description');
        $whereData              =   array();
        $data                   =   array();
        if(is_array($_POST['del'])){
            $whereData['id']    =   array('in',pylode(',',$_POST['del']));
            $data['type']       =   'all'; 
        }else{
            $whereData['id']    =   $_POST['del'];
            $data['type']       =   'one';  
        }
        
        $return =   $descM  ->  delDesClass($whereData,$data);
        $this   ->  cache_action();
        $this->admin_json($return['errcode'], $return['msg']);
    }

    function ajax_action()
    {
        $descM              =   $this   ->  MODEL('description');
        $whereData['id']    =   $_POST['id'];
        $addData['sort']    =   $_POST['sort'];
        $addData['name']    =   $_POST['name'];
        
        $descM  ->  upDesClass($addData,$whereData);
        $this   ->  cache_action();
        $this->render_json(0, '');
    }
    function cache_action(){

        include_once(LIB_PATH."cache.class.php");
        $cacheclass = new cache(PLUS_PATH,$this->obj);

        $makecache  =   $cacheclass ->  desc_cache("desc.cache.php");
    }
}