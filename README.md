# TermGameEngine

TermGameEngine is a lightweight terminal game engine designed to help developers quickly build text-based games.

设想：
这个引擎就应该像是一个OS一样，不过是周期性的，重复的调用一些syscall，并完成一些逻辑
目前应当存在这样一个东西：资源管理器，其在引擎初始化的时候，就被写好了作为一个全局实例。其有两个部分：资源数量和对应的管理期，资源数量不变，管理器则管理其情况。
资源包含以下部分：

* prefab：被预制的游戏物品，普遍具有相同的API,可以之后由另一个cargo去生成。
* window：窗口
