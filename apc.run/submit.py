#!/usr/bin/env python3
""" Submit to apc.run.
"""
import sys
import os
import json
from types import SimpleNamespace
import requests
import cerberus
import yaml


def read_apc_config(apc_config_path):
    """Read apc-specific config
    """
    with open(apc_config_path, encoding='utf-8') as file:
        config = yaml.safe_load(file)
    val = cerberus.Validator({
        'username': {'required': True, 'type': 'string'},
        'password': {'required': True, 'type': 'string'},
    })
    if not val.validate(config):
        print(f'Invalid config: {apc_config_path} {val.errors}',
              file=sys.stderr)
        sys.exit(1)
    return config

def read_contest_config(contest_config_path):
    """Read contest-specific config
    """
    with open(contest_config_path, encoding='utf-8') as file:
        config = yaml.safe_load(file)
    val = cerberus.Validator({
        'contest_id': {'required': True, 'type': 'integer'},
        'problem_map': {'required': True, 'type': 'dict'},
    })
    if not val.validate(config):
        print('Invalid config: {} {}'.format(contest_config_path, val.errors),
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
        'name': {'type': 'string'}
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
        print('Invalid languages config: {} {}'.format(language_config_path, val.errors),
              file=sys.stderr)
        sys.exit(1)
    return languages


def get_csrftoken(dry_run):
    """
    GETs csrftoken.
    This function does not need cookies.
    """
    url = 'https://apc.run/api/profile'
    if dry_run:
        print("> Would get CSRF token from {}".format(url))
        return 'CSRFTokenDummy'
    # This endpoint provides csrf_token for free.
    resp = requests.get(url, timeout=5)
    if resp.status_code // 100 != 2:
        print(resp.status_code, file=sys.stderr)
        print(resp.text, file=sys.stderr)
        print(resp.headers, file=sys.stderr)
        print(resp.cookies, file=sys.stderr)
        sys.exit(1)
    return resp.cookies['csrftoken']


def get_contest_info(contest_id):
    """
    This function does not need cookies
    """
    resp = requests.get('https://apc.run/api/contest', {'id': contest_id}, timeout=5)
    if resp.status_code // 100 != 2:
        print(resp.status_code, file=sys.stderr)
        print(resp.text, file=sys.stderr)
        print(resp.headers, file=sys.stderr)
        print(resp.cookies, file=sys.stderr)
        sys.exit(1)
    data = json.loads(resp.text)
    return {
        'status': data['data']['status'],
        'title': data['data']['title'],
    }

def get_problem_info(problem_name):
    """
    This function does not need cookies
    """
    resp = requests.get('https://apc.run/api/problem', {'problem_id': problem_name}, timeout=5)
    if resp.status_code // 100 != 2:
        print(resp.status_code, file=sys.stderr)
        print(resp.text, file=sys.stderr)
        print(resp.headers, file=sys.stderr)
        print(resp.cookies, file=sys.stderr)
        sys.exit(1)
    data = json.loads(resp.text)
    return {
        'id': data['data']['id'],
        'title': data['data']['title'],
        'languages': data['data']['languages'],
    }


def login(payload, cookies, dry_run):
    """login
    {'username': .., 'password': ..}
    """
    csrftoken = cookies.csrftoken
    headers = {
        'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64; rv:57.0) Gecko/20100101 Firefox/57.0',
        'cookie': 'csrftoken=' + csrftoken,
        'referer': 'https://apc.run/',
        'x-csrftoken': csrftoken,
    }
    if dry_run:
        print('> Would POST to https://apc.run/api/login as {}'.format(payload['username']))
        cookies.sessionid = 'DummySessionId'
    else:
        resp = requests.post("https://apc.run/api/login", payload, headers=headers, timeout=5)
        cookies.csrftoken = resp.cookies['csrftoken']
        cookies.sessionid = resp.cookies['sessionid']


def submit(payload, cookies, dry_run):
    """submit
    {'problem_id': .., 'code': .., 'language': ..}
    """
    headers = {
        'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64; rv:57.0) Gecko/20100101 Firefox/57.0',
        'cookie': 'csrftoken=' + cookies.csrftoken + '; sessionid=' + cookies.sessionid,
        'referer': 'https://apc.run/',
        'x-csrftoken': cookies.csrftoken,
    }
    if dry_run:
        print('> Would POST to https://apc.run/api/submission, {}'.format(payload))
    else:
        resp = requests.post('https://apc.run/api/submission', payload, headers=headers, timeout=5)
        if resp.status_code // 100 != 2:
            print("Submission failed!", file=sys.stderr)
            print(resp.status_code, file=sys.stderr)
            print(resp.text, file=sys.stderr)
            print(resp.headers, file=sys.stderr)
            print(resp.cookies, file=sys.stderr)
        else:
            data = json.loads(resp.text)['data']
            print('id = ' + data['submission_id'])

def logout(cookies, dry_run):
    """logout
    """
    headers = {
        'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64; rv:57.0) Gecko/20100101 Firefox/57.0',
        'cookie': 'csrftoken=' + cookies.csrftoken + '; sessionid=' + cookies.sessionid,
        'referer': 'https://apc.run/',
        'content-type': 'application/json',
        'x-csrftoken': cookies.csrftoken,
    }
    if dry_run:
        print('> Would GET https://apc.run/api/logout')
    else:
        resp = requests.get('https://apc.run/api/logout', headers=headers, timeout=5)
        if resp.status_code // 100 != 2:
            print("Logout failed!", file=sys.stderr)
            print(resp.status_code, file=sys.stderr)
            print(resp.text, file=sys.stderr)
            print(resp.headers, file=sys.stderr)
            print(resp.cookies, file=sys.stderr)


def main():
    """main
    """
    if len(sys.argv) != 2:
        print("./submit.py PROBLEM_ID.ext")
        sys.exit(1)

    # If DRY_RUN=1, this program emits what would be executed in
    # the real run.
    dry_run = False
    if 'DRY_RUN' in os.environ and os.environ['DRY_RUN'] == '1':
        dry_run = True

    script_dir = os.path.dirname(os.path.abspath(__file__))
    apc_config_path = os.path.join(script_dir, 'apc_config.yml')
    language_config_path = os.path.join(script_dir, '..', 'languages.yml')

    apc_config = read_apc_config(apc_config_path)
    username = apc_config['username']
    password = apc_config['password']
    languages = read_language_config(language_config_path)

    filename = sys.argv[1]
    relfilename = os.path.relpath(sys.argv[1], script_dir)
    (problem_name, extension) = os.path.splitext(relfilename)
    if extension == '':
        print("\x1b[34minvalid extension\x1b[0m")
        sys.exit(1)
    language = next(x for x in languages if x['extension'] == extension[1:])['name']

    with open(filename, encoding='utf-8') as file:
        source = file.read()

    csrftoken = get_csrftoken(dry_run)

    # Login
    cookies = SimpleNamespace(csrftoken=csrftoken, sessionid=None)
    login({
        "username": username,
        "password": password,
    }, cookies, dry_run)

    # Get problem_id from problem_name
    if problem_name.count('/') > 0:
        # problem_name = contest_name + "/" + problem_name_in_contest
        comp = list(problem_name.split('/'))
        if len(comp) != 2:
            print(f'File too deep: {filename}', file=sys.stderr)
            sys.exit(1)
        contest_name = comp[0]
        problem_name_in_contest = comp[1]
        filedir = os.path.dirname(os.path.abspath(filename))
        conffile = os.path.join(filedir, 'contest.yml')
        conf = read_contest_config(conffile)
        contest_info = get_contest_info(conf['contest_id'])
        # If contest.status == "-1" then the contest is finished
        # hence no submissions are allowed.
        # https://github.com/QingdaoU/OnlineJudgeFE/blob/oj_2.7.5/src/store/modules/contest.js#L62-L63
        if contest_info['status'] == '-1':
            if problem_name_in_contest not in conf['problem_map']:
                # TODO automatically convert
                # and update the contest.yml file for automatic conversion
                newname = contest_name + '_' + problem_name_in_contest
                print('Want to add new entry {}: {}'.format(problem_name_in_contest, newname))
                sys.exit(1)
            result = conf['problem_map'][problem_name_in_contest]
            print(f'Conversion happened: {problem_name} => {result}')
            problem_name = result
        else:
            print("In-contest submission! TODO")
            sys.exit(1)

    data = get_problem_info(problem_name)
    problem_id = data['id']
    problem_title = data['title']
    available_languages = data['languages']

    # Submit
    if language not in available_languages:
        print('language {} not available: {}'.format(language, available_languages))
        sys.exit(1)
    payload = {
        'problem_id': problem_id,
        'code': source,
        'language': language
    }
    print("Submitting \x1b[34m{}\x1b[0m as \x1b[34m{}\x1b[0m".format(problem_name, language))
    print("\tto \x1b[32m{}\x1b[0m (id={})...".format(
        problem_title, problem_id))
    submit(payload, cookies, dry_run)

    # Log out
    logout(cookies, dry_run)


if __name__ == '__main__':
    main()
