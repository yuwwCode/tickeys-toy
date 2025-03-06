import threading
import redis
import json
import keyboard
import os
import time
# 连接到 Redis 服务器


def is_root() -> bool:
    return os.getuid() == 0


my_key_event = {
    "name": "a",
    "key_type": "down",
}
channel = "py_to_rs_channel"


def on_key_event(event):
    # 判断事件类型，可能是按键按下（down）或释放（up）
    # print(f"Key {event.name} was pressed.")
    my_key_event["name"] = event.name
    my_key_event["key_type"] = event.event_type
    json_string = json.dumps(my_key_event)
    print(json_string)
    r.publish(channel, json_string)


# 检查是否以root权限运行
if not is_root():
    print("请使用root权限运行")


# 尝试连接到Redis服务器
try:
    r = redis.Redis(host="localhost", port=6379, db=0)
except:  # noqa: E722
    print("python与redis连接失败")

# 检查rs是否做好接受信号的准备
ready_bytes = r.get("rs_to_py_flag")
# 将字节类型转换为字符串类型
ready = ready_bytes.decode("utf-8") if ready_bytes else None
if ready != "ready":
    print("rs未做好接受信号的准备")
while ready != "ready":
    ready_bytes = r.get("rs_to_py_flag")
    # 将字节类型转换为字符串类型
    ready = ready_bytes.decode("utf-8") if ready_bytes else None
    # print(ready, type(ready))
    time.sleep(0.1)
r.delete("rs_to_py_flag")
print("rs已做好接受信号的准备")

try:
    # 开始监听所有键盘事件，当有事件发生Backspace时调用 on_key_event 函数
    keyboard.hook(on_key_event)
    print("开始监听键盘事件")
except:
    print("监听键盘事件失败")


try:
    # 保持程序运行，持续监听键盘事件
    keyboard.wait()
except KeyboardInterrupt:
    print("Stopped listening for keyboard events.")
