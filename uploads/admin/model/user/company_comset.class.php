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
class company_comset_controller extends adminCommon
{
    //会员-企业-企业设置

//region 企业设置
    function index_action()
    {
        $ratingM = $this->MODEL('rating');
        $qy_rows = $ratingM->getList(array('category' => 1, 'orderby' => array('sort,desc')));
        $comServerS = $ratingM->getComServiceList(array('display' => 1));
        $config = array(
            //----审核信息
            //企业会员
            'com_status' => $this->config['com_status'],
            //发布职位
            'com_job_status' => $this->config['com_job_status'],
            //发布兼职
            'com_partjob_status' => $this->config['com_partjob_status'],
            //企业资质
            'com_cert_status' => $this->config['com_cert_status'],
            //企业LOGO
            'com_logo_status' => $this->config['com_logo_status'],
            //企业环境
            'com_show_status' => $this->config['com_show_status'],
            //企业横幅
            'com_banner_status' => $this->config['com_banner_status'],
            //企业修改
            'com_revise_status' => $this->config['com_revise_status'],
            
            //邀请面试模板
            'com_yqmb_status' => $this->config['com_yqmb_status'],
            //----强制操作
            //信息完善
            'com_enforce_info' => $this->config['com_enforce_info'],
            //手机认证
            'com_enforce_mobilecert' => $this->config['com_enforce_mobilecert'],
            //邮箱认证
            'com_enforce_emailcert' => $this->config['com_enforce_emailcert'],
            //企业资质
            'com_enforce_licensecert' => $this->config['com_enforce_licensecert'],
            //地理位置
            'com_enforce_setposition' => $this->config['com_enforce_setposition'],
            //关注微信公众号
            'com_gzgzh' => $this->config['com_gzgzh'],
            //----企业资质上传项
            //统一社会信用代码
            'com_social_credit' => $this->config['com_social_credit'],
            //经办人身份证件
            'com_cert_owner' => $this->config['com_cert_owner'],
            //承诺函
            'com_cert_wt' => $this->config['com_cert_wt'],
            //其他材料
            'com_cert_other' => $this->config['com_cert_other'],
            //委托书/承诺函范本
            'exa_cert_wt' => $this->config['exa_cert_wt'],
            //----功能开关
            //求职咨询
            'com_message' => $this->config['com_message'],
            
            //薪资面议
            'com_job_myswitch' => $this->config['com_job_myswitch'],
            //性别选项
            'com_job_sexswitch' => $this->config['com_job_sexswitch'],
            //认证企业职位免审核
            'com_free_status' => $this->config['com_free_status'],
            //招聘数据
            'com_zpdata' => isset($this->config['com_zpdata']) ? $this->config['com_zpdata'] : 1, // 默认开启
            //----职位刷新
            //职位刷新
            'com_job_reserve' => $this->config['com_job_reserve'],
            //预约刷新时间间隔
            'sy_reserve_refresh_interval' => $this->config['sy_reserve_refresh_interval'],
            //预约刷新消费套餐
            'sy_reserve_refresh_price' => $this->config['sy_reserve_refresh_price'],
            //刷新职位增值服务
            'sy_reserve_service_id' => $this->config['sy_reserve_service_id'],
            //----人才搜索
            //已登录会员
            'com_search' => $this->config['com_search'],
            //已审核会员
            'com_status_search' => $this->config['com_status_search'],
            //----人才下载
            //在招职位
            'com_lietou_job' => $this->config['com_lietou_job'],
            //企业搜索器
            'com_finder' => $this->config['com_finder'],
            //邀请面试模板数
            'com_yqmb_num' => $this->config['com_yqmb_num'],
            //会员到期提醒
            'sy_maturityday' => $this->config['sy_maturityday'],
            //展示职位投递数限制
            'sy_sq_job_num' => $this->config['sy_sq_job_num'],
            // 职位置顶（移动端首页）
            'joblist_top_index' => $this->config['joblist_top_index'],
            //职位列表置顶
            'joblist_top' => $this->config['joblist_top'],
            //职位名称锁定
            'joblock' => $this->config['joblock'],
            //职位投递要求限制
            'sqjob_req' => $this->config['sqjob_req'],
            //名企排序
            'hotcom_top' => $this->config['hotcom_top'],
            //会员到期职位下架
            'jobunder' => $this->config['jobunder'],
            //职位下架延期设置
            'job_under_delay' => $this->config['job_under_delay'],
            //查看企业联系方式
            'com_link_look' => $this->config['com_link_look'],
            'com_login_link' => $this->config['com_login_link'],
            //屏蔽企业联系方式
            'com_link_no' => $this->config['com_link_no'],
            //联系方式提示语
            'sy_link_tips' => $this->config['sy_link_tips'],
            //职位全文搜索
            'job_full_text_search' => $this->config['job_full_text_search'],
        );
        $this->render_json(0, '', array(
            'config' => $config ?: (object)[],
            'qy_rows' => $qy_rows,
            'com_servers' => $comServerS,
            'com_link_no' => isset($this->config['com_link_no']) && strlen(trim($this->config['com_link_no'])) ? @explode(',', $this->config['com_link_no']) : [],
        ));
    }

