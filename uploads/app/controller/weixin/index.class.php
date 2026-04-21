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

class index_controller extends common
{

    function index_action()
    {

        $M  =   $this->MODEL('weixin');

        if ($_GET["echostr"]) {

            $M->valid($_GET['echostr'], $_GET['signature'], $_GET['timestamp'], $_GET['nonce']);
        } else {

            $postStr    =   file_get_contents("php://input");

            if (!empty($postStr)) {

                libxml_disable_entity_loader(true);

                $postObj        =   simplexml_load_string($postStr, 'SimpleXMLElement', LIBXML_NOCDATA);
                $fromUsername   =   $this->pregWxid($postObj->FromUserName);
                $toUsername     =   $this->pregWxid($postObj->ToUserName);
                $keyword        =   trim($this->gpc2sql($postObj->Content));
                $times          =   time();
                $MsgType        =   $postObj->MsgType;

                $topTpl         =   "<xml>
				   <ToUserName><![CDATA[%s]]></ToUserName>
				   <FromUserName><![CDATA[%s]]></FromUserName>
				   <CreateTime>%s</CreateTime>
				   <MsgType><![CDATA[%s]]></MsgType>
				   ";

                $bottomStr      =   "<FuncFlag>0</FuncFlag></xml>";

                //默认定义
                $this->MsgType  =   'text';
                $centerStr      =   "<Content><![CDATA[未找到相关数据]]></Content>";
                
                if (empty($fromUsername) || empty($toUsername)){
                    // 参数经过处理有，有异常，直接返回成功
                    echo "success";
                    exit();
                }
                // 公众号内所有事件，都查询是否绑定 -start
                $openidNeedSearch = true;
                if ($MsgType == 'event') {
                    if ($postObj->Event == 'subscribe' || $postObj->Event =='SCAN') {
                        if (!empty($postObj->EventKey)) {
                            // 带参扫码事件，非纯数字的需要处理（纯数字的是扫码登录或者扫码绑定，那边已有处理）
                            if (is_numeric($postObj->EventKey)){
                                $openidNeedSearch = false;
                            }
                        }
                    }elseif ($postObj->Event == 'CLICK'){
                        // 点击事件，符合下面的，里面都有查询处理
                        if (!empty($postObj->EventKey)) {
                            if (in_array($postObj->EventKey, array('我的帐号','我的消息','面试邀请','简历浏览','刷新简历','刷新职位','简历投递','兼职报名'))){
                                $openidNeedSearch = false;
                            }
                        }
                    }
                }
                if ($openidNeedSearch){
                    $bind = $this->upUserInfo($fromUsername);
                }
                // 公众号内所有事件，都查询是否绑定 -end
                
                if ($MsgType == 'event') {

                    $MsgEvent   =   $postObj->Event;

                    if ($MsgEvent == 'subscribe') {

                        //判断是否为带参数扫码登录
                        if (!empty($postObj->EventKey)) {

                            //取出场景值
                            $wxloginid  =   str_replace('qrscene_', '', $postObj->EventKey);

                            //判断场景来源
                            if (strpos($wxloginid, 'weixin_') !== false) {
                                //调用客服消息发送关注欢迎语
                                $M->sendCustom($fromUsername, 'text');

                                //调用客服消息发送增量欢迎方式 图片/小程序
                                if ($this->config['wx_welcom_type'] == 'wxcompic') {

                                    $M->sendCustom($fromUsername, 'image');
                                } elseif ($this->config['wx_welcom_type'] == 'wxcomxcx') {

                                    $M->sendCustom($fromUsername, 'miniprogrampage');
                                }
                                // 强制关注公众号场景码
                                if (strpos($wxloginid, 'gzhid_') !== false){
                                    // 账号没有通过上面自动绑定函数，绑定成功的
                                    if (isset($bind) && $bind == false){
                                        $this->bindUserByGzh($wxloginid, $fromUsername);
                                    }
                                    //返回success
                                    echo "success";
                                    exit();
                                }else{
                                    // 其他场景码，回复图文消息
                                    $Return         =   $M->sendPubLink($wxloginid);
                                    $centerStr      =   $Return['centerStr'];
                                    $this->MsgType  =   $Return['MsgType'];
                                }
                            } elseif (strpos($wxloginid, 'xcx_') !== false) {

                                //获取场景码对应小程序卡片数据
                                $xcxInfo    =   $M->getXcxPubLink($wxloginid);

                                //调用客服消息发送欢迎语
                                $M->sendCustom($fromUsername);

                                //如果有增量欢迎方式 只发送图片 小程序不发送与下方重复
                                if ($this->config['wx_welcom_type'] == 'wxcompic') {

                                    $M->sendCustom($fromUsername, 'image', array());
                                }

                                //调用客服消息发送场景码对应小程序卡片
                                $M->sendCustom($fromUsername, 'miniprogrampage', $xcxInfo);

                                //返回success
                                echo "success";
                                exit();
                            } elseif(strpos($wxloginid, 'adminlogin_') !== false){//管理員後臺登錄
                                
                                $loginType  =   $M->isAdminWxlogin($fromUsername, $wxloginid);
                                $result = $loginType['result'];
                                $msg = $loginType['msg'];
                                
                                if (!$result) {
                                    $centerStr = "<Content><![CDATA[".$msg."]]></Content>";
                                } else {
                                    $centerStr = "<Content><![CDATA[扫码登录成功！]]></Content>";
                                }

                                $this->MsgType = 'text';

                            }else {

                                $loginType  =   $M->isWxlogin($fromUsername, $wxloginid);

                                if (is_array($loginType)) {

                                    $result = $loginType['result'];
                                    $utype = $loginType['type'];
                                } else {

                                    $result = $loginType;
                                }
                                if (!$result) {
                                    if (isset($utype) && $utype == 'amminwxbind') {

                                        $centerStr = "<Content><![CDATA[要绑定的管理员账号不存在！]]></Content>";
                                    } else {
                                        if (isset($utype) && $utype == 'regbindacount') {

                                            $centerStr = "<Content><![CDATA[扫码成功,请绑定已有账号或直接创建新账号！]]></Content>";
                                        } else if ($utype == 'regphone') {

                                            $centerStr = "<Content><![CDATA[扫码成功，基于安全需要，请绑定手机号，绑定后可以使用该手机号快速登录]]></Content>";
                                        } else {

                                            $centerStr = "<Content><![CDATA[扫码关注成功！]]></Content>";
                                        }
                                    }
                                } else {
                                    if (isset($utype) && $utype == 'amminwxbind' || $utype == 'userwxbind') {

                                        $centerStr = "<Content><![CDATA[扫码绑定成功！]]></Content>";
                                    } else {

                                        $centerStr = "<Content><![CDATA[扫码登录成功！]]></Content>";
                                    }
                                }
                                $this->setSubscribe($fromUsername, 1);
                                $this->MsgType = 'text';
                            }

                        } else {

                            if (!$this->config['wx_welcom_type'] || $this->config['wx_welcom_type'] == 'nowxcom') {

                                if ($this->config['wx_welcom']) {

                                    $centerStr = "<Content><![CDATA[" . $this->config['wx_welcom']."]]></Content>";
                                } else {

                                    $centerStr = "<Content><![CDATA[欢迎您关注" . $this->config['sy_webname'] . "！\n 1：您可以直接回复关键字如【销售】、【销售 XX公司】查找您想要的职位\n绑定您的账户体验更多精彩功能\n感谢您的关注！"."]]></Content>";
                                }
                                
                                $this->MsgType = 'text';
                            } else {

                                //调用客服消息发送关注欢迎语
                                $M->sendCustom($fromUsername, 'text');

                                //调用客服消息发送增量欢迎方式 图片/小程序
                                if ($this->config['wx_welcom_type'] == 'wxcompic') {

                                    $M->sendCustom($fromUsername, 'image', array());
                                } elseif ($this->config['wx_welcom_type'] == 'wxcomxcx') {

                                    $M->sendCustom($fromUsername, 'miniprogrampage');
                                }

                                echo "success";
                                exit();
                            }
                        }
                    } else if ($MsgEvent == 'SCAN'){ //已关注二维码待参数事件

                        //判断是否为带参数扫码登录

                        $wxloginid  =   $postObj->EventKey;

                        //判断场景来源
                        if (strpos($wxloginid, 'weixin_') !== false) {

                            // 强制关注公众号场景码
                            if (strpos($wxloginid, 'gzhid_') !== false){
                                // 账号没有通过上面自动绑定函数，绑定成功的
                                if (isset($bind) && $bind == false){
                                    $this->bindUserByGzh($wxloginid, $fromUsername);
                                }
                                //返回success
                                echo "success";
                                exit();
                            }else{
                                // 其他场景码，回复图文消息
                                $Return         =   $M->sendPubLink($wxloginid);
                                $centerStr      =   $Return['centerStr'];
                                $this->MsgType  =   $Return['MsgType'];
                            }

                        } elseif (strpos($wxloginid, 'xcx_') !== false) {

                            //获取场景码对应小程序卡片数据
                            $xcxInfo        =   $M->getXcxPubLink($wxloginid);

                            //调用客服消息发送场景码对应小程序卡片
                            $M->sendCustom($fromUsername, 'miniprogrampage', $xcxInfo);

                            //返回success
                            echo "success";
                            exit();
                        } elseif(strpos($wxloginid, 'adminlogin_') !== false){//管理員後臺登錄
                                
                            $loginType  =   $M->isAdminWxlogin($fromUsername, $wxloginid);
                            $result = $loginType['result'];
                            $msg = $loginType['msg'];
                            
                            if (!$result) {
                                $centerStr = "<Content><![CDATA[".$msg."]]></Content>";
                            } else {
                                $centerStr = "<Content><![CDATA[扫码登录成功！]]></Content>";
                            }

                            $this->MsgType = 'text';

                        }else {

                            //未绑定则提示
                            $loginType      =   $M->isWxlogin($fromUsername, $wxloginid);
                            if (is_array($loginType)) {

                                $result     =   $loginType['result'];
                                $utype      =   $loginType['type'];
                            } else {

                                $result     =   $loginType;
                            }
                            if (!$result) {
                                if (isset($utype) && $utype == 'amminwxbind') {

                                    $centerStr      =   "<Content><![CDATA[要绑定的管理员账号不存在！]]></Content>";
                                } else {
                                    if (isset($utype) && $utype == 'regbindacount') {

                                        $centerStr  =   "<Content><![CDATA[扫码成功,请绑定已有账号或直接创建新账号！]]></Content>";
                                    } else if ($utype == 'regphone') {

                                        $centerStr  =   "<Content><![CDATA[扫码成功，基于安全需要，请绑定手机号，绑定后可以使用该手机号快速登录！]]></Content>";
                                    } else {
                                        $centerStr  =   "<Content><![CDATA[扫码关注成功！]]></Content>";
                                    }
                                }
                            } else {
                                if (isset($utype) && $utype == 'amminwxbind' || $utype == 'userwxbind') {

                                    $centerStr      =   "<Content><![CDATA[扫码绑定成功！]]></Content>";
                                } else {

                                    $centerStr      =   "<Content><![CDATA[扫码登录成功！]]></Content>";
                                }
                            }
                            $this->MsgType = 'text';
                        }
                    } else if ($MsgEvent == 'CLICK') {

                        $ismatch    =   true;
                        $EventKey   =   $postObj->EventKey;

                        if ($EventKey == '我的帐号') {

                            $Return =   $M->bindUser($fromUsername);
                        } elseif ($EventKey == '我的消息') {

                            $Return =   $M->myMsg($fromUsername);
                        } elseif ($EventKey == '面试邀请') {

                            $Return =   $M->Audition($fromUsername);
                        } elseif ($EventKey == '简历浏览') {

                            $Return =   $M->lookResume($fromUsername);
                        } elseif ($EventKey == '刷新简历') {

                            $Return =   $M->refResume($fromUsername);
                        } elseif ($EventKey == '推荐职位') {

                            $Return =   $M->recJob($fromUsername);
                        } elseif ($EventKey == '刷新职位') {

                            $Return =   $M->refJob($fromUsername);
                        } elseif ($EventKey == '简历投递') {

                            $Return =   $M->ApplyJob($fromUsername);
                        } elseif ($EventKey == '兼职报名') {

                            $Return =   $M->PartApply($fromUsername);
                        } elseif ($EventKey == '职位搜索') {

                            if ($this->config['wx_search']) {

                                $Return['centerStr']    =   "<Content><![CDATA[" . $this->config['wx_search'] . "]]></Content>";
                            } else {

                                $Return['centerStr']    =   "<Content><![CDATA[直接回复城市、职位、公司名称等关键字搜索您需要的职位信息。\n 如：【经理】、【xx公司】]]></Content>";
                            }

                            $Return['MsgType']          =   'text';

                        } elseif ($EventKey == '周边职位') {

                            $Return['centerStr']        =   "<Content><![CDATA[/可怜 亲，把您的位置先发我一下。\n\n方法：点屏幕左下角输入框旁的“+”，选择“位置”，点“发送”。]]></Content>";
                            $Return['MsgType']          =   'text';
                        }else{
                            $ismatch  =  false;
                        }
                        if ($ismatch){
                            // 按照已有程序，匹配到的，回复匹配到的内容
                            $centerStr      =   $Return['centerStr'];
                            $this->MsgType  =   $Return['MsgType'];
                        }else{
                            // 没有匹配到的，按自动回复处理
                            $Returnone      =   $M->searchKeyword($fromUsername,$EventKey);
                            // $centerStr      =   $Returnone['centerStr'];
                            // $this->MsgType  =   $Returnone['MsgType'];
                            echo '';exit();
                        }
                    } else if ($MsgEvent == 'unsubscribe'){ //  取消关注

                        $this->setSubscribe($fromUsername, 0);
                    }
                } elseif ($MsgType == 'LOCATION') {

                    $latitude   =   $postObj->Location_X;
                    $longitude  =   $postObj->Location_Y;
                    $url        =   "http://api.map.baidu.com/geocoder/v2/?ak=42966293429086ba859198a2a69bedad&callback=renderReverse&location=" . $latitude . "," . $longitude . "&output=json";
                    $mapinfo    =   file_get_contents($url);
                    $mapinfo    =   str_replace(array('renderReverse&&renderReverse(', ')'), '', $mapinfo);
                    $map_info   =   json_decode($mapinfo, true);

                    if ($map_info['result']['addressComponent']['district']) {

                        $Return         =   $M->searchJob($map_info['result']['addressComponent']['district'], 1);
                        $centerStr      =   $Return['centerStr'];
                        $this->MsgType  =   $Return['MsgType'];
                    } else {

                        $Return['centerStr']    =   "<Content><![CDATA[未找到相关数据]]></Content>";
                        $Return['MsgType']      =   'text';
                    }

                } else if ($MsgType == 'text') {

                    if ($keyword) {

                        $Returnone      =   $M->searchKeyword($fromUsername,$keyword);
                        // $centerStr      =   $Returnone['centerStr'];
                        // $this->MsgType  =   $Returnone['MsgType'];
                        
                    }
                    if ($Returnone['centerStr'] == "") {

                        $Returntwo      =   $M->searchJob($keyword);
                        $centerStr      =   $Returntwo['centerStr'];
                        $this->MsgType  =   $Returntwo['MsgType'];
                    }else{
                        echo '';exit();

                    }
                }

                $topStr = sprintf($topTpl, $fromUsername, $toUsername, $times, $this->MsgType);
                echo $topStr . $centerStr . $bottomStr;
            }
        }
    }

