<?php

/*
 * $Author ：gengzs
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2023 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */
class set_config_controller extends adminCommon{

    /**
     * 系统-网站设置
     */
    function index_action(){
        $data = array();
        if ($this->config['sy_ossurl'] == ''){
            $data['sy_ossurl'] = $this->config['sy_weburl'];
        }else{
            $data['sy_ossurl'] = $this->config['sy_ossurl'];
        }
        if ($this->config['sy_web_online']==1){
            $data['sy_web_online_status'] = true;
        }else{
            $data['sy_web_online_status'] = false;
        }
        if ($this->config['sy_iscsrf']==1){
            $data['sy_iscsrf_status'] = true;
        }else{
            $data['sy_iscsrf_status'] = false;
        }
        if ($this->config['sy_istemplate']==1){
            $data['sy_istemplate_status'] = true;
        }else{
            $data['sy_istemplate_status'] = false;
        }
        $data['sy_webname'] = $this->config['sy_webname'];
        $data['sy_weburl'] = $this->config['sy_weburl'];
        $data['sy_webkeyword'] = $this->config['sy_webkeyword'];
        $data['sy_webmeta'] = $this->config['sy_webmeta'];
        $data['sy_webcopyright'] = $this->config['sy_webcopyright'];
        $data['sy_webtongji'] = $this->config['sy_webtongji'];
        $data['sy_webemail'] = $this->config['sy_webemail'];
        $data['sy_webmoblie'] = $this->config['sy_webmoblie'];
        $data['sy_webrecord'] = $this->config['sy_webrecord'];
        $data['sy_websecord'] = $this->config['sy_websecord'];
        $data['sy_perfor'] = $this->config['sy_perfor'];
        $data['sy_hrlicense'] = $this->config['sy_hrlicense'];
        $data['sy_webtel'] = $this->config['sy_webtel'];
        $data['sy_qq'] = $this->config['sy_qq'];
        $data['sy_freewebtel'] = $this->config['sy_freewebtel'];
        $data['sy_comwebtel'] = $this->config['sy_comwebtel'];
        $data['sy_worktime'] = $this->config['sy_worktime'];
        $data['sy_listnum'] = $this->config['sy_listnum'];
        $data['sy_webadd'] = $this->config['sy_webadd'];
        $data['sy_webclose'] = $this->config['sy_webclose'];
        $data['sy_safekey'] = $this->config['sy_safekey'];
        $data['sy_bannedip'] = $this->config['sy_bannedip'];
        $data['sy_fkeyword_all'] = $this->config['sy_fkeyword_all'];
        $data['sy_useragent'] = $this->config['sy_useragent'];
        $data['sy_bannedip_alert'] = $this->config['sy_bannedip_alert'];
        $data['sy_fkeyword'] = $this->config['sy_fkeyword'];
        $data['sy_list_login'] = $this->config['sy_list_login'];
        $data['sy_list_agent'] = $this->config['sy_list_agent'];
        $data['code_web'] = $this->config['code_web'];
        $data['code_kind'] = $this->config['code_kind'];
        $data['code_type'] = $this->config['code_type'];
        $data['code_filetype'] = $this->config['code_filetype'];
        $data['code_width'] = $this->config['code_width'];
        $data['code_height'] = $this->config['code_height'];
        $data['sy_geetestid'] = $this->config['sy_geetestid'];
        $data['sy_geetestkey'] = $this->config['sy_geetestkey'];
        $data['sy_dxappid'] = $this->config['sy_dxappid'];
        $data['sy_dxappsecret'] = $this->config['sy_dxappsecret'];
        $data['sy_geetestmid'] = $this->config['sy_geetestmid'];
        $data['sy_geetestmkey'] = $this->config['sy_geetestmkey'];
        $data['sy_vaptcha_vid'] = $this->config['sy_vaptcha_vid'];
        $data['sy_vaptcha_key'] = $this->config['sy_vaptcha_key'];
        $data['sy_tecentappid'] = $this->config['sy_tecentappid'];
        $data['sy_tecentappsecret'] = $this->config['sy_tecentappsecret'];
        $data['sy_tecentsecretid'] = $this->config['sy_tecentsecretid'];
        $data['sy_tecentsecretkey'] = $this->config['sy_tecentsecretkey'];
        $data['code_strlength'] = $this->config['code_strlength'];
        $data['sy_logo'] = $this->config['sy_logo'];
        $data['sy_member_logo'] = $this->config['sy_member_logo'];
        $data['sy_unit_logo'] = $this->config['sy_unit_logo'];
        $data['sy_lt_logo'] = $this->config['sy_lt_logo'];
        $data['sy_ltmember_logo'] = $this->config['sy_ltmember_logo'];
        $data['sy_px_logo'] = $this->config['sy_px_logo'];
        $data['sy_wap_logo'] = $this->config['sy_wap_logo'];
        $data['sy_wap_qcode'] = $this->config['sy_wap_qcode'];
        $data['sy_androidu_qcode'] = $this->config['sy_androidu_qcode'];
        $data['sy_iosu_qcode'] = $this->config['sy_iosu_qcode'];
        $data['sy_zph_icon'] = $this->config['sy_zph_icon'];
        $data['sy_zphbanner_icon'] = $this->config['sy_zphbanner_icon'];
        $data['sy_cplogo'] = $this->config['sy_cplogo'];
        $data['sy_gongzhaologo'] = $this->config['sy_gongzhaologo'];
        $data['mapurl'] = 'https://webapi.amap.com/maps?v=2.0&key=' . $this->config['map_key'];
        $data['map_tocity'] = $this->config['map_tocity'];
        $data['map_key'] = $this->config['map_key'];
        $data['map_secret'] = $this->config['map_secret'];
        $data['bmap_webak'] = $this->config['bmap_webak'];
        $data['map_x'] = $this->config['map_x'];
        $data['map_y'] = $this->config['map_y'];
        $data['ismemcache'] = $this->config['ismemcache'];
        $data['memcachehost'] = $this->config['memcachehost'];
        $data['memcacheport'] = $this->config['memcacheport'];
        $data['memcachetime'] = $this->config['memcachetime'];
        $data['webcache'] = $this->config['webcache'];
        $data['webcachetime'] = $this->config['webcachetime'];
        $data['issmartycache'] = $this->config['issmartycache'];
        $data['smartycachetime'] = $this->config['smartycachetime'];
        $data['pic_maxsize'] = $this->config['pic_maxsize'];
        $data['file_maxsize'] = $this->config['file_maxsize'];
        $data['pic_type'] = $this->config['pic_type'];
        $data['file_type'] = $this->config['file_type'];
        $data['is_picself'] = $this->config['is_picself'];
        $data['is_picthumb'] = $this->config['is_picthumb'];
        $data['is_watermark'] = $this->config['is_watermark'];
        $data['wmark_position'] = $this->config['wmark_position'];
        $data['sy_watermark'] = $this->config['sy_watermark'];

        $this->render_json(0,'',$data);

    }

