# pc_looper
A 12next perfect clear finder for PuyoPuyo Tetris

Please don't ues it in online mod!

请不要在PvP模式中使用！

## board.rs
参考 <https://github.com/wirelyre/tetra-tools/blob/main/basic/src/gameplay.rs>

方块的旋转、移动和放置的实现。

新增：
+ 搜索时的剪枝判断实现。
+ 启发函数的实现。

## place.rs
参考 <https://github.com/wirelyre/tetra-tools/blob/main/basic/src/piece_placer.rs>

用于生成搜索时所需的 frontier 表，即生成当前方块可以放置在当前场地的所有位置

新增：
+ 对重复放置结果进行剪枝

## ppt.rs
参考 <https://github.com/naari3/pc_assist/blob/master/src/ppt.rs>

用于读取游戏信息

## search.rs
实现：
+ 最佳优先搜索pc

## show.rs
实现：
+ 展示结果
