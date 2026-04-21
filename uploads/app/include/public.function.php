<?php

/*
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 *
 */


/**
 * 功能性通用函数库
 */

/**
 * @desc 检测手机号格式
 * @param $mobile
 * @return bool
 */
function CheckMobile($mobile)
{
    if (!preg_match("/1[3456789]{1}\d{9}$/", trim($mobile))) {
        return false;
    } else {
        return true;
    }
}

function CheckRegUser($str)
{

    if (!preg_match("/^[\x{4e00}-\x{9fa5}A-Za-z0-9\-@#.\$_!]+$/u", $str)) {
        return false;
    } else {
        return true;
    }
}

function CheckTell($str)
{

    if (preg_match("/^[0-9-]+?$/", $str) == 0) {
        return false;
    } else {
        return true;
    }
}

/**
 * 检测邮箱
 * @param $email
 * @return bool
 */
function CheckRegEmail($email)
{
    if (!preg_match('/^([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9\-]+@([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9]+\.[a-zA-Z]{2,4}$/', $email)) {
        return false;
    } else {
        return true;
    }
}
/**
 简历审核/修改审核  判断时间
*/
function resumeTimeState($state){

    global $config;

    if($state=='0'){

        $start = $config['resume_statetime_start'];

        $end   = $config['resume_statetime_end'];

        if($start && $end){

            $now   = time();
            $today = date('Y-m-d');
            
            $s = strtotime($today.' '.$start.':00');
            $e = strtotime($today.' '.$end.':00');
            
            if(
                ($start<$end && $s<=$now && $e>=$now) 
                || ($start>$end && ($s<=$now || $e>=$now))
                || ($start==$end)
                ){//在要求审核的时间内

                $state=0;
            }else{
                $state=1;
            }
        }
    }
    return $state;
}
/**
 * 用户名注册复杂度判断
 * @param $name
 * @return string
 */
function regUserNameComplex($name)
{

    $msg = '';

    if ($name) {

        global $config;

        $reg_namemaxlen =   $config['sy_reg_namemaxlen'];//用户名长度最大值
        $reg_nameminlen =   $config['sy_reg_nameminlen'];//用户名长度最小值
        $reg_name_sp    =   $config['reg_name_sp'];//是否必须包含其它字符@!#.$-_
        $reg_name_zm    =   $config['reg_name_zm'];//是否必须包含字母
        $reg_name_num   =   $config['reg_name_num'];//是否必须包含数字

        $namelen        =   mb_strlen($name);
        //长度
        if ($namelen < $reg_nameminlen || $namelen > $reg_namemaxlen) {

            $msg    = '用户名应在' . $reg_nameminlen . '-' . $reg_namemaxlen . '位字符之间！';
        } else {

            $smsg   =   $zmsg   =   $nmsg   =   $douhao =   '';

            //数字
            if ($reg_name_num == 1) {
                if (!preg_match("/[0-9]+/u", $name)) {

                    $nmsg   =   '数字';
                    $douhao =   '，';
                }
            }
            //字母
            if ($reg_name_zm == 1) {
                if (!preg_match('/[a-zA-Z]+/u', $name)) {

                    $zmsg   =   $douhao . '字母';
                    $douhao =   '，';
                }
            }
            //其它字符
            if ($reg_name_sp == 1) {
                if (!preg_match('/[-@#.$_!]+/u', $name)) {

                    $smsg   =   $douhao . '其它字符@!#.$-_';
                }
            }

            if ($nmsg || $zmsg || $smsg) {

                $msg        =   '用户名必须包含' . $nmsg . $zmsg . $smsg;
            }
        }
    }
    return $msg;
}

/**
 * 验证密码复杂度
 * @param $name
 * @return string
 */
function regPassWordComplex($name)
{

    $msg    =   '';

    if ($name) {

        global $config;

        $reg_pw_sp  =   $config['reg_pw_sp'];   //  是否必须包含其它字符@!#.$-_
        $reg_pw_zm  =   $config['reg_pw_zm'];   //  是否必须包含字母
        $reg_pw_num =   $config['reg_pw_num'];  //  是否必须包含数字

        $smsg   =   $zmsg   =   $nmsg   =   $douhao =   '';
        //数字
        if ($reg_pw_num == 1) {
            if (!preg_match("/[0-9]+/u", $name)) {

                $nmsg   =   '数字';
                $douhao =   '，';
            }
        }
        //字母
        if ($reg_pw_zm == 1) {
            if (!preg_match('/[a-zA-Z]+/u', $name)) {

                $zmsg   =   $douhao . '字母';
                $douhao =   '，';
            }
        }
        //其它字符
        if ($reg_pw_sp == 1) {
            if (!preg_match('/[-@#.$_!]+/u', $name)) {

                $smsg   =   $douhao . '其它字符@!#.$-_';
            }
        }
        if ($nmsg || $zmsg || $smsg) {

            $msg        =   '密码必须包含' . $nmsg . $zmsg . $smsg;
        }
    }
    return $msg;
}

/**
 * 把数组生成字符串
 * @param $obj
 * @param bool $withKey
 * @param bool $two
 * @return array|string
 */
function ArrayToString($obj, $withKey = true, $two = false)
{
    if (empty($obj)) return array();

    $objType    =   gettype($obj);

    if ($objType == 'array') {

        $objString  =   "array(";
        foreach ($obj as $objkey => $objv) {

            $objkey = setSecFilter($objkey);
            if ($withKey) $objString .= "\"$objkey\"=>";

            $vtype  =   gettype($objv);
            
            if(!in_array($vtype,array('array','object'))){
                
                $objv = setSecFilter($objv);
            }
            if ($vtype == 'integer') {

                $objString  .=  "$objv,";
            } else if ($vtype == 'double') {

                $objString  .=  "$objv,";
            } else if ($vtype == 'string') {

                $objv       =   str_replace('"', "\\\"", $objv);
                $objString  .=  "\"" . $objv . "\",";
            } else if ($vtype == 'array') {

                $objString  .=  "" . ArrayToString($objv, $withKey, $two) . ",";
            } else if ($vtype == 'object') {

                $objString  .=  "" . ArrayToString($objv, $withKey, $two) . ",";
            } else {

                $objString  .=  "\"" . $objv . "\",";
            }
        }
        $objString  =   substr($objString, 0, -1) . "";
        return $objString . ")\n";
    }
}

/**
 * 获取真实IP地址否则返回Unknown
 */
function fun_ip_get()
{

    if (!empty($_SERVER['HTTP_CLIENT_IP'])) {

        return is_ip($_SERVER['HTTP_CLIENT_IP']);
    } elseif (!empty($_SERVER['HTTP_X_FORWARDED_FOR'])) {

        return is_ip($_SERVER['HTTP_X_FORWARDED_FOR']);
    } else {

        return is_ip($_SERVER['REMOTE_ADDR']);
    }
}
/**
 *users:王旭
 *Data:2023/5/6
 *Time:16:02
 * @param $ip
 * @return string
 */
function getIpAddress($ip){
    $config =   '';

    include(PLUS_PATH . 'config.php');

    $ip_address = '';
    if ($config['sy_ip'] == 1) {
        $url = 'https://u.phpyun.com/ip2addr';
        $url .= '?appSecret=' . $config['sy_ip_appsecret'] . '&appKey=' . $config['sy_ip_appkey'] . '&ip=' . $ip;
        $file_contents = CurlGet($url);
        if ($file_contents) {
            $res = json_decode($file_contents, 1);
            if (isset($res['code']) && $res['code'] == 200) {// 查询成功
                $ip_address = $res['data']['addr'];
            }
        }
    }
    return $ip_address;
}
/**
 *users:王旭
 *Data:2023/5/12
 *Time:17:57
 * @param $phone
 * @return string
 */
