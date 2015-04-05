#!/usr/bin/env python3

import os
import shutil
import subprocess
import tempfile

REPO = 'git@github.com:nicholasbishop/dumbmath-rs.git'
BRANCH = 'gh-pages'
COMMIT_MSG = 'Automatic-ish rustdoc update'


def run_cmd(cmd, cwd=None):
    print(' '.join(cmd))
    subprocess.check_call(cmd, cwd=cwd)


def main():
    run_cmd(['cargo', 'doc'])

    with tempfile.TemporaryDirectory(prefix='update-gh-pages-') as tmp_dir:
        run_cmd(['git', 'clone', REPO, tmp_dir, '--branch', BRANCH])
        dst_doc_dir = os.path.join(tmp_dir, 'doc')

        print('rm -r', dst_doc_dir)
        shutil.rmtree(dst_doc_dir)

        print('cp -r', 'target/doc', dst_doc_dir)
        shutil.copytree('target/doc', dst_doc_dir)

        run_cmd(['git', 'add', 'doc'], cwd=tmp_dir)
        run_cmd(['git', 'commit', 'doc', '-m', COMMIT_MSG], cwd=tmp_dir)
        run_cmd(['git', 'push'], cwd=tmp_dir)


if __name__ == '__main__':
    main()
