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
class generate_cache_controller extends adminCommon
{
    function index_action()
    {
        $list = array();
        include(CONFIG_PATH . "db.data.php");
        if ($arr_data['cache']) {
            foreach ($arr_data['cache'] as $key => $value) {
                $list[] = array('id' => $key, 'name' => $value);
            }
        }
        $this->render_json(0, '', $list);
    }
    
    function cache_action()
    {
        if ($_POST["madeall"]) {
            $configM = $this->MODEL('config');
            /*生成四位JS，css识别码*/
            $cachecode = rand(1000, 9999);
            $configM->setConfig(array('cachecode' => $cachecode));
            $this->web_config();
            include(LIB_PATH."cache.class.php");
            set_time_limit(200);
            $cache = $_POST['cache'];
            $cacheclass = new cache(PLUS_PATH, $this->obj);
            
            if (@in_array("1", $cache)) {
                $makecache = $cacheclass->city_cache("city.cache.php");
            }
            if (@in_array("2", $cache)) {
                $makecache = $cacheclass->industry_cache("industry.cache.php");
            }
            if (@in_array("3", $cache)) {
                $makecache = $cacheclass->job_cache("job.cache.php");
            }
            if (@in_array("4", $cache)) {
                $makecache = $cacheclass->user_cache("user.cache.php");
            }
            if (@in_array("5", $cache)) {
                $makecache = $cacheclass->com_cache("com.cache.php");
            }
            if (@in_array("6", $cache)) {
                $makecache = $cacheclass->domain_cache("domain_cache.php");
            }
            if (@in_array("7", $cache)) {
                $makecache = $this->del_dir("../data/templates_c", 1);
                $makecache = $this->del_dir("../data/cache", 1);
                $cacheclass->regconfig_cache("reg.cache.php"); // 生成注册配置缓存
            }
            if (@in_array("8", $cache)) {
                $makecache = $cacheclass->seo_cache("seo.cache.php");
            }
            if (@in_array("9", $cache)) {
                $makecache = $cacheclass->menu_cache("menu.cache.php");
            }
            if (@in_array("10", $cache)) {
                $makecache = $cacheclass->part_cache("part.cache.php");
            }
            if (@in_array("11", $cache)) {
                $makecache = $cacheclass->link_cache("link.cache.php");
            }
            if (@in_array("12", $cache)) {
                $makecache = $cacheclass->group_cache("group.cache.php");
            }
            if (@in_array('13', $cache)) {
                $makecache = $cacheclass->redeem_cache("redeem.cache.php");
            }
            if (@in_array('14', $cache)) {
                $adM = $this->MODEL('ad');
                $makecache = $adM->model_ad_arr();
            }
            if (@in_array('15', $cache)) {
                $makecache = $cacheclass->reason_cache("reason.cache.php");
            }
            if (@in_array('16', $cache)) {
                $makecache = $cacheclass->integralclass_cache("integralclass.cache.php");
            }
            
            if (@in_array('18', $cache)) {
                $makecache = $cacheclass->moblienav_cache("moblienav.cache.php");
            }
            if (@in_array('19', $cache)) {
                $makecache = $cacheclass->navmap_cache("navmap.cache.php");
            }
            if (@in_array('20', $cache)) {
                $makecache = $cacheclass->ask_cache("ask.cache.php");
            }
            
            if (@in_array("23", $cache)) {
                $makecache = $cacheclass->introduce_cache("introduce.cache.php");
            }
            if (@in_array("24", $cache)) {
                $makecache = $cacheclass->keyword_cache("keyword.cache.php");
            }
            if (@in_array("25", $cache)) {
                $makecache = $cacheclass->desc_cache("desc.cache.php");
            }
            if (@in_array("26", $cache)) {
                $makecache = $cacheclass->database_cache("dbstruct.cache.php");
            }
            if (@in_array("27", $cache)) {
                $makecache = $cacheclass->emailconfig_cache("emailconfig.cache.php");
            }
            
            if (@in_array("29", $cache)) {
                $makecache = $cacheclass->cron_cache("cron.cache.php");
            }
            if (@in_array("30", $cache)) {
                $wxAppM = $this->MODEL('wxapp');
                $makecache = $wxAppM->tplappCache();
            }
            if ($makecache) {
                $this->admin_json(0, "生成(ID:$makecache)成功！");
            } else {
                $this->admin_json(8, "生成(ID:$makecache)失败！");
            }
        }
    }
}