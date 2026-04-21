<?php

/**
 * Class domain_controller
 *
 * @author shiy
 * @version 7.0
 */
class domain_list_controller extends adminCommon
{
    //  分站列表
    function index_action()
    {
        
        $siteM  =   $this->MODEL('site');
        
        $where  =   array();
        
        $keywordStr =   trim($_POST['keyword']);
        if (isset($keywordStr) && !empty($keywordStr)) {
            
            $where['title'] =   array('like', $keywordStr);
        }
        
        $page       =   !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize   =   !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        
        $pageM      =   $this->MODEL('page');
        $pages      =   $pageM->adminPageList('domain', $where, $page, array('limit' => $pageSize, 'maxPage' => true));
        
        $list   =   array();
        $total  =   intval($pages['total']);
        if ($total > 0) {
            if (isset($_POST['order'])) {
                
                $where['orderby']   =   $_POST['t']. ' ,'. $_POST['order'];
            } else {
                
                $where['orderby']   =   'id';
            }
            
            $where['limit'] =   $pages['limit'];
            $list   =   $siteM->getList($where);
        }
        
        $result =   array(
            
            'list'  =>  $list,
            'total' =>  intval($total),
            'pageSize'  =>  $pageSize,
            'pageSizes' =>  $pages['page_sizes']
        );
        $this->render_json(0, '', $result);
    }
    //  分站详细信息
    function domainInfo_action()
    {
        
        $siteM  =   $this->MODEL('site');
        $info   =   $siteM->getInfo(array('id' => $_POST['id']));
        $this->render_json(0,'', $info);
    }
    //  更新分站缓存
    private function DomainArr()
    {

        include(LIB_PATH.'cache.class.php');
        $cacheclass =   new cache(PLUS_PATH, $this->obj);
        $cacheclass->domain_cache('domain_cache.php');
    }

    //  分站配置默认参数
    function config_action()
    {

        $this->render_json(0, '', array(
            'sy_web_site'   =>  $this->config['sy_web_site'],
            'sy_gotocity'   =>  $this->config['sy_gotocity'],
            'sy_indexcity'  =>  $this->config['sy_indexcity'],
            'sy_indexdomain'=>  $this->config['sy_indexdomain'],
            'sy_onedomain'  =>  $this->config['sy_onedomain']
        ));
    }

    //  提交分站配置
    function configSave_action()
    {

        if (isset($_POST['domainConfig']) && intval($_POST['domainConfig']) == 1){

            $_POST  =   $this->post_trim($_POST);
            if (!empty($_POST['sy_indexdomain'])) {
                if (stripos($_POST['sy_indexdomain'], 'http') === false) {
                    if (stripos($this->config['sy_weburl'], 'https://') !== false) {

                        $protocol = 'https://';
                    } else {

                        $protocol = 'http://';
                    }

                    $_POST['sy_indexdomain']    =   $protocol.$_POST['sy_indexdomain'];
                }
            }

            $data['sy_web_site']    =   $_POST['sy_web_site'];
            $data['sy_gotocity']    =   $_POST['sy_gotocity'];
            $data['sy_indexcity']   =   $_POST['sy_indexcity'];
            $data['sy_indexdomain'] =   $_POST['sy_indexdomain'];
            $data['sy_onedomain']   =   $_POST['sy_onedomain'];

            $configM=   $this->MODEL('config');
            $configM->setConfig($data);
            $this->web_config();
            $this->admin_json(0, '分站设置成功');
        }
    }
    //  开启/关闭分站
    function changeDomainType_action()
    {

        if ($_POST['id'] && $_POST['type']) {

            $this->MODEL('site')->upInfo(array('type' => $_POST['type']), array('id' => $_POST['id']));
            $this->DomainArr();

            $msg    =   $_POST['type'] == 1 ? '分站已启用' : '分站已关闭';
            $this->render_json(0, $msg);
        }else{

            $this->render_json(1, '参数错误，请重试！');
        }
    }

