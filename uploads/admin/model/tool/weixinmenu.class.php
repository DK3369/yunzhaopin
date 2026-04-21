<?php

class weixinmenu_controller extends adminCommon{
    /**
     * 公众号设置
     */
    function index_action(){
        
        $data = array(
            'wx_name'       => $this->config['wx_name'],
            'backurl'           => $this->config['sy_weburl'].'/weixin/index.php',
            'wx_token'   => $this->config['wx_token'],

            'wx_appid'       => $this->config['wx_appid'],
            'wx_appsecret'       => $this->config['wx_appsecret'],

            'wx_welcom'  => $this->config['wx_welcom'],
            'wx_welcom_type'  => !empty($this->config['wx_welcom_type'])?$this->config['wx_welcom_type']:'nowxcom',
            'sy_wxcom_pic' => checkpic($this->config['sy_wxcom_pic']),
            'wx_search'  => $this->config['wx_search'],
            'wx_search_no'  => $this->config['wx_search_no'],
            'sy_wx_qcode'  => checkpic($this->config['sy_wx_qcode']),
            'sy_wx_logo'  => checkpic($this->config['sy_wx_logo']),
            'sy_wx_sharelogo'  => checkpic($this->config['sy_wx_sharelogo']),
            'wx_rz'  => $this->config['wx_rz'],
            'wx_author'  => $this->config['wx_author'],
            'wx_author_htlogin'  => isset($this->config['wx_author_htlogin']) ? $this->config['wx_author_htlogin'] : '1',
            'wx_popWin'  => $this->config['wx_popWin'],
        );
        
        $this->render_json(0,'ok', $data);
    }

    /**
     *users:王旭
     *Data:2023/10/21
     *Time:8:55
     * 保存配置
     */
    function save_action(){
        
        $_POST =  $this->post_trim($_POST);
        
        $configM  =  $this->MODEL('config');
        
        $configM -> setConfig($_POST);
        
        $this->web_config();
        
        $this->render_json(0,'微信配置更新成功！');
    }

    /**
     *users:王旭
     *Data:2023/10/21
     *Time:8:56
     * 获取微信菜单数据
     */
    function wxnav_action(){
        //实例化
        $weiXinM            =   $this->MODEL('weixin');

        $where['id']        =   array('>',0);

        $where['orderby']   =   'sort,asc';

        $navlist            =   $weiXinM->getWxNavList($where);

        $data['list']    =   array_values($navlist);

        $this->render_json(0,'',$data);
    }

    /**
     *users:王旭
     *Data:2023/10/21
     *Time:8:56
     * 删除微信菜单数据
     */
    function delnav_action(){

        //实例化
        $weiXinM    =   $this->MODEL('weixin');

        if(is_array($_POST['del'])){

            $where['id']    =   array('in',pylode(',',$_POST['del']));

            $where['keyid'] =   array('in',pylode(',',$_POST['del']),'or');

            $weiXinM->delWxNav($where,array('type'=>'all'));

            $error = 0;
            $msg = '微信菜单(ID:'.pylode(',',$_POST['del']).')删除成功！';
        }else{

            $where['id']    =   (int)$_POST['del'];

            $where['keyid'] =   array('=',(int)$_POST['del'],'or');

            //删除微信菜单及子菜单
            $id =   $weiXinM->delWxNav($where,array('type'=>'one'));

            if($id){
                $error = 0;
                $msg = '微信菜单(ID:'.$_POST['del'].')删除成功！';
            }else{
                $error = 1;
                $msg = '删除失败！';
            }
        }

        $this->admin_json($error,$msg);
    }

