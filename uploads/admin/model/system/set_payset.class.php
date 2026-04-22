<?php

class set_payset_controller extends adminCommon{
    /**
     * 设置-支付设置
     */
	function index_action(){
	    // 支付宝配置信息
        if ($this->config['alipaytype'] == "1") {
            $dir = "alipay";
        }
        @include(APP_PATH . "data/api/" . $dir . "/alipay_data.php");
		if (empty($alipaydata)) {
            $alipaydata = array(
                'alipaytype' => '1',
                'sy_alipayname' => '',
                'sy_alipayKeyType' => '1',//加密模式，默认：MD5加密
                'sy_alipayid' => '',
                'sy_alipaycode' => '',
                'sy_alipayemail' => '',
                'sy_alipayappid' => '',
                'sy_alipayprivatekey' => '',
                'sy_alipaypublickey' => '',

            );
        }
        //财付通配置信息
        @include(APP_PATH . "data/api/tenpay/tenpay_data.php");
        // 银行转账
        $ConfigM = $this->MODEL('config');
        $bankrows = $ConfigM->getBankList();
        $this->render_json(0,'ok', array(
            'config' => $this->config,
            'alipaydata' => $alipaydata,
            'tenpaydata' => $tenpaydata,
            'bankrows' => $bankrows
        ));
	}
    /**
	 * 支付设置-安装、卸载
	 */
    function save_action()
    {
        $ConfigM = $this->MODEL('config');
        if ($_POST['config']) {
            unset($_POST['config']);
            foreach ($_POST as $key => $v) {
                $config = $ConfigM->getNum(array('name' => $key));
                if ($config == false) {
                    $ConfigM->addInfo(array('name' => $key, 'config' => $v));
                } else {
                    $ConfigM->upInfo(array('name' => $key), array('config' => $v));
                }
            }
            $this->web_config();
            $this->render_json(0, '修改成功');
        }
    }
    /**
     * 支付宝设置
     */
    function alipay_action()
    {
        $ConfigM = $this->MODEL('config');
        if ($_POST['pay_config']) {
            $alipaya['sy_weburl'] = $this->config['sy_weburl'];
            $alipaya['alipaytype'] = trim($_POST['alipaytype']);
            $alipaya['sy_alipayname'] = trim($_POST['sy_alipayname']);
            $alipaya['sy_alipayKeyType'] = trim($_POST['sy_alipayKeyType']);//加密方式：1-MD5; 2-公钥加密
            $alipaya['sy_alipayid'] = trim($_POST['sy_alipayid']);
            $alipaya['sy_alipaycode'] = trim($_POST['sy_alipaycode']);
            $alipaya['sy_alipayemail'] = trim($_POST['sy_alipayemail']);
            $alipaya['sy_alipayappid'] = trim($_POST['sy_alipayappid']);
            $alipaya['sy_alipayprivatekey'] = trim($_POST['sy_alipayprivatekey']);
            $alipaya['sy_alipaypublickey'] = trim($_POST['sy_alipaypublickey']);
            if ($_POST['alipaytype'] == "1") {
                $dir = "alipay";
            }
            $alipay_v = $ConfigM->getInfo(array('name' => 'alipaytype'));
            if (empty($alipay_v)) {
                $ConfigM->addInfo(array('config' => $_POST['alipaytype'], 'name' => 'alipaytype'));
            } else {
                $ConfigM->upInfo(array('name' => 'alipaytype'), array('config' => $_POST['alipaytype']));
            }
            $this->web_config();
            made_web(APP_PATH . "data/api/" . $dir . "/alipay_data.php", ArrayToString($alipaya), "alipaydata");
            $this->admin_json(0, "支付宝配置成功！");
        }
    }
    
    /*
     * 财付通设置
     */
    function tenpay_action()
    {
        if ($_POST['pay_config']) {
            $tenpay['sy_weburl'] = $this->config['sy_weburl'];
            $tenpay['sy_tenpayid'] = trim($_POST['sy_tenpayid']);
            $tenpay['sy_tenpaycode'] = trim($_POST['sy_tenpaycode']);
            made_web(APP_PATH . "data/api/tenpay/tenpay_data.php", ArrayToString($tenpay), "tenpaydata");
            $this->admin_json(0, "财付通配置成功！");
        }
    }
    /*
     * 银行卡转账
     */
    function bank_action()
    {
        $ConfigM = $this->MODEL('config');
        if ($_POST['pay_config']) {
            $postData = array(
                'name' => $_POST['name'],
                'bank_name' => $_POST['bank_name'],
                'bank_number' => $_POST['bank_number'],
                'bank_address' => $_POST['bank_address'],
            );
            $bankInfo = $ConfigM->getBankInfo(array('bank_number' => $_POST['bank_number']), array('field'=>'id,bank_number'));
            if ((!empty($_POST['id']) && $bankInfo && $bankInfo['id'] != $_POST['id']) || (empty($_POST['id']) && $bankInfo)) {
                $this->render_json(1, '银行卡已存在');
            }
            
            if (!$_POST['id']) {
                $bank = $ConfigM->addBank($postData);
                $this->admin_json(0, "银行卡(ID:" . $bank . ")添加成功！");
            } else {
                $ConfigM->upBank(array('id' => $_POST['id']), $postData);
                $this->admin_json(0, "银行卡(ID:" . $_POST['id'] . ")修改成功！");
            }
        }
    }

    function del_action()
    {
        $ConfigM = $this->MODEL('config');
        $ConfigM->delBank(array('id' => $_POST['del']));
        $this->admin_json(0, "银行卡(ID:" . $_POST['del'] . ")删除成功！");
    }
}
?>