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
class set_web_config_controller extends adminCommon{

	function index_action()
	{
        $city = $this->getCity();
        $config = array(
            'sy_seo_rewrite'=> $this->config['sy_seo_rewrite'],
            'sy_header_fix'=> $this->config['sy_header_fix'],
            'sy_footer_fix'=> $this->config['sy_footer_fix'],
            'sy_linksq'=> $this->config['sy_linksq'],
            'sy_wap_jump'=> $this->config['sy_wap_jump'],
            'sy_pc_jump_wap'=> $this->config['sy_pc_jump_wap'],
            'sy_h5_share'=> $this->config['sy_h5_share'],
            'sy_advice_mobilecode'=> $this->config['sy_advice_mobilecode'],
            'sy_job_lookfx'=> $this->config['sy_job_lookfx'],
            'sy_wxwap_list'=> $this->config['sy_wxwap_list'],
            'sy_wap_comtpl'=> $this->config['sy_wap_comtpl'],
            'sy_uni_comtpl'=> $this->config['sy_uni_comtpl'],
            'sy_news_rewrite'=> $this->config['sy_news_rewrite'],
            'sy_ewm_type'=> $this->config['sy_ewm_type'],
            'sy_default_userclass'=> $this->config['sy_default_userclass'],
            'sy_default_comclass'=> $this->config['sy_default_comclass'],
            'resume_salarytype'=> $this->config['resume_salarytype'],
            'sy_indexpage'=> $this->config['sy_indexpage'],
            'sy_datacycle'=> $this->config['sy_datacycle'],
            'sy_datacycle_job'=> $this->config['sy_datacycle_job'],
            'sy_datacycle_com'=> $this->config['sy_datacycle_com'],
            'sy_logintime'=> $this->config['sy_logintime'],
            'sy_login_type'=> $this->config['sy_login_type'],
            'sy_resume_visitors'=> $this->config['sy_resume_visitors'],
            'sy_adclick'=> $this->config['sy_adclick'],
            'sy_recommend_day_num'=> $this->config['sy_recommend_day_num'],
            'sy_recommend_interval'=> $this->config['sy_recommend_interval'],
            'sy_resumeout_day_num'=> $this->config['sy_resumeout_day_num'],
            'sy_resumeout_interval'=> $this->config['sy_resumeout_interval'],
            'sy_zhanzhang_baidu'=> $this->config['sy_zhanzhang_baidu'],
            'sy_outlinks'=> $this->config['sy_outlinks'],
            'sy_shenming'=> $this->config['sy_shenming'],
            'sy_job_hits'=> $this->config['sy_job_hits'],
            'sy_web_city_one'=> $this->config['sy_web_city_one'],
            'sy_web_city_two'=> $this->config['sy_web_city_two'],
            'sy_sxsjgs'=> $this->config['sy_sxsjgs'],
            'sy_closeOrder'=> $this->config['sy_closeOrder'],
            'sy_autoref'=> $this->config['sy_autoref'],
            'sy_autorefrand'=> $this->config['sy_autorefrand'],
        );
        $this->render_json('0','',['config'=> $config,'province'=>$city]);
	}

    function city_action(){

        $cityId = $_POST['city_id'];
        $city = $this->getCity($cityId);

        $this->render_json('0','',['city'=>$city]);
    }

    public function  getCity($cityId = ''){
        $cacheM     =   $this->MODEL('cache');
        $options    =   array('city');
        $cache      =   $cacheM -> GetCache($options);
        if ($cityId){
            $cityArr = $cache['city_type'][$cityId];
        }else{
            $cityArr = $cache['city_index'];
        }
        $city = [];
        foreach ($cityArr as $v){
            $city[] = [
                'label' => $cache['city_name'][$v],
                'value' => $v
            ];
        }
        return $city;
    }
	function save_action()
	{

        $configM  =  $this->MODEL('config');

        $configM -> setConfig($_POST);
        $this->web_config();

        $this->render_json('0','页面设置修改成功！');

	}
}

?>