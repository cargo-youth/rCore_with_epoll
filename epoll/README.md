https://github.com/liexusong/linux-source-code-analyze/blob/master/epoll-principle.md

# 实现思路
- 系统调用sys_epoll_create()
  - 调用 ep_alloc() 函数创建并初始化一个 eventpoll 对象。
  - 调用 anon_inode_getfd() 函数把 eventpoll 对象映射到一个文件句柄，并返回这个文件句柄。