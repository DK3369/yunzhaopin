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

class set_seo_controller extends adminCommon
{
    /**
     * SEO设置
     */
    public function index_action()
    {
        $seoM = $this->MODEL('seo');

        /* @var $arr_data */
        include(CONFIG_PATH . "/db.data.php");

        if (!empty($_POST['action'])) { // 获取seo列表
            $seolist = $seoM->getSeoList(array('seomodel' => $_POST['action']));
            $data['seolist'] = $seolist;
        } else {
            $data['seomodel'] = $arr_data['seomodel'];
        }

        $this->render_json(0, 'ok', $data);
    }

    /**
     * SEO添加
     */
    function seoadd_action()
    {
        $seoM = $this->MODEL('seo');

        /* @var $arr_data */
        include(CONFIG_PATH . "db.data.php");
        $data['seomodel'] = $arr_data['seomodel'];
        $data['seoconfig'] = $arr_data['seoconfig'];

        //提取分站内容
        $cacheM = $this->MODEL('cache');
        $domain = $cacheM->GetCache('domain');
        $data['Dname'] = (object)$domain['Dname'];

        !empty($_POST['id']) && $data['info'] = $seoM->getSeoInfo(array('id' => $_POST['id']));

        $this->render_json(0, 'ok', $data);
    }

    /**
     * SEO保存
     */
    function save_action()
    {
        $seoM = $this->MODEL('seo');

        $postData = array(
            'seoname' => $_POST['seoname'],
            'ident' => $_POST['ident'],
            'seomodel' => $_POST['seomodel'],
            'title' => $_POST['title'],
            'keywords' => $_POST['keywords'],
            'php_url' => $_POST['php_url'],
            'rewrite_url' => $_POST['rewrite_url'],
            'php_wap_url' => $_POST['php_wap_url'],
            'rewrite_wap_url' => $_POST['rewrite_wap_url'],
            'description' => $_POST['description'],
            'did' => $_POST['did'],
            'time' => time()
        );

        if (!empty($_POST['id'])) {
            $return = $seoM->upSeo(array('id' => $_POST['id']), $postData);
            $msg = "SEO 修改成功（ID：{$_POST['id']}）";
        } else {
            $return = $seoM->addSeo($postData);
            $msg = "SEO 添加成功（ID：{$return}）";
        }

        $return && $this->seocache(); // 缓存生成

        $this->admin_json($return ? 0 : 1, $msg);
    }

    /**
     * 删除SEO
     */
    function del_action()
    {
        $seoM = $this->MODEL('seo');
        if ($_POST) {
            $return = $seoM->delSeo(array('id' => intval($_POST['id'])));
            if ($return) {
                $this->seocache();
                $this->admin_json(0, 'SEO(ID:' . $_POST['del'] . ')删除成功！');
            } else {
                $this->admin_json(1, 'SEO(ID:' . $_POST['del'] . ')删除失败！');
            }
        }
    }

    /**
     * 更新SEO缓存
     */
    function seocache()
    {
        include(LIB_PATH . "cache.class.php");
        $cacheclass = new cache(PLUS_PATH, $this->obj);
        $cacheclass->seo_cache("seo.cache.php");
    }
}

?>