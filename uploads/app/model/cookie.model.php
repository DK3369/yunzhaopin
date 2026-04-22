<?php

class cookie_model extends model
{
    private function _getCookieDomain()
    {
        global $pageType;
        if ($this->config['sy_web_site'] == '1' || ((isset($pageType) && $pageType == 'wap' && strpos($this->config['sy_wapdomain'], '/wap') === false && $this->config['sy_wapdomain']))) {

            if ($this->config['sy_web_site'] == '1' && $this->config['sy_onedomain'] != "") {

                $weburl = get_domain($this->config['sy_onedomain']);
            } elseif ($this->config['sy_web_site'] == '1' && $this->config['sy_indexdomain'] != "") {

                $weburl = get_domain($this->config['sy_indexdomain']);
            } elseif ($this->config['sy_wapdomain']) {

                $weburl = get_domain($this->config['sy_wapdomain']);
            } else {

                $weburl = get_domain($this->config['sy_weburl']);
            }
        } else {
            $weburl = '';
        }
        return $weburl;
    }

    public function setcookie($name, $value, $time = 0)
    {
        $weburl = $this->_getCookieDomain();
        if (is_array($value)) {
            foreach ($value as $k => $v) {
                SetCookie($name . '[' . $k . ']', $v, $time, '/', $weburl);
            }
        } else {
            SetCookie($name, $value, $time, "/", $weburl);
			if($time == 0 && $weburl){
                SetCookie($name, $value, $time, "/");
            }
        }
    }

    public function add_cookie($uid, $username, $salt, $email, $pass, $type, $expire = "1", $userdid = '', $isadmin = '0')
    {
        if ($expire) {
            $expire_date = $expire * 86400;
        } else {
            $expire_date = 86400;
        }

        if ($this->config['did'] && $userdid == '') {
            $userdid = $this->config['did'];
        }

        $this->setcookie("uid", $uid, time() + $expire_date);
        $this->setcookie("shell", md5($username . $pass . $salt), time() + $expire_date);
        $this->setcookie("usertype", $type, time() + $expire_date);
        $this->setcookie("userdid", $userdid, time() + $expire_date);
        $this->setcookie("amtype", $isadmin, time() + $expire_date);
    }

    public function unset_cookie($auid = null)
    {

        $this->setcookie("uid", "", 0);
        $this->setcookie("shell", "", 0);
        $this->setcookie("usertype", "", 0);
        $this->setcookie("userdid", "", 0);


        $this->setcookie("exprefresh", "", 0);
        $this->setcookie("jobrefresh", "", 0);
        $this->setcookie("support", "", 0);

        $this->setcookie("wxloginid", "", 0);
        $this->setcookie("amtype", "", 0);
    }
}