    /**
     *users:王旭
     *Data:2023/10/21
     *Time:8:57
     * 保存菜单
     */
    function savenav_action(){

        //实例化
        $weiXinM    =   $this->MODEL('weixin');

        $logM       =   $this->MODEL('log');

        if($_POST['name'] && $_POST['keyid']!==''){

            $data['name']       =   $_POST['name'];

            $data['key']        =   $_POST['key'];

            $data['keyid']      =   $_POST['keyid'];

            $data['url']        =   $_POST['url'];

            $data['type']       =   $_POST['type'];

            $data['sort']       =   $_POST['sort'];

            $data['appid']      =   $_POST['appid'];

            $data['apppage']    =   $_POST['apppage'];

            $where['name']      =   $_POST['name'];

            if($_POST['keyid']>0){

                if(!$_POST['key'] && $_POST['type']=='click'){
                    $this->render_json(1);
                
                }elseif($_POST['type']=='miniprogram' && (!$_POST['url'] || !$_POST['appid'] || !$_POST['apppage'])){

                    $this->render_json(1);

                }elseif($_POST['type']=='view' && !$_POST['url']){

                    $this->render_json(1);

                }else{

                    $where['PHPYUNBTWSTART']        =   '';

                    $where['name']          =   $_POST['name'];

                    $where['keyid']         =   $_POST['keyid'];

                    $where['PHPYUNBTWEND']  =   '';

                }
            }

            if($_POST['navid']>0){

                $where['id']    =   array('<>',$_POST['navid']);

            }

            $nav = $weiXinM->getWxNavNum($where);

            if($nav>0){

                $this->render_json(2);

            }

            unset($_POST['pytoken']);

            if($_POST['navid']>0){

                $navid = $_POST['navid'];

                unset($_POST['navid']);

                $upWhere['id']  =   $navid;

                $weiXinM->upWxNavInfo($upWhere,$data);

                $logM   ->addAdminLog('微信菜单(ID:'.$navid.')修改成功');

            }else{

                $navid  =   $weiXinM->addWxNavInfo($data);

                $logM   ->addAdminLog('微信菜单(ID:'.$navid.')添加成功');
            }

            $this->render_json(3);

        }else{

            $this->render_json(1);

        }

    }

    function ajaxnav_action(){
        //实例化
        $weiXinM    =   $this->MODEL('weixin');

        $logM       =   $this->MODEL('log');

        if($_POST['sort']){

            $upWhere['id']  =   $_POST['id'];

            $data['sort']   =   $_POST['sort'];

            $weiXinM->upWxNavInfo($upWhere,$data);

            $logM->addAdminLog('微信菜单(ID:'.$_POST['id'].')排序修改成功');
        }

        if($_POST['name']){

            $upWhere['id']  =   $_POST['id'];

            $data['name']   =   $_POST['name'];

            $weiXinM->upWxNavInfo($upWhere,$data);

            $logM->addAdminLog('微信菜单(ID:'.$_POST['id'].')名称修改成功');
        }

        $this->render_json(1);
    }

    /**
     *users:王旭
     *Data:2023/10/21
     *Time:8:57
     * 同步微信菜单到微信服务器
     */
    function creatnav_action(){

        //实例化
        $weiXinM            =   $this->MODEL('weixin');

        $where['id']        =   array('>',0);

        $where['orderby']   =   array('keyid,asc','sort,asc');

        $creatNav   =   $weiXinM->creatWxNavList($where);

        if(is_array($creatNav)) {

            $Token = getToken();

            $DelUrl = 'https://api.weixin.qq.com/cgi-bin/menu/delete?access_token='.$Token;
            CurlPost($DelUrl);

            $Url = 'https://api.weixin.qq.com/cgi-bin/menu/create?access_token='.$Token;
            $result = CurlPost($Url,urldecode(json_encode($creatNav)));

            $Info = json_decode($result);

            if($Info->errcode=='0' || $Info->errmsg=='ok'){

                $error=0;
                $msg = '微信菜单创建成功！';

            }else{
                $error=1;
                $msg = '错误代码:'.$Info->errcode.',错误信息:'.$Info->errmsg;
            }
        }else{
            $error=1;
            $msg = '暂无菜单信息';
        }

        $this->render_json($error,$msg);
    }

