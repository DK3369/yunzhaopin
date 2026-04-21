<?php

/**
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

class set_tplset_controller extends adminCommon
{

    function public_action()
    {
        include_once('model/style_class.php');//引用操作文件
    }

    /**
     * 风格管理
     */
    function index_action()
    {
        $this->public_action();

        $style  =   new style($this->obj);
        $list  =   $style->model_list_action();
        $sy_style = $this->config['style'];
        $imgarr  = array();
        foreach ($list as $k=>$v){
            $imgarr[] = $v['img'];
        }
        $data = array(
            'list'=> $list,
            'sy_style'=> $sy_style,
            'imgarr'=> $imgarr
        );
        $this->render_json(0,'ok',$data);
    }
    function stylesave_action()
    {
        $this->public_action();

        $style  =   new style($this->obj);
        $style->model_save_action($_POST);
        $this->admin_json(0,'信息修改成功');
    }

    function check_style_action()
    {
        if ($_POST['dir'] != "") {
            $data['style']  =   $_POST['dir'];
            $configM        =   $this->MODEL('config');
            $configM->setConfig($data);
            $this->web_config();
            $this->admin_json(0,'模板风格更换成功');
        } else {
            $this->admin_json(1,'该目录无效');
        }
    }

    /**
     * 企业模板
     */
    function comtpl_action()
    {

        $tplM   =   $this->MODEL('tpl');
        $list   =   $tplM->getComtplList(array('orderby' => 'id, desc'));
        $imgarr  = array();
        foreach ($list as $k=>$v){
            $imgarr[] = $v['pic_n'];
        }
        $data = array(
            'list'=> $list,
            'imgarr'=> $imgarr
        );
        $this->render_json(0,'ok',$data);
    }

    function comptplsave_action()
    {
        $tplM   =   $this->MODEL('tpl');
        if ($_FILES['file']['tmp_name']) {
            $upArr  =   array(
                'file'  =>  $_FILES['file'],
                'dir'   =>  'company',
                'type'  =>  'thumb',
                'thumb' =>  array(
                    'width'         =>  350,
                    'height'        =>  350,
                    'newNamePre'    =>  '_S_'
                )
            );
            $uploadM    =   $this->MODEL('upload');
            $pic        =   $uploadM->newUpload($upArr);
            if (!empty($pic['msg'])) {
                $this->render_json(1,$pic['msg']);
            } elseif (!empty($pic['picurl'])) {
                $pictures       =   $pic['picurl'];
            }
        }
        if (isset($pictures)) {

            $data['pic']        =   $pictures;
        }
        $this->com_sava_action($_POST['url']);

        $data['name']           =   $_POST['name'];
        $data['url']            =   $_POST['url'];
        $data['status']         =   $_POST['status'];
        $data['price']          =   $_POST['price'];
        $data['service_uid']    =   !empty($_POST['service_uid']) ? $_POST['service_uid'] : 0;

        if ($_POST['id']) {

            $return =   $tplM->upComtpl($data, array("id" => $_POST['id']));
        } else {

            $return =   $tplM->addComtpl($data);
        }
        $this->admin_json($return['errcode']==9?0:1,$return['msg']);
    }

    /**
     * 企业站模板添加及修改
     * @param $url
     */
    function com_sava_action($url)
    {
        //验证URL 必须只能是数字字母形式
        if (!ctype_alnum($url)) {
            $this->render_json(1,'目录名称只能是字母或数字');
        }
        if (!is_dir("../app/template/company/".$url)) {
            mkdir("../app/template/company/".$url, 0777, true);
        }
    }

    /**
     * 删除企业模板
     */
    function comtpldel_action()
    {
        $del    =   $_POST['id'];
        if (!$del) {
            $this->render_json(1,'请先选择');
        }
        $return =   $this->MODEL('tpl')->delComtpl($del);
        $this->admin_json($return['errcode']==9?0:1,$return['msg']);
    }

    /**
     * 简历模版
     */
    function resumetpl_action()
    {
        $list   =   $this->MODEL('tpl')->getResumetplList(array('orderby' => 'id, desc'));
        $imgarr  = array();
        foreach ($list as $k=>$v){
            $imgarr[] = $v['pic_n'];
        }
        $data = array(
            'list'=> $list,
            'imgarr'=> $imgarr
        );
        $this->render_json(0,'ok',$data);
    }
    function resumetplsave_action()
    {
        $tplM   =   $this->MODEL('tpl');
        if ($_FILES['file']['tmp_name']) {
            $upArr  =   array(
                'file'  =>  $_FILES['file'],
                'dir'   =>  'resume'
            );
            $uploadM    =   $this->MODEL('upload');
            $picr       =   $uploadM->newUpload($upArr);
            if (!empty($picr['msg'])) {
                $this->render_json(1,$picr['msg']);
            } elseif (!empty($picr['picurl'])) {
                $pictures   =   $picr['picurl'];
            }
        }
        if (isset($pictures)) {
            $data['pic']       =   $pictures;
        }
        $this->resumetpl_sava_action($_POST['url']);
        $data['name']           =   $_POST['name'];
        $data['url']            =   $_POST['url'];
        $data['status']         =   $_POST['status'];
        $data['price']          =   $_POST['price'];
        $data['service_uid']    =   !empty($_POST['service_uid']) ? $_POST['service_uid'] : 0;
        if ($_POST['id']) {
            $return =   $tplM->upResumetpl($data, array("id" => $_POST['id']));
        } else {
            $return =   $tplM->addResumetpl($data);
        }
        $this->admin_json($return['errcode']==9?0:1,$return['msg']);
    }

    function resumetpl_sava_action($url)
    {
        if (!ctype_alnum($url)) {
            $this->render_json(1,'目录名称只能是字母或数字');
        }
        if (!is_dir("../app/template/resume/" . $url)) {
            mkdir("../app/template/resume/" . $url, 0777, true);
        }
    }

    function resumetpldel_action()
    {
        $del    =   $_POST['id'];
        if (!$del) {
            $this->render_json(1,'请先选择');
        }
        $return =   $this->MODEL('tpl')->delResumetpl($del);

        $this->admin_json($return['errcode']==9?0:1,$return['msg']);
    }

    function pcindextpl_action(){

        $list	=	$this -> MODEL('tpl') -> getTplindexList(array('orderby' => 'id, desc'));
        $imgarr  = array();
        foreach ($list as $k=>$v){
            $imgarr[] = $v['pic_n'];
        }
        $data = array(
            'list'=> $list,
            'imgarr'=> $imgarr,
        );
        $this->render_json(0,'ok',$data);
    }

    function indextplsave_action(){
        $tplM	=	$this -> MODEL('tpl');
        if($_FILES['file']['tmp_name']!=''){
            // pc端上传
            $upArr    =  array(
                'file'  =>  $_FILES['file'],
                'dir'   =>  'tplindex'
            );

            $uploadM  =  $this->MODEL('upload');
            $pic      =  $uploadM->newUpload($upArr);
            if (!empty($pic['msg'])){
                $this->render_json(1,$pic['msg']);
            }elseif (!empty($pic['picurl'])){
                $picurl         =   $pic['picurl'];
            }
        }
        if(isset($picurl)){
            $data['pic']		=	$picurl;
        }
        $time			=	explode(",",$_POST['time']);
        $data['stime']	=	strtotime($time[0]);
        $data['etime']	=	strtotime($time[1].' 23:59:59');
        $data['name']   =   $_POST['name'];
        $data['height']   =   $_POST['height'];
        $data['se']   =   $_POST['se'];
        $data['status']   =   $_POST['status'];

        if($_POST['id']){
            $return		=	$tplM -> upTplindex($data,array("id"=>$_POST['id']));
        }else{
            $return		=	$tplM -> addTplindex($data);
        }
        $this->cache();
        $this->admin_json($return['errcode']==9?0:1,$return['msg']);
    }
    function indextpldel_action(){
        $tplM	=	$this -> MODEL('tpl');
        $del=$_POST['id'];
        if($del){
            $return	=	$tplM -> delTplindex($del);
            $this->cache();
            $this->admin_json($return['errcode']==9?0:1,$return['msg']);
        }else{
            $this->cache();
            $this->render_json(1,'请先选择');
        }
    }
    /*生成主题的缓存文件*/
    function cache(){
        include_once(LIB_PATH."cache.class.php");
        $cacheclass= new cache(PLUS_PATH,$this->obj);
        $makecache=$cacheclass->indextpl_cache("indextpl.cache.php");
    }
}