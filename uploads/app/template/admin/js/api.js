// 请求拦截器
axios.interceptors.request.use(function (config) {
    // 在发送请求之前做些什么

    // 不传递默认开启loading，传递 hideloading=true 则不显示loading
    if (!config.hideloading) {
        showFullScreenLoading();
    }

    // let token = localStorage.getItem("token"); // 传token
    config.headers['Content-Type'] = "application/x-www-form-urlencoded";
    // config.headers['token'] = token;  //Authorization

    return config;
}, function (error) {
    // 对请求错误做些什么
    console.log('requestError', error);
    return Promise.reject(error);
});

// 响应拦截器
axios.interceptors.response.use(function (response) {
    // 2xx 范围内的状态码都会触发该函数。
    // 响应后则清除定时器、关闭加载动画
    tryHideFullScreenLoading();
    return response;
}, function (error) {
    // 超出 2xx 范围的状态码都会触发该函数。
    // 对响应错误做点什么
    console.log('responseError', error)

    // 响应后则清除定时器、关闭加载动画
    tryHideFullScreenLoading();

    /**
     * @code 登录过期 token验证失败 根据后端调
     */
    // if (response.data.code == 401) {
    //     // TODO 跳转登录
    // }

    // 权限校验
    if (error.response.status == 777) {
        message.warning(error.response.data.msg);
        // vue.$router.push('/login');
        // window.parent.location.href= window.location.origin+'/admin/#/index';
        //window.parent.location.reload();
        // this.$router.push({
        //     path: '/index',
        //     params: {}
        // }, () => {})
    }

    let err = {
        code: 999,
        msg: '未知异常，请联系管理员！'
    };

    return Promise.reject(err);
});

let baseUrl = localStorage.getItem("baseUrl") + '?';
let version = Date.now();

/**
 * POST 请求
 * @param url
 * @param params 接口参数
 * @param config 配置
 *        {hideloading: true} 隐藏加载动画
 * @param newBase 请求地址，部分情况下，还没有baseUrl，需要传一下
 * @returns {*}
 */
function httpPost(url, params = null, config = {}, newBase = '') {
    baseUrl = localStorage.getItem("baseUrl") + '?';
	if(!params){
		var params = {
			pytoken: localStorage.getItem("pytoken")
		};
	}else{
	    let prototype = Object.prototype.toString.apply(params);
        if (prototype == '[object FormData]') {
            params.append('pytoken', localStorage.getItem("pytoken"));
        } else {
            params.pytoken = localStorage.getItem("pytoken");
        }
	}
    try {
        const response = axios.post(newBase ? `${newBase}?${url}` : `${baseUrl}${url}`, params, config);
        return response;
    } catch (error) {
        return error;
    }
}

function getUrlParams(location = window.location) {
    var qs = '';
    if (location.search) {
        qs = location.search.substr(1); // 获取url中"?"符后的字串
    } else if (location.hash) { // 获取url中#符后的参数
        qs = location.hash.substr(location.hash.indexOf('?') + 1);
    }

    var args = {}, // 保存参数数据的对象
        items = qs.length ? qs.split("&") : [], // 取得每一个参数项,
        item = null,
        len = items.length;

    for(var i = 0; i < len; i++) {
        item = items[i].split("=");
        var name = decodeURIComponent(item[0]),
            value = decodeURIComponent(item[1]);
        if(name) {
            args[name] = value;
        }
    }
    return args;
}
let loadingTimeout = null;
// 显示loading加载动画
let loading;
function startLoading() {
    loading = Vue.prototype.$loading({
        lock: true,
        text: 'Loading',
        spinner: 'el-icon-loading',
        background: 'rgba(57, 61, 73, 0.5)',
    });

}
// 清除loading加载动画
function endLoading(){
    clearTimeout(loadingTimeout);
    loadingTimeout = null;
    if(loading !== undefined){
        loading.close();
    }
}
// 需要考虑一个问题，可能同时发起多个请求，而我们显示动画是一次性的，即等当前所有请求完毕后再清除动画。
// 声明一个对象用于存储请求个数
let needLoadingRequestCount = 0;
function showFullScreenLoading() {
    if (needLoadingRequestCount === 0) {
        // 800毫秒没有响应，则显示加载动画
        loadingTimeout = setTimeout(() => {
            startLoading();
        }, 800);
    }
    needLoadingRequestCount++;
}