    /**
     *users:王旭
     *Data:2023/10/21
     *Time:8:58
     * 获取自动回复数据
     */
    function zdkeyword_action(){

        $weiXinM    =   $this->MODEL('weixin');


        if(trim($_POST['keyword'])){

            $where['keyword']   =   array('like',trim($_POST['keyword']));
        }

        $pageM          =   $this->MODEL('page');
        $pages          =   $pageM->adminPageList('wxzdkeyword', $where,$_POST['page'],array('limit'=>$_POST['limit']));

        if ($pages['total'] > 0) {

            $where['orderby']       =   'time,desc';
        
            $where['limit']     =   $pages['limit'];
            $list   =   $weiXinM->getWxzdkeywordList($where);
        }

        $data['list']           =   !empty($list['list'])?$list['list']:array();
        $data['total']          =   $pages['total'];
        $data['page_sizes']     =   $pages['page_sizes'];
        $data['page_size']     =   $pages['page_size'];
        $this->render_json(0, '', $data);
    }


    function delkeyword_action(){

        //实例化
        $weiXinM    =   $this->MODEL('weixin');

        if($_POST['del']){

            if(is_array($_POST['del'])){

                $where['id']    =   array('in',pylode(',',$_POST['del']));

                $del            =   $weiXinM->delWxzdkeyword($where,array('type'=>'all'));

                $layer_type     =   1;

                $delid          =   pylode(',',$_POST['del']);

            }else{

                $where['id']    =   (int)$_POST['del'];

                $del            =   $weiXinM->delWxzdkeyword($where,array('type'=>'one'));

                $layer_type     =   0;

                $delid          =   (int)$_POST['del'];
            }

            if($del){
                $error = 0;
                $msg = '(ID:'.$delid.')删除成功！';
            }else{
                $error = 1;
                $msg = '删除失败！';
            }

        }else{

            $error = 1;

            $msg = '请选择要删除的内容！';
        }
        

        $this->render_json($error,$msg);

    }
    function getzdkeyword_action(){

        $weiXinM        =   $this->MODEL('weixin');
        
        $where['id']    =   (int)$_POST['id'];

        $row    =   $weiXinM->getWxzdkeyword($where);

        $data['row'] = $row;
        

        $this->render_json(0,'',$data);
    }