function getMoblieAddress($phone){

    $config =   '';

    include(PLUS_PATH . 'config.php');
    $moblie_address = '';
    if ($config['sy_mobile'] == 1) {
        $url = 'https://u.phpyun.com/mlocation';
        $url .= '?appSecret=' . $config['sy_mobile_appsecret'] . '&appKey=' . $config['sy_mobile_appkey'] . '&phone=' . $phone;
        $location_contents = CurlGet($url);
        if ($location_contents) {
            $res = json_decode($location_contents, 1);
            if (isset($res['code']) && $res['code'] == 200) {// 认证成功
                $moblie_address = $res['data']['location'];
            }
        }
    }
    return $moblie_address;
}
/**
 * @param $str
 * @return string
 */
function is_ip($str)
{

    if (stripos($str, ',') !== false) {

        $strArr =   explode(',', $str);
        $ip     =   $strArr[0];
    } else {

        $ip     =   $str;
    }

    if (filter_var($ip, FILTER_VALIDATE_IP, FILTER_FLAG_IPV4)) {

        return  $ip;
    } elseif (filter_var($ip, FILTER_VALIDATE_IP, FILTER_FLAG_IPV6)) {

        return  $ip;
    } else {

        return  '未知IP';
    }
}


/**
 * 获取城市当前处于第几级
 * @param $id
 * @param $parent
 * @param int $lev
 * @return int
 */
function getLev($id, $parent, $lev = 1)
{

    $lhead  =   0;
    if ($parent[$id] > $lhead) {   //存在父ID 则继续向下探寻 直到父ID 为一级类别

        return getLev($parent[$id], $parent, ($lev + 1));
    } else {

        return $lev;
    }
}

// 转utf8
function transCodeToUTF8($str)
{
    $encode = mb_detect_encoding($str, ['ASCII', 'UTF-8', 'GB2312', 'GBK', 'BIG5', 'EUC-CN']);
    return 'UTF-8' === $encode ? $str : mb_convert_encoding($str, 'UTF-8', $encode);
}

/**
 * 返回当前城市
 */
function getLocalCity()
{
    $ip =   fun_ip_get();
    $cityInfo   =   array();
    if ($ip != "127.0.0.1") {

        $url = "https://whois.pconline.com.cn/ipJson.jsp?ip=" . $ip . "&json=true";
        $ret = transCodeToUTF8(curlGet($url));
    }
    if (false !== $ret) {

        $output = json_decode($ret, true);

        if (isset($output['city']) && $output['city']) {

            $local = str_replace(array('市', '县', '区'), '', $output['city']);
        } else if (isset($output['pro']) && $output['pro']) {

            $local = str_replace('省', '', $output['pro']);
        }

        include(PLUS_PATH."domain_cache.php");
        foreach ($site_domain as $key => $value) {
            if (strpos($value['cityname'], $local) !== false && $value['fz_type'] == 1) {

                $cityInfo = $value;
                break;
            }
        }
    }
    return $cityInfo;
}

function go_to_city($config)
{

    $city   =   getLocalCity();

    SetCookie("gotocity", '1', time() + 3600, "/");//一个小时内不再判断

    if (!empty($city)) {
        if ($city['mode'] == '1') {

            header('Location: http://' . $city['host']);
        } else {

            header('Location: ' . $config['sy_weburl'] . '/' . $city['indexdir'] . '/');
        }
        exit();//停止执行
    }
}

/**
 * 判断来路
 * @param string $default
 * @return mixed|string
 */
function dreferer($default = '')
{

    $referer        =   isset($_SERVER['HTTP_REFERER']) ? $_SERVER['HTTP_REFERER'] : '';

    if (strpos('a' . $referer, Url('user', 'login'))) {

        $referer    =   $default;
    } else {

        $referer    =   substr($referer, -1) == '?' ? substr($referer, 0, -1) : $referer;
    }
    return $referer;
}

/**
 * 判断是否是手机或PC客户端来路
 */
function UserAgent()
{
    $user_agent =   (!isset($_SERVER['HTTP_USER_AGENT'])) ? FALSE : $_SERVER['HTTP_USER_AGENT'];

    if ((preg_match("/(iphone|ipod|android)/i", strtolower($user_agent))) and strstr(strtolower($user_agent), 'webkit')) {

        return true;
    } else if (trim($user_agent) == '' or preg_match("/(nokia|sony|ericsson|mot|htc|samsung|sgh|lg|philips|lenovo|ucweb|opera mobi|windows mobile|blackberry)/i", strtolower($user_agent))) {

        return true;
    } else {

        return true;
    }
}

/**
 * 获取顶级域名
 * @param $host
 * @return mixed
 */
function get_domain($host)
{

    $host       =   strtolower($host);

    if (strpos($host, '/') !== false) {
        $parse  =   @parse_url($host);
        $host   =   $parse['host'];
    }
    $topDomain  =   array('com', 'edu', 'gov', 'int', 'mil', 'net', 'org', 'biz', 'info', 'pro', 'name', 'museum', 'coop', 'aero', 'xxx', 'idv', 'mobi', 'cc', 'me');
    $str        =   '';
    foreach ($topDomain as $v) {
        $str    .=  ($str ? '|' : '') . $v;
    }
    $matchStr   =   "[^\.]+\.(?:(" . $str . ")|\w{2}|((" . $str . ")\.\w{2}))$";
    if (preg_match("/" . $matchStr . "/is", $host, $matchS)) {

        $domain =   $matchS['0'];
    } else {

        $domain =   $host;
    }
    return $domain;
}

function setSecFilter($arr=''){
    
    $SecOne=array("'","(",")","`","$");
    
    $SecTwo=array("“","（","）","“","￥");
    
    $arr = str_replace($SecOne,$SecTwo,$arr);
    
    return $arr;
}
/**
 * 配置文件生成方法可定义数组名
 * @param $dir
 * @param $array
 * @param $config
 */
function made_web($dir, $array, $config)
{

    $content    =   "<?php \n";
    $content    .=  "\$$config=" . $array . ";";
    $content    .=  " \n";
    $content    .=  "?>";
    $fpIndex    =   @fopen($dir, "w+");
    @fwrite($fpIndex, $content);
    @fclose($fpIndex);
}

/**
 * 配置文件生成方法，数组遍历打印
 * @param $dir
 * @param $array
 * @return
 */
function made_web_array($dir, $array)
{

    $content    =   "<?php \n";

    if (is_array($array)) {
        foreach ($array as $key => $v) {
            if (is_array($v)) {

                $content    .=  "\$$key=array(";
                $content    .=  made_string($v);
                $content    .=  ");";
            } else {

                $v  =   str_replace("'", "\\'", $v);
                $v  =   str_replace("\"", "'", $v);
                $v  =   str_replace("\$", "", $v);
                $content    .=  "\$$key=" . $v . ";";
            }
            $content        .=  " \n";
        }
    }
    $content    .=  "?>";

    $fpIndex    =   @fopen($dir, "w+");
    $fw         =   @fwrite($fpIndex, $content);
    @fclose($fpIndex);
    return $fw;
}

/**
 * 字符串生成方法，可拼接
 * @param $array
 * @param string $string
 * @return string
 */
function made_string($array, $string = '')
{

    if (is_array($array) && !empty($array)) {
        $i  =   0;
        foreach ($array as $key => $value) {
            if ($i > 0) {

                $string .=  ',';
            }
            if (is_array($value)) {

                $string .=  "'" . $key . "'=>array(" . made_string($value) . ")";
            } else {

                $string .=  "'" . $key . "'=>'" . str_replace('\$', '', $value) . "'";
            }
            $i++;
        }
    }
    return $string;
}

/**
 * 删除文件方法
 * @param $delFiles
 * @return bool
 */