function tryHideFullScreenLoading() {
    if (needLoadingRequestCount <= 0){
        return;
    }
    needLoadingRequestCount--;
    if (needLoadingRequestCount === 0) {
        endLoading();
    }
}

const message = new Vue({
    methods: {
        open(options) {
            this.$message(options);
        },

        /**
         * 确认框显示
         * @param msg 提示信息
         * @param confirmFun 确定回调函数
         * @param confirmButtonText 确定按钮文字
         * @param title 标题
         * @param type 是否有图标 success / info / warning / error
         * @param showCancelButton 是否显示取消按钮
         * @param cancelButtonText 取消按钮文字
         * @param cancelFun 取消回调
         */
        confirm(msg, confirmFun = null, confirmButtonText = '', title = '', type = '', showCancelButton = true, cancelButtonText = '', cancelFun = null) {
            this.$confirm(msg, title ? title : '温馨提示', {
                confirmButtonText: confirmButtonText ? confirmButtonText : '确定',
                cancelButtonText: cancelButtonText ? cancelButtonText : '取消',
                type: type,
                showCancelButton: showCancelButton
            }).then(() => {
                typeof(confirmFun) == 'function' && confirmFun();
            }).catch(() => {
                typeof(cancelFun) == 'function' && cancelFun();
            });
        },

        /**
         * alert 消息提示
         * @param msg 提示语
         * @param confirmFun 回调方法
         * @param confirmButtonText 按钮文字
         */
        alert(msg, confirmFun = null, confirmButtonText = '') {
            this.$alert(msg, {
                confirmButtonText: confirmButtonText ? confirmButtonText : '确定',
                callback: action => {
                    typeof(confirmFun) == 'function' && confirmFun();
                }
            });
        },

        /**
         * 参数说明
         * @param message 提示语
         * @param closeFun 提示语结束后执行的关闭回调函数
         * @param duration 显示时长，单位毫秒
         * @param options 参考官方的自定议参数，文档地址：https://element.eleme.cn/#/zh-CN/component/message
         */
        success(message, closeFun = null, duration = 2000, options = {}) {
            this.commonMsg('success', message, closeFun, duration, options);
        },

        error(message, closeFun = null, duration = 2000, options = {}) {
            this.commonMsg('error', message, closeFun, duration, options);
        },

        warning(message, closeFun = null, duration = 2000, options = {}) {
            this.commonMsg('warning', message, closeFun, duration, options);
        },

        commonMsg(type, message, closeFun, duration, options) {
            if (!options.message) {
                options.message = message
            }
            if (!options.type) {
                options.type = type
            }
            if (!options.onClose && closeFun) {
                options.onClose = closeFun
            }
            if (!options.duration) {
                options.duration = duration
            }
            this.$message(options);
        }
    }
})

/**
 * 删除弹窗
 * @param _this
 * @param params
 * @param delFun 删除方法名称，传参如 this.delFun
 * @param msg 删除操作提示语
 * @param cancelFun 删除提示框取消操作方法
 */
function delConfirm(_this, params, delFun, msg, cancelFun) {
    if (typeof msg == 'undefined' || msg == ''){
        msg = '你确定要删除当前项吗？';
    }
    _this.$confirm(msg, '温馨提示', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
    }).then(() => {
        delFun(params);
    }).catch(() => {
        typeof(cancelFun) == 'function' && cancelFun();
    });
}

// 判断arr是否为一个数组，返回一个bool值
function isArray (arr) {
    return Object.prototype.toString.call(arr) === '[object Array]';
}

// 深度克隆
function deepClone (obj) {
    // 对常见的“非”值，直接返回原来值
    if([null, undefined, NaN, false].includes(obj)) return obj;
    if(typeof obj !== "object" && typeof obj !== 'function') {
        //原始类型直接返回
        return obj;
    }
    var o = isArray(obj) ? [] : {};
    for(let i in obj) {
        if(obj.hasOwnProperty(i)){
            o[i] = typeof obj[i] === "object" ? deepClone(obj[i]) : obj[i];
        }
    }
    return o;
}

// 格式化日期-年月
function formatMonth(date) {
    let year = date.getFullYear(),
        month = date.getMonth()+1;

    return `${year}-${month < 10 ? '0' + month : month}`;
}

