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
class users_userset_controller extends adminCommon
{
    //会员-个人-个人设置
//region 会员-个人-个人设置：个人设置
    function index_action()
    {
        $return = array();
        $return['config'] = array(
            //----信息审核
            //优质简历审核
            'user_height_resume' => $this->config['user_height_resume'],
            //身份证审核
            'user_idcard_status' => $this->config['user_idcard_status'],
            //求职咨询审核
            'user_msg_status' => $this->config['user_msg_status'],
            //头像审核
            'user_photo_status' => $this->config['user_photo_status'],
            //作品审核
            'rshow_photo_status' => $this->config['rshow_photo_status'],
            //委托简历审核
            'user_trust_status' => $this->config['user_trust_status'],


            //----简历审核
            //创建简历审核
            'resume_status' => $this->config['resume_status'],
            //修改简历审核
            'user_revise_state' => $this->config['user_revise_state'],
            'resume_statetime_start' => $this->config['resume_statetime_start'],
            'resume_statetime_end' => $this->config['resume_statetime_end'],


            //----强制操作
            //创建简历
            'user_resume_status' => $this->config['user_resume_status'],
            //关注微信公众号
            'user_gzgzh' => $this->config['user_gzgzh'],
            //快速投递
            'resume_kstd' => $this->config['resume_kstd'],


            //简历创建必填项
            //工作经历
            'resume_create_exp' => $this->config['resume_create_exp'],
            //教育经历
            'resume_create_edu' => $this->config['resume_create_edu'],
            //项目经历
            'resume_create_project' => $this->config['resume_create_project'],
            'expcreate' => $this->config['expcreate'],
            'educreate' => $this->config['educreate'],
            'sy_resume_job_classid' => $this->config['sy_resume_job_classid'],
            //跨职位类别投递，是否要判断完整度
            'sy_resume_kh_td' => $this->config['sy_resume_kh_td'],
            //快速投递，是否遵从简历必填项
            'resume_kstd_req' => $this->config['resume_kstd_req'],


            //申请职位要求简历完整度达到
            'user_sqintegrity' => $this->config['user_sqintegrity'],


            //----简历置顶要求
            //工作经历
            'user_work_regiser' => $this->config['user_work_regiser'],
            //教育经历
            'user_edu_regiser' => $this->config['user_edu_regiser'],
            //项目经历
            'user_project_regiser' => $this->config['user_project_regiser'],


            //简历求职意向字数
            'sy_rname_num' => $this->config['sy_rname_num'],
            //个人会员发布简历数
            'user_number' => $this->config['user_number'],
            //个人搜索器数量
            'user_finder' => $this->config['user_finder'],
            //个人会员向网站委托简历数
            'user_trust_number' => $this->config['user_trust_number'],
            //姓名展示
            'user_name' => $this->config['user_name'],
            //个人简历头像
            'user_pic' => $this->config['user_pic'],
            //个人简历刷新类型
            'resume_sx' => $this->config['resume_sx'],
            //简历模糊化
            'resume_open_check' => $this->config['resume_open_check'],
            //个人用户访问简历权限
            'sy_user_visit_resume' => $this->config['sy_user_visit_resume'],
            //待审核简历可以投递
            'sy_shresume_applyjob' => $this->config['sy_shresume_applyjob'],
            //拥有简历才可报名兼职
            'com_resume_partapply' => $this->config['com_resume_partapply'],
            //简历姓名限制
            'sy_resumename_num' => $this->config['sy_resumename_num'],
            //简历重复投递周期
            'sq_resume_interval' => $this->config['sq_resume_interval'],
        );
        //处理默认选中值
        $cache = $this->MODEL('cache')->GetCache(array('user', 'job'));
        $jobname = $cache['job_name'];
        $jobclassids = explode(',', $this->config['sy_resume_job_classid']);
        $selected = array();
        foreach ($jobclassids as $k=>$v){
            $selected[$v] = $jobname[$v];
        }
        $return['selected'] = $selected;
        $this->render_json(0, '', $return);
    }

    function indexBaseData_action()
    {
        $cache = $this->MODEL('cache')->GetCache(array('user', 'job'));
        $this->render_json(0, '', $cache);
    }

