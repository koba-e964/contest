#!/usr/bin/env python3

import sys
import os
import json
import requests
import cerberus
import yaml
from selenium import webdriver
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.chrome.service import Service


def read_apc_config(apc_config_path):
    """Read apc-specific config
    """
    with open(apc_config_path) as file:
        config = yaml.safe_load(file)
    v = cerberus.Validator({
        'username': {'required': True, 'type': 'string'},
        'password': {'required': True, 'type': 'string'},
        'chrome_executable_path': {'required': True, 'type': 'string'},
    })
    if not v.validate(config):
        print('Invalid config: {} {}'.format(apc_config_path, v.errors),
              file=sys.stderr)
        sys.exit(1)
    return config


def get_csrftoken(url, chrome_executable_path, dry_run):
    if dry_run:
        print("> Would get CSRF token from {}".format(url))
        return 'CSRFTokenDummy'
    # Headless
    # https://prcmyself.com/selenium-scraping-with-browser-hidden
    options = Options()
    options.add_argument('--headless')

    service = Service(chrome_executable_path)

    driver = webdriver.Chrome(service=service, options=options)

    driver.get(url)
    csrftoken = driver.get_cookie("csrftoken")['value']
    driver.close()
    return csrftoken


def main():
    if len(sys.argv) != 2:
        print("./submit.py PROBLEM_ID.ext")
        sys.exit(1)

    # If DRY_RUN=1, this program emits what would be executed in
    # the real run.
    dry_run = False
    if 'DRY_RUN' in os.environ and os.environ['DRY_RUN'] == '1':
        dry_run = True

    filename = sys.argv[1]

    top = "https://apc.run"

    script_dir = os.path.dirname(os.path.abspath(__file__))
    apc_config_path = os.path.join(script_dir, 'apc_config.yml')

    apc_config = read_apc_config(apc_config_path)
    chrome_executable_path = apc_config['chrome_executable_path']
    username = apc_config['username']
    password = apc_config['password']

    (problem_name, extension) = os.path.splitext(filename)
    # TODO when problem_name is contest_name + "/" + problem_name_in_contest,
    # automatically convert
    # and create a yml file for automatic conversion
    # If contest.status == "-1" then the contest is finished hence no submissions are allowed.
    # ref: https://github.com/QingdaoU/OnlineJudgeFE/blob/oj_2.7.5/src/store/modules/contest.js#L62-L63
    print(problem_name)
    if extension == '':
        print("\x1b[34minvalid extension\x1b[0m")
        sys.exit(1)

    with open(filename) as file:
        source = file.read()

    csrftoken = get_csrftoken(top, chrome_executable_path, dry_run)

    # Login
    headers = {
        'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64; rv:57.0) Gecko/20100101 Firefox/57.0',
        'cookie': 'csrftoken=' + csrftoken,
        'referer': 'https://apc.run/',
        'x-csrftoken': csrftoken,
    }
    if dry_run:
        print('> Would POST to https://apc.run/api/login as {}'.format(username))
        sessionid = 'DummySessionId'
    else:
        resp = requests.post("https://apc.run/api/login", {"username": username, "password": password}, headers=headers)
        csrftoken = resp.cookies['csrftoken']
        sessionid = resp.cookies['sessionid']

    # Get problem_id from problem_name
    problem_id = None
    problem_title = ''
    available_languages = []
    if dry_run and False:
        pass
    else:
        resp = requests.get('https://apc.run/api/problem', {'problem_id': problem_name})
        data = json.loads(resp.text)
        problem_id = data['data']['id']
        problem_title = data['data']['title']
        available_languages = data['data']['languages']
        if resp.status_code // 100 != 2:
            print(resp.status_code, file=sys.stderr)
            print(resp.text, file=sys.stderr)
            print(resp.headers, file=sys.stderr)
            print(resp.cookies, file=sys.stderr)
            sys.exit(1)

    # Submit
    headers = {
        'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64; rv:57.0) Gecko/20100101 Firefox/57.0',
        'cookie': 'csrftoken=' + csrftoken + '; sessionid=' + sessionid,
        'referer': 'https://apc.run/',
        'x-csrftoken': csrftoken,
    }
    code = source
    language = 'C++' # TODO
    payload = {
        'problem_id': problem_id,
        'code': code,
        'language': language
    }
    print("Submitting \x1b[34m{}\x1b[0m as \x1b[34m{}\x1b[0m".format(problem_name, language))
    print("\tto \x1b[32m{}\x1b[0m (id={})...".format(
        problem_title, problem_id))
    if dry_run:
        print('> Would POST to https://apc.run/api/submission, {}'.format(payload))
    else:
        resp = requests.post('https://apc.run/api/submission', payload, headers=headers)
        if resp.status_code // 100 != 2:
            print("Submission failed!", file=sys.stderr)
            print(resp.status_code, file=sys.stderr)
            print(resp.text, file=sys.stderr)
            print(resp.headers, file=sys.stderr)
            print(resp.cookies, file=sys.stderr)

    # Log out
    headers = {
        'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64; rv:57.0) Gecko/20100101 Firefox/57.0',
        'cookie': 'csrftoken=' + csrftoken + '; sessionid=' + sessionid,
        'referer': 'https://apc.run/',
        'content-type': 'application/json',
        'x-csrftoken': csrftoken,
    }
    if dry_run:
        print('> Would GET https://apc.run/api/logout')
    else:
        resp = requests.get('https://apc.run/api/logout', headers=headers)
        if resp.status_code // 100 != 2:
            print("Logout failed!", file=sys.stderr)
            print(resp.status_code, file=sys.stderr)
            print(resp.text, file=sys.stderr)
            print(resp.headers, file=sys.stderr)
            print(resp.cookies, file=sys.stderr)


main()