    /**
     * @desc 通过公众号内所有事件获取OPENID，查询UNIONID，更新用户
     * @param $openId
     * @return bool
     */
    private function upUserInfo($openId)
    {
        $bind   =   false;
        // 先判断是否绑定了微信开放平台，没绑定的，不需要处理
        if ((isset($this->config['mini_wxopen']) && $this->config['mini_wxopen'] == 1) || (isset($this->config['sy_app_open']) && $this->config['sy_app_open'] == 1)) {

            $userInfoM  =   $this->MODEL('userinfo');
            $uInfo      =   $userInfoM->getInfo(array('wxid' => $openId), array('field' => 'uid'));
            // 未绑定公众号openid的，需要查询
            if (empty($uInfo)) {
                $weixinM    =   $this->MODEL('weixin');
                $user       =   $weixinM->getWxUser($openId);
                if (isset($user['unionid'])) {

                    $uInfo  =   $userInfoM->getInfo(array('unionid' => $user['unionid'], 'wxid' => ''), array('field' => 'uid'));

                    if (!empty($uInfo)) {

                        $userInfoM->upInfo(array('uid' => $uInfo['uid']), array('subscribe' => $user['subscribe'], 'wxid' => $openId, 'wxbindtime' => time()));
                        $bind   =   true;
                    }
                }
            }
        }
        return $bind;
    }

