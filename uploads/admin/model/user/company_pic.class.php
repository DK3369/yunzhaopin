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
class company_pic_controller extends adminCommon
{
    /**
     * 会员-企业-认证&审核  企业LOGO审核
     */
//region 企业LOGO审核
    function index_action()
    {
        $pageM = $this->MODEL('page');
        $CompanyM = $this->MODEL('company');
        $where = array();
        $where['logo'] = array('<>', '');
        $keytype = intval($_POST['type']);
        $keyword = trim($_POST['keyword']);
        if (!empty($keyword)) {
            if ($keytype == 2) {
                $where['uid'] = array('=', $keyword);
            } else {
                $where['name'] = array('like', $keyword);
            }
        }
        if (isset($_POST['status'])) {
            $status = intval($_POST['status']);
            $where['logo_status'] = $status;
        }

        //提取分页
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('company', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $list = array();
        if ($pages['total'] > 0) {
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = array('logo_status,desc', 'uid,desc');
            }
            $where['limit'] = $pages['limit'];
            $ListNew = $CompanyM->getList($where, array('field' => 'uid,name,logo,logo_status', array('logo' => '2')));
            $list = $ListNew ? $ListNew['list'] : array();
        }



        $this->render_json(0, '', array('list' => $list, 'total' => intval($pages['total']),
            'perPage' => $pageSize, 'pageSizes' => $pageSizes,));
    }
    public function getLogoStatist_action(){
        //数据统计
        $CompanyM = $this->MODEL('company');
        $numAll = intval($CompanyM->getCompanyNum(array('logo' => array('<>', ''))));
        $numAudited = intval($CompanyM->getCompanyNum(array('logo' => array('<>', ''), 'logo_status' => 0)));
        $numUnaudited = intval($CompanyM->getCompanyNum(array('logo' => array('<>', ''), 'logo_status' => 1)));
        $data = array(
            'numAll' => $numAll,
            'numAudited' => $numAudited,
            'numUnaudited' => $numUnaudited,
        );
        $this->render_json(0,'',$data);
    }
    /**
     * @desc 企业LOGO审核
     */
    function status_action()
    {
        $companyM = $this->MODEL('company');
        $post = array(
            'logo_status' => intval($_POST['status']),
            'logo_statusbody' => $_POST['statusbody']
        );
        $return = $companyM->statusLogo($_POST['uid'], array('post' => $post));
        if ($return['errcode'] == '9' || $return['errcode'] === 0) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }

    /**
     * @desc 企业LOGO审核说明
     */
    function getStatusBody_action()
    {
        $companyM = $this->MODEL('company');
        $company = $companyM->getInfo(intval($_POST['uid']), array('field' => 'logo_statusbody'));
        $this->render_json(0, '', trim($company['logo_statusbody']));
    }
//endregion
//region 企业环境审核
    /**
     * @desc 后台企业 -   图片管理    -   企业环境
     */
    function show_action()
    {
        $companyM = $this->MODEL('company');
        $where = array();
        $keytype = intval($_POST['type']);
        $keyword = trim($_POST['keyword']);
        if ($_POST['comid']) {
            $where['uid'] = $_POST['comid'];
        }
        if (!empty($keyword)) {
            if ($keytype == 2) {
                $where['uid'] = array('=', $keyword);
            } else {
                $ListNew = $companyM->getList(array('name' => array('like', $keyword)), array('field' => 'uid'));
                $com = $ListNew['list'];
                foreach ($com as $v) {
                    $uid[] = $v['uid'];
                }
                $where['uid'] = array('in', pylode(',', $uid));
            }
        }
        if (isset($_POST['status'])) {
            $status = intval($_POST['status']);
            $where['status'] = $status;
        }
        //提取分页
        $pageM = $this->MODEL('page');
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('company_show', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $list = array();
        if ($pages['total'] > 0) {
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = array('status,desc', 'id,desc');
            }
            $where['limit'] = $pages['limit'];
            $list = $companyM->getCompanyShowList($where);
        }

        $this->render_json(0, '', array('list' => $list, 'total' => intval($pages['total']),
            'perPage' => $pageSize, 'pageSizes' => $pageSizes,));
    }
    public function getHjStatist_action(){
        //数据统计
        $companyM = $this->MODEL('company');
        $numAll = intval($companyM->getComShowNum());
        $numAudited = intval($companyM->getComShowNum(array('status' => 0)));
        $numUnaudited = intval($companyM->getComShowNum(array('status' => 1)));
        $data = array(
            'numAll' => $numAll,
            'numAudited' => $numAudited,
            'numUnaudited' => $numUnaudited,
        );
        $this->render_json(0,'',$data);
    }
    /**
     * @desc 企业环境审核说明
     */
    function getShowStatusBody_action()
    {
        $companyM = $this->MODEL('company');
        $comshow = $companyM->getCompanyShowInfo(intval($_POST['id']), array('field' => 'statusbody'));
        $this->render_json(0, '', trim($comshow['statusbody']));
    }

