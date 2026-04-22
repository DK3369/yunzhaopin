<?php

class initDeal{

	private $ishatccess = false;//是否是伪静态链接
	private $url = '';
	public  $seoreg_arr = array();
	public  $reg_arr = array();
	public  $iswap = false;
	
	private $hatreg_wap_arr = array(
		'#^\/c_(.*)\.html#i'=>'yunurl=c_$1',
		'#^\/job\/([0-9]+)\.html(.*)#i'=>'c=job&a=comapply&id=$1',
	);
	private $hatreg_arr = array(
		'#^\/(.*)\/c_(.*)\.html#i'=>'m=$1&yunurl=c_$2',

		'#^\/company\/list\/(.*)-(.*)-(.*)-(.*)-(.*)-(.*)-(.*)\.html?(\?*)(.*)#i'=>'m=company&city=$1&mun=$2&welfare=$3&hy=$4&pr=$5&rec=$6&page=$7&keyword=$9',
		'#^\/company\/([0-9]+)\.html#i'=>'m=company&c=show&id=$1',
		'#^\/company\/(.*)\.html#i'=>'m=company&yunurl=$1',
		
		'#^\/ask\/(.*)\.html#i'=>'m=ask&yunurl=$1',
		
		'#^\/resume\/list\/(.*)-(.*)-(.*)-(.*)-(.*)-(.*)-(.*)-(.*)\.html?(\?*)(.*)#i'=>'m=resume&c=search&job=$1&city=$2&salary=$3&age=$4&all=$5&tp=$6&order=$7&page=$8&keyword=$10',
		
		'#^\/once\/list\/(.*)-(.*)-(.*)\.html?(\?*)(.*)#i'=>'m=once&city=$1&add_time=$2&page=$3&keyword=$5',
		'#^\/once\/(.*)\.html#i'=>'m=once&yunurl=$1',
		
		'#^\/tiny\/list\/(.*)-(.*)-(.*)-(.*)-(.*)\.html?(\?*)(.*)#i'=>'m=tiny&city=$1&sex=$2&exp=$3&add_time=$4&page=$5&keyword=$7',
		'#^\/tiny\/(.*)\.html#i'=>'m=tiny&yunurl=$1',
		
		'#^\/map\/(.*)\.html#i'=>'m=map&yunurl=$1',

		'#^\/evaluate\/(.*)\.html#i'=>'m=evaluate&yunurl=$1',
		
		'#^\/resume\/([0-9]+)\.html#i'=>'m=resume&c=show&id=$1',
		'#^\/resume\/(.*)\.html#i'=>'m=resume&yunurl=$1',

		'#^\/wap\/announcement\/(.*)\.html(.*)#i'=>'m=wap&c=announcement&id=$1',
		'#^\/wap\/job\/(.*)\.html(.*)#i'=>'m=wap&c=job&a=comapply&id=$1',
		'#^\/wap\/specialjob\/(.*)\.html(.*)#i'=>'m=wap&c=special&a=job&id=$1',
		'#^\/wap\/([a-z]+)/([0-9]+)\.html(.*)#i'=>'m=wap&c=$1&a=show&id=$2',
		'#^\/wap\/((?!member)[a-z]+)\/#i'=>'m=wap&c=$1',
		'#^\/wap\/(.*)\.html(.*)#i'=>'m=wap&yunurl=$1',
		
		'#^\/zph\/(.*)\.html(.*)#i'=>'m=zph&yunurl=$1',
		
		'#^\/m_(.*)\.html#i'=>'yunurl=m_$1',
		'#^\/c_(.*)\.html#i'=>'yunurl=c_$1',

		'#^\/announcement\/(.*)\.html(.*)#i'=>'m=announcement&yunurl=$1',
		
		'#^\/part\/list\/(.*)-(.*)-(.*)-(.*)-(.*).html?(\?*)(.*)#i'=>'m=part&city=$1&part_type=$2&cycle=$3&order=$4&page=$5&keyword=$7',
		'#^\/part\/(.*)\.html(.*)#i'=>'m=part&yunurl=$1',

		'#^\/register\/(.*)\.html(.*)#i'=>'m=register&yunurl=$1',

		'#^\/job\/list\/(.*)-(.*)-(.*)-(.*)-(.*)-(.*)-(.*)-(.*)\.html?(\??)(.*)#i'=>'m=job&c=search&job=$1&city=$2&salary=$3&all=$4&tp=$5&cert=$6&order=$7&page=$8&keyword=$10',
		'#^\/job\/([0-9]+)\.html#i'=>'m=job&c=comapply&id=$1',

		'#^\/company\/company-show-(.*)\.html#i'=>'m=company&id=$1',
		'#^\/company\/(.*)\/(.*)\.html#i'=>'m=company&tp=$1&id=$2',

		'#^\/article\/([0-9]+)\.html#i'=>'m=article&c=show&id=$1',
		'#^\/article\/(.*)\.html(.*)#i'=>'m=article&yunurl=$1',

		'#^\/redeem\/list\/(.*)-(.*)-(.*)-(.*)\.html#i'=>'m=redeem&c=list&intinfo=$1&nid=$2&tnid=$3&page=$4',

		'#^\/gongzhao\/(.*)\.html#i'=>'m=gongzhao&yunurl=$1',
        
        '#^\/job\/([0-9a-zA-Z]+)\/zp([0-9a-zA-Z]+)\/index.php?(\?*)(.*)#i'=>'m=job&ecity=$1&ejob=$2&$4',
        '#^\/job\/([0-9a-zA-Z]+)\/zp([0-9a-zA-Z]+)\/#i'=>'m=job&c=search&ecity=$1&ejob=$2',
        '#^\/job\/zp([0-9a-zA-Z]+)\/index.php?(\?*)(.*)#i'=>'m=job&ejob=$1&$3',
        '#^\/job\/([0-9a-zA-Z]+)\/index.php?(\?*)(.*)#i'=>'m=job&ecity=$1&$3',
		'#^\/job\/zp([0-9a-zA-Z]+)\/#i'=>'m=job&c=search&ejob=$1',
		'#^\/job\/([0-9a-zA-Z]+)\/#i'=>'m=job&c=search&ecity=$1',
        
        '#^\/resume\/([0-9a-zA-Z]+)\/qz([0-9a-zA-Z]+)\/index.php?(\?*)(.*)#i'=>'m=resume&ecity=$1&ejob=$2&$4',
        '#^\/resume\/([0-9a-zA-Z]+)\/qz([0-9a-zA-Z]+)\/#i'=>'m=resume&c=search&ecity=$1&ejob=$2',
        '#^\/resume\/qz([0-9a-zA-Z]+)\/index.php?(\?*)(.*)#i'=>'m=resume&ejob=$1&$3',
		'#^\/resume\/([0-9a-zA-Z]+)\/index.php?(\?*)(.*)#i'=>'m=resume&ecity=$1&$3',
		'#^\/resume\/qz([0-9a-zA-Z]+)\/#i'=>'m=resume&c=search&ejob=$1',
		'#^\/resume\/([0-9a-zA-Z]+)\/#i'=>'m=resume&c=search&ecity=$1',
	);
	public  $domain = false;
	function __construct(){

		global $config,$seo;
		
		$seoreg_arr = array();

		$reg_type = 'rewrite_reg';

		if (!empty($config['sy_wapdomain']) && $config['sy_wapdomain'] == $_SERVER['HTTP_HOST']) {
            $this->iswap = true;
            $reg_type = 'rewrite_wap_reg';
        }

		foreach ($seo as $k => $v) {
			$v = reset($v);
			
			if($v[$reg_type] && !empty($v[$reg_type])){
				
				$regpair = array();

				foreach ($v[$reg_type] as $key => $value) {
					$key = str_replace(array("（","）","￥"),array("(",")","$"),$key);
					$value = str_replace(array("（","）","￥"),array("(",")","$"),$value);

					$regpair[$key] = $value;
				}

				$seoreg_arr = array_merge($seoreg_arr,$regpair);
			}
		}
		$this->seoreg_arr = $seoreg_arr;


		$hatreg_arr = $this->iswap?$this->hatreg_wap_arr:$this->hatreg_arr;

		$hatreg_key_arr = array_keys($hatreg_arr);

		//用户自定义的伪静态，规则如果和默认的规则一样，就取出覆盖默认规则，并从自定义数组中去除以防止先后顺序不同导致匹配错误
		foreach ($seoreg_arr as $key => $value) {
			if(in_array($key,$hatreg_key_arr)){
				$hatreg_arr[$key] = $value;
				unset($seoreg_arr[$key]);
			}
		}

		$this->reg_arr = $seoreg_arr+$hatreg_arr;//用户设定的优先匹配
		
	}
	function getAllParam(){

		global $init_type,$pageType;

		include(PLUS_PATH."/admindir.php");

		$param = array();

		$uri_arr=$this->parseUrl();
		
		$uri_arr = array_filter($uri_arr);
		if($this->domain){
			return array();
		}
		if(empty($uri_arr)){
			return array();
		}
	    $pagetype_arr = array('wap');

	    $init_stay = array($admindir,'siteadmin','api','install','update');//保留的入口
	    
	    if(in_array($uri_arr[0],$init_stay)){
	    	return array();
	    }else if($uri_arr[0]=='member'){
    		//pc会员中心模块
    		$init_type = 'member';
    	}else if(!$this->iswap){

    		$param['m'] = $uri_arr[0];
	    	if(in_array($uri_arr[0],$pagetype_arr)){
	    		$pageType = $uri_arr[0];
	    	}
    	}
	    
	    //伪静态参数处理
	    if($this->ishatccess){
	    	unset($param['m']);
	    	$res = $this->getHatccessParam();

	    }else{
	    	$res = $this->getNoHatccessParam();
	    }
	    
	    $param = array_merge($param,$res);
	    
	    $return['param'] = $param;

	    return $return;
	}
	function getNoHatccessParam(){

		$url = $this->url;

		$param = array();

		if (strpos($url, '?')) {
            $url = substr($url,strpos($url, '?')+1);

            $param_arr = explode('&',$url);

			foreach ($param_arr as $pk => $pv) {

				$p = explode('=',$pv);

				$param[$p[0]] = $p[1];
			}
        }

		return $param;
	}
	//处理伪静态参数
	function getHatccessParam(){

		$url = $this->url;
		$param = array();

		$reg_arr = $this->reg_arr;
		
		$res = '';
		
		foreach ($reg_arr as $reg => $v) {
			$preg_res = array();
			preg_match($reg, $url, $preg_res);

			if($preg_res[0]){
			    
				$res = preg_replace($reg,$v,$url);
				break;
			}
			
		}

		if($res){
			
			$param_arr = explode('&',$res);

			foreach ($param_arr as $pk => $pv) {

				$p = explode('=',$pv);

				$param[$p[0]] = $p[1];
			}
		}
		
		return $param;
		
	}
	