    /**
     * @desc 通过扫强制关注公众号场景码，来关注公众号，并将账号绑定微信公众号
     * @param string $wxloginid
     * @param string $wxid
     */
    private function bindUserByGzh($wxloginid = '', $wxid = ''){
        
        $strarr	= explode('_',$wxloginid);
        $userid = $strarr[2];
        
        $userInfoM  =   $this->MODEL('userinfo');
        $uInfo      =   $userInfoM->getInfo(array('uid' => $userid), array('field'=>'`usertype`,`wxid`'));
        if (empty($uInfo['wxid']) && !empty($wxid)){
            // 先清掉其他账号，绑定此微信号的
            $userInfoM->upInfo(array('wxid'=>$wxid), array('wxopenid' => '', 'wxid' => '','unionid' => '', 'subscribe' => 2));
            // 再绑定现账号
            $userInfoM->upInfo(array('uid' => $userid), array('wxid'=>$wxid,'wxbindtime'=>time(), 'subscribe' => 1));

            // 会员日志
            $LogM       =   $this->MODEL('log');
            $logContent =   '账号认证：换绑微信';
            $logDetail  =   '通过强制关注公众号场景码，换绑微信';
            $LogM->addMemberLog($userid, $uInfo['usertype'], $logContent, 12, 2, $logDetail);
			$this->MODEL('integral')->invtalCheck($userid,$uInfo['usertype'],'integral_bind_wx','微信扫码绑定',28);
        }
        // 修改扫码记录
        $weixinM = $this->MODEL('weixin');
        $weixinM->upWxlogin(array('wxloginid'=>$wxloginid), array('status'=>2, 'wxid'=>$wxid));
    }

