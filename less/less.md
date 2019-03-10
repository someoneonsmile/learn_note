## less 学习记录

### 语法

* 变量
     ```less
     @nice-blue: #5B83AD;
     @light-blue: @nice-blue + #111;
     #header {color: @light-blue;}
     ```
    > ### 甚至可以用变量名定义为变量
     ```less
     @fnord: "I am fnord";
     @var: 'fnord';
     content: @@var;
     ```
     > ### 注意: less 中的变量为完全的"常量", 所以只能定义一次

* 类可以直接在别的类中调用
     ```less 
     .bordered {
       border-top: dotted 1px black;
       border-bottom: solid 2px black;
     } 
     ```
     > ### 当需要在其它类中引入时只需要像下面这样调用就可以了
     ```less
      #menu a{
         color: #111;
         .bordered;
     }
     ```

* 带参数混合
    > 像定义函数一样定义一个带参数的属性集合
    ```less
    .border-radius (@radius) {
        border-radius: @radius;
        -moz-border-radius: @radius;
        -wekit-border-radius: @radius;
    }
    ```
    在其它class中像这样调用它:
    ```less
    #header {
        .border-radius (4px);
    }
    .button {
        .border-radius (6px);
    }
    ```
    > 给参数设置默认值
    ```less
    .border-radius (@radius: 5px) {
        border-radius: @radius;
        -moz-border-radius: @radius;
        -webkit-border-radius: @radius;
    }
    ```
    > 不带参数的属性集合

    作用: 如果你想隐藏这个属性集合, 不让它暴露到`css`中去,   
    但是你还想在其他的属性集合中引用, 你会发现这个方法非常的好用
     ```less
    .wrap () {
        text-wrap: wrap;
        white-space: pre-wrap;
        white-space: -moz-pre-wrap;
        word-wrap: break-word;
    }
    pre { .wrap }
     ```
     
* @arguments 变量
    > @arguments 包含了所有传递进来的参数,   
    > 如果你不想单独处理每一个参数的话就可以像这样写
    ```less
    .box-shadow (@x: 0, @y: 0, @blur: 1px, @color: #000) {
        box-shadown: @arguments;
        -moz-box-shadown: @arguments;
        -webkit-box-shadown: @arguments;
    }
    .box-shadow(2px, 5px);
    ```

* 模式匹配
    > 有些情况下 我们想根据传入的参数来改变混合的默认呈现
    ```less
    .mixin (dark, @color) {
        color: darken(@color, 10%);
    }
    .mixin (light, @color) {
        color: lighten(@color, 10%);
    }
    .mixin (@_, @color) {
        display: block;
    }
    ```
    现在, 如果运行:
    ```less
    @switch: light;
    .class {
        .mixin(@switch, #888);
    }
    ```
    就会得到下列css:
    ```less
    .class {
        color: #a2a2a2;
        display: block;
    }
    ```
    也可匹配多个参数
    ```less
    .mixin (@a) {
        color: @a;
    }
    .mixin (@a, @b) {
        color: fade(@a, @b);
    }
    ```
    > 对所有匹配的规则都会应用 
    
* 导引表达式
    > 当我们想根据表达式进行匹配, 而非根据值和参数匹配时, 导引就显得非常有用.  
    > 如果你对函数式编程非常熟悉, 那么你很可能已经使用过导引.  
    > 为了尽可能地保留css的可声明性, less通过导引混合而非 `if/else` 语句  
    > 来实现条件判断, 因为前者已在 `@media query` 特性中被定义
    ```less
    .mixin (@a) when (lightness(@a) >= 50%) {
        background-color: black;
    }
    .mixin (@a) when (lightness(@a) < 50%) {
        background-color: white;
    }
    .mixin (@a) {
        color: @a;
    }
    ```
    > 导引中 `when` 关键字用以定义一个导引序列
    
    导引序列使用逗号','分割, 当且仅当所有条件都符合时, 才会被视为匹配成功
    ```less
    .mixin (@a) when (@a > 10), (@a < -10){ ... }
    ```
    
    > 导引中可用的全部比较运算有: `> >= = =< <`  
    > 导引中 `true` 除关键字的值都被视为布尔假

    下面两种效果相同
    ```less
    .truth (@a) when (@a) {....}
    .truth (@a) when (@a = true) {....}
    ```

    > 导引中常见的检测函式
    ```less
    iscolor
    isnumber
    isstring
    iskeyword
    isurl
    ```
    如果想判断一个值是纯数字, 还是某个单位量, 可以用下列函式
    ```less
    ispixel
    ispercentage
    isem
    ```
    
