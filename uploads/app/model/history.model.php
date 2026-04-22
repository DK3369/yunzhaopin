<?php


class history_model extends model
{

    function addHistory($name, $value)
    {
        global $config;
        
        if ($config['sy_onedomain'] != "") {
        
            $weburl = get_domain($this->config['sy_onedomain']);
            
        } elseif ($config['sy_indexdomain'] != "") {
            
            $weburl = get_domain($this->config['sy_indexdomain']);
            
        } else {
            
            $weburl = get_domain($this->config['sy_weburl']);
            
        }
        
        if ($_COOKIE[$name]) {
        
            $Arr = explode(',', $_COOKIE[$name]);
        }
        
        $Arr[] = $value;
        
        if ($this->config['sy_web_site'] == "1") {
            
            SetCookie($name, @implode(',', $Arr), time() + 86400, "/", $weburl);
        } else {
            
            SetCookie($name, @implode(',', $Arr), time() + 86400, "/");
        }
    }
}
?>