    /**
     * @desc 关注/取消关注 微信公众号
     * @param $openId
     * @param $subscribe
     */
    private function setSubscribe($openId, $subscribe)
    {

        $userInfoM  =   $this->MODEL('userinfo');
        $uInfo      =   $userInfoM->getInfo(array('wxid' => $openId), array('field' => '`uid`,`usertype`'));
        if (isset($uInfo) && !empty($uInfo)) {

            $userInfoM->upInfo(array('uid' => $uInfo['uid'], 'wxid' => $openId), array('subscribe' => $subscribe));
            $LogM       =   $this->MODEL('log');
            $subContent =   $subscribe == 1 ? '关注微信公众号' : '取消关注微信公众号';
            $logContent =   '账号认证：'.$subContent;
            $LogM->addMemberLog($uInfo['uid'], $uInfo['usertype'], $logContent, 12, 2);
        }
    }

    function gpc2sql($str)
    {

        $arr    =   array("sleep" => "Ｓleep", " and " => " an d ", " or " => " Ｏr ", "xor" => "xＯr", "%20" => " ", "select" => "Ｓelect", "update" => "Ｕpdate", "count" => "Ｃount", "chr" => "Ｃhr", "truncate" => "Ｔruncate", "union" => "Ｕnion", "delete" => "Ｄelete", "insert" => "Ｉnsert", "\"" => "“", "'" => "“", "--" => "- -", "\(" => "（", "\)" => "）", "00000000" => "OOOOOOOO", "0x" => "Ox", "0b" => "Ob");
        foreach ($arr as $key => $v) {
            $str = preg_replace('/' . $key . '/isU', $v, $str);
        }
        return $str;
    }
    // 正则校验wxid
    function pregWxid($str){
        if (preg_match("/^[_A-Za-z0-9\-]{5,40}$/", $str)){
            return (string)$str;
        }else{
            return '';
        }
    }

