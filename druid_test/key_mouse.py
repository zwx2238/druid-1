from enum import Enum


class KeyEvent:
    def __init__(self, **kwargs):
        self.keys = list(kwargs.keys())
        for k, v in kwargs.items():
            setattr(self, k, v)

    def serialize(self):
        return {
            k: getattr(self, k)
            for k in self.keys
        }


class MouseEvent:
    def __init__(self, **kwargs):
        self.keys = [
                        'pos', 'buttons', 'mods', 'count', 'button', 'wheel_delta'
                    ] + ['window_pos', 'focus', 'from_command']
        for k, v in kwargs.items():
            setattr(self, k, v)

    def serialize(self):
        d = {}
        for k in self.keys:
            try:
                d[k] = getattr(self, k)
            except AttributeError:
                pass
        return d


class ScreenshotEvent:
    def __init__(self, **kwargs):
        self.selector = kwargs['selector']

    def serialize(self):
        return {
            'selector': self.selector
        }


class EventType(Enum):
    KeyDown = 'KeyDown'
    KeyUp = 'KeyUp'
    MouseDown = 'MouseDown'
    MouseUp = 'MouseUp'
    MouseMove = 'MouseMove'
    Wheel = 'Wheel'
    Screenshot = 'Screenshot'


class Event:
    def __init__(self, event_type, event):
        self.event_type = event_type
        self.event = event

    def serialize(self):
        return {
            self.event_type.value: self.event.serialize()
        }


class WidgetEvent:
    def __init__(self, selector, event, window_id):
        self.selector = selector
        self.event = event
        self.window_id = window_id

    def serialize(self):
        return {
            'selector': self.selector,
            'window_id': self.window_id,
            'event': self.event.serialize()
        }


def parse_event(event_json):
    selector = event_json['selector']
    window_id = event_json['window_id']
    event_json = event_json['event']
    assert len(event_json) == 1
    event_type = list(event_json.keys())[0]
    event_type = EventType(event_type)

    event = event_json[event_type.value]
    if event_type in [EventType.KeyDown, EventType.KeyUp]:
        event = KeyEvent(**event)
    elif event_type in [EventType.MouseDown, EventType.MouseUp, EventType.MouseMove, EventType.Wheel]:
        del event['window_pos']
        del event['focus']
        del event['from_command']
        if event_type != EventType.Wheel:
            del event['wheel_delta']

        event = MouseEvent(**event)
    elif event_type in [EventType.Screenshot]:
        event = ScreenshotEvent(**event)

    event = Event(event_type, event)

    return WidgetEvent(selector, event, window_id)


def load_event(event_json):
    selector = event_json['selector']
    window_id = event_json['window_id']
    event_json = event_json['event']
    assert len(event_json) == 1
    event_type = list(event_json.keys())[0]
    event_type = EventType(event_type)

    event = event_json[event_type.value]
    if event_type in [EventType.KeyDown, EventType.KeyUp]:
        event = KeyEvent(**event)
    elif event_type in [EventType.MouseDown, EventType.MouseUp, EventType.MouseMove, EventType.Wheel]:
        event = MouseEvent(**event)
        event.window_pos = {
            "x": 0,
            "y": 0
        }
        event.focus = False
        event.from_command = False
        if event_type != EventType.Wheel:
            event.wheel_delta = {
                "x": 0,
                "y": 0
            }
    elif event_type in [EventType.Screenshot]:
        event = ScreenshotEvent(**event)
    event = Event(event_type, event)

    return WidgetEvent(selector, event, window_id)
