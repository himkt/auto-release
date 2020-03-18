import datetime
from typing import Dict
import warnings
import os

from github import Github


if __name__ == '__main__':
    print('Please tell me when the oldest PR is opened: ', end='')
    days = int(input())
    timestamp_to_stop_fetch = datetime.datetime.now() - \
        datetime.timedelta(days=days + 2)  # NOTE +2 is an offset

    print('Please put milestone name you want to make a release note: ', end='')
    milestone = input()

    token = os.getenv("GITHUB_ACCESS_TOKEN")
    assert token is not None, '`GITHUB_ACCESS_TOKEN` should not be None'
    g = Github(token)
    repo = g.get_repo("optuna/optuna")

    label2prs: Dict[str, list] = {'No label': []}

    for pr in repo.get_pulls(state="closed"):

        if pr.created_at <= timestamp_to_stop_fetch:
            break

        if not pr.is_merged():
            continue

        if pr.milestone is None:
            warnings.warn(f'PR without milestone is found (#{pr.number})')
            continue

        if pr.milestone.title != milestone:
            continue

        if len(pr.labels) == 0:
            label = 'No label'

        else:
            if len(pr.labels) > 1:
                warnings.warn(f'More than 1 labels are assigned (#{pr.number})')

            label = pr.labels[0].name

        entry = pr.title + f' (#{pr.number})'
        label2prs[label] = label2prs.get(label, []) + [entry]

    for label, prs in label2prs.items():
        print(f'# {label}\n')
        for entry in reversed(prs):
            print(f'- {entry}')
        print()