    function saveZdKeyword_action(){

        $_POST = $this->post_trim($_POST);

        $wxM = $this->MODEL('weixin');

        $uploadM = $this->MODEL('upload');

        $del_idarr = $_POST['del_idarr'];

        $newpic = array();

        $_POST['content']  =  stripslashes($_POST['content']);

        
        $con_post = json_decode($_POST['content'],true);
        
        //表单检验
        if($_POST['title']==''){
            $this->render_json(1,'规则名称不能为空！');
        }

        if($_POST['keyword']==''){
            $this->render_json(2,'关键字不能为空！');
        }

        foreach ($_FILES['newpic'] as $nk => $nv) {
            foreach ($nv as $nkk => $nvv) {
                $newpic[$nkk][$nk] = $nvv;
            }
        }
        
        $addval = array();
        $editval = array();
        $time = time();

        $pkey = 0;
        foreach($con_post as $ck=>$cv){

            $conval = array();
            $content = array();

            $conval['sort'] = $cv['sort'];
            $conval['msgtype'] = $cv['msgtype'];
            $conval['time'] = $time;
            $cv['newimage'] = '';

            if($cv['addpic']==1){
                $cv['newimage'] = $newpic[$pkey];
                $pkey++;
            }

            if($cv['msgtype']=='text'){//文字

                if($cv['content']==''){
                    $this->render_json(3,'回复内容不能为空！');
                }

                $conval['content'] = str_replace(array('&apos;', '&quot;'), array("'", '"'),$cv['content']);

            }else if($cv['msgtype']=='image'){

                if($cv['media_id']=='' && $cv['newimage']==''){
                    $this->render_json(4,'请上传图片！');
                }

                if($cv['newimage']!=''){

                    $upArr  =   array(
                        'file'   =>  $cv['newimage'],
                        'dir'    =>  'wx',
                    );
                    
                    $result =   $uploadM->newUpload($upArr);

                    if (!empty($result['msg'])){
                        $this->render_json(5, $result['msg']);
                    }

                    $pic    =   $result['picurl'];
                    


                    $upMedia=   $wxM->upMedia($pic, $upArr);
                    if (!$upMedia['media_id']) {
                        $this->render_json(6, '图片素材上传失败！'.$upMedia['errmsg']);
                    } else {
                        $content['media_id'] = $upMedia['media_id'];
                        $content['image_n'] = $pic;

                        $conval['content'] = serialize($content);
                    }
                }
                
            }else if($cv['msgtype']=='xcx'){
                if($cv['xcx_title']==''){
                    $this->render_json(7, '卡片标题不能为空！');
                }
                if($cv['xcx_appid']==''){
                    $this->render_json(8, '小程序AppID不能为空！');
                }
                if($cv['xcx_pagepath']==''){
                    $this->render_json(9, '小程序路径不能为空！');
                }
                if($cv['media_id']=='' && $cv['newimage']==''){
                    $this->render_json(10, '请上传小程序封面图！');
                }

                $content['xcx_title'] = str_replace(array('&apos;', '&quot;'), array("'", '"'),$cv['xcx_title']);
                $content['xcx_appid'] = $cv['xcx_appid'];
                $content['xcx_pagepath'] = $cv['xcx_pagepath'];

                if($cv['newimage']!=''){

                    $upArr  =   array(
                        'file'   =>  $cv['newimage'],
                        'dir'    =>  'wx',
                    );
                    
                    $result =   $uploadM->newUpload($upArr);

                    if (!empty($result['msg'])){
                        $this->render_json(11, $result['msg']);
                    }

                    $pic    =   $result['picurl'];
                    


                    $upMedia=   $wxM->upMedia($pic, $upArr);
                    if (!$upMedia['media_id']) {
                        $this->render_json(12, '图片素材上传失败！'.$upMedia['errmsg']);
                    } else {
                        $content['media_id'] = $upMedia['media_id'];
                        $content['image_n'] = $pic;
                    }
                }else{
                    $content['media_id'] = $cv['media_id'];
                    $content['image_n'] = $cv['image_n'];
                }

                $conval['content'] = serialize($content);
            }
            if($cv['isadd']==1){//新增消息
                $addval[] = $conval;
            }else{//修改消息
                $conval['id'] = $cv['id'];
                $editval[] = $conval;
            }
        }
        
        //表单检验结束

        //添加，修改关键词和标题
        $data['title']      =   $_POST['title'];

        $data['keyword']    =   $_POST['keyword'];

        $data['time']       =   $time;

        if($_POST['id']){

            $zdWhere['id']      =   $_POST['id'];

            $kid = $_POST['id'];

            $wxM->upWxzdkeyword($zdWhere,$data);

        }else{

            $kid = $wxM->addWxzdkeyword($data);

        }
        //添加、修改、删除规则下消息
        if($kid){
            if(!empty($del_idarr)){
                $wxM->delWxzdCon(array('id'=>array('in',pylode(',',$del_idarr)),'kid'=>$kid),array('type'=>'all'));
            }
            if(!empty($addval)){

                foreach($addval as $avk=>$avv){
                    $avv['kid'] = $kid;
                    $wxM->addWxzdCon($avv);
                }
                
            }
            if(!empty($editval)){
                foreach($editval as $ek=>$ev){
                    $id=$ev['id'];unset($ev['id']);
                    $wxM->upWxzdCon(array('id'=>$id),$ev);
                }
            }
        }

        $this->render_json(0,'保存成功');
    }
}
?>