// 格式化日期
function formatDate(date) {
    let year = date.getFullYear(),
        month = date.getMonth()+1,
        day = date.getDate();

    return `${year}-${month < 10 ? '0' + month : month}-${day < 10 ? '0' + day : day}`;
}

// 格式化日期时间
function formatDatetime(date) {
    let hours = date.getHours(),
        minutes = date.getMinutes(),
        seconds = date.getSeconds();

    return formatDate(date) + ` ${hours < 10 ? '0' + hours : hours}:${minutes < 10 ? '0' + minutes : minutes}:${seconds < 10 ? '0' + seconds : seconds}`;
}

//  jsMath函数 Start
function accAdd(arg1, arg2) {
    if (isNaN(arg1)) {
        arg1 = 0;
    }
    if (isNaN(arg2)) {
        arg2 = 0;
    }
    arg1 = Number(arg1);
    arg2 = Number(arg2);
    var r1, r2, m, c;
    try {
        r1 = arg1.toString().split(".")[1].length;
    }
    catch (e) {
        r1 = 0;
    }
    try {
        r2 = arg2.toString().split(".")[1].length;
    }
    catch (e) {
        r2 = 0;
    }
    c = Math.abs(r1 - r2);
    m = Math.pow(10, Math.max(r1, r2));
    if (c > 0) {
        var cm = Math.pow(10, c);
        if (r1 > r2) {
            arg1 = Number(arg1.toString().replace(".", ""));
            arg2 = Number(arg2.toString().replace(".", "")) * cm;
        } else {
            arg1 = Number(arg1.toString().replace(".", "")) * cm;
            arg2 = Number(arg2.toString().replace(".", ""));
        }
    } else {
        arg1 = Number(arg1.toString().replace(".", ""));
        arg2 = Number(arg2.toString().replace(".", ""));
    }
    return (arg1 + arg2) / m;
}
function accSub(arg1, arg2) {
    arg1 = $.trim(arg1);
    arg2 = $.trim(arg2);
    return accAdd(arg1, -arg2);
}
function accMul(arg1, arg2) {
    arg1 = $.trim(arg1);
    arg2 = $.trim(arg2);
    var m = 0,
        s1 = arg1.toString(),
        s2 = arg2.toString();
    try {
        m += s1.split(".")[1].length
    } catch (e) {}
    try {
        m += s2.split(".")[1].length
    } catch (e) {}
    return Number(s1.replace(".", "")) * Number(s2.replace(".", "")) / Math.pow(10, m)
}
function accDiv(arg1, arg2) {
    arg1 = $.trim(arg1);
    arg2 = $.trim(arg2);
    var t1 = 0,
        t2 = 0,
        r1, r2;
    try {
        t1 = arg1.toString().split(".")[1].length
    } catch (e) {}
    try {
        t2 = arg2.toString().split(".")[1].length
    } catch (e) {}
    with(Math) {
        r1 = Number(arg1.toString().replace(".", ""));
        r2 = Number(arg2.toString().replace(".", ""));
        return (r1 / r2) * pow(10, t2 - t1);
    }
}
//  jsMath函数 End

//Cascader 城市级联选择器 options数据处理
function cityCascader(obj={}){

    var cidata = ctdata = cndata = options = [];

    if(typeof ci!='undefined' && ci!='new Array()' && ci.length>0){
        cidata = ci;
    }
    if(typeof ct!='undefined' && ct!='new Array()' &&  ct.length>0){
        ctdata = ct;
    }
    if(typeof cn!='undefined' && cn!='new Array()' &&  cn.length>0){
        cndata = cn;
    }
    if(typeof city_parent!='undefined' && city_parent!='new Array()' &&  city_parent.length>0){
        cpdata = city_parent;
    }

    var one = two = three = null;
    var children = children2 = [];
    var option = option1 = {};

    for(let i in cidata){

        option = {};

        one = two = three = null;

        children = children2 = [];

        one = cidata[i];

        if(typeof ctdata[one]!='undefined' && ctdata[one].length>0){

            for(let j in ctdata[one]){

                option1 = {};

                children2 = [];

                two = ctdata[one][j];

                if(typeof ctdata[two]!='undefined' && ctdata[two].length>0){

                    for(let k in ctdata[two]){

                        three = ctdata[two][k];

                        children2.push({value:three.toString(),label:cndata[three]});

                    }
                }

                option1 = {value:two.toString(),label:cndata[two]};

                if(children2.length>0){
                    option1.children = children2;
                }

                children.push(option1);
            }
        }

        option = {value:one.toString(),label:cndata[one],children:children};

        if(children.length>0){
            option.children = children;
        }

        options.push(option);
    }

    return options;
}

