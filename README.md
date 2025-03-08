# 基于rust tokio库实现的按键音频模拟软件。
# 适用于linux fedora41系统

## 运行需求:
```
redis-server
python3
```

## 编译需求:
```
rustc
cargo
```

### python：  
采用python的keyboard库实现对全局键盘事件的监听  
采用redis的发布订阅功能实现按键事件向rust应用的传输  
使用json格式传输按键事件  

### rust：  
使用serde库实现对json数据的反序列化  
使用redis库实现对redis的操作  
使用tokio库的异步功能实现按键音的同步播放  

## 运行：  
### 依次运行
```
redis-server

python3 hello.py

cargo run --release
```

# Todo:   
1.增加按键音的选择 done  
2.增加按键音的随机播放 done  
3.增加按键音的音量控制 todo  
3.添加gui控制功能   todo  
4.编译打包,发布  todo  
5.自定义按键音  done