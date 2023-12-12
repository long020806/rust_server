# mysql 删除delete百万数据后数据库依旧很卡
使用 truncate table users; 删除后恢复

```
https://blog.51cto.com/u_16175462/7335958
```

# 事务提交和单语句新增
10w 事务提交 14s 单语句新增 1342ms
单语句新增效率极高但考虑sql长度
```
tx = pool.begin().await?;
...
tx.commit().await?;

or

INSERT INTO USERS(username) VALUES('test1'),('test2'),('test3')...
```