    /**
     * 系统-网站设置-网站logo
     */
    function save_logo_action(){
        if (!$_POST['waterconfig']) {
            $this->render_json(1,'参数错误,请重试');
        }
        $this->web_config();
        $this->render_json(0,'操作成功');
    }

    // 保存
    function save_action(){

        if ($_POST['config'] == 'uploadconfig'){

            // 上传参数为空，保存默认值
            if (!$_POST['pic_maxsize'] || ($_POST['pic_maxsize'] == '' || $_POST['pic_maxsize'] < 1)){
                $_POST['pic_maxsize'] = 5;
            }
            if (!$_POST['file_maxsize'] || ($_POST['file_maxsize'] == '' || $_POST['file_maxsize'] < 1)){
                $_POST['file_maxsize'] = 5;
            }

            if (!$_POST['pic_type']){
                $_POST['pic_type'] = 'jpg,png,jpeg,bmp,gif';
            }else{
                $pic_type			=  explode(',',str_replace(' ','',$_POST['pic_type']));

                //禁止后台设定可执行程序后缀
                foreach($pic_type as $pickey => $picvalue){
                    $new_pic_type	=	strtolower(str_replace('.','',trim($picvalue)));
                    if(in_array($new_pic_type,array('php','asp','aspx','jsp','exe','do'))){

                        unset($pic_type[$pickey]);
                    }
                }
                $_POST['pic_type']	=	implode(',',$pic_type);
            }
            if (!$_POST['file_type']){
                $_POST['file_type'] = 'rar,zip,doc,docx,xls';
            }else{
                $file_type			=  explode(',',str_replace(' ','',$_POST['file_type']));
                //禁止后台设定可执行程序后缀
                foreach($file_type as $filekey => $filevalue){

                    $new_file_type	=	strtolower(str_replace('.','',trim($filevalue)));
                    if(in_array($new_file_type,array('php','asp','aspx','jsp','exe','do'))){
                        unset($file_type[$filekey]);
                    }
                }
                $_POST['file_type']	=	implode(',',$file_type);
            }
            if (!$_POST['wmark_position']){
                $_POST['wmark_position'] = 1;
            }
        }

        if(isset($_POST['ctype']) && $_POST['ctype']=='mapconfig'){
            $_POST['bmap_webak'] = $_POST['con_post'];
            $_POST['map_key'] = $_POST['content'];
            $_POST['map_secret'] = $_POST['map_secret'];
            unset($_POST['con_post']);
            unset($_POST['content']);
        }
        if (isset($_POST['map_key'])) {
            $_POST['mapurl'] = 'https://webapi.amap.com/maps?v=2.0&key=' . $_POST['map_key'];
        }

        if (!empty($_POST['sy_weburl'])) {
            $weburl = trim($_POST['sy_weburl']);
            if (stripos($weburl, 'http') === false){
                $this->render_json('1','网站地址缺少http://或https://');
            }
            // 保存域名时，相关的内容要重新保存，防止域名或http头改变后，有关功能异常
            if (!empty($this->config['map_key'])){
                // 地图地址
                $_POST['mapurl'] = 'https://webapi.amap.com/maps?v=2.0&key=' . $this->config['map_key'];
            }
            if (!empty($this->config['sy_indexdomain'])){
                // 分站默认域名
                $protocol  =  getprotocol($weburl);
                $indexUrl  =  parse_url($this->config['sy_indexdomain']);
                $indexPath =  !empty($indexUrl['path']) ? $indexUrl['path'] : '';
                $_POST['sy_indexdomain']  =  $protocol . $indexUrl['host'] . $indexPath;
            }
            if (file_exists(DATA_PATH.'/api/alipay/alipay_data.php')){
                // 支付宝配置参数中网站域名，跟随调整
                @include(DATA_PATH.'api/alipay/alipay_data.php');
                if (!empty($alipaydata)){
                    $alipaydata['sy_weburl']  =  $weburl;
                    made_web(DATA_PATH.'api/alipay/alipay_data.php',ArrayToString($alipaydata),'alipaydata');
                }
            }
        }
        $configM  =  $this->MODEL('config');
        if (isset($_POST['code_kind']) && $_POST['code_kind'] == 6) {// 腾讯云验证码
            PHP_VERSION < 7 && $this->layer_msg("腾讯云验证码要求php版本>=7.0！", 8, 1, '');
        }
        $configM -> setConfig($_POST);
        // 判断验证字符
        if ($_POST['code_strlength'] < 5) {
            $this->web_config();
            $this->render_json('0','网站配置设置成功!');
        } else {
            $this->render_json('1','验证码字符数不要大于4!');
        }
    }

    // 加载模板缓存
    function settplcache_action(){
        include (CONFIG_PATH . "db.data.php");
        include (PLUS_PATH . "cache.config.php");
        $modelconfig = $arr_data['modelconfig'];

        foreach ($modelconfig as $key => $value) {
            $newModel[$key]['value'] = $value;
            $newModel[$key]['cache'] = $cache_config['sy_' . $key . '_cache'];
        }
        $this->render_json(0,'操作成功',['newModel'=>$newModel,'sy_index_cache'=>$cache_config['sy_index_cache']]);

    }

    // 保存设置模板缓存
    function savetplcache_action(){

        include(CONFIG_PATH . "db.data.php");
        $modelconfig = array_keys($arr_data['modelconfig']);
        $config_new = array();
        foreach ($_POST as $key => $v) {
            $model = explode('_', $key);
            if (in_array($model[1], $modelconfig) || $model[1] == 'index') {
                $config_new[$key] = $v;
            }
        }

        made_web(PLUS_PATH . 'cache.config.php', ArrayToString($config_new), 'cache_config');

        $this->render_json(0, '操作成功');

    }


}
?>