    function save_action()
    {
        if ($_POST['config']) {
            $post = array(
                'user_number' => $_POST['user_number'],
                'user_finder' => $_POST['user_finder'],
                'sy_rname_num' => $_POST['sy_rname_num'] ? $_POST['sy_rname_num'] : 10,
                'user_work_regiser' => $_POST['user_work_regiser'],
                'user_edu_regiser' => $_POST['user_edu_regiser'],
                'user_project_regiser' => $_POST['user_project_regiser'],
                //TODO
                //简历完善度达到80%以上验证，是否开启 1是开启 0是关闭  暂时不需要
                //简历完整度字段名称为：user_integrity_eighty
                //'user_integrity_eighty' => $_POST['user_integrity_eighty'],
                'user_trust_number' => $_POST['user_trust_number'],
                'user_revise_state' => $_POST['user_revise_state'],
                'resume_status' => $_POST['resume_status'],
                'resume_open_check' => $_POST['resume_open_check'],
                'user_resume_status' => $_POST['user_resume_status'],
                'resume_kstd' => $_POST['resume_kstd'],
                'user_gzgzh' => $_POST['user_gzgzh'],
                'resume_statetime_start' => $_POST['resume_statetime_start'],
                'resume_statetime_end' => $_POST['resume_statetime_end'],
                'user_height_resume' => $_POST['user_height_resume'],
                'user_idcard_status' => $_POST['user_idcard_status'],
                'user_msg_status' => $_POST['user_msg_status'],
                'com_resume_partapply' => $_POST['com_resume_partapply'],
                'resume_sx' => $_POST['resume_sx'],
                'user_trust_status' => $_POST['user_trust_status'],
                'user_photo_status' => $_POST['user_photo_status'],
                'rshow_photo_status' => $_POST['rshow_photo_status'],
                'user_name' => $_POST['user_name'],
                'user_pic' => $_POST['user_pic'],
                'user_sqintegrity' => $_POST['user_sqintegrity'],
                'resume_create_edu' => $_POST['resume_create_edu'],
                'resume_create_exp' => $_POST['resume_create_exp'],
                'resume_create_project' => $_POST['resume_create_project'],
                'educreate' => $_POST['educreate'],
                'expcreate' => $_POST['expcreate'],
                'sy_user_visit_resume' => $_POST['sy_user_visit_resume'],
                'sy_shresume_applyjob' => $_POST['sy_shresume_applyjob'],
                'sy_resumename_num' => $_POST['sy_resumename_num'],
                'sy_resume_job_classid' => $_POST['sy_resume_job_classid'],
                'sq_resume_interval' => $_POST['sq_resume_interval'],
                'sy_resume_kh_td' => $_POST['sy_resume_kh_td'],
                'resume_kstd_req' => $_POST['resume_kstd_req'],
            );
            $configM = $this->MODEL('config');
            $configM->setConfig($post);
            $this->web_config();
            $this->admin_json(0, '个人设置配置修改成功');
        }
    }
//endregion
//region 会员-个人-个人设置: 头像设置
    /**
     * 会员-个人-个人设置: 头像设置
     */
    function logo_action()
    {
        $curl = $this->config['sy_ossurl'];
        $config = array(
            //默认男生头像
            'sy_member_icon_arr' => $this->config['sy_member_icon_arr'] ? $this->config['sy_member_icon_arr'] : [],
            //默认女生头像
            'sy_member_iconv_arr' => $this->config['sy_member_iconv_arr'] ? $this->config['sy_member_iconv_arr'] : [],
            //默认职业技能和作品
            'sy_member_skill' => $this->config['sy_member_skill'],
            'curl' =>$curl
        );
        $this->render_json(0, '', $config);
    }

    /**
     * 会员-个人-个人设置: 保存头像
     */
    function saveLogo_action()
    {
        if ($_POST['submit']) {
            $fileArray = array();
            if ($_FILES) {
                $fileArray = format_files($_FILES);
                $UploadM = $this->MODEL('upload');
            }
            $return = array();
            $man_addpicArr = array();
            //默认男生头像，不完成路径
            $manicon_sys = !empty($_POST['manicon_sys']) ? $_POST['manicon_sys'] : array();
            $woman_addpicArr = array();
            //默认女生头像，不完整路径
            $womanicon_sys = !empty($_POST['womanicon_sys']) ? $_POST['womanicon_sys'] : array();

            foreach ($fileArray as $file) {
                //上传的男生头像
                if ($file['key'] == 'man_files') {
                    $upArr = array(
                        'file' => $file,
                        'dir' => 'logo',
                    );
                    $result = $UploadM->newUpload($upArr);
                    if (!empty($result['msg'])) {
                        $this->render_json(8, $return['msg']);
                    } else if (!empty($result['picurl'])) {
                        $man_addpicArr[] = $result['picurl'];
                    }
                }

                //上传的女生头像
                if ($file['key'] == 'woman_files') {
                    $upArr = array(
                        'file' => $file,
                        'dir' => 'logo',
                    );
                    $result = $UploadM->newUpload($upArr);
                    if (!empty($result['msg'])) {
                        $this->render_json(8, $return['msg']);
                    } else if (!empty($result['picurl'])) {
                        $woman_addpicArr[] = $result['picurl'];
                    }
                }
            }


            $manicon = array_merge($manicon_sys, $man_addpicArr);
            $manicon = array_slice($manicon, 0, 6);
            if (empty($manicon)) {
                $this->render_json(8, '至少保留一份默认头像');
            }
            $womanicon = array_merge($womanicon_sys, $woman_addpicArr);
            $womanicon = array_slice($womanicon, 0, 6);
            if (empty($womanicon)) {
                $this->render_json(8, '至少保留一份默认头像');
            }
            $memberlogo = array(
                'sy_member_icon_arr' => !empty($manicon) ? serialize($manicon) : '',
                'sy_member_icon' => !empty($manicon[0]) ? $manicon[0] : '',
                'sy_member_iconv_arr' => !empty($womanicon) ? serialize($womanicon) : '',
                'sy_member_iconv' => !empty($womanicon[0]) ? $womanicon[0] : '',
            );
            $configM = $this->MODEL("config");
            $configM->setConfig($memberlogo);
            $this->web_config();
            $this->admin_json(0, '个人头像配置设置成功');
        }
    }

//region 会员-个人-个人设置: 消费设置
    /**
     * 会员-个人-个人设置: 消费设置
     */
    function userspend_action()
    {
        $config = array(
            //个人简历置顶费用
            'integral_resume_top' => $this->config['integral_resume_top'],
            //个人委托简历费用
            'pay_trust_resume' => $this->config['pay_trust_resume'],
        );
        $this->render_json(0, '', array('config' => $config));
    }

    /**
     * 会员-个人-个人设置: 消费设置保存
     */
    function saveSpend_action()
    {
        if ($_POST['config']) {
            $post = array(
                'integral_resume_top_type' => $_POST['integral_resume_top_type'],
                'integral_resume_top' => $_POST['integral_resume_top'],
                'pay_trust_resume' => $_POST['pay_trust_resume'],
            );
            $configM = $this->MODEL('config');
            $configM->setConfig($post);
            $this->web_config();
            $this->admin_json(0, '个人消费设置配置修改成功');
        }
    }
//endregion
}