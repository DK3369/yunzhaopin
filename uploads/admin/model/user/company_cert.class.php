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
class company_cert_controller extends adminCommon
{
    /**
     * 企业认证审核列表
     */
    function index_action()
    {
        $pageM = $this->MODEL('page');
        $companyM = $this->MODEL('company');
        $where = array();
        //企业认证
        $where['type'] = 3;
        if ($_POST['status']) {
            $status = intval($_POST['status']);
            $where['status'] = $status == 3 ? 0 : $status;
        }
        if ($_POST['keyword']) {
            $keyword = trim($_POST['keyword']);
            $cwhere['name'] = array('like', $keyword);
            $ctList = $companyM->getList($cwhere, array('field' => 'uid'));
            $comapant = $ctList['list'];
            $comids = array();
            foreach ($comapant as $v) {
                $comids[] = $v['uid'];
            }
            $where['uid'] = array('in', pylode(',', $comids));
        }
        if ($_POST['end']) {
            if ($_POST['end'] == '1') {
                $where['ctime'] = array('>=', strtotime('today'));
            } else {
                $where['ctime'] = array('>=', strtotime('-' . intval($_POST['end']) . ' day'));
            }
        }
        //提取分页
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('company_cert', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $list = array();
        // 分页数大于0的情况下 执行列表查询
        if ($pages['total'] > 0) {
            // limit order 只有在列表查询时才需要
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = array('status,asc', 'id,desc');
            }
            $where['limit'] = $pages['limit'];
            $list = $companyM->getCertList($where, array('utype' => 'comcert'));
        }
        $this->render_json(0, '', array('list' => $list, 'total' => intval($pages['total']),
            'perPage' => $pageSize, 'pageSizes' => $pageSizes,
        ));
    }
    public function getCertStatist_action(){
        //数据统计
        $MsgNum = $this->MODEL('msgNum');
        $comCertNum = $MsgNum->comCertNum(true);
        $comCertAll = $comCertNum['comCertAll'] ? $comCertNum['comCertAll'] : 0;
        $comCert1 = $comCertNum['comCert1'] ? $comCertNum['comCert1'] : 0;
        $comCert2 = $comCertNum['comCert2'] ? $comCertNum['comCert2'] : 0;
        $data = array(
            'comCertAll' => $comCertAll,
            'comCert1' => $comCert1,
            'comCert2' => $comCert2,
        );
        $this->render_json(0,'',$data);
    }
    public function getConfigData_action(){
        $config = array();
        //统一社会信用代码
        $com_social_credit = intval($this->config['com_social_credit']);
        //经办人身份证
        $com_cert_owner = intval($this->config['com_cert_owner']);
        //委托书/承诺函
        $com_cert_wt = intval($this->config['com_cert_wt']);
        //其他材料
        $com_cert_other = intval($this->config['com_cert_other']);
        $config['com_social_credit'] = $com_social_credit;
        $config['com_cert_owner'] = $com_cert_owner;
        $config['com_cert_wt'] = $com_cert_wt;
        $config['com_cert_other'] = $com_cert_other;

        $this->render_json(0,'',$config);
    }
    function sbody_action()
    {
        $CompanyM = $this->MODEL('company');
        $where['type'] = 3;
        $where['uid'] = intval($_POST['uid']);
        $info = $CompanyM->getCertInfo($where, array('field' => 'statusbody'));
        $this->render_json(0, '', trim($info['statusbody']));
    }