    function save_action()
    {
        if ($_POST["config"]) {
            unset($_POST["config"]);
            unset($_POST['pytoken']);
            //region 企业设置
            //委托书/承诺函范本
            if ($_FILES['exa_cert_wt_files']) {
                $upArr = array(
                    'file' => $_FILES['exa_cert_wt_files'],
                    'dir' => 'comdoc'
                );
                $uploadM = $this->MODEL('upload');
                $result = $uploadM->uploadDoc($upArr);
                if ($result['msg']) {
                    $this->render_json(8, $result['msg']);
                } else {
                    $_POST['exa_cert_wt'] = $result['docurl'];
                }
            }
            //endregion
            //region 套餐设置
            //强制金额消费项目
            if (isset($_POST['sy_only_price'])) {
                $_POST['sy_only_price'] = $_POST['sy_only_price'] ? @implode(',', $_POST['sy_only_price']) : '';
            }
            //单独购买
            if (isset($_POST['com_single_can'])) {
                $_POST['com_single_can'] = $_POST['com_single_can'] ? @implode(',', $_POST['com_single_can']) : '';
            }
            //会员套餐开放控制
            if (isset($_POST['com_package_open'])) {
                $_POST['com_package_open'] = $_POST['com_package_open'] ? @implode(',', $_POST['com_package_open']) : '';
            }
            //endregion
            $configM = $this->MODEL("config");
            $configM->setConfig($_POST);
            $this->web_config();
            $this->admin_json(0, '企业设置配置修改成功！');
        }
    }
//endregion
//region 头像设置
    function logo_action()
    {
        if ($_POST['submit']) {
            $this->web_config();
            $this->admin_json(0, '会员头像配置设置成功！');
        }
        $config = array(
            //默认LOGO
            'sy_unit_icon' =>checkpic($this->config['sy_unit_icon']) ,
            //默认顾问头像
            'sy_guwen' => checkpic($this->config['sy_guwen']),
            //默认企业横幅
            'sy_banner' => checkpic($this->config['sy_banner']),
            //未上传二维码提示图
            'sy_member_ewm' => checkpic($this->config['sy_member_ewm']),
        );
        $this->render_json(0, '', array('config' => $config));
    }
//endregion
//region 套餐设置
    function rating_action()
    {
        $need = $_POST['need'];

        $config = array();
        if ($need == 'all') {
            $config = array(
                'integral_pricename' => $this->config['integral_pricename'],
                
                //企业会员模式
                'com_vip_type' => $this->config['com_vip_type'],
                //套餐消费模式
                'com_integral_online' => $this->config['com_integral_online'],
                //可单独购买
                'com_single_can' => isset($this->config['com_single_can']) && strlen(trim($this->config['com_single_can'])) ? @explode(',', $this->config['com_single_can']) : [],
                //强制金额消费项目
                'sy_only_price' => isset($this->config['sy_only_price']) && strlen(trim($this->config['sy_only_price'])) ? @explode(',', $this->config['sy_only_price']) : [],
                //职位推广取消
                'tg_back' => $this->config['tg_back'],
                //购买会员套餐累加
                'rating_add' => isset($this->config['rating_add']) && strlen(trim($this->config['rating_add'])) ? @explode(',', $this->config['rating_add']) : [],
                //会员职位免审核
                'job_ms_rating' => isset($this->config['job_ms_rating']) && strlen(trim($this->config['job_ms_rating'])) ? @explode(',', $this->config['job_ms_rating']): [],
                //会员套餐开放控制
                'com_package_open' => isset($this->config['com_package_open']) && strlen(trim($this->config['com_package_open'])) ? @explode(',', $this->config['com_package_open']) : [],
                //企业注册默认等级
                'com_rating' => $this->config['com_rating'],
                //会员到期后默认为
                'com_vip_done' => $this->config['com_vip_done'],
            );
        }
        $rating = $this->MODEL('rating');
        $qy_rows = $rating->getList(array('category' => 1, 'orderby' => array('sort,desc')));
        $expireVip  =   array('id' => '999', 'name' => '过期会员');
        $tcPackage  =   $qy_rows;
        $tcPackage[]=   $expireVip;
        $this->render_json(0, '', array(
            'config' => $config,
            'qy_rows' => $qy_rows,
            'tcPackage' => $tcPackage,//会员套餐开放控制
        ));
    }
//endregion

//region 消费设置
    function comspend_action()
    {
        $config = array(
            'integral_pricename' => $this->config['integral_pricename'],
            'integral_proportion' => $this->config['integral_proportion'],
            'integral_priceunit' => $this->config['integral_priceunit'],
            //上架职位
            'integral_job' => $this->config['integral_job'],
            //下载人才简历
            'integral_down_resume' => $this->config['integral_down_resume'],
            //邀请人才面试
            'integral_interview' => $this->config['integral_interview'],
            //刷新职位
            'integral_jobefresh' => $this->config['integral_jobefresh'],
            //发布紧急招聘
            'com_urgent' => $this->config['com_urgent'],
            //发布置顶职位
            'integral_job_top' => $this->config['integral_job_top'],
            //发布推荐招聘
            'com_recjob' => $this->config['com_recjob'],
            //职位自动刷新
            'job_auto' => $this->config['job_auto'],
        );
        //下载人才简历定价
        $configM = $this->MODEL('config');
        $row = $configM->getInfo(array('name' => 'integral_down_resume_dayprice'));
        $marr = explode(':', $row['config']);
        foreach ($marr as $v) {
            $narr = explode('_', $v);
            $data[] = array('days' => $narr[0], 'price' => $narr[1]);
        }
        $this->render_json(0, '', array('config' => $config, 'data' => $data));
    }
    function saveComspend_action(){
        if ($_POST["config"]) {
            unset($_POST["config"]);
            unset($_POST['pytoken']);
            $configM = $this->MODEL("config");
            $configM->setConfig($_POST);
            $this->web_config();
            $this->admin_json(0, '消费设置配置修改成功！');
        }
    }
//endregion
}