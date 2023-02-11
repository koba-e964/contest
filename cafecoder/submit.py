#!/usr/bin/env python3
"""Submit a solution file to yukicoder.
"""

import sys
import json
import os
import http
import cerberus
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


def read_language_config(language_config_path):
    """Read language config
    """
    with open(language_config_path, encoding='utf-8') as file:
        languages = yaml.safe_load(file)
    # Cerberus does not support top-level list values;
    # therefore, we use a hack here: wrapping languages with a dummy key-value pair.
    value = {'dummy': languages}
    item_schema = {
        'extension': {'type': 'string'},
        'yukicoder_name': {'type': 'string'}
    }
    value_schema = {
        'dummy': {
            'type': 'list',
            'schema': {
                'type': 'dict',
                'schema': item_schema,
                'allow_unknown': True
            }
        }
    }
    val = cerberus.Validator(value_schema)
    if not val.validate(value):
        print(f'Invalid languages config: {language_config_path} {val.errors}',
              file=sys.stderr)
        sys.exit(1)
    return languages


def sign_in(cafecoder_config):
    sign_in_url = 'https://api.cafecoder.top/api/auth/sign_in'
    resp = requests.post(sign_in_url, cafecoder_config, headers={
    }, timeout=5)
    if resp.status_code == http.HTTPStatus.UNAUTHORIZED:
        print('Login failed! name or password in cafecoder_config is wrong.')
        sys.exit(1)
    return {
        'uid': resp.headers['uid'],
        'client': resp.headers['client'],
        'access-token': resp.headers['access-token'],
    }

def get_contest_info(contest_name):
    contest_info_url = f'https://api.cafecoder.top/api/contests/{contest_name}'
    resp = requests.get(contest_info_url, timeout=5)
    value = json.loads(resp.text)
    task_schema = {
        'slug': {'type': 'string'},
        'name': {'type': 'string'},
        'position': {'type': 'string'},
    }
    value_schema = {
        'tasks': {
            'type': 'list',
            'schema': {
                'type': 'dict',
                'schema': task_schema,
                'allow_unknown': True
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


def sign_out(auth):
    url = 'https://api.cafecoder.top/api/auth/sign_out'
    resp = requests.delete(url, headers=auth, timeout=5)
    value = json.loads(resp.text)
    if value['success'] is not True:
        print("Error!", file=sys.stderr)
        print(resp.text, file=sys.stderr)
        print(resp.headers, file=sys.stderr)
        sys.exit(1)


def get_problem_info(filename):
    """Gets information about the problem from the given filename.
    """
    source_dir = os.path.dirname(os.path.abspath(filename))
    contest_name = os.path.basename(source_dir)
    print(f'contest_name = {contest_name}')

    (problem_name, extension) = os.path.splitext(filename)
    problem_name = os.path.basename(problem_name)
    return {
        'contest_name': contest_name,
        'problem_name': problem_name,
        'extension': extension,
    }


def submit_to_task(problem_info, task, filename, languages, auth):
    """POST to submit the given code.
    """
    headers = auth.copy()
    lang = 'rust:1.48.0' # TODO
    headers['lang'] = lang
    url = f'https://api.cafecoder.top/api/contests/{problem_info["contest_name"]}/tasks/{task["slug"]}/submit'
    with open(filename, encoding='utf-8') as file:
        body = file.read()
    resp = requests.post(url, body, headers=headers, timeout=5)
    if resp.status_code != 204:
        print(resp.status_code, file=sys.stderr)
        print(resp.headers, file=sys.stderr)
        sys.exit(1)


def main():
    """main function
    """
    if len(sys.argv) != 2:
        print("./submit.py PROBLEM_ID.ext")
        sys.exit(1)

    script_dir = os.path.dirname(os.path.abspath(__file__))
    cafecoder_config_path = os.path.join(script_dir, 'cafecoder_config')
    language_config_path = os.path.join(script_dir, '..', 'languages.yml')

    cafecoder_config = read_cafecoder_config(cafecoder_config_path)
    languages = read_language_config(language_config_path)

    problem_info = get_problem_info(sys.argv[1])

    auth = sign_in(cafecoder_config)

    info = get_contest_info(problem_info['contest_name'])

    task = None
    for task0 in info['tasks']:
        if task0['position'].lower() == problem_info['problem_name']:
            task = task0
            break


    print('Submitting...')
    print(f'problem_name = {problem_info["problem_name"]}, task_name = {task["name"]}')
    submit_to_task(problem_info, task, sys.argv[1], languages, auth)
    sign_out(auth)


main()
