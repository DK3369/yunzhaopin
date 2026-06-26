<?php

class Yun_I18n
{
    var $langPath = '';
    var $defaultLang = 'zh_cn';
    var $currentLang = 'zh_cn';
    var $fallbackLang = 'zh_cn';
    var $messages = array();
    var $fallbackMessages = array();
    var $autoMessages = array();
    var $fallbackAutoMessages = array();
    var $autoAliases = array();
    var $available = array();

    function Yun_I18n($langPath, $defaultLang = 'zh_cn')
    {
        $this->__construct($langPath, $defaultLang);
    }

    function __construct($langPath, $defaultLang = 'zh_cn')
    {
        $this->langPath = rtrim($langPath, '/\\') . '/';
        $this->available = $this->scanAvailableLanguages();
        $this->defaultLang = $this->normalizeLang($defaultLang);

        if (!$this->isAvailable($this->defaultLang)) {
            $this->defaultLang = $this->fallbackLang;
        }

        $this->fallbackMessages = $this->loadMessages($this->fallbackLang);
        $this->fallbackAutoMessages = $this->loadAutoMessages($this->fallbackLang);
        $this->autoAliases = $this->loadAutoAliases();
        $this->setLang($this->defaultLang);
    }

    function scanAvailableLanguages()
    {
        $langs = array();
        if (is_dir($this->langPath)) {
            $files = glob($this->langPath . '*.php');
            if (is_array($files)) {
                foreach ($files as $file) {
                    $code = basename($file, '.php');
                    if (preg_match('/^[a-z]{2}(_[a-z]{2})?$/', $code)) {
                        $langs[$code] = $code;
                    }
                }
            }
        }
        if (!isset($langs[$this->fallbackLang])) {
            $langs[$this->fallbackLang] = $this->fallbackLang;
        }
        return $langs;
    }

    function detectLang()
    {
        $candidates = array();

        if (isset($_GET['lang'])) {
            $candidates[] = $_GET['lang'];
        }
        if (isset($_COOKIE['lang'])) {
            $candidates[] = $_COOKIE['lang'];
        }
        if (isset($_SERVER['HTTP_ACCEPT_LANGUAGE'])) {
            $parts = explode(',', $_SERVER['HTTP_ACCEPT_LANGUAGE']);
            foreach ($parts as $part) {
                $langParts = explode(';', $part, 2);
                $lang = trim($langParts[0]);
                if ($lang != '') {
                    $candidates[] = $lang;
                }
            }
        }

        foreach ($candidates as $candidate) {
            $lang = $this->normalizeLang($candidate);
            if ($this->isAvailable($lang)) {
                return $lang;
            }
            $short = substr($lang, 0, 2);
            foreach ($this->available as $available) {
                if (substr($available, 0, 2) == $short) {
                    return $available;
                }
            }
        }

        return $this->defaultLang;
    }

    function setLang($lang)
    {
        $lang = $this->normalizeLang($lang);
        if (!$this->isAvailable($lang)) {
            $lang = $this->defaultLang;
        }

        $this->currentLang = $lang;
        $this->messages = $this->loadMessages($lang);
        $this->autoMessages = $this->loadAutoMessages($lang);
        $this->autoAliases = $this->loadAutoAliases();
        return $this->currentLang;
    }

    function getLang()
    {
        return $this->currentLang;
    }

    function getAvailable()
    {
        return array_values($this->available);
    }

    function getMeta()
    {
        $meta = $this->getValue('_meta', $this->messages);
        return is_array($meta) ? $meta : array();
    }

    function isAutoKey($key)
    {
        if (!is_string($key) || !preg_match('/^([a-z][a-z0-9_]*)_([0-9]{5})$/', $key, $m)) {
            return false;
        }
        return count(explode('_', $m[1])) <= 3;
    }

    function t($key, $params = array(), $default = '')
    {
        $value = $this->getValue($key, $this->messages);
        if ($value === null && $this->currentLang != $this->fallbackLang) {
            $value = $this->getValue($key, $this->fallbackMessages);
        }
        if ($value === null && $this->isAutoKey($key)) {
            if (isset($this->autoMessages[$key])) {
                $value = $this->autoMessages[$key];
            } elseif ($this->currentLang != $this->fallbackLang && isset($this->fallbackAutoMessages[$key])) {
                $value = $this->fallbackAutoMessages[$key];
            }
        }
        if ($value === null) {
            $value = $default !== '' ? $default : $key;
        }
        if (is_array($value)) {
            return $default !== '' ? $default : $key;
        }
        return $this->replaceParams($value, $params);
    }

    function all()
    {
        return $this->messages;
    }