function delfiledir($delFiles)
{
    $delFiles   =   stripslashes($delFiles);
    $delFiles   =   str_replace("../", "", $delFiles);
    $delFiles   =   str_replace("./", "", $delFiles);
    $delFiles   =   "../" . $delFiles;
    $p_delFiles =   path_tidy($delFiles);
    if ($p_delFiles != $delFiles) {
        die;
    }
    if (is_file($delFiles)) {
        @unlink($delFiles);
    } else {
        $dh = @opendir($delFiles);
        while ($file = @readdir($dh)) {
            if ($file != "." && $file != "..") {
                $fullpath = $delFiles . "/" . $file;
                if (@is_dir($fullpath)) {
                    delfiledir($fullpath);
                } else {
                    @unlink($fullpath);
                }
            }
        }
        @closedir($dh);
        if (@rmdir($delFiles)) {
            return true;
        } else {
            return false;
        }
    }
}

/**
 * 路径'//'处理成'/'
 * @param $path
 * @return string
 */
function path_tidy($path)
{
    $tidy   =   array();
    $path   =   strtr($path, '\\', '/');
    foreach (explode('/', $path) as $i => $item) {
        if ($item == '' || $item == '.') {
            continue;
        }
        if ($item == '..' && end($tidy) != '..' && $i > 0) {
            array_pop($tidy);
            continue;
        }
        $tidy[] = $item;
    }
    return ($path[0] == '/' ? '/' : '') . implode('/', $tidy);
}

/**
 * 图片删除
 * 索引 2 给出的是图像的类型，返回的是数字
 * 其中 1 = GIF，2 = JPG，3 = PNG，4 = SWF，5 = PSD，6 = BMP，7 = TIFF(intel byte order)，8 = TIFF(motorola byte order)，9 = JPC，10 = JP2，11 = JPX，12 = JB2，13 = SWC，14 = IFF，15 = WBMP，16 = XBM
 * @param $pic
 */
function unlink_pic($pic)
{
    $picType    =   getimagesize($pic);
    if ($picType[2] == '1' || $picType[2] == '2' || $picType[2] == '3') {

        @unlink($pic);
    }
}

/**
 * 批量操作中的ID检查，只能是数字字母与系统内置的分隔符
 *
 * @param $string
 * @param $array
 * @return int
 */
function pylode($string, $array)
{
    if (is_array($array)) {

        $str    =   @implode($string, $array);
    } else {

        $str    =   $array;
    }
    if (!preg_match("/^[0-9a-zA-Z" . $string . "]+$/", $str)) {
        $str = 0;
    }
    return $str;
}

/**
 * 获取微信 TOKEN
 * @return mixed
 */
function getToken()
{

    $config     =   '';
    include(PLUS_PATH . 'configcache.php');

    if (isset($configcache)){

        $Token      =   $configcache['token'];
        $TokenTime  =   $configcache['token_time'];
        $NowTime    =   time();

        if (($NowTime - $TokenTime) > 300 || !$Token) {

            include(PLUS_PATH.'config.php');
            $Appid      =   $config['wx_appid'];
            $Appsecert  =   $config['wx_appsecret'];
            $Url        =   'https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid=' . $Appid . '&secret=' . $Appsecert;
            $CurlReturn =   CurlPost($Url);

            $Token      =   json_decode($CurlReturn);

            $configcache['token']       =   $Token->access_token;
            $configcache['token_time']  =   time();
            if ($configcache['token']) {

                made_web(PLUS_PATH . "configcache.php", ArrayToString($configcache), "configcache");
            }
            return $configcache['token'];
        } else {

            return $Token;
        }
    }
}

/**
 * 获取企业微信 TOKEN
 * @remark 每个应用有独立的secret，获取到的access_token只能本应用使用，所以每个应用的access_token应该分开来获取
 *
 * @param null $type || 区分不同应用；msg：后台管理消息推送；customer：客户联系
 * @return mixed
 */
function getWxQyToken($type = null)
{

    $config =   '';

    include(PLUS_PATH . 'configcache.php');

    if (isset($configcache)) {

        if ($type == 'link'){                   //  客户联系

            $Token      =   $configcache['work_token'];
            $TokenTime  =   $configcache['work_token_time'];
        }else if ($type == 'msg'){              //  应用：消息推送(后台管理员消息推送)

            $Token      =   $configcache['wxqy_token'];
            $TokenTime  =   $configcache['wxqy_token_time'];
        }else if ($type == 'photo'){            //  应用：客户图像

            $Token      =   $configcache['work_photo_token'];
            $TokenTime  =   $configcache['work_photo_token_time'];
        }

        $NowTime    =   time();

        if (($NowTime - $TokenTime) > 7000 || !$Token) {

            include(PLUS_PATH . 'config.php');

            $corpid     =   $config['wx_qy_corpid'];

            if ($type == 'link'){           //  客户联系

                $secert =   $config['sy_workwx_secret'];
            }else if ($type == 'msg') {     //  应用：消息推送（后台管理员消息推送）

                $secert =   $config['wx_qy_secret'];
            }else if ($type == 'photo'){    //  应用：客户图像

                $secert =   $config['wx_photo_secret'];
            }

            $Url        =   'https://qyapi.weixin.qq.com/cgi-bin/gettoken?corpid=' . $corpid . '&corpsecret=' . $secert;
            $CurlReturn =   CurlPost($Url);

            $Token      =   json_decode($CurlReturn);

            if ($type == 'link'){

                $configcache['work_token']      =   $Token->access_token;
                $configcache['work_token_time'] =   time();
            }elseif ($type == 'msg'){

                $configcache['wxqy_token']      =   $Token->access_token;
                $configcache['wxqy_token_time'] =   time();
            }elseif ($type == 'photo'){

                $configcache['wx_photo_token']     =   $Token->access_token;
                $configcache['wx_photo_token_time']=   time();
            }

            if ($configcache['token']) {
                made_web(PLUS_PATH . "configcache.php", ArrayToString($configcache), "configcache");
            }

            if ($type == 'link'){

                return $configcache['work_token'];
            }else if ($type == 'msg'){

                return $configcache['wxqy_token'];
            }elseif ($type == 'photo'){

                return $configcache['wx_photo_token'];
            }
        } else {

            return $Token;
        }
    }
}

/**
 * @desc    企业微信 获取 jsapi_ticket
 * @param null $type| link:企业的 jsapi_ticket（外部沟通管理：添加自定义详情页使用）；agent:应用的 jsapi_ticket
 * @return array
 */
function getJsApiTicket($type = null)
{

    include(PLUS_PATH . 'configcache.php');

    if (isset($configcache)) {

        if ($type == 'link') {

            $Ticket = $configcache['jsapi_ticket'];
            $TicketTime = $configcache['jsapi_ticket_time'];
        } elseif ($type == 'agent') {

            $Ticket = $configcache['agent_jsapi_ticket'];
            $TicketTime = $configcache['agent_jsapi_ticket_time'];
        }

        $NowTime = time();
        if (($NowTime - $TicketTime) > 7000 || !$Ticket) {

            if ($type == 'link') {

                $Url = 'https://qyapi.weixin.qq.com/cgi-bin/get_jsapi_ticket?access_token=' . getWxQyToken('link');
                $CurlReturn = CurlGet($Url);
                $Ticket = json_decode($CurlReturn);

                $configcache['jsapi_ticket'] = $Ticket->ticket;
                $configcache['jsapi_ticket_time'] = time();
                if ($configcache['jsapi_ticket']) {
                    made_web(PLUS_PATH . "configcache.php", ArrayToString($configcache), "configcache");
                }
                return array('ticket' => $configcache['jsapi_ticket'], 'ticket_time' => $configcache['jsapi_ticket_time']);
            }else if ($type == 'agent'){

                $Url = 'https://qyapi.weixin.qq.com/cgi-bin/ticket/get?access_token=' . getWxQyToken('photo').'&type=agent_config';
                $CurlReturn = CurlGet($Url);
                $Ticket = json_decode($CurlReturn);

                $configcache['agent_jsapi_ticket'] = $Ticket->ticket;
                $configcache['agent_jsapi_ticket_time'] = time();
                if ($configcache['photo_jsapi_ticket']) {
                    made_web(PLUS_PATH . "configcache.php", ArrayToString($configcache), "configcache");
                }
                return array('ticket' => $configcache['agent_jsapi_ticket'], 'ticket_time' => $configcache['agent_jsapi_ticket_time']);
            }
        } else {

            return array('ticket' => $Ticket, 'ticket_time' => $TicketTime);
        }
    }
}

