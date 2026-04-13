use crate::make_cmake_packages;

make_cmake_packages!(
    { FastFetch, "fastfetch", tarball_url = "https://github.com/fastfetch-cli/fastfetch/archive/refs/tags/2.61.0.tar.gz", configure = { args = vec![
        "-DCMAKE_SYSTEM_NAME=Seele".to_string(),
        "-DCMAKE_POSITION_INDEPENDENT_CODE=OFF".to_string(),
        "-DBUILD_FLASHFETCH=OFF".to_string(),
        "-DBUILD_TESTS=OFF".to_string(),
        "-DENABLE_WAYLAND=OFF".to_string(),
        "-DENABLE_XCB_RANDR=OFF".to_string(),
        "-DENABLE_XRANDR=OFF".to_string(),
        "-DENABLE_DRM=OFF".to_string(),
        "-DENABLE_DRM_AMDGPU=OFF".to_string(),
        "-DENABLE_GIO=OFF".to_string(),
        "-DENABLE_DCONF=OFF".to_string(),
        "-DENABLE_DBUS=OFF".to_string(),
        "-DENABLE_SQLITE3=OFF".to_string(),
        "-DENABLE_RPM=OFF".to_string(),
        "-DENABLE_IMAGEMAGICK7=OFF".to_string(),
        "-DENABLE_IMAGEMAGICK6=OFF".to_string(),
        "-DENABLE_CHAFA=OFF".to_string(),
        "-DENABLE_EGL=OFF".to_string(),
        "-DENABLE_GLX=OFF".to_string(),
        "-DENABLE_OPENCL=OFF".to_string(),
        "-DENABLE_PULSE=OFF".to_string(),
        "-DENABLE_DDCUTIL=OFF".to_string(),
        "-DENABLE_ELF=OFF".to_string(),
        "-DENABLE_LIBZFS=OFF".to_string(),
        "-DENABLE_EMBEDDED_PCIIDS=OFF".to_string(),
        "-DENABLE_EMBEDDED_AMDGPUIDS=OFF".to_string(),
        "-DENABLE_WORDEXP=OFF".to_string(),
    ] } },
    { RenderTM, "rendertm", git_url = "https://github.com/xiaoyi1212/RenderTM" }
);
