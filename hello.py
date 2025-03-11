import json
import keyboard
import os


def is_root() -> bool:
    return os.getuid() == 0


def on_key_event(event):
    # 判断事件类型，可能是按键按下（down）或释放（up）
    # print(f"Key {event.name} was pressed.")
    my_key_event["name"] = event.name
    my_key_event["key_type"] = event.event_type
    json_string = json.dumps(my_key_event)
    print(json_string, flush=True)


my_key_event = {
    "name": "a",
    "key_type": "down",
}


keyboard.hook(on_key_event)


# 保持程序运行，持续监听键盘事件
keyboard.wait()
