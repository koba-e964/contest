#!/usr/bin/env python3
"""Checks the submissions and emit the result to statuses.json.
"""

import sys
import datetime
import json
import os
import cerberus
import dateutil.parser
import requests
import yaml


def read_cafecoder_config(cafecoder_config_path):
    """Read cafecoder-specific config
    """
    with open(cafecoder_config_path, encoding='utf-8') as file:
        config = yaml.safe_load(file)
    val = cerberus.Validator({'name': {'type': 'string'}, 'password': {'type': 'string'}})
    if not val.validate(config):
        print(f'Invalid config: {cafecoder_config_path} {val.errors}',
              file=sys.stderr)
        sys.exit(1)
    return config


def read_contest_config(contest_config_path):
    """Read contest-specific config
    """
    try:
        with open(contest_config_path, encoding='utf-8') as file:
            config = yaml.safe_load(file)
    except OSError:
        config = {}
    contest_schema = {
        'skipped': {
            'type': 'boolean',
            'default': False,
        },
        'ignore': {
            'type': 'list',
            'schema': {'type': 'string'},
            'default_setter': lambda _doc: [],
        },
    }
    val = cerberus.Validator(contest_schema)
    if not val.validate(config):
        print(f'Invalid config: {contest_config_path} {val.errors}',
              file=sys.stderr)
        sys.exit(1)
    return val.normalized(config)


def get_contests():
    """Get all contests visible without logging in.
    """
    contests_url = 'https://api.cafecoder.top/api/contests'
    resp = requests.get(contests_url)
    value = json.loads(resp.text)
    contest_schema = {
        'slug': {'type': 'string'},
        'name': {'type': 'string'},
        'kind': {'type': 'string'},
        'start_at': {'type': 'string'},
        'end_at': {'type': 'string'},
    }
    val = cerberus.Validator({'dummy': {
        'type': 'list',
        'schema': {
            'type': 'dict',
            'schema': contest_schema,
            'allow_unknown': True,
        },
    }})
    if not val.validate({'dummy': value}):
        print(f'Invalid response: {value}\n{val.errors}', file=sys.stderr)
        sys.exit(1)
    return value


def get_contest_info(contest_name):
    """Gets information of a contest.
    """
    contest_info_url = f'https://api.cafecoder.top/api/contests/{contest_name}'
    resp = requests.get(contest_info_url)
    value = json.loads(resp.text)
    task_schema = {
        'slug': {'type': 'string'},
        'name': {'type': 'string'},
        'position': {'type': 'string'},
    }
    value_schema = {
        'start_at': {'type': 'string'},
        'end_at': {'type': 'string'},
        'tasks': {
            'nullable': True,
            'type': 'list',
            'schema': {
                'type': 'dict',
                'schema': task_schema,
                'allow_unknown': True,
            },
        },
    }

    val = cerberus.Validator({'dummy': {
        'type': 'dict',
        'schema': value_schema,
        'allow_unknown': True,
    }})
    if not val.validate({'dummy': value}):
        print(f'Invalid response: {value}\n{val.errors}', file=sys.stderr)
        sys.exit(1)
    return value


def get_submissions(username, contest_slug):
    """Gets submissions to contest_slug made by username.
    """
    options = {
        'filter': [
            {'target': 'user', 'value': username},
        ],
    }
    options = json.dumps(options)
    url = f'https://api.cafecoder.top/api/contests/{contest_slug}/submits/all'
    payload = {
        'page': 1,
        'count': 100,
        'options': options,
    }
    resp = requests.get(url, payload)
    return json.loads(resp.text)


def get_statuses(username, contest_slug):
    """
    Finds which problems username solved.
    If this function returned None, then the contest has not started yet.
    """
    contest_info = get_contest_info(contest_slug)
    now = datetime.datetime.now(datetime.timezone.utc)
    start_at = dateutil.parser.parse(contest_info['start_at'])
    if now < start_at:
        return None
    tasks = contest_info['tasks']
    ntasks = len(tasks)
    taskpos = [None] * ntasks
    table = {}
    for i, item in enumerate(tasks):
        slug = item['slug']
        table[slug] = i
        taskpos[i] = item['position']
    ans = get_submissions(username, contest_slug)
    subs = ans['data']
    solved = [False] * ntasks
    for sub in subs:
        task_slug = sub['task']['slug']
        status = sub['status']
        index = table[task_slug]
        if status == 'AC':
            solved[index] = True
    return (taskpos, solved)

def main():
    """main function
    """
    script_dir = os.path.dirname(os.path.abspath(__file__))
    cafecoder_config_path = os.path.join(script_dir, 'cafecoder_config')
    tofile = os.path.join(script_dir, 'statuses.json')

    cafecoder_config = read_cafecoder_config(cafecoder_config_path)

    username = cafecoder_config['name']

    contests = get_contests()
    data = []
    for contest in contests:
        contest_slug = contest['slug']
        contest_config_path = os.path.join(script_dir, contest_slug, 'contest.yml')
        contest_config = read_contest_config(contest_config_path)
        if contest_config['skipped']:
            continue
        ignore = contest_config['ignore']
        result = get_statuses(username, contest_slug)
        if result is None:
            # Not started
            continue
        (taskpos, solved) = result
        size = len(taskpos)
        statuses = [None] * size
        for i in range(size):
            ignored = taskpos[i] in ignore
            statuses[i] = {
                'position': taskpos[i],
                'ignored': ignored,
                'solved': solved[i],
            }
        current = {
            'slug': contest_slug,
            'statuses': statuses,
        }
        data.append(current)
    with open(tofile, mode='w', encoding='utf-8') as filep:
        json.dump(data, filep, indent=2)

main()