	/**
     * 获取请求地址，二级域名uri转换成相应模块
     * @return mixed
     */
    public function getRequestUrl()
    {
    	global $config;

    	$protocol   =   $this->getprotocol($config['sy_weburl']);
    	$host       =   $protocol.$_SERVER['HTTP_HOST'];
    	$uri		=	'';
    	if(!strpos($host, 'localhost') && !strpos($host, '127.0.0.1')){

    		$uri = $_SERVER['REQUEST_URI'];
    	}else{

    		$uri = str_replace($config['sy_weburl'],'',$host.$_SERVER['REQUEST_URI']);
    		
    	}
    	$uri = urldecode($uri);

    	if ($config['sy_web_site'] == '1' && !isset($_POST['xcxCode'])) {
    		
    		$indexDir = str_replace('/','',$uri);
		    // 本地路径不做验证

		    if ($indexDir!='' && !strpos($host, 'localhost') && !strpos($host, '127.0.0.1')) {

	        	include(PLUS_PATH."/domain_cache.php");
	        	if (is_array($site_domain)) {
	                foreach ($site_domain as $key => $value) {
	                    if ($indexDir != '' && $value['indexdir'] == $indexDir && $value['mode'] == '2') {
	                    	$_GET['indexdir'] = $indexDir;
	                    	$this->domain = true;
	                        break;
	                    }
	                }
	            }
		        
		    }
		}
    	if($config['sy_weburl'] != $host){
			//除了wap，二级域名转换相应模块
    		include (CONFIG_PATH . "db.data.php");
    		$modelconfig = $arr_data['modelconfig'];
        
	        foreach ($modelconfig as $key => $value) {

	            if($key!='wap' && $_SERVER['HTTP_HOST'] == $config['sy_' . $key . 'domain']){

	            	$uri = $config['sy_' . $key . 'dir'].$uri;

	            	break;
	            }
	        }
    	}
        
        return $uri;
    }

