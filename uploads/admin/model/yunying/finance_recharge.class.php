<?php

/**
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2022 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

class finance_recharge_controller extends adminCommon
{
    function index_action()
    {

        $RatingM    =   $this->MODEL('rating');
        $rating_list=   $RatingM->getList(array('category' => 1), array('field' => 'id,name,service_price,service_time'));

        $freeRating = $RatingM->getList(array('category' => 1,'service_time'=>0), array('field' => 'id,name,service_price,service_time'));

        $service_list = $RatingM	->	getComServiceList(array('display'=>'1','orderby'=>'sort,desc'));

        $this->render_json(0,'',[
            'rating_list'   =>$rating_list,
            'ratingid'  =>$freeRating[0]['id'],
            'service_list'  =>$service_list,
            'integral_pricename'=>$this->config['integral_pricename'],
            'integral_priceunit'=>$this->config['integral_priceunit'],
        ]);

    }

    function jifenSave_action(){

        if (!$_POST['userarr']){
            $this->render_json(1,'参数错误');
        }
        if ($_POST['integral']<1){
            $this->render_json(1,'输入的积分不能为0');
        }
        $OrderM     =   $this->MODEL('companyorder');
        $userarr    =   str_replace('，', ',', trim($_POST['userarr']));
        $userarr    =   @explode(',', trim($userarr));

        $post       =   array(
            'fs'        =>  intval($_POST['fs']),
            'integral' =>  trim($_POST['integral']),
            'order_price'   =>   $_POST['order_price'],
            'remark'    =>  trim($_POST['remark'])
        );

        $return     =   $OrderM->PayMember($userarr, $post);
        if ($return['errcode']== 9){
            $this->admin_json(0,$return['msg']);
        }else{
            $this->admin_json(1,$return['msg']);
        }
    }

    /**
     * 后台开通会员
     */
    function comvip_action()
    {
        $OrderM =   $this->MODEL('companyorder');

        $post   =   array(
            'username'  =>  trim($_POST['username']),
            'comname'   =>  trim($_POST['comname']),
            'ratingid'  =>  intval($_POST['ratingid']),
            'vipprice'  =>  $_POST['vipprice'],
            'leijia'    =>  intval($_POST['leijia']),
            'remark'    =>  $_POST['remark'],
            'uid'       =>  intval($_POST['uid']),
            'vipetime'       =>  $_POST['vipetime']?$_POST['vipetime']:'不限',
        );
        $return =   $OrderM->ComVip($post);
        if ($return['errcode']== 9){
            $this->admin_json(0,$return['msg']);
        }else{
            $this->admin_json(1,$return['msg']);
        }
    }
    function comservice_action(){
        $OrderM =   $this->MODEL('companyorder');

        $post   =   array(

            'username'  =>  trim($_POST['username2']),
            'comname'   =>  trim($_POST['comname2']),
            'serviceid'  =>  intval($_POST['serviceid']),
            'service_package'=>intval($_POST['service_package']),
            'service_price'=>$_POST['service_price'],
            'uid'       =>  intval($_POST['uid'])
        );

        $return =   $OrderM->comservice($post);
        if ($return['errcode']== 9){
            $this->admin_json(0,$return['msg']);
        }else{
            $this->admin_json(1,$return['msg']);
        }
    }
    function getservice_action(){
        $ratingM	=	$this	->	MODEL('rating');
        $info		=	$ratingM	->	getComSerDetailList(array('type'=>$_POST['type'],'orderby'=>'sort,desc'));
        if ($info){
            $this->render_json(0,'',$info);
        }else{
            $this->render_json(1,'参数错误,请重试');
        }

    }

    function searchname_action()
    {

        $name   =   $this->post_trim($_POST['username']);
        $OrderM =   $this->MODEL('companyorder');
        $return =   $OrderM->SearchName($name);
        echo json_encode($return);
        die;
    }

    function searchcom_action()
    {

        $name   =   $this->post_trim($_POST['comname']);
        $OrderM =   $this->MODEL('companyorder');
        $return =   $OrderM->SearchCom($name);
        echo json_encode($return);
        die;
    }


}

?>