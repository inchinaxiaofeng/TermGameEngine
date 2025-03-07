# TermGameEngine

TermGameEngine is a lightweight terminal game engine designed to help developers quickly build text-based games.

请你继续修改，我的要求如下：

核心组件（tge）应当包含：
引擎本身将由以下组件构成

* 渲染混合输出
* 全局资源管理
  * 场景资源传递
* 场景
  4.
  * 场景坐标系
  * 对象
    * 空对象
    * 预制对象
    * 对象工厂
  * 场景资源管理
* 相机
  * 渲染
  * 控制台
  * 镜头
    * 画布
      * 窗口
    * 特效

# 核心组件TGE

TGE有以下几种状态：

* Init
* Start
* Update
* Exit

Init状态只能出现在引擎启动的时候，此时创建场景资源管理器的实例。
Start状态是加载一个场景的时候进入。
Update是在一个场景内的时候处于。
Exit被调用退出函数后进入，此时所有场景资源管理器脱离生命周期，并进行资源处理以及性能统计输出。

在Debug模式中，还有一个Debug状态，这个状态可以在任何一个阶段进入，并且执行Debug代码，然后返回。

## 渲染

渲染核心将会由Layer低到Layer高的，具有渲染特性的东西。
渲染核心的主要工作流程如下：

1. 获得激活镜头的渲染前特效，对整个屏幕进行一次输入。
2. 将可见物品序列中，具有可渲染特性的Layer最小的东西渲染到屏幕上
  具体：将其空间坐标转化为屏幕坐标，如果处于屏幕边界，进行对应截断，然后重定位光标到对应位置，绘制一行，如此重复。
3. 取出下一个Layer的东西，继续渲染。
4. 取得当前镜头的激活画布，将窗口Layer最小的东西渲染到屏幕上，如上。不过，窗口本身不需要进行坐标转化。
5. 以上流程中，如果激活镜头有渲染中特效，那么加入特效
6. 加入渲染后特效，然后将对应的渲染结果存入对应激活镜头的缓冲帧。

随后，渲染核心寻找下一个激活镜头，并且重复上述流程。每一个镜头都可以使用一个独立渲染线程进行渲染。

## 相机

相机模块需要定期采样绑定自身的镜头的缓冲帧，并且将缓冲帧内容输出到屏幕。如果当前相机模块没有被绑定镜头的话，则搜索所有的激活镜头中优先级最高的那个，然后进行输出。每一个相机镜头都可以在游戏进行中随时切换。

## 实体

实体是一个复杂的模块，它在全局资源管理器或场景资源管理器中被创建。

实体本身自带三个特性：

* 实体ID
* 实体名称
* 坐标

实体可以混入的特性：

* 2D刚体（工具特性）
* 镜头
* 渲染
* 事件
* 交互
* 监听
* 广播
* 实体工厂

实体被资源管理器创建并被资源管理器持有的，叫做**实体实例**。
**实体实例**的*所有权*一定属于创建它的资源管理器，除非被特殊的函数转移所有权到别的资源管理器。所有权转移的函数只有全局资源管理器实现了。

用户可以进行如下操作：
当一个实体实例由全局资源管理器在引擎Init阶段被创建出来的，这个实体实例被叫做先验实例。这些实例：
> （1）不可以被全局资源管理器删除；
> （2）不能被以任何形式拷贝
> （3）只能被不可变借用
> （4）不能被“引入”
当一个实体实例由全局资源管理器在引擎Start阶段创建出来，这个实体实例被称为预制实例：
> （1）可以被全局资源管理器删除
> （2）可以进行可变借用
> （3）不能被以任何形式拷贝
> （4）可以用全局资源管理器进行“引入”，将深度克隆后的实体实例所有权交给当前场景资源管理器。成为引入实例。
当一个实体实例由场景资源管理器在引擎Start阶段（也就是对应场景的加载）创建，这些实体实例被称为场景实例：
> （1）可以将所有权转移到全局资源管理器中，成为预制实例。
> （2）不可以被场景资源管理器删除
> （3）可以进行可变借用
> （4）不可以进行任何拷贝
当一个实体实例由场景资源管理器在引擎Update阶段创建，这些实体被称为引入实例：
> （1）可以将所有权转移到全局资源管理器中，成为预制实例
> （2）可以被场景资源管理器删除
> （3）可以进行可变借用
> （4）不可以进行任何拷贝
全局资源管理器在Update、Exit阶段将不能调用创建的接口。在Init，Exit阶段，场景资源管理不处于其生命周期中，自然不能被使用。

