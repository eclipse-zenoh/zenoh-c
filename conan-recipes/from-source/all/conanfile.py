from conan import ConanFile
from conan.errors import ConanInvalidConfiguration
from conan.tools.files import apply_conandata_patches, get, copy, export_conandata_patches
from conan.tools.cmake import CMake, CMakeToolchain, cmake_layout, CMakeDeps
import os

required_conan_version = ">=1.53.0"

class ZenohCPackageConan(ConanFile):
    name = "zenohc"
    description = "C-API for Eclipse Zenoh: Zero Overhead Pub/sub, Store/Query and Compute protocol"
    tags = ["iot", "networking", "robotics", "messaging", "ros2", "edge-computing", "micro-controller"]
    license = "Apache License 2.0"
    author = "ZettaScale Zenoh Team"

    url = "https://github.com/conan-io/conan-center-index"
    homepage = "https://github.com/eclipse-zenoh/zenoh-c"

    package_type = "library"
    settings = "os", "compiler", "build_type", "arch"

    options = {
        "shared": [True, False],
        "fPIC": [True, False],
        "ZENOHC_BUILD_WITH_LOGGER_AUTOINIT":[True, False],
        "ZENOHC_BUILD_WITH_SHARED_MEMORY":[True, False],
        "ZENOHC_CARGO_FLAGS": ["ANY"],
    }

    default_options = {
        "shared": False,
        "fPIC": True,
        "ZENOHC_BUILD_WITH_LOGGER_AUTOINIT": True,
        "ZENOHC_BUILD_WITH_SHARED_MEMORY": True,
        "ZENOHC_CARGO_FLAGS": "",
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

    def export_sources(self):
        export_conandata_patches(self)

    def config_options(self):
        if self.settings.os == "Windows":
            del self.options.fPIC

    def configure(self):
        if self.options.shared:
            self.options.rm_safe("fPIC")
        self.settings.rm_safe("compiler.cppstd")
        self.settings.rm_safe("compiler.libcxx")

    def layout(self):
        cmake_layout(self)

    def validate(self):
        if (self.settings.os, self.settings.arch) not in self._supported_platforms:
            raise ConanInvalidConfiguration("{}/{} combination is not supported".format(self.settings.os, self.settings.arch))

    def build_requirements(self):
        self.tool_requires("cmake/[>=3.16 <4]")

    def source(self):
        get(self, **self.conan_data["sources"][self.version], strip_root=True)

    def generate(self):
        tc = CMakeToolchain(self)
        for opt, val in self.options.items():
            tc.variables[opt] = val
        tc.variables["ZENOHC_LIB_STATIC"] = str(not self.options.shared)
        tc.variables["ZENOHC_INSTALL_STATIC_LIBRARY"] = tc.variables["ZENOHC_LIB_STATIC"]
    
        tc.generate()
        deps = CMakeDeps(self)
        deps.generate()

    def build(self):
        apply_conandata_patches(self)
        cmake = CMake(self)
        cmake.configure()
        cmake.build()

    def package(self):
        copy(self, "LICENSE", self.source_folder, os.path.join(self.package_folder, "licenses"))
        cmake = CMake(self)
        cmake.install()

    def package_info(self):
        if self.settings.build_type == "Debug":
            self.cpp_info.libs = ["zenohcd"]
        else:
            self.cpp_info.libs = ["zenohc"]
        
        self.cpp_info.set_property("cmake_file_name", "zenohc")
        self.cpp_info.set_property("cmake_target_name", "zenohc::lib")
        self.cpp_info.set_property("cmake_target_aliases", [f"zenohc::{'shared' if self.options.shared else 'static'}"])

        if self.settings.os == "Windows":
            self.cpp_info.system_libs = ["ws2_32", "crypt32", "secur32", "bcrypt", "ncrypt", "userenv", "ntdll", "iphlpapi", "runtimeobject"]
        elif self.settings.os == "Linux":
            self.cpp_info.system_libs = ["rt", "pthread", "m", "dl"]
        elif self.settings.os == "Macos":
            self.cpp_info.frameworks = ["Foundation", "Security"]