/**
 * 获取微信 JS TOKEN
 */
function getWxTicket()
{

    include(PLUS_PATH.'configcache.php');

    if (isset($configcache)) {

        $Ticket     =   $configcache['ticket'];
        $TicketTime =   $configcache['ticket_time'];
        $NowTime    =   time();
        if (($NowTime - $TicketTime) > 7000 || !$Ticket) {

            $Url        =   'https://api.weixin.qq.com/cgi-bin/ticket/getticket?access_token=' . getToken() . '&type=jsapi';
            $CurlReturn =   CurlPost($Url);
            $Ticket     =   json_decode($CurlReturn);

            $configcache['ticket']      =   $Ticket->ticket;
            $configcache['ticket_time'] =   time();
            if ($configcache['ticket']) {
                made_web(PLUS_PATH . "configcache.php", ArrayToString($configcache), "configcache");
            }
            return $configcache['ticket'];
        } else {
            return $Ticket;
        }
    }
}

/**
 * 初始化微信SDK参数
 * @param string $url
 * @return array
 */
function getWxJsSdk($url = '')
{
    include(PLUS_PATH.'config.php');

    $Ticket =   getWxTicket();

    if (empty($url)) {
        if (!empty($config['sy_wapdomain'])) {
            if ($config['sy_wapssl'] == '1') {

                $protocol   =   'https://';
            } else {

                $protocol   =   'http://';
            }
        } else {
            $protocol       =   getprotocol($config['sy_weburl']);
        }

        $url    =   $protocol . $_SERVER[HTTP_HOST] . $_SERVER[REQUEST_URI];
    }
    $timestamp  =   time();
    $chars      =   "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    $nonceStr   =   "";
    for ($i = 0; $i < 16; $i++) {

        $nonceStr   .=  substr($chars, mt_rand(0, strlen($chars) - 1), 1);
    }
    // 这里参数的顺序要按照 key 值 ASCII 码升序排序
    $string     =   "jsapi_ticket=" . $Ticket . "&noncestr=" . $nonceStr . "&timestamp=" . $timestamp . "&url=" . $url;

    $signature  =   sha1($string);

    $signPackage    =   array(

        "appId"         =>  $config['wx_appid'],
        "nonceStr"      =>  $nonceStr,
        "timestamp"     =>  $timestamp,
        "url"           =>  $url,
        "signature"     =>  $signature,
        "rawString"     =>  $string
    );
    return $signPackage;
}

/**
 * CURL POST提交
 *
 * @param $url
 * @param string $data
 * @param int $multiple
 * @param string $headers
 * @return
 */
function CurlPost($url, $data = '', $multiple = 1, $headers = '')
{

    $ch =   curl_init();
    curl_setopt($ch, CURLOPT_URL, $url);
    curl_setopt($ch, CURLOPT_SSL_VERIFYHOST, 2);
    if ($headers) {

        curl_setopt($ch, CURLOPT_HTTPHEADER, $headers);
    }
    if ($data != '') {

        curl_setopt($ch, CURLOPT_POST, 1);
        if ($multiple == 1) {

            curl_setopt($ch, CURLOPT_POSTFIELDS, $data);
        } elseif ($multiple == 2) {

            curl_setopt($ch, CURLOPT_POSTFIELDS, http_build_query($data));
        }
    }
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, 1);
    curl_setopt($ch, CURLOPT_SSL_VERIFYPEER, FALSE);
    $Return = curl_exec($ch);
    if (curl_errno($ch)) {
        //echo 'Errno'.curl_error($ch);
    }
    curl_close($ch);
    return $Return;
}

/**
 * CURL GET提交
 * @param $url
 * @return
 */