    /**
     * @desc 企业环境审核
     */
    function showStatus_action()
    {
        $CompanyM = $this->MODEL('company');
        $status = intval($_POST['status']);
        $statusbody = trim($_POST['statusbody']);
        $post = array(
            'status' => $status,
            'statusbody' => $statusbody
        );
        $return = $CompanyM->statusShow($_POST['sid'], array('post' => $post));
        if ($return['errcode'] == '9' || $return['errcode'] === 0) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }
//endregion
//region 企业横幅审核
    /**
     * @desc 后台企业 -   图片管理    -   企业横幅
     */
    function banner_action()
    {
        $companyM = $this->MODEL('company');
        $where = array();
        $keytype = intval($_POST['type']);
        $keyword = trim($_POST['keyword']);
        if (!empty($keyword)) {
            if ($keytype == 2) {
                $where['uid'] = array('=', $keyword);
            } else {
                $ListNew = $companyM->getList(array('name' => array('like', $keyword)), array('field' => 'uid'));
                $com = $ListNew['list'];
                foreach ($com as $v) {
                    $uid[] = $v['uid'];
                }
                $where['uid'] = array('in', pylode(',', $uid));
            }
        }

        if (isset($_POST['status'])) {
            $status = intval($_POST['status']);
            $where['status'] = $status;
        }
        //提取分页
        $pageM = $this->MODEL('page');
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('banner', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $list = array();
        if ($pages['total'] > 0) {
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = array('status,desc', 'id,desc');
            }
            $where['limit'] = $pages['limit'];
            $list = $companyM->getBannerList($where);
        }



        $this->render_json(0, '', array('list' => $list, 'total' => intval($pages['total']),
            'perPage' => $pageSize, 'pageSizes' => $pageSizes));
    }
    public function getBannerStatist_action(){
        //数据统计
        $companyM = $this->MODEL('company');
        $numAll = intval($companyM->getBannerNum());
        $numAudited = intval($companyM->getBannerNum(array('status' => 0)));
        $numUnaudited = intval($companyM->getBannerNum(array('status' => 1)));
        $data = array(
            'numAll' => $numAll,
            'numAudited' => $numAudited,
            'numUnaudited' => $numUnaudited,
        );
        $this->render_json(0,'',$data);
    }
    /**
     * @desc 企业横幅审核说明
     */
    function getBannerStatusBody_action()
    {
        $companyM = $this->MODEL('company');
        $company = $companyM->getBannerInfo(intval($_POST['id']), array('field' => 'statusbody'));
        $this->render_json(0, '', trim($company['statusbody']));

    }

    /**
     * @desc 企业环境横幅
     */
    function bannerStatus_action()
    {
        $CompanyM = $this->MODEL('company');
        $status = intval($_POST['status']);
        $statusbody = trim($_POST['statusbody']);
        $post = array(
            'status' => $status,
            'statusbody' => $statusbody
        );
        $return = $CompanyM->statusBanner($_POST['sid'], array('post' => $post));
        if ($return['errcode'] == '9' || $return['errcode'] === 0) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }
//endregion

    /**
     * @desc 后台  企业 修改 图片
     */
    function uploadsave_action()
    {
        $CompanyM = $this->MODEL('company');
        $_POST = $this->post_trim($_POST);
        $id = $_POST['id'];
        $UploadM = $this->MODEL('upload');
        if ($_POST['update']) {
            switch ($_POST['type']) {
                case 'logo':
                    $dir = 'company';
                    $watermark = 0;//watermark :0无视后台的水印开关不添加水印，只用于后台上传水印图片时，以及pc上传头像裁剪保存之前
                    break;
                case 'show':
                    $dir = 'show';
                    break;
                case 'banner':
                    $dir = 'company';
                    break;
            }

            if ($_FILES['file']['tmp_name']) {
                $upArr = array(
                    'file' => $_FILES['file'],
                    'dir' => $dir
                );

                $uploadM = $this->MODEL('upload');

                if (isset($watermark)) {
                    $upArr['watermark'] = $watermark;
                }
                $pic = $uploadM->newUpload($upArr);

                if (!empty($pic['msg'])) {

                    $this->render_json(8, $pic['msg']);

                } elseif (!empty($pic['picurl'])) {

                    $pictures = $pic['picurl'];
                }
            }

            if ($_POST['type'] == 'logo') {
                if (isset($pictures)) {
                    $nbid = $CompanyM->upLogo(array('uid' => $id), array('thumb' => $pictures, 'utype' => 'admin'));
                    $this->automsg('管理员操作：修改企业logo', $id);
                }
                isset($nbid) ? $this->admin_json(0, '企业logo(ID:' . $id . ')修改成功！') : $this->render_json(8, '修改失败！');
            }
            if ($_POST['type'] == 'show') {
                $row = $CompanyM->getCompanyShowInfo($id, array('field' => 'picurl,title,uid'));
                $data = array(
                    'sort' => $_POST['sort'],
                    'title' => $_POST['title'],
                    'ctime' => time()
                );
                if (isset($pictures)) {
                    $data['picurl'] = $pictures;
                }
                $nbid = $CompanyM->upCompanyShow($id, $data);
                $this->automsg('管理员：修改企业环境(ID:' . $id . ')', $row['uid']);
                isset($nbid) ? $this->admin_json(0, '企业环境(ID:' . $id . ')修改成功！') : $this->render_json(8, '修改失败！');
            }
            if ($_POST['type'] == 'banner') {
                $row = $CompanyM->getBannerInfo($id, array('field' => 'pic,uid'));
                if (isset($pictures)) {
                    $data['pic'] = $pictures;
                    $nbid = $CompanyM->upBanner($id, $data);
                    $this->automsg('管理员修改企业横幅', $row['uid']);
                }
                isset($nbid) ? $this->admin_json(0, '企业横幅(ID:' . $id . ')修改成功！') : $this->render_json(8, '修改失败！');
            }
        }
    }

