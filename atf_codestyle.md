[回到主页](/知识分享/RTOS)

---
# RTOS编程规范

[[_TOC_]]

本规范旨在提高RTOS代码的可维护性、可阅读性，涉及编码过程中的各种命名方式、格式定义等，适用于RTOS软件的编程（C语言）。
规范按照 [RFC2119](https://datatracker.ietf.org/doc/html/rfc2119) 中定义的 IETF 标准使用“_MUST_”、“_MUST NOT_”、“_REQUIRED_”、“_SHALL_”、“_SHALL NOT_”、“_SHOULD_”、“_SHOULD NOT_”、“_RECOMMENDED_”、“_MAY_”和“_OPTIONAL_”字样。
目前只覆盖ZEKU自研的代码（ZEKU从头开发的代码），对于在外部SDK、库等即有代码上的修改，暂不做强制要求，跟随即有代码风格即可。


## 1	格式
### 1.1	缩进
必须使用4个空格字符缩进[_MUST_]，不得使用Tab字符[_MUST NOT_]。

如果你的编辑器是vim，可以在你的.vimrc里添加如下设置：
```
set ts=4
set sw=4
set sts=4
set ai
set si
```
例外：switch-case语句中， switch和其从属的case语句“必须”要对齐[_MUST_]，例如：

```c
switch (suffix) {
case 'G':
case 'g':
    mem <<= 30;
    break;
default:
    break;
}

```

## 1.2	长行截断
### 1.2.1	每行长度原则上不会超过80个字符[_RECOMMENDED_]
以下情形例外：
1)	消息打印接口，字符串参数可不截断，便于根据log搜索源码。
2)	其他可显著增加可读性的情况。
### 1.2.2	截断原则
1)	截断到下一行的长度小于上一行的长度；
2)	截断的每一个片段需要有一定的意义，不要随机截断；
3)	截断的片段明显短于原语句，且位置明显靠右；
4)	对于函数参数列表，截断到下一行的参数与上一行的参数位置对齐。
## 1.3	花括号
花括号的位置约定：
起始括号在一行的最后位置，结束括号在一行的开始位置[_MUST_]。例如：
	
```
if (x is true) {
    we do y;
}
```

例外：对于函数，起始括号也要在一行的开始位置，单独一行[_MUST_]。
	
```c
int function(int x)
{
    body of function;
}
```

特别地，条件判断后面的语句块一律用花括号括起来，无论其后的语句块是单语句还是多行[_SHOULD_]。此类语句包含：if-else、while、for等等

```c
if (condition) {
    do_this();
} else {
    do_that();
}
```

## 1.4	空格和TAB
本规范中，原则上编码过程中任何情况都不使用Tab作为隔离和缩进[_SHOULD NOT_]。
### 1.4.1	在绝大多数关键词后面使用空格 [_SHOULD_]
例如：
`if, switch, case, for, do, while`
除了
`sizeof, typeof, alignof, __attribute__`
这些使用括号的关键字，例如：
	`s = sizeof(struct file);`
也不要在括号里添加空格。
### 1.4.2	指针变量或指针函数，“*”字符要紧邻变量名或函数名 [_MUST_]
例如：

```c
uint32_t *linux_banner;  
uint64_t memparse(char *ptr, char **retptr);  
char *match_strdup(substring_t *s);
```
  
### 1.4.3	二元或三元操作符，符号两边使用空格 [_MUST_]
例如：
`=  +  -  <  >  *  /  %  |  &  ^  <=  >=  ==  !=    ? :`
### 1.4.4	一元操作符号不要加空格 [_MUST NOT_]
例如：
	`&  *  ++  --  ~  !  sizeof   typeof   alignof   __attribute__   defined`
### 1.4.5	结构体成员访问操作符号 “.”和”->” 不要加空格 [_MUST NOT_]
### 1.4.6	函数有多个入参时，参数列表中逗号后面要加空格 [_MUST_]
### 1.4.7	函数调用、声明和定义时，函数名与函数列表的左括号间不要换行、不要有空格 [_MUST NOT_]
### 1.4.8	一行的末尾不要加空格或Tab [_MUST NOT_]
### 1.4.9	空行不允许有空格或Tab [_MUST NOT_]
## 1.5	C文件布局
在C文件内，文件包含、宏定义、类型定义、函数声明、全局变量声明、全局变量定义等函数体以外的代码，集中放置在所有函数体的上面（文件头部），并请依次、分类放置[_REQUIRED_]。不要嵌入各个函数体之间[_NOT RECOMMENDED_]。各类声明和定义之间及函数体之间请加空行区隔[_RECOMMENDED_]。例如：