    //  删除分站信息
    function delDomain_action()
    {

        $siteM = $this->MODEL('site');
        $delId = $_POST['id'];
        if (!empty($delId)) {

            $return = $siteM->delDomain($delId);

            if ($return['error'] == 0) {

                $this->DomainArr();
                $this->admin_json(0, $return['msg']);
            } else {

                $this->admin_json(1, $return['msg']);
            }
        } else {

            $this->render_json(1, '参数错误，请重试！');
        }
    }

    //  添加/修改分站，所用缓存数据（城市、行业、分站目录）
    function getDomainCache_action()
    {

        $cacheM =   $this->MODEL('cache');

        $cache  =   $cacheM->GetCache(array('city', 'hy'));

        $cityNameArr        =   $cache['city_name'];
        foreach ($cache['city_index'] as $k=>$v) {
            $provinceArr[]  =   array('id' => $v, 'name' => $cityNameArr[$v]);
        }
        foreach ($cache['city_type'] as $k => $cityV){
            foreach ($cityV as $v){
                $cityArr[]  =   array('id' => $v, 'pid' => $k, 'name' => $cityNameArr[$v]);
            }
        }

        $industryNameArr    =   $cache['industry_name'];
        foreach ($cache['industry_index'] as $k => $v){
            $industryArr[]  =   array('id' => $v, 'name' => $industryNameArr[$v]);
        }

        include_once('model/style_class.php');
        $style      =   new style($this->obj);
        $styleList  =   $style->model_list_action();

        $dataArr    =   array(

            'styleList'     =>  $styleList,
            'industryArr'   =>  $industryArr,
            'provinceArr'   =>  $provinceArr,
            'cityArr'       =>  $cityArr,
            'picMaxSize'    =>  $this->config['pic_maxsize'] ? $this->config['pic_maxsize'] : 5,
            'picType'       =>  $this->config['pic_type'] ? $this->config['pic_type'] : 'jpg,png,jpeg,bmp,gif'
        );

        $this->render_json(0, '', $dataArr);
    }

    //  添加/修改分站信息
    function saveDomain_action()
    {

        if ($_POST['title'] == '' || ($_POST['domain'] == '' && $_POST['indexdir'] == '') || ($_POST['province'] == '' && $_POST['hy'] == '')){

            $this->render_json(1, '信息不完整，请重新填写！');
        }
        if (isset($_POST['domain']) && $_POST['domain'] != '') {

            $domain =   @str_replace(array('http://', 'https://'), '', $_POST['domain']);
            if ($domain == str_replace(array('http://', 'https://'), '', $this->config['sy_weburl'])) {

                $this->render_json(1, '分站域名和总站域名一致，请重新填写！');
            }
        }

        $fzType =   intval($_POST['fz_type']);
        if ($fzType == 1) {

            $_POST['hy']    =   '';
        } else {

            $_POST['province']      =   '';
            $_POST['cityid']        =   '';
            $_POST['three_cityid']  =   '';
        }

        if ($_FILES['file']['tmp_name'] != '') {
            $upArr = array(
                'file' => $_FILES['file'],
                'dir' => 'logo'
            );

            $uploadM = $this->MODEL('upload');

            $pic = $uploadM->newUpload($upArr);
            if (!empty($pic['msg'])) {

                $this->render_json(1, $pic['msg']);
            } elseif (!empty($pic['picurl'])) {

                $_POST['weblogo'] = $pic['picurl'];
            }
        }

        $siteM  =   $this->MODEL('site');

        if ((int)$_POST['mode'] == 1) {
            $whereData  =   array('domain' => $_POST['domain']);
        } else {
            $whereData  =   array('indexdir' => $_POST['indexdir']);
        }
        if (isset($_POST['id'])){
            $whereData['id']    =   array('<>', $_POST['id']);
        }
        $domain_list    =   $siteM->getInfo($whereData);
        if (is_array($domain_list)) {
            $this->render_json(1, '该域名已经被绑定!');
        }

        if (isset($_POST['id'])){

            $result =   $siteM->addInfo($_POST, array('id' => $_POST['id']));
            $this->DomainArr();
            $this->admin_json(0, '分站（ID：'.$_POST['id'].'）更新成功！');
        }else{

            $result =   $siteM->addInfo($_POST);
            $this->DomainArr();
            $this->admin_json(0, '分站（ID：'.$result.'）创建成功！');
        }
    }
}
?>