#!/usr/bin/env python3
"""Submit a solution file to yukicoder.
"""

import sys
import json
import os
import cerberus
import requests
import yaml

def read_yukicoder_config(yukicoder_config_path):
    """Read yukicoder-specific config
    """
    with open(yukicoder_config_path, encoding='utf-8') as file:
        config = yaml.safe_load(file)
    val = cerberus.Validator({'api_key': {'type': 'string'}})
    if not val.validate(config):
        print('Invalid config: {} {}'.format(yukicoder_config_path, val.errors),
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
        print('Invalid languages config: {} {}'.format(language_config_path, val.errors),
              file=sys.stderr)
        sys.exit(1)
    return languages


def main():
    """main
    """
    if len(sys.argv) != 2:
        print("./submit.py PROBLEM_ID.ext")
        sys.exit(1)

    filename = sys.argv[1]

    script_dir = os.path.dirname(os.path.abspath(__file__))
    yukicoder_config_path = os.path.join(script_dir, 'yukicoder_config')
    language_config_path = os.path.join(script_dir, '..', 'languages.yml')

    yukicoder_config = read_yukicoder_config(yukicoder_config_path)

    languages = read_language_config(language_config_path)


    (problem_name, extension) = os.path.splitext(filename)
    if extension == '':
        print("\x1b[34minvalid extension\x1b[0m")
        sys.exit(1)

    lang = next(x for x in languages if x['extension'] == extension[1:])['yukicoder_name']
    with open(filename, encoding='utf-8') as file:
        source = file.read()

    api_key = yukicoder_config['api_key']

    url = f'https://yukicoder.me/api/v1/problems/no/{problem_name}'
    resp = requests.get(url, headers={
        'Authorization': "bearer " + api_key,
        'Accept': 'application/json'
    })
    response = json.loads(resp.text)

    # Find problem_id.
    if 'ProblemId' not in response:
        print('Invalid response: {}', response)
        sys.exit(2)
    problem_id = response['ProblemId']

    post_url = f'https://yukicoder.me/api/v1/problems/{problem_id}/submit'

    print("Submitting \x1b[34m{}\x1b[0m as \x1b[34m{}\x1b[0m".format(filename, lang))
    print("\tto \x1b[32m{}\x1b[0m (problem_id = {})".format(problem_name, problem_id))

    resp = requests.post(post_url, {
        'lang': lang, 'source': source,
    }, headers={
        'Authorization': "bearer " + api_key,
        'Accept': 'application/json'
    })

    response = json.loads(resp.text)

    print(response)

    if 'SubmissionId' not in response:
        print(response)
        print("\x1b[34mSubmission unsuccessful\x1b[0m")
        sys.exit(1)

main()
