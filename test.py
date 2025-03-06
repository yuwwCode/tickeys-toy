import keyboard


def on_key_event(event):
    is_caps_on = keyboard.is_modifier("caps lock")
    print("caps is ", is_caps_on)
    name = event.name
    types = event.event_type
    if is_caps_on:
        name = name.upper()
    # 判断事件类型，可能是按键按下（down）或释放（up）
    if event.event_type == keyboard.KEY_DOWN:
        print(f"Key {name} was pressed.")
    elif event.event_type == keyboard.KEY_UP:
        print(f"Key {name} was released.")


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
