(function () {
    var root = document.documentElement;
    var checkTimer = null;
    var fallbackTimer = null;
    var released = false;

    function addClass(name) {
        if (root.classList) {
            root.classList.add(name);
        } else if ((' ' + root.className + ' ').indexOf(' ' + name + ' ') === -1) {
            root.className += ' ' + name;
        }
    }

    function removeClass(name) {
        if (root.classList) {
            root.classList.remove(name);
        } else {
            root.className = (' ' + root.className + ' ').replace(' ' + name + ' ', ' ').replace(/^\s+|\s+$/g, '');
        }
    }

    function hasMountedRoot() {
        if (!document.body) {
            return false;
        }
        var nodes = document.body.querySelectorAll('[id]');
        for (var i = 0; i < nodes.length; i++) {
            var vm = nodes[i].__vue__;
            if (vm && vm.$root === vm && vm.$el === nodes[i]) {
                return true;
            }
        }
        return false;
    }

    function reveal() {
        if (released) {
            return;
        }
        released = true;
        if (checkTimer) {
            window.clearTimeout(checkTimer);
        }
        if (fallbackTimer) {
            window.clearTimeout(fallbackTimer);
        }
        removeClass('admin-vue-booting');
        addClass('admin-vue-ready');
        window.setTimeout(function () {
            removeClass('admin-vue-ready');
        }, 180);
    }

    function revealAfterPaint() {
        var nextFrame = window.requestAnimationFrame || function (callback) {
            return window.setTimeout(callback, 0);
        };
        nextFrame(function () {
            nextFrame(reveal);
        });
    }

    function waitForVue() {
        if (hasMountedRoot()) {
            revealAfterPaint();
            return;
        }
        checkTimer = window.setTimeout(waitForVue, 32);
    }

    addClass('admin-vue-booting');
    document.writeln("<style id='admin-vue-boot-style'>html.admin-vue-booting{min-height:100%;background:#f5f7fa}html.admin-vue-booting body{visibility:hidden!important}html.admin-vue-booting:before{content:'';position:fixed;z-index:2147483647;top:50%;left:50%;width:30px;height:30px;margin:-18px 0 0 -18px;border:3px solid rgba(64,158,255,.2);border-top-color:#409eff;border-radius:50%;animation:admin-vue-boot-spin .75s linear infinite}html.admin-vue-ready body{animation:admin-vue-boot-fade .16s ease-out both}@keyframes admin-vue-boot-spin{to{transform:rotate(360deg)}}@keyframes admin-vue-boot-fade{from{opacity:0}to{opacity:1}}@media (prefers-reduced-motion:reduce){html.admin-vue-booting:before{animation:none}html.admin-vue-ready body{animation:none}}</style>");
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', waitForVue);
    } else {
        waitForVue();
    }
    fallbackTimer = window.setTimeout(reveal, 8000);
    window.adminVueReveal = reveal;
})();

document.writeln("<meta charset=\'UTF-8\'>");
document.writeln("<meta http-equiv=\'X-UA-Compatible\' content=\'IE=edge\'>");
document.writeln("<meta name=\'viewport\' content=\'width=device-width, initial-scale=1.0\'>");
document.writeln("<link rel=\'stylesheet\' href=\'../../../admin/js/element.css\'>");
document.writeln("<script src=\'../../../admin/js/vue.min.js\'></script>");
document.writeln("<script src=\'../../../admin/js/jquery.min.js\'></script>");
document.writeln("<script src=\'../../../admin/js/element.js\'></script>");
document.writeln("<script src=\'../../../admin/js/echarts.min.js\'></script>");
document.writeln("<link rel=\'stylesheet\' href=\'../../../admin/adstyle/phpyun.css\'>");
document.writeln("<script src=\'../../../admin/js/axios.min.js\'></script>");
document.writeln("<script src=\'../../../admin/js/api.js\'></script>");
