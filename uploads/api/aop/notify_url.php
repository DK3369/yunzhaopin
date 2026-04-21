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

error_reporting(0);
require_once 'AopClient.php';
require_once(dirname(dirname(dirname(__FILE__)))."/data/api/alipay/alipay_aop.php");
require_once(dirname(dirname(dirname(__FILE__)))."/global.php");

$aop = new AopClient();
$aop->alipayrsaPublicKey = $alipaydata['sy_publicKey'];
$aop->signType = 'RSA2';

$requestParamsArr  =  $_POST;

//除去sign、sign_type两个参数外，凡是通知返回回来的参数皆是待验签的参数。
unset($requestParamsArr['sign_type']);
$requestParamsArr['fund_bill_list']  =  stripslashes($_POST['fund_bill_list']);
//验证RSA2格式签名
$verify_result  =  $aop->rsaCheckV2($requestParamsArr, $aop->alipayrsaPublicKey,$aop->signType);

if($verify_result) {
	if(!preg_match('/^[0-9]+$/', $_POST['out_trade_no'])){
		die;
	}
    //验证成功
    //获取支付宝的反馈参数
    $dingdan           = $_POST['out_trade_no'];	    //获取支付宝传递过来的订单号
    $total_amount      = $_POST['total_amount'];	    //获取支付宝传递过来的总价格

    if($_POST['trade_status'] == 'TRADE_FINISHED' ||$_POST['trade_status'] == 'TRADE_SUCCESS') {    //交易成功结束
        //放入订单交易完成后的数据库更新程序代码，请务必保证echo出来的信息只有success
        //为了保证不被重复调用，或重复执行数据库更新程序，请判断该笔交易状态是否是订单未处理状态 

		//根据订单号更新订单，把订单处理成交易成功
		require_once(APP_PATH.'app/public/common.php');
		require_once(LIB_PATH.'ApiPay.class.php');

		$apiPay = new apipay($phpyun,$db,$db_config['def'],'index');
		
		$return = $apiPay->payAll($dingdan,$total_amount,'alipay');
		if ($return==2){
		    echo "success";
		}
		 
	} else {
        echo "success";		//其他状态判断。普通即时到帐中，其他状态不用判断，直接打印success。
    }

}else {
    //验证失败
    echo "fail";
}
?>