//Cascader 职能级联选择器 options数据处理
function jobCascader(obj={}){

    var jidata = jtdata = jndata = options = [];

    if(typeof ji!='undefined' && ji!='new Array()' && ji.length>0){
        jidata = ji;
    }
    if(typeof jt!='undefined' && jt!='new Array()' &&  jt.length>0){
        jtdata = jt;
    }
    if(typeof jn!='undefined' && jn!='new Array()' &&  jn.length>0){
        jndata = jn;
    }

    var one = two = three = null;
    var children = children2 = [];
    var option = option1 = {};

    for(let i in jidata){

        option = {};

        one = two = three = null;

        children = children2 = [];

        one = jidata[i];

        if(typeof jtdata[one]!='undefined' && jtdata[one].length>0){

            for(let j in jtdata[one]){

                option1 = {};

                children2 = [];

                two = jtdata[one][j];

                if(typeof jtdata[two]!='undefined' && jtdata[two].length>0){

                    for(let k in jtdata[two]){

                        three = jtdata[two][k];

                        children2.push({value:three,label:jndata[three]});

                    }
                }

                option1 = {value:two,label:jndata[two]};

                if(children2.length>0){
                    option1.children = children2;
                }

                children.push(option1);
            }
        }

        option = {value:one,label:jndata[one],children:children};

        if(children.length>0){
            option.children = children;
        }

        options.push(option);
    }

    return options;
}

