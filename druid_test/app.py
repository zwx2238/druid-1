import os

from flask import Flask, request

from druid_test.element import parse_element, manager
from druid_test.key_mouse import parse_event

import logging

log = logging.getLogger('werkzeug')
log.setLevel(logging.ERROR)

app = Flask(__name__)


@app.route('/store_event', methods=['POST'])
def store_event():
    if manager.recording:
        event = parse_event(request.json)
        manager.store_event(event)
    return ''


@app.route('/update_layout', methods=['POST'])
def update_layout():
    element = parse_element(request.json)
    manager.update_layout(element)
    return ''


@app.route('/start_record', methods=['POST'])
def start_record():
    manager.start_record()
    return ''


@app.route('/execute', methods=['POST'])
def execute():
    path = request.args['path']
    events = manager.execute(path)
    return events


@app.route('/export', methods=['POST'])
def export():
    path = request.args['path']
    manager.export(path)
    return ''


@app.route('/screenshot', methods=['POST'])
def screenshot():
    selector = request.args['selector']
    manager.screenshot(selector)
    return ''


@app.route('/list_paths', methods=['POST'])
def list_paths():
    return [
        path[:-5]
        for path in list(os.listdir('tests'))
    ]


@app.route('/delete_path', methods=['POST'])
def delete_path():
    path = request.args['path']
    os.remove(f'tests/{path}.json')
    return ''


app.run(port=10001)
