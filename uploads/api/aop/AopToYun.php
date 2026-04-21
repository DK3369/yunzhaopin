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
class AlipayMini{
    
    function alipay($data)
    {
        require_once 'AopClient.php';
        require_once 'request/AlipayTradeCreateRequest.php';
        require_once 'request/AlipaySystemOauthTokenRequest.php';
        
        require (DATA_PATH."api/alipay/alipay_aop.php");
        
        $aop = new AopClient();
        $aop->gatewayUrl = 'https://openapi.alipay.com/gateway.do';
        $aop->appId = $alipaydata['sy_xcxAppid'];
        $aop->rsaPrivateKey = $alipaydata['sy_privateKey'];
        $aop->alipayrsaPublicKey = $alipaydata['sy_publicKey'];
        $aop->apiVersion = '1.0';
        $aop->signType = 'RSA2';
        $aop->postCharset='utf-8';
        $aop->format='json';
        
        //1 获取用户user_id
        $t_request = new AlipaySystemOauthTokenRequest();
        $t_request->setGrantType("authorization_code");
        $t_request->setCode($data['code']);
        
        //自定义抛出异常
        set_exception_handler('myException');
        
        $t_result = $aop->execute($t_request);
        
        $responseNode = str_replace(".", "_", $t_request->getApiMethodName()) . "_response";
        $resultCode = $t_result->$responseNode->code;
        
        if(!empty($resultCode)&&$resultCode != 10000){
            
            return array('msg'=>$t_result->$responseNode->sub_msg);
        } else {
            
            $user_id  =  $t_result->$responseNode->user_id;
        }
        //2 创建交易订单
        $c_request = new AlipayTradeCreateRequest();
        $c_request->setBizContent("{" .
            "\"out_trade_no\":\"".$data['id']."\"," .
            "\"total_amount\":\"".$data['total_amount']."\"," .
            "\"subject\":\"".$data['subject']."\",".
            "\"buyer_id\":\"".$user_id."\"".
            "}");
        //$c_request->setReturnUrl($returnUrl);
        $c_request->setNotifyUrl($alipaydata['sy_weburl'].'/api/aop/notify_url.php');
        $c_result = $aop->execute($c_request);
        
        $responseNode = str_replace(".", "_", $c_request->getApiMethodName()) . "_response";
        $resultCode = $c_result->$responseNode->code;
        if(!empty($resultCode)&&$resultCode == 10000){
            
            return array('trade_no'=>$c_result->$responseNode->trade_no);
            
        } else {
            
            return array('msg'=>$t_result->$responseNode->sub_msg);
        }
    }
}

function myException($exception)
{
    echo "<b>Exception:</b> " , $exception->getMessage();
}
?>