function CurlGet($url,$data=array(), $headers = array())
{
    $ch         =   curl_init();
    $timeout    =   20;
    curl_setopt($ch, CURLOPT_URL, $url);
    curl_setopt($ch, CURLOPT_HEADER, false);
    if (!empty($headers)) {
        
        curl_setopt($ch, CURLOPT_HTTPHEADER, $headers);
    }else{
        curl_setopt($ch, CURLOPT_HEADER, false);
    }
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, 1);
    if($data['refer']){
        curl_setopt ($ch, CURLOPT_REFERER,$data['refer']);
    }
    curl_setopt($ch, CURLOPT_SSL_VERIFYPEER, FALSE); // 跳过证书检查
    curl_setopt($ch, CURLOPT_SSL_VERIFYHOST, 2);  // 从证书中检查SSL加密算法是否存在
    curl_setopt($ch, CURLOPT_CONNECTTIMEOUT, $timeout);
    $Return     =   curl_exec($ch);
    if (curl_errno($ch)) {
        //echo 'Errno' . curl_error($ch);
    }
    curl_close($ch);
    return $Return;
}
function latlngCheck($data = array()){

    $return = array('lat'=>'','lng'=>'');

    if(!empty($data['lat']) && !empty($data['lng'])){

        $lngpreg = "/^[\+\-]?((?:[0-9]|[1-9][0-9]|1[0-7][0-9])\.([0-9]*)|[0-9]|[1-9][0-9]|1[0-7][0-9]|180)$/";
        $latpreg = "/^[\+\-]?((?:[0-9]|[1-8][0-9])\.([0-9]*|[0-9]|[1-8][0-9]|90))$/";

        if(preg_match($latpreg, $data['lat']) && preg_match($lngpreg, $data['lng'])){

            $return['lat'] = $data['lat'];
            $return['lng'] = $data['lng'];
        
        };
    }
    
    return $return;
}
function curlMappic($data=array()){
    
    include(PLUS_PATH.'config.php');

    $return  =array();

    if (!empty($config['map_key'])){

        $latlng = latlngCheck(array('lat'=>$data['lat'],'lng'=>$data['lng']));

        $data['lng'] = $latlng['lng'];
        $data['lat'] = $latlng['lat'];

        if(empty($data['lng']) || empty($data['lat'])){
            $return['error'] = -1;
            $return['msg'] = '坐标异常请重试';
            return $return;
        }
        $center = $data['lng'] . ',' . $data['lat'];  // lng,lat
        // 高德
        $url = 'https://api.map.baidu.com/staticimage/v2';
        
        $params = array(
            'ak'        =>  $config['map_key'],
            'copyright' =>  $data['copyright']?$data['copyright']:1,
            'center'    =>  $center,
            'width'     =>  $data['width']?$data['width']:320,
            'height'    =>  $data['height']?$data['height']:140,
            'zoom'      =>  $data['zoom']?$data['zoom']:14,
            'markers'   =>  $center
        );

        $preSignStr = getSignContent($params);
        
        $url = $url . "?" . $preSignStr;
        
        $curldata = array();
        if($data['refer']){
            $curldata['refer']=$data['refer'];
        }
        

        $back = CurlGet($url,$curldata);
        $json = json_decode($back, true);
        
        if (!$json){
            
            // 重新定义文件名称 图片一律用 jpeg
            $filename  =  time().'_S_'.rand(1000,9999).'.png';
            //自定义目录名称
            $dirName = APP_PATH . 'data/upload/mappic/' . date('Ymd');
            //定义新名称以及目录
            if (!file_exists($dirName)){
                mkdir($dirName, 0777, true);
            }
            $res = fopen($dirName . '/' . $filename, 'a');
            fwrite($res, $back);
            fclose($res);
            
            if($config['sy_oss'] == 1){
                
                include_once(LIB_PATH.'oss/ossupload.class.php');
                $ossUpload  =  new ossUpload();
                $pic        =  $ossUpload -> uploadLocalImg($dirName . '/' . $filename);
                
                $picUrl  =  $pic['picurl'];
            }else{
                
                $picUrl =   str_replace(APP_PATH.'data', './data', $dirName . '/' . $filename);
            }
            
            $return['error'] = 0;
            $return['msg'] = '';
            $return['url'] = $picUrl;
        }else{
            $return['error'] = -1;
            $return['msg'] = $json['message'];
        }
    }else {
        $return['error'] = -1;
        $return['msg'] = '地图key未配置';
    }
    return $return;
}
//高德逆地址解析
function bd_reverse_geocoding($data=array()){

    $return = array('error'=>0);

    if ($data['bmap_webak']) {

        $latlng = latlngCheck(array('lat'=>$data['lat'],'lng'=>$data['lng']));

        $data['lng'] = $latlng['lng'];
        $data['lat'] = $latlng['lat'];

        if(empty($data['lng']) || empty($data['lat'])){
            $return['error'] = -1;
            $return['msg'] = '坐标异常请重试';
            return $return;
        }

        $center = $data['lng'] . ',' . $data['lat'];  // lng,lat
        // 高德
        $url = 'https://restapi.amap.com/v3/geocode/regeo';
        
        $params = array(
            'key'       =>  $data['bmap_webak'],
            'output'    =>  'json',
            'location'  =>  $center
        );

        $preSignStr = getSignContent($params);
        
        $url = $url . "?" . $preSignStr;
        
        $curldata = array();
        

        $back = CurlGet($url,$curldata);
        $res = json_decode($back, true);

        if($res['status']==1){
            $return['error'] = 1;
            $return['data'] = $res['regeocode'];
        }else{
            $return['msg'] = '高德接口错误码：'.$res['status'];
        }
    }

    return $return;
}
function getPoi($data=array()){

    include(PLUS_PATH.'config.php');
    
    $res = '';

    if($config['bmap_webak'] && $data['address']){

        $url = 'https://restapi.amap.com/v3/assistant/inputtips?key='. $config['bmap_webak'] .'&keywords='.$data['address'];
        
        $res = CurlGet($url);

        $resArr = json_decode($res, 1);
        if ($resArr['tips']) {
            // 去除经纬度为空的记录
            foreach ($resArr['tips'] as $k => $v) {
                if (empty($v['location'])) {
                    unset($resArr['tips'][$k]);
                }
            }
            $resArr['tips'] = array_values($resArr['tips']);
            $resArr['count'] = count($resArr['tips']);
            $res = json_encode($resArr);
        }
    }
    
    return  $res;
}
function getSignContent($params){
        
    ksort($params);
    
    $stringToBeSigned = "";
    $i = 0;
    foreach ($params as $k => $v) {
        if (checkEmpty($v) === false && substr($v, 0, 1) != "@") {
            
            if ($i == 0) {
                $stringToBeSigned .= "$k" . "=" . "$v";
            } else {
                $stringToBeSigned .= "&" . "$k" . "=" . "$v";
            }
            $i++;
        }
    }
    unset ($k, $v);
    
    return $stringToBeSigned;
}
function checkEmpty($value) {
    if (!isset($value)){
        return true;
    }
    if ($value === null){
        return true;
    }
    if (trim($value) === ""){
        return true;
    }
    return false;
}

/**
 * 跳转手机页面
 * @param $config
 * @return string
 */
function wapJump($config)
{

    global $ModuleName;
    $Loaction   =   '';
    $mArray     =   array('qqconnect', 'sinaconnect', 'call', 'datav', 'shop', 'promoter');
    $cArray     =   array('clickhits', 'wjump');

    if ($config['sy_wap_jump'] == '1' && !in_array($ModuleName, $mArray) && !in_array($_GET['c'], $cArray)) {

        if (isMobileUser() && !strpos(strtolower($_SERVER['REQUEST_URI']), 'wap') && $_SERVER['HTTP_HOST'] != $config['sy_wapdomain']) {

            include(PLUS_PATH . "jump.cache.php");

            if ($_GET['c']) {
                if ($ModuleName =='article' && $_GET['c'] == 'list'){

                }else{
                    $_GET['a']  =   $_GET['c'];
                }
            }
            if ($ModuleName && $ModuleName != 'index') {
                $_GET['c']  =   $ModuleName;
                if (isset($wapA) && $wapA[$ModuleName][$_GET['a']]) {

                    $_GET['a']  =   $wapA[$ModuleName][$_GET['a']];
                }
            }
            if ($_GET['c']) {
                $jumpGet['c']   =   $_GET['c'];
                unset($_GET['c']);
            }
            if ($_GET['a']) {
                $jumpGet['a']   =   $_GET['a'];
                unset($_GET['a']);
            }

            if (!empty($_GET)) {

                foreach ($_GET as $key => $value) {

                    if ($key == 'keyword') {
                        $jumpGet[$key] = $value;
                    }

                    if ($key == 'code') {
                        $jumpGet[$key] = $value;
                    }

                    if (!empty($value)) {

                        $jumpGet[$key] = $value;
                    }
                }
            }

            $Loaction   =   Url('wap', $jumpGet);
		}elseif (empty($_GET['m']) && $_GET['c'] == 'wap'){

            $Loaction   =   Url('wap');
        }
    }
    return $Loaction;
}
/**
 * 跳转PC页面
 * @param $config
 * @return string
 */
function pcJump($config)
{
    
    $Loaction   =   '';
    // 电脑点访问WAP,不需跳转的链接
    $c = isset($_GET['c']) ? $_GET['c'] : '';
    $a = isset($_GET['a']) ? $_GET['a'] : '';
    $cArray = array('work');
    $aArray = array('getHbBase', 'dataShowIndex');
    
    if (!isMobileUser() && $config['sy_pc_jump_wap'] != '1' && !in_array($c,$cArray) && !in_array($a,$aArray)) {

        

            include(PLUS_PATH . "jump.cache.php");
			
			if ($_GET['c']) {
                 $_GET['m']  =   $_GET['c'];
				 unset($_GET['c']);
            }
            if ($_GET['a']) {
                 $_GET['c']  =   $_GET['a'];
				 unset($_GET['a']);
            }
            if ($_GET['m']) {
                $modName   =   $_GET['m'];
                unset($_GET['m']);
            }else{
				$modName = 'index';
			}
            if ($_GET['c']) {
                $jumpGet['c']   =   $_GET['c'];
                unset($_GET['c']);
            }
			
            if (!empty($_GET)) {

                foreach ($_GET as $key => $value) {

                    if (!empty($value)) {

                        $jumpGet[$key] = $value;
                    }
                }
            }
			
			$Loaction   =   Url($modName, $jumpGet);
    }
	
    return $Loaction;
}
/**
 * 使用终端是否手机判断方法
 * @return bool
 */
function isMobileUser()
{
    // miniprogramenv电脑端微信小程序
    $uachar =   '/(nokia|sony|ericsson|mot|samsung|sgh|lg|philips|panasonic|alcatel|lenovo|cldc|midp|mobile|phone|iphone|ipad|ipod|android|symbian|smartphone|miniprogramenv)/i';
    
    $ua     =   strtolower($_SERVER['HTTP_USER_AGENT']);

    if (preg_match($uachar, $ua)) {

        return true;
    } else {

        return false;
    }
}

/**
 * 获取随机数
 * @param int $length
 * @return
 */
function gt_Generate_code($length = 6)
{
    return rand(pow(10, ($length - 1)), pow(10, $length) - 1);
}

