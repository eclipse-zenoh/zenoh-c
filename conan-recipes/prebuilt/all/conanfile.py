#
# Copyright (c) 2024 ZettaScale Technology
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
from conan import ConanFile
from conan.errors import ConanInvalidConfiguration
from conan.tools.files import copy, download, get
from conan.tools.scm import Version

import platform
import os

required_conan_version = ">=1.53.0"

class ZenohCPackageConan(ConanFile):
    name = "zenohc"
    description = "C-API for Eclipse Zenoh: Zero Overhead Pub/Sub, Store/Query and Compute protocol"
    tags = ["iot", "networking", "robotics", "messaging", "ros2", "edge-computing", "micro-controller", "pre-built"]
    license = "Apache License 2.0"
    author = "ZettaScale Zenoh Team <zenoh@zettascale.tech>"

    url = "https://github.com/conan-io/conan-center-index"
    homepage = "https://github.com/eclipse-zenoh/zenoh-c"

    package_type = "library"
    settings = "os", "compiler", "build_type", "arch"

    options = {
        "shared": [True],
    }
    default_options = {
        "shared": True,
    }

    @property
    def _supported_platforms(self):
        return [
            ("Windows", "x86_64"),
            ("Linux", "x86_64"),
            ("Linux", "armv6"),
            ("Linux", "armv7hf"),
            ("Linux", "armv8"),
            ("Macos", "x86_64"),
            ("Macos", "armv8"),
        ]

    def layout(self):
        pass

    def configure(self):
        self.settings.rm_safe("compiler.cppstd")
        self.settings.rm_safe("compiler.libcxx")

    def package_id(self):
        del self.info.settings.compiler
        del self.info.settings.build_type

    def validate(self):
        if (self.settings.os, self.settings.arch) not in self._supported_platforms:
            raise ConanInvalidConfiguration("{}/{} target is not supported".format(self.settings.os, self.settings.arch))
        if self.settings.os == "Linux":
            libver = platform.libc_ver()
            print(libver)
            if libver[0] == "glibc" and Version(libver[1]) < '2.29':
                raise ConanInvalidConfiguration("This library requires glibc >= 2.29")

    def source(self):
        pass

    def build(self):
        get(self, **self.conan_data["sources"][self.version][str(self.settings.os)][str(self.settings.arch)])
        download(self, **self.conan_data["sources"][self.version]["license"], filename="LICENSE")

    def package(self):
        copy(self, "LICENSE", self.build_folder, os.path.join(self.package_folder, "licenses"))
        copy(self, "*", os.path.join(self.build_folder, "lib"), os.path.join(self.package_folder, "lib"))
        copy(self, "*", os.path.join(self.build_folder, "include"), os.path.join(self.package_folder, "include"))

    def package_info(self):
        self.cpp_info.set_property("cmake_file_name", "zenohc")
        self.cpp_info.set_property("cmake_target_name", "zenohc::lib")

        self.cpp_info.libs = ["zenohc"]
        self.cpp_info.libdirs = ["lib"]
        self.cpp_info.includedirs = ["include"]
