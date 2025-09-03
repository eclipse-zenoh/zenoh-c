#
# Copyright (c) 2022 ZettaScale Technology
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors:
#   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
#


import subprocess, os

read_the_docs_build = os.environ.get('READTHEDOCS', None) == 'True'
if read_the_docs_build:
    subprocess.call('doxygen', shell=True)

# -- Project information -----------------------------------------------------
project = 'zenoh-c'
copyright = '2017, 2022 ZettaScale Technology'
author = 'ZettaScale Zenoh team'
with open("../version.txt", "rt") as f:
    release = f.read()

# -- General configuration ---------------------------------------------------
master_doc = 'index'
extensions = ['breathe']
language = 'c'
breathe_projects = {"zenoh-c": "./doxyxml/xml/"}
breathe_default_project = "zenoh-c"

# -- Options for HTML output -------------------------------------------------
html_theme = 'sphinx_rtd_theme'

# ----------------------------------------------------------------------------
