# Summary

[前言](introduction.md)

# 第一部分　起步

- [认识 Bevy](ch01-00-getting-to-know-bevy.md)
  - [游戏引擎与 Bevy](ch01-01-what-is-bevy.md)
  - [ECS 思维模型](ch01-02-ecs-mental-model.md)
  - [安装与项目搭建](ch01-03-installation.md)
  - [如何使用本书](ch01-04-how-to-use-this-book.md)
- [第一个 Bevy App](ch02-00-your-first-bevy-app.md)
  - [App 与 System](ch02-01-app-and-systems.md)
  - [Plugin：App 能力的来源](ch02-02-plugins.md)
  - [一个会动的 Sprite](ch02-03-a-moving-sprite.md)

# 第二部分　ECS——Bevy 的心脏

- [Entity 与 Component](ch03-00-entities-and-components.md)
  - [组件与实体](ch03-01-components-and-entities.md)
  - [Commands：排队修改世界](ch03-02-commands.md)
  - [Required Components 与 Bundle](ch03-03-required-components-and-bundles.md)
- [System 与 Query](ch04-00-systems-and-queries.md)
  - [系统与系统参数](ch04-01-systems-and-system-params.md)
  - [Query：精确取数据](ch04-02-query-data.md)
  - [过滤器与变更检测](ch04-03-query-filters.md)
  - [借用冲突与 ParamSet](ch04-04-borrow-conflicts-and-paramset.md)
- [Resource——全局唯一数据](ch05-00-resources.md)
  - [定义与访问 Resource](ch05-01-defining-and-accessing-resources.md)
  - [资源的有无](ch05-02-resource-presence.md)
  - [初始化：init_resource 与 FromWorld](ch05-03-initialization-order.md)
  - [资源的变更检测](ch05-04-resource-change-detection.md)
- [Schedule 与执行顺序](ch06-00-schedules-and-execution-order.md)
  - [Main 调度全家](ch06-01-the-main-schedule-family.md)
  - [排序：chain、before 与 after](ch06-02-ordering-chain-before-after.md)
  - [SystemSet：成组排序](ch06-03-system-sets.md)
  - [run_if：条件运行](ch06-04-run-conditions.md)
  - [同步点与命令应用时机](ch06-05-sync-points.md)
- [Message——缓冲消息](ch07-00-messages.md)
  - [收发消息](ch07-01-writing-and-reading-messages.md)
  - [一写多读](ch07-02-one-writer-many-readers.md)
  - [双缓冲与清理时机](ch07-03-double-buffering-and-cleanup.md)
  - [消息工具箱](ch07-04-message-toolbox.md)
  - [AppExit：用消息谢幕](ch07-05-app-exit.md)