//Cascader 职能级联选择器 设置默认Value处理
function jobCasaderValue(idStr = null) {

    if (idStr != null) {

        var idArr  =   idStr.split(',');

        var jiData = jtData = jnData = defValue = [];

        if (typeof ji != 'undefined' && ji != 'new Array()' && ji.length > 0) {
            jiData = ji;
        }
        if (typeof jt != 'undefined' && jt != 'new Array()' && jt.length > 0) {
            jtData = jt;
        }
        if (typeof jn != 'undefined' && jn != 'new Array()' && jn.length > 0) {
            jnData = jn;
        }

        var one = two = three = null;

        for (let i in jiData) {
            if (idArr.indexOf(jiData[i]+'') > -1) {

                defValue.push([jiData[i]]);
            }else{

                one = jiData[i];

                if(typeof jtData[one]!='undefined' && jtData[one].length>0){

                    for(let j in jtData[one]){

                        if (idArr.indexOf(jtData[one][j]+'') > -1) {

                            defValue.push([one ,jtData[one][j]]);
                        }else{

                            two = jtData[one][j];

                            if(typeof jtData[two]!='undefined' && jtData[two].length>0){
                                for(let k in jtData[two]){

                                    if (idArr.indexOf(jtData[two][k]+'') > -1) {

                                        defValue.push([one, two, jtData[two][k]]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        return defValue;
    }
}

//select 职能/城市类别搜索 获取可选项
function jobcityclass_slist(obj={}){

    var keyword = '';//搜索关键词
    var type = '';//jobclass，cityclass
    var choosed = [];//已选项

    if(typeof obj.keyword!='undefined' && obj.keyword.trim()!=''){
        keyword = obj.keyword;
    }
    if(typeof obj.type!='undefined'){
        type = obj.type;
    }
    if(typeof obj.choosed!='undefined'){
        choosed = obj.choosed;
    }

    var fsArr   =   [],
        thisclass   =   [],
        fs_parent   =   [],
        fsi         =   [],
        fst         =   [],
        fsn         =   [],
        fsone       =   [],
        fstwo       =   [],
        fsthr       =   [],
        rfs         =   [];

    if(type=='jobclass'){
        if(typeof ji!='undefined' && ji!='new Array()' && ji.length>0){
            fsi = ji;
        }
        if(typeof jt!='undefined' && jt!='new Array()' &&  jt.length>0){
            fst = jt;
        }
        if(typeof job_parent!='undefined' && job_parent!='new Array()' &&  job_parent.length>0){
            fs_parent = job_parent;
        }
        if(typeof jn!='undefined' && jn!='new Array()' &&  jn.length>0){
            fsn = jn;
        }
    }else if(type=='cityclass'){
        if(typeof ci!='undefined' && ci!='new Array()' && ci.length>0){
            fsi = ci;
        }
        if(typeof ct!='undefined' && ct!='new Array()' &&  ct.length>0){
            fst = ct;
        }
        if(typeof city_parent!='undefined' && city_parent!='new Array()' &&  city_parent.length>0){
            fs_parent = city_parent;
        }
        if(typeof cn!='undefined' && cn!='new Array()' &&  cn.length>0){
            fsn = cn;
        }
    }



    var oneselected =twoselected = threeselected= false;

    var isSelected = function(val){
        val = parseInt(val);
        if(choosed.length>0 && choosed.indexOf(val)!=-1){
            return true;
        }else{
            return false;
        }
    };

    if(keyword!=''){

        if(fsn.length>0){
            var keyword  =   keyword.toLowerCase(),
                itemv   =   '';
            fsn.forEach(function(item,index){
                itemv   =   item.toString().toLowerCase();

                if(itemv.indexOf(keyword)!= -1){//当前级（可为1/2/3级）
                    thisclass.push(index);
                }
            })
        }


        if(thisclass.length>0){
            for(var i=0;i<thisclass.length;i++){

                var t=thisclass[i];
                for(var lev = 1;fs_parent[t]>0;t = fs_parent[t]){
                    lev++;
                }

                if(lev==1){
                    fsone.push(thisclass[i]);
                }else if(lev==2){
                    fstwo.push(thisclass[i]);
                }else{
                    fsthr.push(thisclass[i]);
                    rfs.push({'three':thisclass[i],'two':fs_parent[thisclass[i]],'one':fs_parent[fs_parent[thisclass[i]]]});
                }

            }
            if(type=='jobclass'){
                fsone = [];
            }


            if(fsone.length>0){

                var hastwo =  false;
                for(var i=0;i<fsone.length;i++){
                    hastwo = false;

                    oneselected =twoselected = threeselected= false;

                    oneselected = isSelected(fsone[i]);

                    fsArr.push({"name":fsn[fsone[i]],"value":fsone[i],"selected":oneselected,"disabled":false,"upname":''});
                    //江苏
                    if(fst && fst.length>0 && fst!='new Array()'){
                        for(var j=0;j<fst[fsone[i]].length;j++){//先判断选项里是否有二级属于此一级
                            if(fstwo.indexOf(parseInt(fst[fsone[i]][j]))!=-1){
                                hastwo = true;
                            }
                        }
                    }

                    if(hastwo){//有二级
                        if(fstwo.length>0){
                            for(var m=0;m<fstwo.length;m++){

                                twoselected = isSelected(fstwo[m]);

                                if(fst[fsone[i]] && (fst[fsone[i]].indexOf(fstwo[m])!=-1 || fst[fsone[i]].indexOf(fstwo[m].toString())!=-1)){
                                    fsArr.push({"name":fsn[fstwo[m]],"value":fstwo[m],"selected":twoselected,"disabled":oneselected,"upclass":1,"upname":''});

                                    if(fsthr.length>0){
                                        for(var t=0;t<fsthr.length;t++){

                                            threeselected = isSelected(fsthr[t]);

                                            if(fst[fstwo[m]] && (fst[fstwo[m]].indexOf(fsthr[t])!=-1 || fst[fstwo[m]].indexOf(fsthr[t].toString())!=-1)){
                                                fsArr.push({"name":fsn[fsthr[t]],"value":fsthr[t],"selected":threeselected,"disabled":(oneselected || twoselected),"upclass":2,"upname":''});
                                                //江苏-宿迁-沭阳
                                                fsthr.splice(t,1);
                                                t--;
                                            }
                                        }
                                    }
                                    fstwo.splice(m,1);
                                    m--;
                                }
                            }
                        }
                    }else{
                        if(rfs.length>0){
                            rfs.forEach(function(item,index){
                                if(parseInt(item.one) == parseInt(fsone[i])){//

                                    threeselected = isSelected(item.three);

                                    fsArr.push({"name":fsn[item.three],"value":item.three,"selected":threeselected,"disabled":oneselected,"upclass":1,"upname":fsn[item.two]});
                                    //江苏-沭阳
                                    fsthr.splice(fsthr.indexOf(parseInt(item.three)),1);
                                }
                            })
                        }
                    }
                }
            }

            if(fstwo.length>0){
                for(var m=0;m<fstwo.length;m++){

                    oneselected =twoselected = threeselected= false;

                    oneselected = isSelected(fs_parent[fstwo[m]]);

                    twoselected = isSelected(fstwo[m]);

                    fsArr.push({"name":fsn[fstwo[m]],"value":fstwo[m],"selected":twoselected,"disabled":oneselected,"upname":fsn[fs_parent[fstwo[m]]]});
                    //宿迁
                    if(fsthr.length>0){
                        for(var t=0;t<fsthr.length;t++){
                            if(fst[fstwo[m]] && (fst[fstwo[m]].indexOf(fsthr[t])!=-1 || fst[fstwo[m]].indexOf(fsthr[t].toString())!=-1)){

                                threeselected = isSelected(fsthr[t]);

                                fsArr.push({"name":fsn[fsthr[t]],"value":fsthr[t],"selected":threeselected,"disabled":(oneselected || twoselected),"upclass":1,"upname":''});
                                //宿迁-沭阳
                                fsthr.splice(t,1);
                                t--;
                            }
                        }
                    }

                }
            }

            if(fsthr.length>0){

                var onev = twov = 0;

                for(var t=0;t<fsthr.length;t++){

                    twov = fs_parent[fsthr[t]];

                    onev = fs_parent[twov];

                    oneselected =twoselected = threeselected= false;

                    oneselected = isSelected(onev);

                    twoselected = isSelected(twov);

                    threeselected = isSelected(fsthr[t]);

                    fsArr.push({"name":fsn[fsthr[t]],"value":fsthr[t],"selected":threeselected,"disabled":(oneselected || twoselected),"upname":fsn[fs_parent[fsthr[t]]]});
                    //沭阳
                }
            }

        }
    }else{

        //只有一级类别时，点击输入框直接展示所有选项
        if(!(fst && fst.length>0)){

            fsone = fsi;

            for(var i=0;i<fsone.length;i++){

                oneselected = isSelected(fsone[i]);

                fsArr.push({"name":fsn[fsone[i]],"value":fsone[i],"selected":oneselected,"disabled":false,"upname":''});
            }

        }
    }

    return fsArr;
}

// 将图片转成base64
function getBase64Image(img) {
    var canvas = document.createElement("canvas");
    canvas.width = img.width;
    canvas.height = img.height;
    var ctx = canvas.getContext("2d");
    ctx.drawImage(img, 0, 0, img.width, img.height);
    var dataURL = canvas.toDataURL("image/png");
    return dataURL;
}
//手机号格式检测
function isjsMobile(obj) {
	var reg= /^[1][3456789]\d{9}$/;

    if (obj.length != 11) return false;
    else if (!reg.test(obj)) return false;
    else if (isNaN(obj)) return false;
    else return true;
}

// 校验参数是否为空
function isEmpty(val) {
    if (val === undefined || $.trim(val) === '') {
        return true;
    }
    return false;
}

function toDate(str){
    var sd=str.split("-");
    return new Date(parseInt(sd[0]),parseInt(sd[1]),parseInt(sd[2]));
}

//对象数组 按某个字段 排序
function sortByField(arr,field,sort){

	return arr.sort(function(a,b){
		if(sort=='desc'){
			return b[field]-a[field];
		}else if(sort=='asc'){
			return a[field]-b[field];
		}
	});
}

function formatMoney(value,row,item,_this){
	let val = (value && value.split("")) || [];
	let sNum = val.toString(); //先转换成字符串类型
	if (sNum.indexOf('.') === 0) {//第一位就是 .
		sNum = '0' + sNum
	}
	sNum = sNum.replace(/[^\d.]/g,"");  //清除“数字”和“.”以外的字符
	sNum = sNum.replace(/\.{2,}/g,"."); //只保留第一个. 清除多余的
	sNum = sNum.replace(".","$#$").replace(/\./g,"").replace("$#$",".");
	sNum = sNum.replace(/^(\-)*(\d+)\.(\d\d).*$/,'$1$2.$3');//只能输入两个小数
	//以上已经过滤，此处控制的是如果没有小数点，首位不能为类似于 01、02的金额
	if(sNum.indexOf(".")< 0 && sNum !==""){
		sNum = parseFloat(sNum);
	}
	_this.$set(row,item,sNum)
}

function scrollToTop(container = '.moduleDome'){
    let ele = document.querySelector(container)
    ele.scrollTo({
        top: 0,
        behavior: 'smooth'
    })
}