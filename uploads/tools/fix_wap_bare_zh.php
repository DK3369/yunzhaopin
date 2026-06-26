<?php
/**
 * Wrap remaining bare Chinese in app/controller/wap and api/wxapp PHP.
 */
define('ROOT', dirname(__DIR__) . '/');

$skipFiles = array(
    'api/wxapp/wap.enum.php',
    'app/include/wap.enum.php',
);

$dirs = array('app/controller/wap', 'api/wxapp');

function shouldSkipLine($line)
{
    // Skip if all Chinese on line is already inside yun_auto_t / WapDbEnum
    if (preg_match_all('/[\x{4e00}-\x{9fff}]+/u', $line, $m)) {
        $bare = preg_replace('/yun_auto_t\s*\([^)]*\)|WapDbEnum::[A-Z_]+/u', '', $line);
        if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $bare)) {
            return true;
        }
    }
    if (preg_match('/^\s*(\*|\/\/|#)/', trim($line))) {
        return true;
    }
    return false;
}

function wrapChineseString($str)
{
    $str = str_replace("'", "\\'", $str);
    return "yun_auto_t('" . $str . "')";
}

function fixLine($line)
{
    if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $line)) {
        return $line;
    }
    if (shouldSkipLine($line)) {
        return $line;
    }

    $orig = $line;

    // yunset with optional spaces around ->
    $line = preg_replace_callback(
        '/(->\s*yunset\s*\(\s*["\'][^"\']+["\']\s*,\s*)(?!yun_auto_t\s*\()(["\'])([^"\']*[\x{4e00}-\x{9fff}][^"\']*)\2(\s*\))/u',
        function ($m) {
            return $m[1] . wrapChineseString($m[3]) . $m[4];
        },
        $line
    );

    // actMsg / ACT_layer_msg
    $line = preg_replace_callback(
        "/(->(?:actMsg|ACT_layer_msg)\s*\([^,]+,\s*)(?!yun_auto_t\s*\()(['\"])([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)\2/u",
        function ($m) {
            return $m[1] . wrapChineseString($m[3]);
        },
        $line
    );

    // ACT_msg_wap with $msg = "..."
    $line = preg_replace_callback(
        "/(->ACT_msg_wap\s*\([^,]+,\s*)\$msg\s*=\s*(['\"])([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)\2/u",
        function ($m) {
            return $m[1] . wrapChineseString($m[3]);
        },
        $line
    );

    // $_SESSION assignments
    $line = preg_replace_callback(
        "/(\$_SESSION\[[^\]]+\]\s*=\s*)(?!yun_auto_t\s*\()(['\"])([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)\2/u",
        function ($m) {
            return $m[1] . wrapChineseString($m[3]);
        },
        $line
    );

    // addMemberLog / LogM->addMemberLog
    $line = preg_replace_callback(
        "/((?:->addMemberLog|member_log)\s*\(\s*)(?!yun_auto_t\s*\()(['\"])([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)\2/u",
        function ($m) {
            return $m[1] . wrapChineseString($m[3]);
        },
        $line
    );

    // str_replace Chinese literals in 3rd arg
    $line = preg_replace_callback(
        "/(str_replace\s*\(\s*)(['\"])([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)\2(\s*,)/u",
        function ($m) {
            return $m[1] . wrapChineseString($m[3]) . $m[4];
        },
        $line
    );

    // array('like','中文' or '转换'
    $line = preg_replace_callback(
        "/(array\s*\(\s*['\"]like['\"]\s*,\s*)(['\"])([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)\2/u",
        function ($m) {
            $inner = $m[3];
            if (strpos($inner, '$') !== false) {
                // keep PHP concat e.g. 转换'.$config
                return $m[0];
            }
            return $m[1] . wrapChineseString($inner);
        },
        $line
    );

    // pay_remark with concat: '转换'.$config
    $line = preg_replace_callback(
        "/(array\s*\(\s*['\"]like['\"]\s*,\s*)['\"]转换['\"]\s*\./u",
        function ($m) {
            return $m[1] . "yun_auto_t('转换').";
        },
        $line
    );

    // invtalCheck - both quote styles
    $line = preg_replace(
        array(
            '/"integral_bind_wx",\s*"微信扫码绑定"/u',
            '/"integral_login",\s*"会员登录"/u',
            "/'integral_bind_wx',\s*'微信扫码绑定'/u",
            "/'integral_login',\s*'会员登录'/u",
        ),
        array(
            '"integral_bind_wx", WapDbEnum::INTEGRAL_BIND_WX',
            '"integral_login", WapDbEnum::INTEGRAL_LOGIN',
            "'integral_bind_wx', WapDbEnum::INTEGRAL_BIND_WX",
            "'integral_login', WapDbEnum::INTEGRAL_LOGIN",
        ),
        $line
    );

    // DB enum '不限' in ternary
    $line = preg_replace(
        "/:\s*'不限'/u",
        ": WapDbEnum::UNLIMITED",
        $line
    );

    // yunset(array('title' => '中文', ...)) single-key inline
    $line = preg_replace_callback(
        "/(->yunset\s*\(\s*array\s*\([^)]*?(?:title|headertitle)\s*=>\s*)(?!yun_auto_t\s*\()(['\"])([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)\2/u",
        function ($m) {
            return $m[1] . wrapChineseString($m[3]);
        },
        $line
    );

    // layer_msg('中文'
    $line = preg_replace_callback(
        "/(->layer_msg\s*\(\s*)(?!yun_auto_t\s*\()(['\"])([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)\2/u",
        function ($m) {
            return $m[1] . wrapChineseString($m[3]);
        },
        $line
    );

    // ACT_msg_wap(..., "中文"
    $line = preg_replace_callback(
        "/(->ACT_msg_wap\s*\([^,]+,\s*)(?!yun_auto_t\s*\()(['\"])([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)\2/u",
        function ($m) {
            return $m[1] . wrapChineseString($m[3]);
        },
        $line
    );

    // ACT_msg(..., "中文"
    $line = preg_replace_callback(
        "/(->ACT_msg\s*\([^,]+,\s*)(?!yun_auto_t\s*\()(['\"])([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)\2/u",
        function ($m) {
            return $m[1] . wrapChineseString($m[3]);
        },
        $line
    );

    // $x['msg'] = "中文" or .=
    $line = preg_replace_callback(
        "/(\$[a-zA-Z_][\w]*(?:\[[^\]]+\])+\s*(?:\.=|=\s*))(?!yun_auto_t\s*\()(['\"])([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)\2/u",
        function ($m) {
            return $m[1] . wrapChineseString($m[3]);
        },
        $line
    );

    // 'msg' => '中文' in arrays (not already wrapped)
    $line = preg_replace_callback(
        "/(['\"]msg['\"]\s*=>\s*)(?!yun_auto_t\s*\()(['\"])([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)\2/u",
        function ($m) {
            return $m[1] . wrapChineseString($m[3]);
        },
        $line
    );

    // ternary ? '中文' : '中文'
    $line = preg_replace_callback(
        "/(\?\s*)(?!yun_auto_t\s*\()(['\"])([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)\2(\s*:\s*)(?!yun_auto_t\s*\()(['\"])([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)\5/u",
        function ($m) {
            return $m[1] . wrapChineseString($m[3]) . $m[4] . wrapChineseString($m[6]);
        },
        $line
    );

    // render_json(..., "中文"
    $line = preg_replace_callback(
        "/(->render_json\s*\([^,]+,\s*)(?!yun_auto_t\s*\()(['\"])([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)\2/u",
        function ($m) {
            return $m[1] . wrapChineseString($m[3]);
        },
        $line
    );

    // member_log("中文"
    $line = preg_replace_callback(
        "/(member_log\s*\(\s*)(?!yun_auto_t\s*\()(['\"])([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)\2/u",
        function ($m) {
            return $m[1] . wrapChineseString($m[3]);
        },
        $line
    );

    // jycheck config literals
    $jycheckMap = array(
        '注册会员' => 'WapDbEnum::CODE_WEB_REGISTER',
        '前台登录' => 'WapDbEnum::CODE_WEB_FRONT_LOGIN',
        '找回密码' => 'WapDbEnum::CODE_WEB_FORGET_PW',
        '意见反馈' => 'WapDbEnum::CODE_WEB_FEEDBACK',
        '店铺招聘' => 'WapDbEnum::CODE_WEB_ONCE_JOB',
        '普工简历' => 'WapDbEnum::CODE_WEB_TINY_RESUME',
    );
    foreach ($jycheckMap as $zh => $const) {
        $line = preg_replace(
            "/jycheck\s*\(([^)]+),\s*['\"]" . preg_quote($zh, '/') . "['\"]\s*\)/u",
            'jycheck($1, ' . $const . ')',
            $line
        );
    }

    $line = preg_replace(
        array(
            '/"integral_bind_wx",\s*"微信扫码绑定"/u',
            '/"integral_login",\s*"会员登录"/u',
            "/'integral_bind_wx',\s*'微信扫码绑定'/u",
            "/'integral_login',\s*'会员登录'/u",
        ),
        array(
            '"integral_bind_wx", WapDbEnum::INTEGRAL_BIND_WX',
            '"integral_login", WapDbEnum::INTEGRAL_LOGIN',
            "'integral_bind_wx', WapDbEnum::INTEGRAL_BIND_WX",
            "'integral_login', WapDbEnum::INTEGRAL_LOGIN",
        ),
        $line
    );

    return $line;
}

$changed = 0;
foreach ($dirs as $dir) {
    $path = ROOT . $dir;
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($path));
    foreach ($it as $f) {
        if (!$f->isFile() || $f->getExtension() !== 'php') {
            continue;
        }
        $rel = str_replace(ROOT, '', $f->getPathname());
        if (in_array($rel, $skipFiles, true)) {
            continue;
        }
        $lines = file($f->getPathname());
        $out = array();
        $fileChanged = false;
        foreach ($lines as $line) {
            $new = fixLine($line);
            if ($new !== $line) {
                $fileChanged = true;
            }
            $out[] = $new;
        }
        if ($fileChanged) {
            file_put_contents($f->getPathname(), implode('', $out));
            echo "FIXED: $rel\n";
            $changed++;
        }
    }
}
echo "Done. $changed files updated.\n";