    function autoT($text)
    {
        if ($text === '') {
            return $text;
        }

        $leading = '';
        $trailing = '';
        if (preg_match('/^(\s*)(.*?)(\s*)$/su', $text, $matches)) {
            $leading = $matches[1];
            $text = $matches[2];
            $trailing = $matches[3];
        }

        if ($text === '') {
            return $leading . $trailing;
        }

        if (isset($this->autoAliases[$text])) {
            $aliasKey = $this->autoAliases[$text];
            if (isset($this->autoMessages[$aliasKey])) {
                return $leading . $this->autoMessages[$aliasKey] . $trailing;
            }
        }

        if (isset($this->autoMessages[$text])) {
            return $leading . $this->autoMessages[$text] . $trailing;
        }
        if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $text)) {
            return $leading . $text . $trailing;
        }
        if ($this->currentLang != $this->fallbackLang && !empty($this->autoMessages)) {
            foreach ($this->autoMessages as $source => $target) {
                if ($this->canUsePartialAutoKey($source) && $target !== "" && strpos($text, $source) !== false) {
                    $text = str_replace($source, $target, $text);
                }
            }
        }
        if ($this->currentLang != $this->fallbackLang && isset($this->fallbackAutoMessages[$text])) {
            return $leading . $this->fallbackAutoMessages[$text] . $trailing;
        }

        return $leading . $text . $trailing;
    }

    function canUsePartialAutoKey($source)
    {
        if ($source === '') {
            return false;
        }
        $len = function_exists('mb_strlen') ? mb_strlen($source, 'UTF-8') : strlen($source);
        return $len > 1;
    }

    function autoArray($value, $key = '')
    {
        if (is_array($value)) {
            $translated = array();
            foreach ($value as $itemKey => $itemValue) {
                $translated[$itemKey] = $this->autoArray($itemValue, $itemKey);
            }
            return $translated;
        }

        if (is_string($value) && $this->shouldTranslateValue($key, $value)) {
            return $this->autoT($value);
        }

        return $value;
    }

    function shouldTranslateValue($key, $value)
    {
        if ($value === '' || !preg_match('/[\x{4e00}-\x{9fff}]/u', $value)) {
            return false;
        }

        if ($key === '' || is_int($key)) {
            return true;
        }

        $key = strtolower($key);
        if (preg_match('/(^|_)(status|state|type|opera|source|usertype|paytype|order_state|cert_type)_(n|name)$/', $key)) {
            return true;
        }
        return in_array($key, array(
            'msg',
            'message',
            'errmsg',
            'error_msg',
            'error_message',
            'tip',
            'tips',
            'notice',
            'alert',
            'contentmsg',
            'statusmsg',
            'statusname',
            'statename',
            'typename',
            'operaname',
            'sbody'
        ));
    }

    function normalizeLang($lang)
    {
        $lang = strtolower(trim($lang));
        $lang = str_replace('-', '_', $lang);
        $lang = preg_replace('/[^a-z_]/', '', $lang);
        if ($lang == 'zh' || $lang == 'zh_cn' || $lang == 'zh_hans') {
            return 'zh_cn';
        }
        return $lang;
    }

    function isAvailable($lang)
    {
        return isset($this->available[$lang]) && is_file($this->langPath . $lang . '.php');
    }

    function loadMessages($lang)
    {
        $file = $this->langPath . $lang . '.php';
        if (is_file($file)) {
            $messages = include($file);
            if (is_array($messages)) {
                return $messages;
            }
        }
        return array();
    }

    function loadAutoMessages($lang)
    {
        $file = $this->langPath . 'auto/' . $lang . '.php';
        if (is_file($file)) {
            $messages = include($file);
            if (is_array($messages)) {
                $this->sortAutoMessages($messages);
                return $messages;
            }
        }
        return array();
    }

    function loadAutoAliases()
    {
        $file = $this->langPath . 'auto/aliases.php';
        if (is_file($file)) {
            $aliases = include($file);
            if (is_array($aliases)) {
                return $aliases;
            }
        }
        return array();
    }

    function sortAutoMessages(&$messages)
    {
        uksort($messages, array($this, 'compareAutoMessageKeys'));
    }

    function compareAutoMessageKeys($a, $b)
    {
        $aLen = function_exists('mb_strlen') ? mb_strlen($a, 'UTF-8') : strlen($a);
        $bLen = function_exists('mb_strlen') ? mb_strlen($b, 'UTF-8') : strlen($b);
        if ($aLen == $bLen) {
            return strcmp($a, $b);
        }
        return $bLen - $aLen;
    }

    function getValue($key, $messages)
    {
        if ($key === '' || !is_array($messages)) {
            return null;
        }
        $parts = explode('.', $key);
        $value = $messages;
        foreach ($parts as $part) {
            if (is_array($value) && array_key_exists($part, $value)) {
                $value = $value[$part];
            } else {
                return null;
            }
        }
        return $value;
    }

    function replaceParams($text, $params)
    {
        if (!is_array($params) || empty($params)) {
            return $text;
        }
        foreach ($params as $key => $value) {
            $text = str_replace('{' . $key . '}', $value, $text);
        }
        return $text;
    }
}

require_once dirname(__FILE__) . '/i18n.functions.php';
