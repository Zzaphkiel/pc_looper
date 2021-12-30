# pc_looper
12next perfect clear finder for PuyoPuyo Tetris

基于最佳优先搜索实现的，读 12next 用于找 perfect clear 的 AI

<br/>

Please don't ues it in PvP game mod!

仅为学习，请不要在 PvP 模式中使用！

<br/>

目前已知但又不知如何修复的 bug:
+ 使用 hold 之后，由于不能再次使用 hold，导致计算的解有误
+ 在一种未知的情况下，放置方块后计算或显示有误
+ 直接双击运行程序会乱码，使用 terminal 打开则不会

## board.rs
> 参考 <https://github.com/wirelyre/tetra-tools/blob/main/basic/src/gameplay.rs>

作用：
+ 方块的移动、旋转、碰撞判定和锁定
+ pc 的判断
+ 搜索时的剪枝判断
+ 搜索时的启发函数
+ 修复了旋转方块时的 bug

## place.rs
> 参考 <https://github.com/wirelyre/tetra-tools/blob/main/basic/src/piece_placer.rs>

作用：
+ 生成搜索时所需的 frontier 表，即生成当前方块可以放置在当前场地的所有位置
+ 对重复放置结果进行剪枝

## ppt.rs
> 参考 <https://github.com/naari3/pc_assist/blob/master/src/ppt.rs>

作用：
+ 读游戏内存，获得当前场地、块序和 hold

## search.rs

作用：
+ 最佳优先搜索 pc

## show.rs

作用：
+ 输出结果
