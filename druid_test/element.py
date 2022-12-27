import itertools
import json

import pyautogui as pyautogui

from druid_test.key_mouse import EventType, load_event, parse_event


class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def serialize(self):
        return {
            'x': self.x,
            'y': self.y,
        }


class Size:
    def __init__(self, w, h):
        self.w = w
        self.h = h

    def serialize(self):
        return {
            'w': self.w,
            'h': self.h,
        }


class Layout:
    def __init__(self, origin, size):
        self.origin = origin
        self.size = size

    def serialize(self):
        return {
            'origin': self.origin.serialize(),
            'size': self.size.serialize(),
        }


class Element:
    def __init__(self, selector, layout):
        self.selector = selector
        self.layout = layout

    def serialize(self):
        return {
            'selector': self.selector,
            'layout': self.layout.serialize(),
        }

    def click(self):
        return [
            self.mouse_down(),
            self.mouse_up()
        ]

    def mouse_down(self):
        return self.mouse_event(EventType.MouseDown)

    def mouse_up(self):
        return self.mouse_event(EventType.MouseUp)

    def mouse_event(self, event_type):
        return {
            "selector": self.selector,
            "window_id": 1,
            "event": {
                event_type.value: {
                    "pos": {
                        "x": self.layout.size.w / 2,
                        "y": self.layout.size.h / 2,
                    },
                    "buttons": 0,
                    "mods": {
                        "bits": 0
                    },
                    "count": 0,
                    "button": "Left"
                }
            }
        }

    def screenshot(self):
        return [{
            "selector": self.selector,
            "window_id": 1,
            "event": {
                "Screenshot": {
                    "selector": self.selector,
                }
            }
        }]


def serialize_events(events):
    filtered_events = []
    for event in events:
        if event.event.event_type == EventType.MouseMove and len(filtered_events) and filtered_events[
            -1].event.event_type == EventType.MouseMove:
            filtered_events.pop()
        filtered_events.append(event)

    return [event.serialize() for event in filtered_events]


def chain(l):
    for x in l:
        if isinstance(x, dict):
            yield x
        else:
            yield from chain(x)


class Manager:
    def __init__(self):
        self.elements = {}
        self.events = []
        self.recording = True
        self.n = 0
        self.m = 0

    def store_event(self, event):
        self.events.append(event)

    def update_layout(self, element):
        self.elements[element.selector] = element

    def execute(self, path):
        self.recording = False

        def gen_digit(x):
            for c in str(x):
                yield find(f'digit-{c}').click()

        def gen(a, op, b):
            return chain([
                gen_digit(a),
                find(f'option-{op}').click(),
                gen_digit(b),
                find('option-=').click(),
                find('result').screenshot(),
                find('option-CE').click()
            ])

        if path != 'current':
            events = json.load(open('tests/' + path + '.json', encoding='utf8'))
            if path == 'add_test':
                events = gen(112300, '+', 94212)

            self.events = [
                parse_event(load_event(event).serialize())
                for event in events
            ]

        return serialize_events([
            load_event(event.serialize())
            for event in self.events
        ])

    def clear_events(self):
        self.events.clear()

    def start_record(self):
        self.clear_events()
        self.recording = not self.recording

    def export(self, path):
        self.recording = False
        events = serialize_events(self.events)

        if path == '':
            path = f'tests/{self.n}.json'
        else:
            path = f'tests/{path}.json'
        self.n += 1
        fp = open(path, 'w', encoding='utf8')
        json.dump(events, fp, ensure_ascii=False, indent=4)

    def screenshot(self, selector):
        origin = self.elements[selector].layout.origin
        size = self.elements[selector].layout.size
        pyautogui.screenshot(f'screenshot/{self.m}.png', region=(origin.x, origin.y, size.w, size.h))
        self.m += 1


manager = Manager()


def find(selector):
    return manager.elements[selector]


def parse_element(element_json):
    selector = element_json['selector']
    layout = element_json['layout']
    origin = layout['origin']
    size = layout['size']

    return Element(
        selector,
        Layout(
            Point(
                origin['x'],
                origin['y'],
            ),
            Size(
                size['w'],
                size['h'],
            )
        )
    )