```c
#include <rthw.h>
#include <rtthread.h>

#define IDLE_THREAD_STACK_SIZE  128
#define _CPUS_NR                RT_CPUS_NR

extern rt_list_t rt_thread_defunct;

static struct rt_thread idle[_CPUS_NR];
static rt_uint8_t rt_thread_stack[_CPUS_NR][IDLE_THREAD_STACK_SIZE];


rt_err_t rt_thread_idle_sethook(void (*hook)(void))
{  
    rt_size_t i;
    rt_base_t level;
    rt_err_t ret = -RT_EFULL;

    /* disable interrupt */
    level = rt_hw_interrupt_disable();

    for (i = 0; i < RT_IDLE_HOOK_LIST_SIZE; i++) {  
        if (idle_hook_list[i] == RT_NULL) {  
            idle_hook_list[i] = hook;
            ret = RT_EOK;
            break;
        }
    }
    /* enable interrupt */
    rt_hw_interrupt_enable(level);

    return ret;
}

```


# 2	命名
## 2.1	命名基本原则
所有命名，都请遵循如下基本原则[_MUST_]:
1.	使用英文，可用数字（如无必需，禁止拼音）
2.	禁止大小写混合（即，不使用驼峰式命名法和帕斯卡命名法）
3.	单词之间用下划线连接（“_”）
4.	具备一定的含义，禁止无意义的名称（如foo，abc等）
5.	名称意义要精准，刚好覆盖要表达的范围。要“特指”、不要“泛指”，不要“大而不当”，也不要“以偏概全”。（如，要表达的内容全是跟“张江”有关的，命名就不要用“上海”）
6.	可使用领域规范的缩写（如wdt，i2c，pll等），但尽量不要自定义缩写（如确有需要，应进行注释说明）
## 2.2	文件夹和文件的命名
文件夹和文件的命名，除遵循基本命名原则以外，另有如下约定：
1.	全部使用小写英文[_MUST_]
2.	精炼出关键词，不宜过长（建议25个字符以内）[_RECOMMENDED_]
3.	文件名添加表征层级的前缀，如“drv_” [_RECOMMENDED_]
## 2.3	函数名
函数命名，遵循基本命名原则以外，另有如下原则：
1.	全部使用小写英文[_MUST_]
2.	对外接口加上表征层级的前缀，如“hal_” [_RECOMMENDED_]
3.	长度原则上不超过35个字符[_RECOMMENDED_]
4.	函数命名必须体现函数的功能，并且含义准确。[_MUST_]
5.	建议的命名格式为: [_RECOMMENDED_]
     _[模块]_[层级]_[功能]_[动作]_[动作内容]_
 例如：
     _ahbdma_hsctl_set: 标识ahbdma模块的hsctl寄存器的set函数_
## 2.4	变量名
变量命名，遵循基本命名原则以外，另有如下规则：
1.	全部使用小写英文[_MUST_]
2.	长度原则上不超过15个字符[_RECOMMENDED_]
3.	变量名必须具备一定的意义，且意义准确[_MUST_]
## 2.5	 宏和枚举的命名
所有常量都以大写英文命名，除此之外以小写英文命名。需要遵循基本原则，另有如下约定：
1.	枚举全部用大写字母[_MUST_]
2.	常量和字符串的宏名称，用大写字母[_MUST_]
3.	宏定义的代码段（所谓的“宏函数”），使用小写字母命名[_MUST_]
_注：某些OS内建的声明和定义，延用OS的写法，使用大写或小写字母。如：_

```c
INIT_BOARD_EXPORT(rt_hw_pin_init);  
INIT_DEVICE_EXPORT(rt_hw_macb_init);
```
  
## 2.6	其他命名
其他未尽情况，请遵循命名的基本原则，使用小写英文命名。