有一些实体实例比较特殊，在第一次进入Start的时候，自动被出创建，并且相当于默认包含进预制实例中，可以直接获得。这些预制实体为：

* 空实体（只有默认的两个特性）
* 镜头实体（默认+镜头）

## 全局资源管理器

这个用于存储预制实体、全局信息的。可以在任意阶段将信息和实体存入这里。其生命周期从Init创建开始，到刚进入Exit。

需要至少能够提供以下服务：
（1）能够指定带有各种特征以及用户自定义特征的实体，并完成创建，将所有权保留到自身。
（2）能够实现给预制实例提供借用、引入的功能

## 场景

场景是用于承载游戏内实体的地方，在游戏过程中创建的实体都存在在场景资源管理器中。调用场景资源管理器创建实体的时候，实体的所有权被场景资源管理器持有。场景资源管理器被全局资源管理器在Init阶段创建。

## 事件

被广播传递的具体信息

## 广播

由全局资源管理器创建，负责监听一个广播源，并且在发生广播的时候，将广播信息传递到监听了对应频道的实体实例上。

# 核心Trait

## 渲染

渲染组件需要以下参数：

渲染层Layer
渲染优先级
渲染的帧动画序列（内容与起始位置）
渲染的当前动画帧
渲染的特效

## 坐标系（Position System）

坐标系可以提供给坐标进行绑定，坐标系将会给坐标提供原点的概念，使得坐标存在意义。坐标系本身是没有坐标的。

坐标组件可以被绑定以下参数

* 坐标系
* 位置

## 坐标（Position）

标定在某一个坐标系中的位置，具有相同坐标系的两个坐标可以判断距离

## 镜头

带有镜头特性的实体实例会判断周围一定范围内是否有其他带有渲染特性的实体实例，并且将其加入自身的可见实例序列，以便渲染模块使用。

镜头至少需要提供这些参数：

* 镜头的大小
* 镜头内的可见实例序列
* 镜头渲染缓冲

### 画布

每一个镜头内都可以创建画布，画布至少具有以下内容：

* Layer
* 优先级
* 画布的位置
* 画布的大小

画布内可以创建窗口，这些窗口是由**画布资源管理器**管理的，和实体很像。但是不存在全局窗口，每一个窗口的所有权由创建他的画布持有。
画布内的窗口不能互相覆盖。

### 窗口

窗口是一个显示框，由字符的边框构成，内部可以展示以下内容：

* 文本框
* ASCII图像
* 艺术字
* 输入框
* 选项

## 广播

广播的特性使得实体实例可以注册一个频率，然后作为广播的发射方，在对应频率上进行传递信息。负责传递信息的信息就是**事件**。

## 监听

监听可以监听一个或多个特定频率，是广播系统的接受方。当监听到一个事件被广播发送的时候，监听可以引发一个不可被发送的本地事件，用于处理一些事件。

## 事件

具有事件属性的实体实例可以创建一个事件，这个事件可以由广播发送。事件可以分为这么几个类型：

* Trigger（触发）
* Hold（保持事件）
* Local（不可以向外部传递）

事件传递的具体内容可以是一个引用、一个借用、一个任意的数据结构、一个及时被调用的函数。

# 扩展Trait

## 2D体组件(2D Body Component)

2D Body Component是用于模拟2D的物体的组件。这个组件具有如下参数：

* 质量
* 力
  * 摩擦力
  * 外力
* 运动
  * 速度
  * 加速度