* less 嵌套
    > 注意: `&` 符号的使用, 如果想写串联选择器, 而不是后代选择器, 就可以用到 `&`  
    > 这点对伪类尤其有用, 如 `:hover` 和 `:focus`.
    ```less
    .bordered {
        &.float {
            float: left; 
        }
        .top {
            margin: 5px; 
        }
    }
    ```

* Color 函数
    > 颜色会先被转化成 HSL 色彩空间, 然后在通道级别操作
    ```less
    lighten(@color, 10%);     // return a color which is 10% *lighter* than @color
    darken(@color, 10%);      // return a color which is 10% *darker* than @color

    saturate(@color, 10%);    // return a color 10% *more* saturated than @color
    desaturate(@color, 10%);  // return a color 10% *less* saturated than @color

    fadein(@color, 10%);      // return a color 10% *less* transparent than @color
    fadeout(@color, 10%);     // return a color 10% *more* transparent than @color
    fade(@color, 50%);        // return @color with 50% transparency

    spin(@color, 10);         // return a color with a 10 degree larger in hue than @color
    spin(@color, -10);        // return a color with a 10 degree smaller hue than @color

    mix(@color1, @color2);    // return a mix of @color1 and @color2
    ```
    提取颜色信息
    ```less
    hue(@color);        // returns the `hue` channel of @color
    saturation(@color); // returns the `saturation` channel of @color
    lightness(@color);  // returns the 'lightness' channel of @color
    ```

* Math 函数
    ```less
    round(1.67); // returns `2`
    ceil(2.4);   // returns `3`
    floor(2.6);  // returns `2`
    ```
    如果你想将一个值转化为百分比，你可以使用percentage 函数
    ```less
    percentage(0.5); // returns `50%`
    ```

* 命名空间
    > 有时候，你可能为了更好组织CSS或者单纯是为了更好的封装，  
    > 将一些变量或者混合模块打包起来, 你可以像下面这样  
    > 在#bundle中定义一些属性集之后可以重复使用:
    ```less
    #bundle {
    .button () {
        display: block;
        border: 1px solid black;
        background-color: grey;
        &:hover { background-color: white }
    }
    .tab { ... }
    .citation { ... }
    }
    ```
    你只需要在 #header a中像这样引入 .button:
    ```less
    #header a {
        color: orange;
        #bundle > .button;
    }
    ```

* Importing
    你可以在main文件中通过下面的形势引入 .less 文件,  
    .less 后缀可带可不带:
    ```less
    @import "lib.less";
    @import "lib";
    ```
    如果你想导入一个CSS文件而且不想LESS对它进行处理，只需要使用.css后缀就可以:
    ```less
    @import "lib.css";
    ```

* 字符串插值
    > 变量可以用 `@{name}` 这样的结构
    ```less
    @base-url: "http://assets.fnord.com";
    background-img: url("@{base-url}/image/bg.png");
    ```

* 避免编译
    > 有时候我们需要输出一些不正确的CSS语法  
    > 或者使用一些 LESS不认识的专有语法.  
    > 要输出这样的值我们可以在字符串前加上一个 ~,  
    > 将避免编译的值用 `""` 包含起来, 例如:

    ```less
    .class {
        filter: ~"ms:alwaysHasItsOwnSyntax.For.Stuff()";
    }
    ```

* JavaScript 表达式
    > JavaScript 表达式也可以在 .less 文件中使用, 可以通过反引号的方式使用

    ```less
    @var: `"hello".toUpperCase() + '!'`;
    ```

    > 注意: 也可以同时使用字符串插值和避免编译

    ```less
    @str: "hello";
    @var: ~`"@{str}".toUpperCase() + '!'`;
    ```
