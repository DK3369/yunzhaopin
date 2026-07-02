<?php



class set_seo_controller extends adminCommon
{
    /**
     * SEO settings.
     */
    public function index_action()
    {
        $seoM = $this->MODEL('seo');

        /* @var $arr_data */
        include(CONFIG_PATH . "/db.data.php");

        if (!empty($_POST['action'])) { // Get SEO list.
            $seolist = $seoM->getSeoList(array('seomodel' => $_POST['action']));
            $data['seolist'] = $seolist;
        } else {
            $data['seomodel'] = $arr_data['seomodel'];
        }

        $this->render_json(0, 'ok', $data);
    }

    /**
     * Add SEO.
     */
    function seoadd_action()
    {
        $seoM = $this->MODEL('seo');

        /* @var $arr_data */
        include(CONFIG_PATH . "db.data.php");
        $data['seomodel'] = $arr_data['seomodel'];
        $data['seoconfig'] = $arr_data['seoconfig'];

        // Load site data.
        $cacheM = $this->MODEL('cache');
        $domain = $cacheM->GetCache('domain');
        $data['Dname'] = (object)$domain['Dname'];

        !empty($_POST['id']) && $data['info'] = $seoM->getSeoInfo(array('id' => $_POST['id']));

        $this->render_json(0, 'ok', $data);
    }

    /**
     * Save SEO.
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
            $successMsg = yun_t('admin_model_00100', array('id' => $_POST['id']));
            $errorMsg = yun_t('admin_model_00102', array('id' => $_POST['id']));
        } else {
            $return = $seoM->addSeo($postData);
            $successMsg = yun_t('admin_model_00101', array('id' => $return));
            $errorMsg = yun_t('admin_model_00103');
        }

        $return && $this->seocache(); // Generate cache.

        $this->admin_json($return ? 0 : 1, $return ? $successMsg : $errorMsg);
    }

    /**
     * Delete SEO.
     */
    function del_action()
    {
        $seoM = $this->MODEL('seo');
        if ($_POST) {
            $return = $seoM->delSeo(array('id' => intval($_POST['id'])));
            if ($return) {
                $this->seocache();
                $this->admin_json(0, yun_t('admin_model_00104', array('id' => $_POST['id'])));
            } else {
                $this->admin_json(1, yun_t('admin_model_00105', array('id' => $_POST['id'])));
            }
        }
    }

    /**
     * Refresh SEO cache.
     */
    function seocache()
    {
        include(LIB_PATH . "cache.class.php");
        $cacheclass = new cache(PLUS_PATH, $this->obj);
        $cacheclass->seo_cache("seo.cache.php");
    }
}

?>
