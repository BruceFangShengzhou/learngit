# ATF代码风格

本文档描述了ATF C语言代码的代码风格，基于[Linux内核代码风格](https://www.kernel.org/doc/html/v4.10/process/coding-style.html)做了一些修改。

代码风格是因人而异的，而且是与时俱进的，如对这个风格有任何建议，麻烦反馈给我们。

## 目录

-  [第一章：文件编码](#toc_17266_29298_1)
-  [第二章：语言](#toc_17266_29298_2)
-  [第三章：C语言标准](#toc_17266_29298_3)
-  [第四章：MISRA Compliance](#toc_17266_29298_4)
-  [第五章：缩进](#toc_17266_29298_5)
-  [第六章：空格](#toc_17266_29298_6)
-  [第七章：代码行长度](#toc_17266_29298_7)
-  [第八章：空行](#toc_17266_29298_8)
-  [第九章：花括号](#toc_17266_29298_9)
-  [第十章：命名](#toc_17266_29298_10)
-  [第十一章：函数属性](#toc_17266_29298_11)
-  [第十二章：对齐](#toc_17266_29298_12)
-  [第十三章：注释](#toc_17266_29298_13)
-  [第十四章：头文件和包含关系](#toc_17266_29298_14)
-  [第十五章：Typedefs](#toc_17266_29298_15)

<span id="toc_17266_29298_1"></span>
## 第一章: 文件编码
---------------------
所有代码必须使用**UTF-8**字符格式编码。在某些情况下，注释和文档可以使用非 ASCII 字符
（例如用于单位的希腊字母），但代码本身仍然限于 **ASCII** 字符。

换行符必须是**Unix**风格，这意味着只有LF换行符(``LF``) 可以用于换行并重置为第一列。
<span id="toc_17266_29298_2"></span>
## 第二章: 语言
---------------------
注释和命名的首选语言是国际英语。
如果某个单词的美式英语和英式英语拼写存在冲突，则使用美式英语拼写。

例外地，当直接提及不使用国际风格的东西时，例如公司名称，应按原样使用现有名称。

<span id="toc_17266_29298_3"></span>
## 第三章: C语言标准
---------------------

ATF使用的C语言模式是GNU99，这是“ISO C99 的 GNU 方言”，这意味着带有 GNU 扩展的ISO C99标准。
GCC 和 Clang 编译器工具链都支持GNU99模式，但 Clang 确实缺乏对少数 GNU 扩展的支持。然而，这些缺失的扩展很少使用，应该不会造成问题。

<span id="toc_17266_29298_4"></span>
## 第四章: MISRA合规性
---------------------
ATF试图遵守[MISRA C:2012 Guidelines](https://www.misra.org.uk/misra-c/)。
Coverity 静态分析用于定期生成当前 MISRA 缺陷的报告并防止添加新缺陷。

项目不可能遵循所有 MISRA规则。
ATF社区维护了一个[电子表格](https://developer.trustedfirmware.org/file/download/lamajxif3w7c4mpjeoo5/PHID-FILE-fp7c7acszn6vliqomyhn/MISRA-and-TF-Analysis-v1.3.ods)，其中列出了所有规则和指令以及我们是否打算遵守它们。
每一个例外都给出了相应的原因。

  **注意** 
  - 强制执行规则并不意味着代码库没有该规则的缺陷，只是在理想情况下它们会被删除。

  **例外**
  - ATF的MISRA分析不考虑第三方库，也不打算修改它们以使其符合MISRA规范。

<span id="toc_17266_29298_5"></span>
## 第五章: 缩进
---------------------
使用制表符进行缩进。禁止使用空格进行缩进，除非使用制表符无法达到专用术语的期望缩进。

制表符间距应设置为8 个字符。

行尾空格是不允许的，必须删除。

<span id="toc_17266_29298_6"></span>
## 第六章: 空格
---------------------
在大多数运算符后面使用空格, 包括:
- 算术运算符 (``+``, ``-``, ``/``, ``*``)
- 赋值运算符 (``=``, ``+=``等)
- 布尔运算符 (``&&``, ``||``)
- 比较运算符 (``<``, ``>``, ``==``等)

当括号和花括号没有换行符分隔时，也应使用空格分隔它们，比如以下示例中的``if``语句：

     int function_foo(bool bar)
     {
         if (bar) {
             function_baz();
         }
     }

  **注意**
  - 函数名与其后的左括号之间没有空格。

控制语句（``if``, ``for``, ``switch``, ``while``等）与后面的左括号必须用一个空格隔开，如上例所示。

<span id="toc_17266_29298_7"></span>
## 第七章: 代码行长度
---------------------
单行代码长度不应超过80个字符。此限制不包括非打印字符，例如换行符。

这条规则是建议的，而不是必须的，稍微超出限制是可以接受的， 否则代码的可读性会显着降低。
为了代码的可读性，可以根据具体情况来进行判断。

<span id="toc_17266_29298_8"></span>
## 第八章: 空行
---------------------
通常，函数之间使用一个空行分隔。在必要情况下，为了增强代码的可读性，可以使用额外的空行。

文件必须以单个换行符结尾。许多编辑器可以选择自动插入并在文件末尾删除多余空行。
<span id="toc_17266_29298_9"></span>
## 第九章: 大括号
---------------------
### 左括号

遵循**Kernighan and Ritchie (K&R)风格，我们把大括号的左括号放在行尾，而把大括号的右括号放在行首，如下是一个'while'循环表达式：
   ``` code c
   while (condition) {
      foo();
      bar();
   }
   ```
这适用于所有除函数外的代码块，和Linux风格保持一致，将函数定义的左括号放在下一行的行首。
如下是一个函数代码块的例子：
   ``` code c
   int my_function(void)
   {
      int a;

      a = 1;
      return a;
   }
   ```
### 条件语句

在条件语句 (如'if', ``for``, ``while`` and ``do``) 中，必须使用大括号包含构成条件主题的语句，不管语句主题是有几行。

  **注意**
  - 在这一点上，为了尽量减少编码错误，ATF代码规范选择适用MISRA规范，而不是Linux编码规范。

ATF代码规范建议如下：
   ``` code c
   if (condition) {
      foo++;
   }
   ```
在单行语句时不使用大括号，是ATF代码不建议的方式：
   ``` code c
   /* This is violating MISRA C 2012: Rule 15.6 */
   if (condition)
      foo++;
   ```
这样做的原因是为了防止在修改条件主体时意外修改了代码的控制流。
如下图代码所示，从下图中的代码，我们很容易认为仅在``condition``为真时，变量``bar``的值才会自加1。
但实际上，变量``bar``的值的自加1不受条件表达式的控制。
当开发人员在添加``bar++;``语句时忘记补上大括号，则程序并不会按照开发人员的预期来执行。
   ``` code c
   /* This is violating MISRA C 2012: Rule 15.6 */
   if (condition)
      foo++;
      bar++;
   ```
<span id="toc_17266_29298_10"></span>
## 第十章: 命名规则
---------------------
### 函数

函数名使用小写字母，用下划线 (``_``)分隔多个单词。例如：

   ``` code c
   void bl2_arch_setup(void)
   {
      ...
   }
   ```

### 局部变量和参数

局部变量和函数参数使用与函数名称相同的格式：小写，多个单词之间用下划线分隔。例如：

   ``` code c
   static void set_scr_el3_from_rm(uint32_t type,
                                   uint32_t interrupt_type_flags,
                                   uint32_t security_state)
   {
      uint32_t flag, bit_pos;
      ...
   }
   ```

### 预编译宏

宏定义的标识符全部使用大写字母。

  ``` code c
  #define BUFFER_SIZE_BYTES 64
  ```
<span id="toc_17266_29298_11"></span>
## 第十一章: 函数属性
---------------------
函数属性放在函数类型之后和函数名称之前。

   ``` code c
   void __init plat_arm_interconnect_init(void);
   ```

<span id="toc_17266_29298_12"></span>
## 第十二章: 对齐
---------------------
使用制表符进行对齐，如果需要，可以添加空格以实现小于制表符大小的粒度。
例如，如果制表符大小为8个字符，为了实现10个字符对齐，则需要使用一个制表符和两个空格来实现。
### Switch 语句对齐

在``switch``语句中，每个``case``语句与``switch``对齐，如下示例：
   ``` code c
  switch (condition) {
  case A:
      foo();
  case B:
      bar();
  default:
      baz();
  }
  ```

### 指针对齐

引用和解引用运算符（``&``和``*``）必须靠近变量名或者函数名，而不是靠近类型名。
   ``` code c
   uint8_t *foo;

   foo = &bar;
   ```

<span id="toc_17266_29298_13"></span>
## 第十三章: 注释
---------------------
不允许使用双斜线(``//``)的注释。允许的注释格式示例如下：   
   ``` code c
  /*
   * This example illustrates the first allowed style for multi-line comments.
   *
   * Blank lines within multi-lines are allowed when they add clarity or when
   * they separate multiple contexts.
   *
   */
   ```

  ``` code c
  /**************************************************************************
   * This is the second allowed style for multi-line comments.
   *
   * In this style, the first and last lines use asterisks that run the full
   * width of the comment at its widest point.
   *
   * This style can be used for additional emphasis.
   *
   *************************************************************************/
  ```
  ``` code c
  /* Single line comments can use this format */
  ```
  ``` code c
  /***************************************************************************
   * This alternative single-line comment style can also be used for emphasis.
   **************************************************************************/
  ```
<span id="toc_17266_29298_14"></span>
## 第十四章: 头文件和包含关系
---------------------
### 头文件格式

头文件格式以文件``"some_driver.h"``为例:

  ``` code c
  #ifndef SOME_DRIVER_H
  #define SOME_DRIVER_H

  <header content>

  #endif /* SOME_DRIVER_H */
  ```
### 头文件排序规则

为了提高代码的可阅读性，方便开发人员维护，头文件引用需要遵循以下分级排序规则：
1. 系统头文件：包括标准的C语言头文件，比如``stddef.h``、``string.h``。
2. 项目头文件：ATF中的项目头文件在根目录下一级目录``include/``目录下。
3. 平台头文件：ATF中的平台头文件是指某一特定平台相关的头文件，在``plat/<platform_name>``目录下。

在同级下，头文件和目录按照字母序排序。

为了方便阅读，不同级的头文件使用空行分隔。

下图是遵循ATF头文件排序规则的一个示例：
  ``` code c
  #include <string.h>

  #include <a_dir/example/a_header.h>
  #include <a_dir/example/b_header.h>
  #include <a_dir/test/a_header.h>
  #include <b_dir/example/a_header.h>

  #include "a_header.h"
  ```
### 头文件书写

下图中的两种头文件书写格式在ATF代码中都是可以接受的。

头文件在同级目录下时，使用``"..."``引用格式。

头文件不在同级目录下时，使用``<...>``引用格式。

  ``` code c
  #include <assert.h>
  #include <errno.h>
  #include <string.h>

  #include "bl1_private.h"
  ```
<span id="toc_17266_29298_15"></span>
## 第十五章: Typedefs
---------------------
### 避免语意不明的typedef定义
如下图示例中的定义是不推荐的：
  ``` code c
  typedef struct {
          int arg1;
          int arg2;
  } my_struct_t;
  ```
下面的定义是更优的：
  ``` code c
  struct my_struct {
          int arg1;
          int arg2;
  };
  ```
一些ATF中的定义同时使用struct/enum类型名称和typedef类型名称。
对于新增的代码，ATF编码规范不建议这样做，因为这违背了MISRA规则-8.3。
该规则规定“对象或函数的所有声明都应使用相同的名称和类型限定符”。

Linux 编码标准也不鼓励新增typedef，并且checkpatch工具会报``warning``。

为了兼容性考虑，对已有的typedef不做要求。

*Copyright (c) 2020, Arm Limited. All rights reserved.*
