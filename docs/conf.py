#
# Copyright (c) 2017, 2020 ADLINK Technology Inc.
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors:
#   ADLINK zenoh team, <zenoh@adlink-labs.tech>
#

# Configuration file for the Sphinx documentation builder.

# -- Project information -----------------------------------------------------
project = 'zenoh-c'
copyright = '2017, 2020 ADLINK Technology Inc'
author = 'ADLINK zenoh team'
release = '0.5.0'

# -- General configuration ---------------------------------------------------
master_doc = 'index'
extensions = ['sphinx_c_autodoc', 'sphinx_c_autodoc.napoleon']
language = 'c'
c_autodoc_roots = ['../include']

# -- Options for HTML output -------------------------------------------------
html_theme = 'sphinx_rtd_theme'

# ----------------------------------------------------------------------------
from clang.cindex import Config
Config.set_library_file('/usr/lib/llvm-6.0/lib/libclang.so.1')