/**
 * @param $config
 * @return string[]
 */
function verifytoken($config)
{

    if ($config['code_kind'] == '3') {

        $check  =   gtGeetest($config);
    } elseif ($config['code_kind'] == '4') {

        $check  =   dxauthcode($config);
    } elseif ($config['code_kind'] == '5') {

        $check  =   vaptchacode($config);
    } elseif ($config['code_kind'] == '6') {
        $check = tecentauthcode($config);
    }
    if ($check) {
        return array('code' => '200');
    } else {
        switch ($config['code_kind']) {
            case '3' :
                $msg = '请滑动滑块进行验证！';
                break;
            case '4' :
                $msg = '请拖动滑块进行验证';
                break;
            case '5' :
                $msg = '请绘制图中手势按钮进行验证';
                break;
			case '6' :
                $msg = '验证失败，请重新验证！';
                break;
        }

        return array('code' => '0', 'msg' => $msg);
    }

}

/**
 * 手势验证码
 * @param array $config
 * @return bool
 */
function vaptchacode($config = array())
{

    if (empty($config)) {

        include(PLUS_PATH . 'config.php');
    }

    $url                =   'http://0.vaptcha.com/verify';
    $data['id']         =   $config['sy_vaptcha_vid'];
    $data['secretkey']  =   $config['sy_vaptcha_key'];
    $data['scene']      =   0;
    $data['token']      =   $_POST['verify_token'];
    $data['ip']         =   fun_ip_get();

    $CurlReturn         =   CurlPost($url, $data);

    $vaptchaReturn      =   json_decode($CurlReturn);
    if ($vaptchaReturn->success == '1') {

        return true;
    } else {

        return false;
    }
}

/**
 * 获取极验验码
 * @param array $config
 * @return bool
 */
function gtGeetest($config = array())
{

    if (!empty($_POST['verify_token'])) {

        $token  =   @explode('*', $_POST['verify_token']);

        $geetest_challenge =   $token[0];
        $geetest_validate  =   $token[1];
        $geetest_seccode   =   $token[2];
        
        if (!empty($geetest_challenge) && !empty($geetest_validate) && !empty($geetest_seccode)) {
            if (!isset($_SESSION)) {
                session_start();
            }
            require_once LIB_PATH . '/class.geetestlib.php';
            if (!$config) {
                include(PLUS_PATH . 'config.php');
            }
            $GtSdk      =   new GeetestLib($config['sy_geetestid'], $config['sy_geetestkey']);
            $user_id    =   $_SESSION['user_id'];
            $data       =   array(
                "user_id"       =>  $user_id, # 网站用户id
                "client_type"   =>  "web", #web:电脑上的浏览器；h5:手机上的浏览器
                "ip_address"    =>  "127.0.0.1" # 请在此处传输用户请求验证时所携带的IP
            );
            
            if ($_SESSION['gtserver'] == 1) {   //服务器正常
                $result =   $GtSdk->success_validate($geetest_challenge, $geetest_validate, $geetest_seccode, $data);
                if ($result) {
                    return true;
                } else {
                    return false;
                }
            } else {  //服务器宕机,走failback模式
                if ($GtSdk->fail_validate($geetest_challenge, $geetest_validate, $geetest_seccode)) {
                    return true;
                } else {
                    return false;
                }
            }
        } else {
            return false;
        }
    } else {
        return false;
    }
}

/**
 * 检验顶象验证码
 * @param array $config
 * @return bool
 */
function dxauthcode($config = array())
{
    include(LIB_PATH . "dxCaptchaClient.class.php");

    /**
     *  构造入参为appId和appSecret :
     *      appId和前端验证码的appId保持一致，appId可公开
     *      appSecret为秘钥，请勿公开
     *
     *      token在前端完成验证后可以获取到，随业务请求发送到后台，token有效期为两分钟
     */
    $appId      =   $config['sy_dxappid'];
    $appSecret  =   $config['sy_dxappsecret'];
    $client     =   new CaptchaClient($appId, $appSecret);
    $client->setTimeOut(2);      //设置超时时间，默认2秒
    # $client->setCaptchaUrl("http://cap.dingxiang-inc.com/api/tokenVerify");

    //特殊情况可以额外指定服务器，默认情况下不需要设置
    $response   =   $client->verifyToken($_POST['verify_token']);

    //确保验证状态是SERVER_SUCCESS，SDK中有容错机制，在网络出现异常的情况会返回通过
    if($response->serverStatus == 'SERVER_SUCCESS'){
        return true;
        /**token验证通过，继续其他流程**/
    } else {
        return false;
        /**token验证失败**/
    }
}

//检验腾讯云验证码
function tecentauthcode($config=array(), $isWechat = 0){
    require_once LIB_PATH . '/tecentcode/vendor/autoload.php';
    try {
        $cred = new \TencentCloud\Common\Credential($config['sy_tecentsecretid'], $config['sy_tecentsecretkey']);
        // 实例化一个http选项，可选的，没有特殊需求可以跳过
        $httpProfile = new \TencentCloud\Common\Profile\HttpProfile();
        $httpProfile->setEndpoint("captcha.tencentcloudapi.com");
        $httpProfile->setLocalProtocol(getprotocol($config['sy_weburl']));// 设置本地域名请求协议，http协议请求时无需校验证书
        // 实例化一个client选项，可选的，没有特殊需求可以跳过
        $clientProfile = new \TencentCloud\Common\Profile\ClientProfile();
        $clientProfile->setHttpProfile($httpProfile);
        // 实例化要请求产品的client对象,clientProfile是可选的
        $client = new \TencentCloud\Captcha\V20190722\CaptchaClient($cred, "", $clientProfile);
        // 实例化一个请求对象,每个接口都会对应一个request对象
        if ($isWechat == 0) {
            $req = new \TencentCloud\Captcha\V20190722\Models\DescribeCaptchaResultRequest();
        } else {
            $req = new \TencentCloud\Captcha\V20190722\Models\DescribeCaptchaMiniResultRequest();
        }

        $params = array(
            "CaptchaType" => 9,
            "Ticket" => $_POST['verify_token'],
            "UserIp" => fun_ip_get(),
            "CaptchaAppId" => intval($config['sy_tecentappid']),
            "AppSecretKey" => $config['sy_tecentappsecret']//"TbdQ8ShRt978nqsDUxc2dGM9m"
        );
        if ($isWechat == 0) {
            $params['Randstr'] = $_POST['verify_str'];
        }
        $req->fromJsonString(json_encode($params));
        // 返回的resp是一个DescribeCaptchaResultResponse的实例，与请求对象对应
        if ($isWechat == 0) {
            $resp = $client->DescribeCaptchaResult($req);
        } else {// 小程序端
            $resp = $client->DescribeCaptchaMiniResult($req);
        }
//        file_put_contents(APP_PATH."/tc_code.txt",date('Y-m-d H:i:s') . "  "  . json_encode($resp) . "\r\n",FILE_APPEND);
        // 输出json格式的字符串回包
        $res = json_decode($resp->toJsonString(), 1);
        if ($res['CaptchaCode'] == 1) {// 验证成功
            return true;
        } else {
            return false;
        }
    } catch(\TencentCloud\Common\Exception\TencentCloudSDKException $e) {
//        file_put_contents(APP_PATH."/tc_code.txt",date('Y-m-d H:i:s') . "  "  . $e->getMessage() . "\r\n",FILE_APPEND);
        return false;
    }
}

/**
 * 获取数字验码
 */
function gtverify()
{
    if (md5(strtolower($_POST['authcode'])) != $_SESSION['authcode'] || empty($_SESSION['authcode'])) {
        unset($_SESSION['authcode']);
        return false;
    }
    return true;
}

/**
 * @return bool
 */
