#!/usr/bin/env python3
"""Uploader
Upload in/out files to yukicoder.

Usage: ./upload.py PROJECT_DIR
Mirror-upload the files in in/ and out/
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
    with open(yukicoder_config_path) as file:
        config = yaml.safe_load(file)
    v = cerberus.Validator({'api_key': {'type': 'string'}})
    if not v.validate(config):
        print('Invalid config: {} {}'.format(yukicoder_config_path, v.errors),
              file=sys.stderr)
        sys.exit(1)
    return config


def read_problem_config(problem_config_path):
    """Read problem-specific config
    """
    with open(problem_config_path) as file:
        config = yaml.safe_load(file)
    v = cerberus.Validator({'problem_id': {'type': 'string'}})
    if not v.validate(config):
        print('Invalid config: {} {}'.format(problem_config_path, v.errors),
              file=sys.stderr)
        sys.exit(1)
    return config
    

def list_files(problem_id, which, api_key):
    """ List files in the judge server.
    which: in or out
    """
    url = "https://yukicoder.me/api/v1/problems/{}/file/{}".\
    format(problem_id, which)
    resp = requests.get(url, headers={
        'Authorization': "bearer " + api_key,
        'Accept': 'application/json'
    })
    response = json.loads(resp.text)
    return response
    

def delete_file(problem_id, which, filename, api_key):
    """ Delete a file in the judge server.
    which: in or out
    """
    url = "https://yukicoder.me/api/v1/problems/{}/file/{}/{}".\
    format(problem_id, which, filename)
    resp = requests.delete(url, headers={
        'Authorization': "bearer " + api_key,
        'Accept': 'application/json'
    })
    response = json.loads(resp.text)
    return response


def upload_files(problem_id, which, filenames, api_key):
    """ Upload files from local to the judge server.
    which: in or out
    """
    url = "https://yukicoder.me/api/v1/problems/{}/file/{}".\
    format(problem_id, which)
    for filename in filenames:
        basename = os.path.basename(filename)
        file = open(filename, 'r')
        file_obj = (basename, file)
        resp = requests.post(url, headers={
            'Authorization': "bearer " + api_key,
            'Accept': 'application/json'
        }, files={'newfiles': file_obj})

    response = json.loads(resp.text)
    return response


def main():
    if len(sys.argv) != 2:
        print("./upload.py PROJECT_DIR", file=sys.stderr)
        sys.exit(1)

    # TODO: support dry-run
    dir = sys.argv[1]
    script_dir = os.path.dirname(os.path.abspath(__file__))

    # yukicoder_config
    yukicoder_config_path = os.path.join(script_dir, 'yukicoder_config')
    yukicoder_config = read_yukicoder_config(yukicoder_config_path)
    api_key = yukicoder_config['api_key']

    # problem_config
    problem_config_path = os.path.join(dir, 'problem.yml')
    problem_config = read_problem_config(problem_config_path)
    problem_id = problem_config['problem_id']

    kinds = ['in', 'out']
    for kind in kinds:
        files = list_files(problem_id, kind, api_key)
        print(kind, files, file=sys.stderr)
        # TODO: identity checking, do not touch identical files
        # Mirror-upload: (1) delete all files in the judge server
        for i, filename in enumerate(files):
            print('{} ({}/{}): Deleting {}...'.format(kind, i, len(files), filename), file=sys.stderr)
            delete_file(problem_id, kind, filename, api_key)
        # Mirror-upload: (2) upload files in kind/
        my_files = os.listdir(kind)
        for i, my_file in enumerate(my_files):
            my_file = os.path.join(kind, my_file)
            # TODO: upload multiple files so that each request < 30MB
            # and #requests is as small as possible
            print('{} ({}/{}): Uploading {}...'.format(kind, i, len(my_files), my_file), file=sys.stderr, end='')
            upload_files(problem_id, kind, [my_file], api_key)
            print('done', file=sys.stderr)

main()
