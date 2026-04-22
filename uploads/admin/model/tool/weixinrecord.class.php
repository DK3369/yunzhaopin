<?php

class weixinrecord_controller extends adminCommon{
    
    function index_action(){
        
        // 查询出字段值为纯数字的，排除带参二维码
        $where['wxloginid']  =  array('regexp','[^0-9.]');
        //实例化
        $weiXinM    =   $this->MODEL('weixin');

        if(trim($_POST['keyword'])){

            if($_GET['wtype']=='1'){

                $where['re_nick']   =   array('like',trim($_GET['keyword']));

            }elseif($_GET['wtype']=='2'){

                $where['username']  =   array('like',trim($_GET['keyword']));

            }

            $where['PHPYUNBTWSTART']        =   '';

            $where['username']      =   array('like',trim($_POST['keyword']));

            $where['re_nick']       =   array('like',trim($_POST['keyword']),'or');

            $where['PHPYUNBTWEND']  =   '';

        }

        if($_POST['status']){

            if($_POST['status']=='2'){

                $status = 0;

            }else{

                $status = $_POST['status'];

            }
            $where['status']        =   $status;

        }

        if($_POST['usertype']){

            $where['usertype']      =   $_POST['usertype'];

        }

        if($_POST['time']){

            if($_POST['time']=='1'){

                $where['time']      =   array('>',strtotime(date('Y-m-d 00:00:00')));

            }else{

                $where['time']      =   array('>',strtotime('-'.intval($_POST['time']).' day'));
            }

        }


        $pageM          =   $this->MODEL('page');
        $pages          =   $pageM->adminPageList('wxqrcode', $where,$_POST['page'],array('limit'=>$_POST['limit']));

        if ($pages['total'] > 0) {

            $where['orderby']       =   'time,desc';
        
            $where['limit']     =   $pages['limit'];
            $list   =   $weiXinM -> getWxQrcodeList($where);
        }

        $data['list']           =   !empty($list['list'])?$list['list']:array();
        $data['total']          =   $pages['total'];
        $data['page_sizes']     =   $pages['page_sizes'];
        $data['page_size']     =   $pages['page_size'];
        $this->render_json(0, '', $data);
    }

    function clearwx_action(){

        //实例化
        $weiXinM    =   $this->MODEL('weixin');

        $where['time']  =   array('<',strtotime('-3 days'));

        $del        =   $weiXinM->delWxqrcode($where,array('type'=>'all'));

        if($del){
            $error = 0;
            $msg = '清理完成！';
        }else{
            $error = 0;
            $msg = '删除失败！';
        }

        $this->render_json($error,$msg);
    
    }
    function userbd_action(){

        $userInfoM =    $this->MODEL('userinfo');

        $where['PHPYUNBTWSTART']    =   '';

        $where['wxid'][]            =   array('<>','');

        $where['wxid'][]            =   array('notnull');

        $where['PHPYUNBTWEND']      =   '';

        if(trim($_POST['keyword'])){

            $where['username']  =   array('like',trim($_POST['keyword']));

        }


        $pageM          =   $this->MODEL('page');
        $pages          =   $pageM->adminPageList('member', $where,$_POST['page'],array('limit'=>$_POST['limit']));

        if ($pages['total'] > 0) {

            $where['orderby']       =   'wxbindtime,desc';
        
            $where['limit']     =   $pages['limit'];
            $List   =   $userInfoM -> getList($where,array('field'=>'`uid`,`username`,`wxid`,`wxbindtime`'));

            foreach ($List as $k => $v) {
                $List[$k]['wxbindtime_n'] = !empty($v['wxbindtime'])?date('Y-m-d H:i:s',$v['wxbindtime']):'';
            }
        }

        $data['list']           =   !empty($List)?$List:array();
        $data['total']          =   $pages['total'];
        $data['page_sizes']     =   $pages['page_sizes'];
        $data['page_size']     =   $pages['page_size'];
        $this->render_json(0, '', $data);
    }

    function deluser_action(){

        //实例化
        $userInfoM  =   $this->MODEL('userinfo');
        if($_POST['del']){

            $where['uid']   =   array('in',pylode(',',$_POST['del']));

            $data['wxid']   =   '';

            $userInfoM->upInfo($where,$data);

            $error = 0;
            $msg = '微信用户(ID:'.pylode(',',$_POST['del']).')取消绑定成功！';

        }else{
            $error = 1;
            $msg = '请选择需要取消绑定的数据';
        }


        $this->render_json($error,$msg);

    }

    function keyword_action(){

        $hotKeyM    =   $this->MODEL('hotkey');

        $where['type']  =   '8';

        if(trim($_POST['keyword'])){

            $where['key_name']  =   array('like',trim($_POST['keyword']));

        }


        $pageM          =   $this->MODEL('page');
        $pages          =   $pageM->adminPageList('hot_key', $where,$_POST['page'],array('limit'=>$_POST['limit']));

        if ($pages['total'] > 0) {

            $where['orderby']       =   'num,desc';
        
            $where['limit']     =   $pages['limit'];

            $List   =   $hotKeyM -> getList($where);

        }

        $data['list']           =   !empty($List)?$List:array();
        $data['total']          =   $pages['total'];
        $data['page_sizes']     =   $pages['page_sizes'];
        $data['page_size']      =   $pages['page_size'];
        $this->render_json(0, '', $data);
    }

    function delkeyword_action(){

        $hotM                           =       $this -> MODEL('hotkey');
        
        if(is_array($_POST['del'])){
            $where['id']                =       array('in',pylode(',',$_POST['del']));
        }else{
            $where['id']                =       $_POST['del'];
        }
        
        $return                         =       $hotM->delHotkey($where);
        
        $this->get_cache();
        $this->render_json($return['code'],$return['msg']);
    }

    function get_cache(){
        include(LIB_PATH."cache.class.php");
        $cacheclass= new cache(PLUS_PATH,$this->obj);
        $makecache=$cacheclass->keyword_cache("keyword.cache.php");
    }
}
?>