function is_weixin()
{
    if (strpos($_SERVER['HTTP_USER_AGENT'], 'MicroMessenger') !== false) {
        return true;
    }
    return false;
}

/**
 * @param array $parseDate
 * @param $time
 * @param $domain
 */
function setcookies($parseDate = array(), $time, $domain)
{

    $domain =   get_domain($domain);
    if (is_array($parseDate)) {
        foreach ($parseDate as $key => $value) {
            SetCookie($key, $value, $time, "/", $domain);
        }
    }
}

// 上面方法得到的密文太长，不适合放入承载信息有限的二维码中
// 加密
function yunEncrypt($str, $key)
{
    $key    =   md5($key);
    $k      =   md5(rand(0, 100)); // 相当于动态密钥
    $k      =   substr($k, 0, 3);
    $tmp    =   "";
    for ($i = 0; $i < strlen($str); $i++) {
        $tmp .= substr($str, $i, 1) ^ substr($key, $i, 1);
    }
    return base64_encode($k . $tmp);
}

// 解密
function yunDecrypt($str, $key)
{
    $len    =   strlen($str);
    $key    =   md5($key);
    $str    =   base64_decode($str);
    $str    =   substr($str, 3, $len - 3);
    $tmp    =   "";
    for ($i = 0; $i < strlen($str); $i++) {
        $tmp .= substr($str, $i, 1) ^ substr($key, $i, 1);
    }
    return $tmp;
}

/**
 * 数组排序
 */
function my_sort($prev, $next)
{
    if ($prev['value'] == $next['value']) return 0;
    return ($prev['value'] < $next['value']) ? 1 : -1;
}
/**
 * @param $prev
 * @param $next
 * @return int
 */
function t_sort($prev, $next)
{
    $p  =   strtotime($prev);
    $n  =   strtotime($next);
    if ($p == $n) return 0;
    return ($p > $n) ? 1 : -1;
}


/**
 * 判断当前服务器是windows系统还是其他系统
 */
function isServerOsWindows()
{
    return stristr(php_uname('s'), 'window') ? true : false;
}

/**
 * @param $serial_str
 * @return mixed
 */
function mb_unserialize($serial_str)
{
    $serial_str = str_replace("\r", "", $serial_str);
    $serial_str = preg_replace_callback('/s:\d+:"(.+?)";/s', 'checkunserialize', $serial_str);
    return unserialize($serial_str);
}

/**
 * @param $r
 * @return string
 */
function checkunserialize($r)
{
    $n = strlen($r[1]);
    return "s:$n:\"$r[1]\";";
}

/**
 * 隐藏部分用户名：手机号或者邮箱
 * @param $str
 * @return string
 */
function sub_string($str)
{

    $length     =   mb_strlen($str);
    if ($length > 5 && (CheckMobile($str) || CheckTell($str) || CheckRegEmail($str))) {

        $str    =   mb_substr($str, 0, 3) . '****' . mb_substr($str, $length - 4, 4);
    }
    return $str;
}

/**
 * 检查图片路径，该路径是否有图片，没有图片替换指定图片
 * @param string $post 默认图片路径
 * @param string $url 现有路径
 * @return string
 */
function checkpic($url = '', $post = '')
{

    global $config;

    if (isset($config['sy_oss']) && $config['sy_oss'] == 1) {
        $curl = $config['sy_ossurl'];
    } else {
        $curl = $config['sy_weburl'];
    }

    $picurl = '';

    if ($url != '') {

        if (strstr($url, 'http') !== false || strstr($url, 'https') !== false) {

            $picurl = $url;

        } else {

            if (strstr($url, '../data') !== false) {

                $picurl = str_replace('../data', $curl . '/data', $url);

            } elseif (strstr($url, './data') !== false) {

                $picurl = str_replace('./data', $curl . '/data', $url);

            } elseif (strstr($url, '/data') !== false) {

                $picurl = str_replace('/data', $curl . '/data', $url);
            } elseif (strstr($url, '.data') !== false) {

                $picurl = str_replace('.data', $curl . '/data', $url);

            } else {

                $picurl = $curl . '/' . $url;
            }
        }
    } else {

        if ($post != '') {

            if (strstr($post, 'http') !== false || strstr($url, 'https') !== false) {

                $picurl = $post;
            } else {
                $picurl = $curl . '/' . $post;

            }
        }
    }
    return $picurl;
}

/**
 * 全局密码生成函数
 * @param $pass
 * @param string $salt
 * @param string $oldpass
 * @return bool
 */
function passCheck($pass, $salt = '', $oldpass = '')
{

    include_once(LIB_PATH . "pwdtype/phpyunpass.php");
    return passwordCheck($pass, $salt, $oldpass);
}

/**
 * @param string $weburl
 * @return string
 */
function getprotocol($weburl = '')
{

    if ($weburl) {
        if (strpos($weburl, 'https://') !== false) {

            $protocol   =   'https://';
        } else {

            $protocol   =   'http://';
        }
    } else {

        $protocol       =   ((isset($_SERVER['HTTPS']) && $_SERVER['HTTPS'] == 'on') || (isset($_SERVER['HTTP_X_FORWARDED_PROTO']) && $_SERVER['HTTP_X_FORWARDED_PROTO'] == 'https')) ? 'https://' : 'http://';
    }
    return $protocol;
}

/**
 * @desc 时间日期格式化为多少天前/后
 *
 * @param sting/int $date_time
 * @param int $type 1、'Y-m-d H:i:s' 2、时间戳
 * @param int $before 1、前 2、后 3、剩余多少天 4、过期多少天
 * @param string $format
 * @return string
 */
function format_datetime($date_time, $type = 1, $before = 1, $format = '')
{
    if ($type == 1) {

        $timestamp = strtotime($date_time);
    } elseif ($type == 2) {

        $timestamp = $date_time;
    }

    if (!empty($format)) {

        return date($format, $timestamp);
    }

    $today  =   strtotime('today');

    if ($before == 1) {

        $difference = time() - $timestamp;

        if ($timestamp > $today) {

            return '今天';
        } elseif ($timestamp > ($today - 86400)) {

            return '昨天';
        } else {

            return ceil($difference / 86400) . '天前';
        }
    } else if ($before == 2) {

        $tomorrow = $today + 86400;

        if ($timestamp < $tomorrow) {

            return '今天';
        } elseif ($timestamp < ($tomorrow + 86400)) {

            return '明天';
        } elseif ($timestamp < ($tomorrow + 172800)) {

            return '后天';
        } elseif ($timestamp < ($tomorrow + 604800)) {

            return '一周后';
        } elseif ($timestamp < ($tomorrow + 2952000)) {

            return '一月后';
        } else {

            $difference = $timestamp - time();
            return ceil($difference / 86400) . '天后';
        }
    } else if ($before == 3) {

        $difference =   $timestamp - $today;
        if (ceil($difference / 86400) > 0){

            return '剩余 '. ceil($difference / 86400) . ' 天';
        }else{

            return '今日到期';
        }
    } else if ($before == 4) {

        $difference = $today - $timestamp;
        return '过期 '.ceil($difference / 86400).' 天';
    }
}

/**
 * @desc 刷新时间日期格式化为多少分钟前；int $type 1、多少分钟前 2、多少小时前 3、'i:s'
 *
 * @param sting/int $date_time
 * @return string
 */
function lastupdateStyle($date_time)
{
    global $config;

    $type           =   $config['sy_sxsjgs'];
    $updateTime     =   floor((time() - $date_time) / 3600);
    if ($date_time < time() && $date_time > strtotime('today')) {

        if ($type == 1) {

            return date('H:i', $date_time);
        } elseif ($type == 2) {
            if ($updateTime >= 1) {

                return $updateTime . '小时前';
            } else {

                return ceil((time() - $date_time) / 60) . '分钟前';
            }
        }
    } else {
        return date('Y-m-d', $date_time);
    }
}
/**
 * 时间格式化：今天（时.分） 今年（月.日） 其他（年-月-日）
 * @param string $time
 */