# 3	编程原则
## 3.1	函数
函数请遵从如下几点：
1)	函数应该简短而漂亮，只完成一件事情。请控制在150行代码以内；[_MUST_]
2)	本地变量的数量，尽量在7个以内，不要超过10个。[_RECOMMENDED_]
3)	函数入参不宜太多，尽量在3个以内，不要超过5个；[_RECOMMENDED_]
4)	如果函数入口参数是空，必须使用 void 作为入口参数；[_MUST_]
5)	如果函数返回值为空，必须在定义和声明时在函数名前加void；[_MUST_]
6)	请谨慎使用static修饰的本地变量；[_RECOMMENDED_]
7)	确认为本文件内使用的函数，请习惯性给函数定义加static；[_REQUIRED_]
8)	提供给外部调用的函数，请在头文件中声明；[_MUST_]
9)	请谨慎构造和调用“不可重入函数”；[_RECOMMENDED_]
## 3.2	全局变量
全局变量在C程序中比较常见，也比较容易出问题。建议如下几点：
1)	如非必要，尽量不要使用全局变量；
2)	尽量不要跨文件引用全局变量（建议）；[_RECOMMENDED_]
3)	确认不会跨文件的全局变量，请加static修饰；[_REQUIRED_]
4)	已经赋初值的全局变量、数组、结构体等，如确认代码运行期不会修改，请加上const 修饰；[_REQUIRED_]
## 3.3	TYPEDEF
数据类型定义，在C程序中比较常见，请遵从如下几点：
1)	不要用typedef定义结构体（和联合体）；[_SHOULD NOT_]
2)	不要用typedef定义指针类型（为便于维护，可以在某些情况下用typedef定义函数指针类型）；[_SHOULD NOT_]
3)	为了实现更好的跨平台兼容，不要直接使用int、long等类型。（对某些库函数不做强制要求）[_SHOULD NOT_]
建议使用typedef依据当前的CPU位宽自行定义16、32、64位整型数据类型。例如，若使用了stdlib，可直接用uint8_t、uint16_t、int32_t、size_t等。
## 3.4	GOTO
请谨慎使用goto，一般只有一种场景使用：从一个函数内部多个地方返回时，要做一些资源释放、清理工作。
若无需释放资源时，直接返回。
## 3.5	注释
### 3.5.1 注释原则
注释请遵从如下原则：
1)	注释风格是C89 “/* … */” [_MUST_]，不要用C99风格“//…”[_MUST NOT_]
2)	不要过度注释。注释是说明代码做了什么（what），而非怎么做（how）；
3)	不在注释中使用Tab
4)	多行注释时，从第二行开始，每行以“*”开头，如：
 
```c
/** 
* @ingroup Hook 
* This function sets a hook function to idle thread loop. When the system performs 
* idle loop, this hook function should be invoked. 
* 
* @param hook the specified hook function 
* 
* @return RT_EOK: set OK 
*         -RT_EFULL: hook list is full 
* 
* @note the hook function must be simple and never be blocked or suspend. 
*/
```
### 3.5.2 版权声明
zeku自有版权代码请加版权声明，其他版权代码请参照具体版权方声明书写([此处](/团队信息分享/SW1/Debug&Stability-Group/Upstream-Info/3.-license-template-for-upstream)有详细介绍)。

C语言类文件头：
```c
/*
 * Copyright (c) 2020-2021 ZEKU Technology(Shanghai) Corp., Ltd.
 * All Rights Reserved
 */
```
其他以“#”开头的注释，范例：
```python
#
# Copyright (c) 2020-2021 ZEKU Technology(Shanghai) Corp., Ltd.
# All Rights Reserved
#
```

## 3.6	头文件
请在h文件中加入重复包含的判断[_MUST_]。例如：

```c
#ifndef __PCC_H__
#define __PCC_H__
  /*blablabla....*/
#endif /* __PCC_H__ */
```


即，添加ifndef，define，等。注意不要重名。
## 3.7	其他
其他补充杂项：
1)	不要出现“死代码”[_SHOULD NOT_]，包括但不限于 #if 0，if 0，使用“/*…*/”、“//…”注释掉的代码等等；
2)	不要出现明显无意义的判断[_SHOULD NOT_]，如if 1，if TRUE，#if 1 等等；
3)	不要在Log中出现人名[_MUST NOT_]。

---
[回到主页](/知识分享/RTOS)
