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
class company_company_controller extends adminCommon
{
    //TODO 6.0 admin/model/admin_company.class.php

    /**
     * @desc 会员企业列表，点击企业用户名成，跳转企业会员中心
     *
     * @param  $_GET['type']，跳转招聘统计分析页面
     */
    function Imitate_action(){

        $userinfoM  =  $this->MODEL('userinfo');

        $member     =  $userinfoM -> getInfo(array('uid'=> intval($_GET['uid'])),array('field'=>'`uid`,`username`,`salt`,`email`,`password`,`usertype`,`did`'));

        $this -> cookie->unset_cookie($_SESSION['auid']);

        $this -> cookie->add_cookie($member['uid'],$member['username'],$member['salt'],$member['email'],$member['password'],2,$this->config['sy_logintime'],$member['did'],'1');

        $typeStr 	=	trim($_GET['type']);

        $url		=	'';

        if(!empty($typeStr)){

            if ($typeStr == 'job') {

                $url = 'index.php?c='.$typeStr;
            }else{

                $url = 'index.php?c='.$typeStr;
            }

        }
        $logM  		=  $this->MODEL('log');

        $content	=	'管理员'.$_SESSION['ausername'].'登录企业账户(ID:'.$member['uid'].')';

        $adminLo	=	$logM -> addAdminLog($content);

        header('Location: '.$this->config['sy_weburl'].'/member/'.$url);
    }

    // 弹窗预览企业详情
    function compreview_action(){

        $ComM  =  $this -> MODEL('company');
        
        $Info  =  $ComM->getInfo(intval($_POST['uid']));

        $this->render_json(0, '', $Info);
    }
}