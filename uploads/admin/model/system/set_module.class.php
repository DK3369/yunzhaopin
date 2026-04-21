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

class set_module_controller extends adminCommon
{
    /**
     * 模块设置
     */
    public function index_action()
    {
        $configM = $this->MODEL('config');

        /* @var $arr_data */
        include(CONFIG_PATH . "db.data.php");
        $modelconfig = $arr_data['modelconfig'];
        unset($modelconfig['error']);
        $config = $configM->getList();
        foreach ($config['list'] as $v) {
            $config_new[$v['name']] = $v['config'];
        }
        foreach ($modelconfig as $key => $val) {
            $newModel[$key]['value'] = $val;
            $newModel[$key]['web'] = $config_new['sy_' . $key . '_web'];
            $newModel[$key]['ssl'] = $config_new['sy_' . $key . 'ssl'];
            $newModel[$key]['domain'] = $config_new['sy_' . $key . 'domain'];
            $newModel[$key]['dir'] = $config_new['sy_' . $key . 'dir'];
        }

        $data['module'] = $newModel;

        $this->render_json(0, 'ok', $data);
    }

    /**
     * 模块设置保存
     */
    public function save_action()
    {
        $navigationM = $this->MODEL('navigation');
        $configM = $this->MODEL('config');

        if (!empty($_POST)) {

            /* @var $arr_data */
            include(CONFIG_PATH . "db.data.php");
            $modelKey = array_keys($arr_data['modelconfig']);

            foreach ($modelKey as $key => $value) {
                if ($_POST[$value]['web'] == '1') {
                    $setSql['display'] = '1';
                } else {
                    $setSql['display'] = '0';
                }
                $navigationM->upNav($setSql, array('config' => $value));
                if (!$_POST[$value]['ssl'] || $_POST[$value]['domain'] == '') {
                    $_POST[$value]['ssl'] = '0';
                }
            }

            foreach ($_POST as $key1 => $val1) {
                foreach ($val1 as $key2 => $val2) { // 循环保存每一项
                    if ($key2 == 'value') { // 不写入缓存，这里做跳过处理
                        continue;
                    }

                    $configName = 'sy_' . $key1 . ($key2 == 'web' ? '_' : '') . $key2; // 基于config配置名称的特殊性，增加特殊处理
                    $config = $configM->getNum(array('name' => $configName));
                    if ($config > 0) {
                        $configM->upInfo(array('name' => $configName), array('config' => $val2));
                    } else {
                        $configM->addInfo(array('name' => $configName, 'config' => $val2));
                    }
                }
            }

            // 缓存清理
            $this->navcache();
            $this->web_config();

            $this->admin_json(0, '模块设置保存成功');
        }
    }

    /**
     * 更新导航缓存
     */
    private function navcache()
    {
        include(LIB_PATH . "cache.class.php");
        $cacheclass = new cache(PLUS_PATH, $this->obj);
        $cacheclass->menu_cache("menu.cache.php");
    }

    /**
     * 导航设置
     */
    function navset_action()
    {
        $navigationM = $this->MODEL('navigation');

        $type = $navigationM->getNavTypeList();
        $nav = $navigationM->getNav(array('config' => $_POST['config']));
        if (!$nav) {
            $nav = array('name' => $_POST['name'], 'config' => $_POST["config"], 'nid' => '1');
        }

        $this->render_json(0, 'ok', compact('type', 'nav'));
    }

    /**
     * 导航保存
     */
    function navsetSave_action()
    {
        $navigationM = $this->MODEL('navigation');

        $postData = array(
            'nid' => $_POST['nid'],
            'eject' => $_POST['eject'],
            'display' => $_POST['display'],
            'name' => $_POST['name'],
            'url' => $this->config['sy_' . $_POST['config'] . 'dir'],
            'sort' => $_POST['sort'],
            'model' => $_POST['model'],
            'bold' => $_POST['bold'],
            'type' => '1',
            'config' => $_POST['config'],
        );
        if ($_POST['id']) {
            $return = $navigationM->upNav($postData, array('id' => $_POST['id']));
            $this->navcache();
        } else {
            $return = $navigationM->addNav($postData);
            $this->navcache();
        }

        if ($return) {
            $this->admin_json(0, '导航设置成功', empty($_POST['id']) ? array('id' => $return) : array());
        } else {
            $this->admin_json(1, '导航设置失败');
        }
    }

    /**
     * seo设置
     */
    function seoshezhi_action()
    {
        $seoM = $this->MODEL('seo');
        if ($_POST["config"]) {
            /* @var $arr_data */
            include(CONFIG_PATH . "db.data.php");
            $seoconfig = $arr_data['seoconfig'];

            //提取分站内容
            $cacheM = $this->MODEL('cache');
            $domain = $cacheM->GetCache('domain');
            $Dname = (object)$domain['Dname'];

            $seo = $seoM->getSeoList(array('seomodel' => $_POST['config']));

            $this->render_json(0, 'ok', compact('seo', 'Dname', 'seoconfig'));
        }

        if ($_POST['id']) {
            $postData = array(
                'seoname' => $_POST['seoname'],
                'ident' => $_POST['ident'],
                'did' => $_POST['did'],
                'title' => $_POST['title'],
                'keywords' => $_POST['keywords'],
                'description' => $_POST['description'],
                'php_url' => $_POST['php_url'],
                'rewrite_url' => $_POST['rewrite_url'],
                'php_wap_url' => $_POST['php_wap_url'],
                'rewrite_wap_url' => $_POST['rewrite_wap_url'],
            );
            $return = $seoM->upSeo(array('id' => $_POST['id']), $postData);
            $this->seocache();

            $this->admin_json($return ? 0 : 1, $return ? 'SEO设置成功' : 'SEO设置失败');
        }
    }

    /**
     * 获取SEO模块信息
     */
    function getseo_action()
    {
        $seoM = $this->MODEL('seo');
        if ($_POST['id']) {
            $seo = $seoM->getSeoInfo(array('id' => $_POST['id']));
            $data['seoname'] = $seo['seoname'];
            $data['ident'] = $seo['ident'];
            $data['rewrite_url'] = $seo['rewrite_url'];
            $data['php_url'] = $seo['php_url'];
            $data['title'] = $seo['title'];
            $data['keywords'] = $seo['keywords'];
            $data['description'] = $seo['description'];
            $data['did'] = $seo['did'];
            $data['php_wap_url'] = $seo['php_wap_url'];
            $data['rewrite_wap_url'] = $seo['rewrite_wap_url'];

            $this->render_json(0, 'ok', $data);
        }
    }

    /**
     * 更新seo缓存
     */
    function seocache()
    {
        include(LIB_PATH . "cache.class.php");
        $cacheclass = new cache(PLUS_PATH, $this->obj);
        $cacheclass->seo_cache("seo.cache.php");
    }
}

?>