    /**
     * @desc 后台     -   企业认证审核
     */
    function status_action()
    {
        // 引用compayorder中MODEL类文件
        $companyorder = $this->MODEL('companyorder');
        $companyM = $this->MODEL('company');
        if ($_POST['status'] == '') {
            $this->render_json(1, '请选择审核状态！');
        }
        if ($_POST['uid']) {
            $uid = $_POST['uid'];
            $status = intval($_POST['status']);
            $comids = @explode(',', $uid);
            if ($status != 1) {
                $yyzz_status = 2;
            } else {
                $yyzz_status = 1;
                // 如果是“审核通过”，判断之前是否有过“审核通过的记录”，没有则增加企业资质审核通过的积分（只有第一次审核通过才加积分）
                if (count($comids) > 1) {
                    $paywhere['com_id'] = array('in', pylode(',', $comids));
                    $paywhere['pay_remark'] = '认证企业资质';
                    $companypay = $companyorder->getPayList($paywhere, array('field' => 'com_id'));
                    foreach ($companypay as $k => $v) {
                        if (in_array($v, $uid)) {
                            unset($uid[$k]);
                        }
                    }
                    foreach ($uid as $v) {
                        $this->MODEL('integral')->invtalCheck($v, 2, 'integral_comcert', '认证企业资质');
                    }
                } elseif ($uid != '') {
                    $paywhere['com_id'] = $uid;
                    $paywhere['pay_remark'] = '认证企业资质';
                    $num = $companyorder->getCompanyPayNum($paywhere);
                    if ($num < 1) {
                        $this->MODEL('integral')->invtalCheck($uid, 2, 'integral_comcert', '认证企业资质');
                    }
                }
            }

            $companycertData = array(
                'status' => $_POST['status'],
                'statusbody' => $_POST['statusbody']
            );

            $id = $companyM->upCertInfo(array('type' => '3', 'uid' => array('in', pylode(',', $uid))), $companycertData, array('utype' => 'admin'));
            // 审核通过时，职位免审核开启，管理勾选同步审核职位，未审核职位同步审核成功
            $jobM = $this->MODEL('job');
            if ($this->config['com_free_status'] == '1' && $_POST['job_status'] && $yyzz_status == 1) {
                $jobM->upInfo(array('state' => '1', 'r_status' => 1), array('state' => '0', 'uid' => array('in', pylode(',', $uid))));
                // 同步将企业设为已审核
                $companyData['r_status'] = 1;
            }
            // 修改审核状态
            $companyData['yyzz_status'] = $yyzz_status;
            if($_POST['name'] && count($comids)==1){
                $companyData['name'] = $_POST['name'];
            }
			$certname=$companyM->getCompanyInfo(array('name'=>$_POST['name'],'uid'=>array('<>',$uid)),array('field'=>'`uid`,`name`'));
			if($certname['name']){
				$this->admin_json(1, '企业名称已被使用');
				
			}
            $companyM->upInfo($comids, '', $companyData);

            if($_POST['name'] && count($comids)==1){

                $companyM->comNameSync($uid);
            }
            $jobM->upInfo(array('yyzz_status' => $yyzz_status), array('uid' => array('in', pylode(',', $uid))));
            $ComA = $companyM->getList(array('uid' => array('in', pylode(',', $uid))), array('field' => 'uid,name,linkmail'));
            $company = $ComA['list'];
            if ($this->config['sy_email_set'] == '1') {
                if (is_array($company)) {
                    $notice = $this->MODEL('notice');
                    foreach ($company as $v) {
                        if ($this->config['sy_email_comcert'] == '1' && $_POST['status'] > 0) {
                            if ($_POST['status'] == '1') {
                                $certinfo = '企业资质审核通过！';
                            } else {
                                $certinfo = '企业资质审核未通过！';
                            }
                            $notice->sendEmailType(array(
                                'email' => $v['linkmail'],
                                'certinfo' => $certinfo,
                                'comname' => $v['name'],
                                'uid' => $v['uid'],
                                'name' => $v['name'],
                                'type' => 'comcert'
                            ));
                        }
                    }
                }
            }

            /* 消息前缀 */
            foreach ($company as $v) {
                $uids[] = $v['uid'];
                /* 处理审核信息 */
                if ($_POST['status'] == 2) {
                    $statusInfo = '很遗憾 , 贵公司企业资质未能通过审核';
                    if ($_POST['statusbody']) {
                        $statusInfo .= ' , 原因：' . $_POST['statusbody'];
                    }
                    $msg[$v['uid']] = $statusInfo;
                } elseif ($_POST['status'] == 1) {
                    $msg[$v['uid']] = '贵公司企业资质审核通过，招聘人才更轻松！';
                }
            }
            //发送系统通知
            $sysmsgM = $this->MODEL('sysmsg');
            $sysmsgM->addInfo(array('uid' => $uids, 'usertype' => 2, 'content' => $msg));
            if ($id) {
                $this->admin_json(0, '企业资质审核(UID:' . $uid . ')设置成功！');
            } else {
                $this->render_json(3, '设置失败！');
            }
        } else {
            $this->render_json(2, '非法操作！');
        }
    }

    /**
     * @desc 删除企业认证
     */
    function del_action()
    {
        $companyM = $this->MODEL('company');
        if (is_array($_POST['del'])) {
            $linkid = $_POST['del'];
        } else {
            $linkid = $_POST['uid'];
        }
        $companyM->upInfo($linkid, '', array('yyzz_status' => 0));
        $err = $companyM->delCert($linkid, array('type' => '3'));
        if($err['errcode'] == 9){
            $this->admin_json(0, $err['msg']);
        } else {
            $this->render_json($err['errcode'], $err['msg']);
        }
    }
}