function timeForYear($time){
    
    if($time > strtotime('today')){
        
        $str  =  '今天 '.date('H:i', $time);
        
    }else if($time > mktime(0,0,0,1,1,date('Y'))){
        
        $str  =  date('m月d日', $time);
        
    }else{
        
        $str  =  date('Y-m-d', $time);
    }
    return $str;
}
/**
 * 处理掉二维数组中值为空的参数
 * @param $arr
 * @return mixed
 */
function removeEmpty($arr)
{

    foreach ($arr as $k => $v) {
        foreach ($v as $mk => $mv) {
            if (empty($mv)) {
                unset($arr[$k][$mk]);
            }
        }
    }
    return $arr;
}

/**
 * 判断会员是否到期
 * @param int $endtime
 * @return boolean
 */
function isVip($endtime = 0)
{

    if ($endtime >= strtotime('today') || $endtime == 0) {

        return true;
    } else {

        return false;
    }
}
/**
 * 处理最低、最高薪资，获取统一薪资
 */
function salaryUnit($minsalary = 0, $maxsalary = 0)
{
    global $config;
    if ($config['resume_salarytype'] == 1){
		$unit = '元';
	}elseif ($config['resume_salarytype'] == 2) {
        $unit = '千';
    } elseif ($config['resume_salarytype'] == 3) {
        $unit = 'K';
    } elseif ($config['resume_salarytype'] == 4) {
        $unit = 'k';
    }
    if(!empty($minsalary) && !empty($maxsalary)){
        
        if ($config['resume_salarytype'] == 1) {
            if($maxsalary<2000){
                $salary = '2000'.$unit.'以下';
            }else{
                $salary = $minsalary.'-'.$maxsalary.$unit;
            }
        }else{
            if($maxsalary<2000){
                $salary = 2 . $unit . '以下';
            }else{
                $salary = floor($minsalary / 1000 * 10) / 10 . '-'. floor($maxsalary / 1000 * 10) / 10 . $unit;
            }
        }
    }elseif(!empty($minsalary)){
        if($config['resume_salarytype']==1){
            $salary = $minsalary.$unit;
        }else{
            $salary = floor($minsalary / 1000 * 10) / 10 . $unit;
        }
    }else{
        $salary = '面议';
    }
    return $salary;
}
/**
 * 产生随机字符串
 */
function createstr($length = 10)
{

    $chars = "abfghijklmprtvwyz0123456789";
    $str = "";
    for ($i = 0; $i < $length; $i++) {
        $str .= substr($chars, mt_rand(0, strlen($chars) - 1), 1);
    }
    return $str;
}

function checkMsgOpen($config)
{

    if ($config["sy_msg_appkey"] == "" || $config["sy_msg_appsecret"] == "" || $config['sy_msg_isopen'] != '1') {

        return false;
    } else {

        return true;
    }
}

/**
 * @desc    时间戳 转化 为具体时间格式
 *          1：当天时间（09：30）
 *          2：当年时间（5月20日）
 *          3：今年之前（2020.05.20）
 * @param   $time
 * @return  false|string
 */
function formatTime($time){

    if ($time > strtotime(date('Y-m-d'))) {

        $time_n =   date('H.i', $time);
    } else if ($time > mktime(0, 0, 0, 1, 1, date('Y'))) {

        $time_n =   date('m月d日', $time);
    } else {

        $time_n =   date('Y-m-d', $time);
    }

    return  $time_n;
}
/**
 * 获取当前月份区间
 *
*/
function get_this_month($time){
    $y = date("Y", $time); //年
    $m = date("m", $time); //月
    $d = date("d", $time); //日
    $t0 = date('t'); // 本月一共有几天
    $r=array();
    $r['start_month'] = mktime(0, 0, 0, $m, 1, $y); // 创建本月开始时间
    $r['end_month'] = mktime(23, 59, 59, $m, $t0, $y); // 创建本月结束时间
    return $r;
}
/**
 * 简历工作经历平均时间 XX月 调整  X年X 月
 * @param $long
 * @return string
 */
function avgToYm($long)
{
    if ($long >= 12) {

        if (bcmod($long, 12) > 0) {

            $return = floor($long / 12) . '年' . bcmod($long, 12);
        } else {

            $return = floor($long / 12) . '年';
        }

    } else {

        $return = $long;
    }

    return $return;
}

/**
 * 校验用户名合法性
 * @param $mobile
 */
function checkRegnameLegal($regname){
    include(PLUS_PATH . "reg.cache.php");

    if (!empty($regConfig['regname'])) {
        if (in_array($regname, $regConfig['regname'])) {
            return false;
        }
    }

    return true;
}

/**
 * 校验手机号合法性
 * @param $mobile
 */
function checkMobileLegal($mobile){
    include(PLUS_PATH . "reg.cache.php");

    if (!empty($regConfig['mobile_white'])) { // 白名单
        if (in_array($mobile, $regConfig['mobile_white'])) {
            return true;
        }
    }
    if (!empty($regConfig['mobile_black'])) { // 黑名单
        if (in_array($mobile, $regConfig['mobile_black'])) {
            return false;
        }
    }
    if (!empty($regConfig['mobile_number'])) { // 号码段
        $number = substr($mobile, 0, 3);
        if (in_array($number, $regConfig['mobile_number'])) {
            return false;
        }
    }

    return true;
}

/**
 * 解析ajax传上来的json字符串
 * @param $obj
 */
function jsJsonDecode($obj){
    return json_decode(str_replace(array("“", "”"), array('"', '"'), stripslashes($obj)), true);
}



//兼容性处理
if(!function_exists('array_column')){
    function array_column($input,$column_key,$index_key=''){
        
        if(!is_array($input)){
             return;
        }

        $results=array();

        if($column_key===null){

            if(!is_string($index_key)&&!is_int($index_key)){
                return false;
            } 

            foreach($input as $_v){

                if(array_key_exists($index_key,$_v)){

                    $results[$_v[$index_key]]=$_v;

                }

            }

            if(empty($results)){
                $results=$input;
            } 

        }else if(!is_string($column_key)&&!is_int($column_key)){

            return false;

        }else{

            if(!is_string($index_key)&&!is_int($index_key)){
                return false;
            } 

            if($index_key===''){

                foreach($input as $_v){

                    if(is_array($_v)&&array_key_exists($column_key,$_v)){

                        $results[]=$_v[$column_key];

                    }

                }

            }else{

                foreach($input as $_v){

                    if(is_array($_v)&&array_key_exists($column_key,$_v)&&array_key_exists($index_key,$_v)){

                        $results[$_v[$index_key]]=$_v[$column_key];

                    }

                }

            }

        }

        return $results;
    }
}

// 获取后台完整路径
function getAdminPath($path = '') {
    global $config;

    /* @var $admindir */
    include(PLUS_PATH."/admindir.php");

    $dir = $admindir ? $admindir : 'admin';

    return $config['sy_weburl'] . '/' . $dir . ($path ? '/' . $path: '');;
}

/**
 * 多文件上传，$_FILE转成数组
 * @param $files $_FILE
 * @return array
 */
function format_files($files)
{
    $fileArray = array();
    $n = 0;
    foreach ($files as $key => $file) {
        if (is_array($file['name'])) {
            $keys = array_keys($file);
            $count = count($file['name']);
            for ($i = 0; $i < $count; $i++) {
                $fileArray[$n]['key'] = $key;
                foreach ($keys as $_key) {
                    $fileArray[$n][$_key] = $file[$_key][$i];
                }
                $n++;
            }
        } else {
            $fileArray[$n] = $file;
            $fileArray[$n]['key'] = $key;
            $n++;
        }
    }

    return $fileArray;
}
?>