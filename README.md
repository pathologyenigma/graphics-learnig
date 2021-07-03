now it's ray tracing the next week.
really learned a lot things.
and found the rust right now is not powerful enough to replace cplusplus.
Also, the way I implement it is not good, so I facing serious performance issue.
for the problem I faced, I choose to using a graphic library to deal with it, so the next one won't be the final book, will be some gfx-hal demo or ash demo.
And after I finish learning those graphic library I will rewrite this and the one weekend one.



光追系列的第二本书，这本书让我学到了更多东西，同时也让我感觉现在的rust和c++比起来还是差了不少。（某些语法问题，其实在之前尝试使用整数模板参数的时候就碰到了）。
不知道是在下用了太多clone还是二叉树节点写的不对，最终代码比c++版的慢了好几个量级，即便开启多线程还是慢得离谱（可能是在下的线程模型不对吧），于是磨刀不误砍柴功，在下选择先学习rust的图形库再来重写该系列（毕竟显卡做这些要块多了）。
接下来的分支会是图形库的demo，可能是gfx-hal，也可能是纯vulkan的库（ash之类的），然后在有把握的时候开始重写这些。
