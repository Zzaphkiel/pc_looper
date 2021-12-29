# pc_looper
A 12next perfect clear finder for PuyoPuyo Tetris

一个基于最佳优先搜索实现的，读 12next 用于找 perfect clear 的 AI

<br/>

Please don't ues it in PvP game mod!

仅为学习，请不要在PvP模式中使用！

<br/> 
使用 terminal 打开就不会乱码~

## board.rs
> 参考 <https://github.com/wirelyre/tetra-tools/blob/main/basic/src/gameplay.rs>

作用：
+ 方块的移动、旋转和锁定
+ pc的判断
+ 搜索时的剪枝判断
+ 搜索时的启发函数
+ 修复了旋转方块时的bug

## place.rs
> 参考 <https://github.com/wirelyre/tetra-tools/blob/main/basic/src/piece_placer.rs>

作用：
+ 生成搜索时所需的 frontier 表，即生成当前方块可以放置在当前场地的所有位置
+ 对重复放置结果进行剪枝

## ppt.rs
> 参考 <https://github.com/naari3/pc_assist/blob/master/src/ppt.rs>

作用：
+ 读游戏内存，获得当前场地、块序和hold

## search.rs

作用：
+ 最佳优先搜索pc

## show.rs

作用：
+ 输出结果