    /**
     * @desc 后台 - 企业图片管理 - 删除
     */
    function del_action()
    {
        $CompanyM = $this->MODEL('company');
        if ($_POST['delid']) {
            $id = intval($_POST['delid']);
            if ($_POST['type'] == 'logo') {
                $delid = $CompanyM->upInfo($id, '', array('logo' => ''));
                $this->automsg('管理员 ：删除企业logo', $id);
                $delid ? $this->admin_json(0, '企业logo(ID:' . pylode(',', $_POST['delid']) . ')删除成功！') : $this->render_json(1, '删除失败！');
            }
            if ($_POST['type'] == 'show') {
                $row = $CompanyM->getCompanyShowInfo($id, array('field' => 'uid,picurl'));
                $delid = $CompanyM->delCompanyShow($id, array('utype' => 'admin'));
                $this->automsg('管理员：删除企业环境(ID:' . $id . ')', $row['uid']);
                $delid ? $this->admin_json(0, '企业环境(ID:' . pylode(',', $id) . ')删除成功！') : $this->render_json(1, '删除失败！');
            }
            if ($_POST['type'] == 'banner') { // 删除企业横幅
                $row = $CompanyM->getBannerInfo($id, array('field' => 'uid,pic'));
                $delid = $CompanyM->delBanner($id);
                $this->automsg('管理员：删除企业横幅', $row['uid']);
                $delid ? $this->admin_json(0, '企业横幅(ID:' . pylode(',', $id) . ')删除成功！') : $this->render_json(1, '删除失败！');
            }
        } else if (is_array($_POST['del'])) {
            if ($_POST['type'] == 'logo') { // 删除logo
                $row = $CompanyM->getList(array('uid' => array('in', pylode(',', $_POST['del'])), 'logo' => array('<>', '')), array('field' => 'uid,logo'));
                $delid = $CompanyM->upInfo($_POST['del'], '', array('logo' => ''));
                if ($row) {
                    foreach ($row as $v) {
                        $this->automsg('管理员操作：删除企业logo', $v['uid']);
                    }
                }
                $delid ? $this->admin_json(0, '企业logo(ID:' . pylode(',', $_POST['del']) . ')删除成功！') : $this->render_json(1, '删除失败！');
            }
            if ($_POST['type'] == 'show') {
                $ids = $_POST['del'];
                $row = $CompanyM->getCompanyShowList(array('id' => array('in', pylode(',', $ids)), 'picurl' => array('<>', '')), array('field' => 'picurl,id,uid'));
                $delid = $CompanyM->delCompanyShow($ids, array('utype' => 'admin'));
                if ($row) {
                    foreach ($row as $v) {
                        $this->automsg('管理员操作：删除企业环境(ID:' . $v['id'] . ')', $v['uid']);
                    }
                }
                $delid ? $this->admin_json(0, '企业环境(ID:' . pylode(',', $ids) . ')删除成功！') : $this->render_json(1, '删除失败！');
            }
            if ($_POST['type'] == 'banner') { // 删除企业横幅
                $ids = $_POST['del'];
                $row = $CompanyM->getBannerList(array('id' => pylode(',', $ids), 'pic' => array('<>', '')), array('field' => 'pic,uid'));
                $delid = $CompanyM->delBanner($ids);
                if ($row) {
                    foreach ($row as $v) {
                        $this->automsg('管理员操作：删除企业横幅', $v['uid']);
                    }
                }
                $delid ? $this->admin_json(0, '企业横幅(ID:' . pylode(',', $ids) . ')删除成功！') : $this->render_json(1, '删除失败！');
            }
        } else {
            $this->render_json(2, '请选择您要删除的图片！');
        }
    }
}
