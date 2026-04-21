<?php
error_reporting(0);

require_once("alipay.config.php");
require_once("lib/alipay_notify.class.php");
require_once(dirname(dirname(dirname(__FILE__)))."/global.php");

if($alipay_config['sign_type'] == 'MD5') {
    $alipayNotify = new AlipayNotify($alipay_config);
    $verify_result = $alipayNotify->verifyReturn();
    if ($verify_result) {


        $out_trade_no = $_GET['out_trade_no'];

        $trade_no = $_GET['trade_no'];

        $result = $_GET['result'];

        $dingdan = $out_trade_no;
        $total_fee = $_GET['total_fee'];
        $sOld_trade_status = "0";

        if (!preg_match('/^[0-9]+$/', $dingdan)) {
            die;
        }

        if (($result == 'TRADE_FINISHED') || ($result == 'TRADE_SUCCESS') || ($result == 'success')) {

            require_once(APP_PATH . 'app/public/common.php');
            require_once(LIB_PATH . 'ApiPay.class.php');

            $apiPay = new apipay($phpyun, $db, $db_config['def'], 'index');

            $apiPay->payAll($out_trade_no, $total_fee, 'wapalipay');
        } else {
            echo '<!DOCTYPE HTML>
                <html>
                    <head>
                	<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
                
                        <title>支付宝即时到账交易接口</title>
                	</head>
                    <body>' . "trade_status=" . $_GET['trade_status'] . '</body></html>';
            die;
        }
    } else {
        echo "验证失败";
        die;
    }
}else{
    require_once dirname(dirname(dirname(__FILE__))).'/api/aop/wap/AopClient.php';
    $data = $_GET;
    $aop = new AopClient();
    $aop->alipayrsaPublicKey = $alipaydata['sy_alipaypublickey'];
    $result = $aop->rsaCheckV1($data, $alipaydata['sy_alipaypublickey'], $alipay_config['sign_type']);
    if($result){
        //放入订单交易完成后的数据库更新程序代码，请务必保证echo出来的信息只有success
        //为了保证不被重复调用，或重复执行数据库更新程序，请判断该笔交易状态是否是订单未处理状态
        $out_trade_no = $data['out_trade_no'];
        $total_fee = $data['total_fee'];

        if (!preg_match('/^[0-9]+$/', $out_trade_no)) {
            die;
        }

        require_once(APP_PATH . 'app/public/common.php');
        require_once(LIB_PATH . 'ApiPay.class.php');

        $apiPay = new apipay($phpyun, $db, $db_config['def'], 'index');

        $apiPay->payAll($out_trade_no, $total_fee, 'wapalipay');
    }else{
        echo "验证失败";
        die;
    }
}
if(!($config['sy_wapdomain'])){
	$wapdomain=$config['sy_weburl'].'/'.$config['sy_wapdir'];
}else{
	$wapdomain='http://'.$config['sy_wapdomain'];
}
$order_sql  =   $db->query("SELECT * FROM `".$db_config["def"]."company_order` WHERE `order_id`='".$out_trade_no."' limit 1");
$orderInfo  =   $db->fetch_array($order_sql);
if (!$orderInfo['fast']) {

    $member_sql =   $db->query("SELECT * FROM `" . $db_config["def"] . "member` WHERE `uid`='" . $_COOKIE['uid'] . "' limit 1");
    $member     =   $db->fetch_array($member_sql);
    if ($member['usertype'] != $_COOKIE['usertype'] || md5($member['username'].$member['password'].$member['salt']) != $_COOKIE['shell']) {
        die;
    }
    if ($member['usertype'] == 1) {

        $Loaction   =   $wapdomain."/member/index.php?c=paylog";
    } elseif ($member['usertype'] == 2) {

        $Loaction   =   $wapdomain."/member/index.php?c=com";
    } elseif ($member['usertype'] == 3) {

        $Loaction   =   $wapdomain."/member/index.php?c=mypay";
    }
} else {

    $Loaction   =   $wapdomain."/index.php?c=once";
}
header("Location:".$Loaction);
?>