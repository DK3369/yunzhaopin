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
class company_news_controller extends adminCommon
{
    function set_search()
    {
        $search_list[] = array("param" => "status", "name" => '审核状态', "value" => array("1" => "已审核", "3" => "未审核", "2" => "未通过"));
        $search_list[] = array("param" => "time", "name" => '发布时间', "value" => array('1' => '今天', '3' => '最近三天', '7' => '最近七天', '15' => '最近半月', '30' => '最近一个月'));
        return $search_list;
    }

    //企业新闻审核
    function index_action()
    {
        $companyM = $this->MODEL('company');
        if ($_POST['time']) {
            if ($_POST['time'] == '1') {
                $where['ctime'] = array('>=', strtotime('today')
                );
            } else {
                $where['ctime'] = array('>=', strtotime('-' . intval($_POST['time']) . ' day'));
            }
        }
        if ($_POST['status']) {
            $status = intval($_POST['status']);
            $where['status'] = $status == 3 ? 0 : $status;
        }
        if ($_POST['keyword']) {
            $keytype = intval($_POST['type']);
            $keyword = trim($_POST['keyword']);
            if ($keytype == 1) {
                $cwhere['name'] = array('like', $keyword);
                $ctList = $companyM->getList($cwhere, array('field' => 'uid'));
                $comapant = $ctList['list'];
                foreach ($comapant as $v) {
                    $comid[] = $v['uid'];
                }
                $where['uid'] = array('in', pylode(',', $comid));
            } elseif ($keytype == 2) {
                $where['title'] = array('like', $keyword);
            }
        }
        //提取分页
        $pageM = $this->MODEL('page');
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('company_news', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $list = array();
        if ($pages['total'] > 0) {
            // limit order 只有在列表查询时才需要
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = array('status,asc', 'id,desc');
            }
            $where['limit'] = $pages['limit'];
            $list = $companyM->getCompanyNewsList($where, array('utype' => 'admin', 'cache' => '1'));
        }
        $this->render_json(0, '', array('list' => $list, 'total' => intval($pages['total']),
            'perPage' => $pageSize, 'pageSizes' => $pageSizes));
    }
    public function getNewsStatist_action(){
        //数据统计
        $companyM = $this->MODEL('company');
        $numAll = intval($companyM->getCompanyNewsNum());
        $numAudited = intval($companyM->getCompanyNewsNum(array('status' => 1)));
        $numUnaudited = intval($companyM->getCompanyNewsNum(array('status' => 3)));
        $numFailed = intval($companyM->getCompanyNewsNum(array('status' => 2)));
        $data = array(
            'numAll' => $numAll,
            'numAudited' => $numAudited,
            'numUnaudited' => $numUnaudited,
            'numFailed' => $numFailed,
        );
        $this->render_json(0,'',$data);
    }
    function statusbody_action()
    {
        $CompanyM = $this->MODEL('company');
        $id = intval($_POST['id']);
        $info = $CompanyM->getCompanyNewsLockInfo($id, array('field' => 'statusbody'));
        $this->render_json(0, '', trim($info['statusbody']));
    }

    function status_action()
    {
        if (empty($_POST['status'])) {
            $this->render_json(8, "请选择审核状态！");
        }
        $CompanyM = $this->MODEL('company');
        $sysmsgM = $this->MODEL('sysmsg');
        $status = intval($_POST['status']);
        $data['status'] = $status;
        $data['statusbody'] = $_POST['statusbody'];
        $allid = @explode(',', $_POST['id']);
        if ($allid) {
            $nid = $CompanyM->upCompanyNewsStatus($allid, $data);
            $where['id'] = array('in', pylode(',', $allid));
            $Cnews = $CompanyM->getCompanyNewsList($where, array(
                'field' => 'uid,title'
            ));
            /* 消息前缀 */
            $tagName = '企业新闻';
            foreach ($Cnews as $v) {
                $uids[] = $v['uid'];
                /* 处理审核信息 */
                if ($data['status'] == 2) {
                    $statusInfo = $tagName . ':' . $v['title'] . '审核未通过';
                    if ($data['statusbody']) {
                        $statusInfo .= ' , 原因：' . $data['statusbody'];
                    }
                    $msg[$v['uid']][] = $statusInfo;
                } elseif ($data['status'] == 1) {
                    $msg[$v['uid']][] = $tagName . ':' . $v['title'] . '已审核通过';
                }
            }
            // 发送系统通知
            $sysmsgM->addInfo(array(
                'uid' => $uids,
                'usertype' => 2,
                'content' => $msg
            ));

            $nid ? $this->admin_json(0, "企业新闻审核(ID:" . pylode(',', $allid) . ")设置成功！") : $this->render_json(8, "设置失败！");
        } else {
            $this->render_json(8, "非法操作！");
        }
    }

    function del_action()
    {
        $CompanyM = $this->Model('company');
        $delID = is_array($_POST['del']) ? $_POST['del'] : $_POST['id'];
        $return = $CompanyM->delCompanyNews($delID, array(
            'uid' => $this->uid,
            'usertype' => $this->usertype
        ));
        if ($return['errcode'] == '9' || $return['errcode'] === 0) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json($return['errcode'], $return['msg']);
        }
    }
}