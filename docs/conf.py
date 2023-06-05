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

# Configuration file for the Sphinx documentation builder.
from clang.cindex import Config

# -- Project information -----------------------------------------------------
project = 'zenoh-c'
copyright = '2017, 2022 ZettaScale Technology'
author = 'ZettaScale Zenoh team'
release = '0.7.2.1'

# -- General configuration ---------------------------------------------------
master_doc = 'index'
extensions = ['sphinx_c_autodoc', 'sphinx_c_autodoc.napoleon']
language = 'c'
c_autodoc_roots = ['../include']

# -- Options for HTML output -------------------------------------------------
html_theme = 'sphinx_rtd_theme'

# ----------------------------------------------------------------------------
Config.set_library_file('/usr/lib/llvm-6.0/lib/libclang.so.1')