	public function parseUrl()
    {
    	

        $url = $this->getRequestUrl();

        $this->url = $url;

        if (strpos($url, '?')) {
            $url = substr($url, 0, strpos($url, '?'));
        }
        
        if(strpos($url,'.html')!=false){
			$this->ishatccess = true;
        }else{
        	$reg_arr = $this->reg_arr;
        	foreach ($reg_arr as $srk => $srv) {
        		preg_match($srk, $url, $preg_res);

				if($preg_res[0]){

					$this->ishatccess = true;
					break;
				}
        	}

        }

        $arr = explode('/', trim($url, '/'));

        

        foreach ($arr as $key => $value) {
            if (trim($value) === 'index.php') {
                unset($arr[$key]);
            }
        }
        $arr = array_values($arr);
        return $arr;
    }
    public function getprotocol($weburl = '')
	{

	    if ($weburl) {
	        if (strpos($weburl, 'https://') !== false) {

	            $protocol   =   'https://';
	        } else {

	            $protocol   =   'http://';
	        }
	    } else {

	        $protocol       =   ((isset($_SERVER['HTTPS']) && $_SERVER['HTTPS'] == 'on') || (isset($_SERVER['HTTP_X_FORWARDED_PROTO']) && $_SERVER['HTTP_X_FORWARDED_PROTO'] == 'https')) ? 'https://' : 'http://';
	    }
	    return $protocol;
	}
}





?>