    /**
     * 处理公众号绑定微信开放平台之前关注公众号绑定微信用户，获取unionid
     */
    function wxBindAll_action()
    {

        $userInfoM  =   $this->MODEL('userinfo');
        $count      =   $userInfoM->getMemberNum(array('wxid' => array('<>', '')));
        $size       =   30;
        $num        =   ceil($count / $size);
        if (!$_GET['num']) {
            $i      =   0;
        } else {
            $i      =   $_GET['num'];
        }
        $rows       =   $userInfoM->getList(array('wxid' => array('<>', ''), 'limit' => array($i * $size, $size), 'orderby' => 'uid,asc'), array('field' => '`uid`, `subscribe`, `wxid`, `unionid`'));
        $weiXinM    =   $this->MODEL('weixin');

        foreach ($rows as $v) {
            if (empty($v['unionid'])) {

                $info   =   $weiXinM->getWxUser($v['wxid']);
                if (!empty($info['unionid'])) {

                    $userInfoM->upInfo(array('uid' => $v['uid']), array('unionid' => $info['unionid'], 'subscribe' => $info['subscribe']));
                }
            }
        }

        if (($i + 1) >= $num) {

            echo "完成";
        } else {

            $getnum = $i + 1;

            echo "<script>location.href='".$this->config['sy_weburl']."/weixin/index.php?c=wxBindAll&num=".$getnum."';</script>";
        }
    }


}

?>