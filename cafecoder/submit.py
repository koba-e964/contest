#!/usr/bin/env python3
"""Submit a solution file to yukicoder.
"""

import sys
import json
import os
import cerberus
import requests
import yaml

def read_cafecoder_config(cafecoder_config_path):
    """Read yukicoder-specific config
    """
    with open(cafecoder_config_path) as file:
        config = yaml.safe_load(file)
    v = cerberus.Validator({'name': {'type': 'string'}, 'password': {'type': 'string'}})
    if not v.validate(config):
        print('Invalid config: {} {}'.format(cafecoder_config_path, v.errors),
              file=sys.stderr)
        sys.exit(1)
    return config


def read_language_config(language_config_path):
    """Read language config
    """
    with open(language_config_path) as file:
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
    v = cerberus.Validator(value_schema)
    if not v.validate(value):
        print('Invalid languages config: {} {}'.format(language_config_path, v.errors),
              file=sys.stderr)
        sys.exit(1)
    return languages


def sign_in(cafecoder_config):
    sign_in_url = 'https://api.cafecoder.top/api/auth/sign_in'
    resp = requests.post(sign_in_url, cafecoder_config, headers={
    })
    return {
        'uid': resp.headers['uid'],
        'client': resp.headers['client'],
        'access-token': resp.headers['access-token'],
    }

def get_contest_info(contest_name):
    contest_info_url = 'https://api.cafecoder.top/api/contests/{}'.format(contest_name)
    resp = requests.get(contest_info_url)
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
        print("Invalid response: {}\n{}".format(value, val.errors), file=sys.stderr)
        sys.exit(1)
    return value


def sign_out(auth):
    url = 'https://api.cafecoder.top/api/auth/sign_out'
    resp = requests.delete(url, headers=auth)
    value = json.loads(resp.text)
    if value['success'] != True:
        print("Error!", file=sys.stderr)
        print(resp.text, file=sys.stderr)
        print(resp.headers, file=sys.stderr)
        sys.exit(1)


def get_problem_info(filename):
    source_dir = os.path.dirname(os.path.abspath(filename))
    contest_name = os.path.basename(source_dir)
    print("contest_name = {}".format(contest_name))
    
    (problem_name, extension) = os.path.splitext(filename)
    problem_name = os.path.basename(problem_name)
    return {
        'contest_name': contest_name,
        'problem_name': problem_name,
        'extension': extension,
    }


def submit_to_task(problem_info, task, filename, languages, auth):
    headers = auth.copy()
    lang = 'rust:1.48.0' # TODO
    headers['lang'] = lang
    url = 'https://api.cafecoder.top/api/contests/{}/tasks/{}/submit'.\
    format(problem_info['contest_name'], task['slug'])
    with open(filename) as file:
        body = file.read()
    resp = requests.post(url, body, headers=headers)
    if resp.status_code != 204:
        print(resp.status_code, file=sys.stderr)
        print(resp.headers, file=sys.stderr)
        sys.exit(1)


def main():
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
    for t in info['tasks']:
        if t['position'].lower() == problem_info['problem_name']:
            task = t
            break


    print('Submitting...')
    print('problem_name = {}, task_name = {}'.
          format(problem_info['problem_name'], task['name']))
    submit_to_task(problem_info, task, sys.argv[1], languages, auth)
    sign_out(auth)
    

main()
