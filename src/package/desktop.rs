use std::fs;
use std::os::unix::fs::symlink;

use crate::build::{build_autotools_with, build_meson};
use crate::command::{CommandSpec, run};
use crate::configure::{configure_autotools, configure_meson};
use crate::cross::target_env;
use crate::fs_utils::{copy_file, ensure_dir};
use crate::install::{install_autotools, install_meson};
use crate::layout::{LIB_BINARY_DIR, relative_dir};
use crate::make_autotools_package;
use crate::make_meson_packages;
use crate::make_meta_package;
use crate::make_package;
use crate::misc::sysroot_dir;
use crate::make_autotools_packages;
use crate::package::feh::Imlib2;
use crate::package::gtk::Cairo;

use crate::package::xorg::{
    Freetype2, LibSm, LibX11, LibXext, LibXfixes, LibXinerama, LibXrandr, LibXrender,
    XorgProto, Zlib,
};
use crate::r#trait::{Package, apply_patch_file};

fn rewrite_openbox_script(sysroot: &std::path::Path, rel: &str) -> crate::types::Result<()> {
    let path = sysroot.join(rel.trim_start_matches('/'));
    if !path.is_file() {
        return Ok(());
    }

    let content = fs::read_to_string(&path)?;
    let content = content
        .replace("#!/bin/sh", "#!/programs/bash")
        .replace("\n    sh ", "\n    /programs/bash ")
        .replace("\n    sh\t", "\n    /programs/bash\t")
        .replace("\n    sh$", "\n    /programs/bash$");
    fs::write(path, content)?;
    Ok(())
}

fn patch_openbox_source(ctx: &crate::types::Context) -> crate::types::Result<()> {
    let paths = Openbox.calc_paths(ctx);
    if paths.patch.exists() {
        apply_patch_file(&paths.src, &paths.patch)?;
    }

    let keyboard_path = paths.src.join("obt/keyboard.c");
    let keyboard = fs::read_to_string(&keyboard_path)?;
    let keyboard = keyboard.replace(
        "    GSList *it;\n    gchar *aname, *aclass;\n\n    aname = g_strdup(g_get_prgname());\n    if (!aname) aname = g_strdup(\"obt\");\n\n    aclass = g_strdup(aname);\n    if (g_ascii_islower(aclass[0]))\n        aclass[0] = g_ascii_toupper(aclass[0]);\n\n    xim = XOpenIM(obt_display, NULL, aname, aclass);\n\n    if (!xim)\n        g_message(\"Failed to open an Input Method\");\n    else {\n        XIMStyles *xim_styles = NULL;\n        char *r;\n\n        /* get the input method styles */\n        r = XGetIMValues(xim, XNQueryInputStyle, &xim_styles, NULL);\n        if (r || !xim_styles)\n            g_message(\"Input Method does not support any styles\");\n        if (xim_styles) {\n            int i;\n\n            /* find a style that doesnt need preedit or status */\n            for (i = 0; i < xim_styles->count_styles; ++i) {\n                if (xim_styles->supported_styles[i] == \n                    (XIMPreeditNothing | XIMStatusNothing))\n                {\n                    xim_style = xim_styles->supported_styles[i];\n                    break;\n                }\n            }\n            XFree(xim_styles);\n        }\n\n        if (!xim_style) {\n            g_message(\"Input Method does not support a usable style\");\n\n            XCloseIM(xim);\n            xim = NULL;\n        }\n    }\n\n    /* any existing contexts need to be recreated for the new input method */\n    for (it = xic_all; it; it = g_slist_next(it))\n        obt_keyboard_context_renew(it->data);\n\n    g_free(aclass);\n    g_free(aname);\n",
        "    GSList *it;\n\n    /* SeeleOS does not provide a working XIM stack yet. */\n    xim = NULL;\n    xim_style = 0;\n\n    /* any existing contexts need to be recreated for the new input method */\n    for (it = xic_all; it; it = g_slist_next(it))\n        obt_keyboard_context_renew(it->data);\n",
    );
    fs::write(&keyboard_path, keyboard)?;

    let openbox_path = paths.src.join("openbox/openbox.c");
    let openbox = fs::read_to_string(&openbox_path)?;
    let mut openbox = openbox.replace(
        "    if (!XSetLocaleModifiers(\"\"))\n",
        "    if (!XSetLocaleModifiers(\"@im=none\"))\n",
    );
    if !openbox.contains("seele_debug_stage(") {
        openbox = openbox.replace(
            "static void run_startup_cmd(void);\n",
            "static void run_startup_cmd(void);\n\
static void seele_debug_stage(const char *stage)\n\
{\n\
    write(2, stage, strlen(stage));\n\
    write(2, \"\\n\", 1);\n\
}\n",
        );
        openbox = openbox.replace(
            "    if (!remote_control)\n        session_startup(argc, argv);\n\n    if (!obt_display_open(NULL))\n",
            "    if (!remote_control)\n        session_startup(argc, argv);\n\n    seele_debug_stage(\"OB: before obt_display_open\");\n    if (!obt_display_open(NULL))\n",
        );
        openbox = openbox.replace(
            "    ob_main_loop = g_main_loop_new(NULL, FALSE);\n",
            "    seele_debug_stage(\"OB: display open ok\");\n    ob_main_loop = g_main_loop_new(NULL, FALSE);\n",
        );
        openbox = openbox.replace(
            "    ob_rr_inst = RrInstanceNew(obt_display, ob_screen);\n",
            "    seele_debug_stage(\"OB: before RrInstanceNew\");\n    ob_rr_inst = RrInstanceNew(obt_display, ob_screen);\n",
        );
        openbox = openbox.replace(
            "    if (screen_annex()) { /* it will be ours! */\n",
            "    seele_debug_stage(\"OB: before screen_annex\");\n    if (screen_annex()) { /* it will be ours! */\n",
        );
        openbox = openbox.replace(
            "        do {\n",
            "        seele_debug_stage(\"OB: screen_annex ok\");\n        do {\n",
        );
        openbox = openbox.replace(
            "                /* register all the available actions */\n                actions_startup(reconfigure);\n",
            "                seele_debug_stage(\"OB: actions_startup\");\n                /* register all the available actions */\n                actions_startup(reconfigure);\n",
        );
        openbox = openbox.replace(
            "            {\n                RrTheme *theme;\n",
            "            seele_debug_stage(\"OB: config loaded\");\n            {\n                RrTheme *theme;\n",
        );
        openbox = openbox.replace(
            "            event_startup(reconfigure);\n",
            "            seele_debug_stage(\"OB: theme loaded\");\n            seele_debug_stage(\"OB: event_startup\");\n            event_startup(reconfigure);\n",
        );
        openbox = openbox.replace(
            "            sn_startup(reconfigure);\n",
            "            seele_debug_stage(\"OB: sn_startup\");\n            sn_startup(reconfigure);\n",
        );
        openbox = openbox.replace(
            "            window_startup(reconfigure);\n",
            "            seele_debug_stage(\"OB: window_startup\");\n            window_startup(reconfigure);\n",
        );
        openbox = openbox.replace(
            "            focus_startup(reconfigure);\n",
            "            seele_debug_stage(\"OB: focus_startup\");\n            focus_startup(reconfigure);\n",
        );
        openbox = openbox.replace(
            "            screen_startup(reconfigure);\n",
            "            seele_debug_stage(\"OB: screen_startup\");\n            screen_startup(reconfigure);\n",
        );
        openbox = openbox.replace(
            "            grab_startup(reconfigure);\n",
            "            seele_debug_stage(\"OB: grab_startup\");\n            grab_startup(reconfigure);\n",
        );
        openbox = openbox.replace(
            "            client_startup(reconfigure);\n",
            "            seele_debug_stage(\"OB: client_startup\");\n            client_startup(reconfigure);\n",
        );
        openbox = openbox.replace(
            "            dock_startup(reconfigure);\n",
            "            seele_debug_stage(\"OB: dock_startup\");\n            dock_startup(reconfigure);\n",
        );
        openbox = openbox.replace(
            "            keyboard_startup(reconfigure);\n",
            "            seele_debug_stage(\"OB: keyboard_startup\");\n            keyboard_startup(reconfigure);\n",
        );
        openbox = openbox.replace(
            "            mouse_startup(reconfigure);\n",
            "            seele_debug_stage(\"OB: mouse_startup\");\n            mouse_startup(reconfigure);\n",
        );
        openbox = openbox.replace(
            "            menu_startup(reconfigure);\n",
            "            seele_debug_stage(\"OB: menu_startup\");\n            menu_startup(reconfigure);\n",
        );
        openbox = openbox.replace(
            "            prompt_startup(reconfigure);\n",
            "            seele_debug_stage(\"OB: prompt_startup\");\n            prompt_startup(reconfigure);\n",
        );
        openbox = openbox.replace(
            "                xqueue_listen();\n",
            "                seele_debug_stage(\"OB: xqueue_listen\");\n                xqueue_listen();\n",
        );
        openbox = openbox.replace(
            "                window_manage_all();\n",
            "                seele_debug_stage(\"OB: window_manage_all\");\n                window_manage_all();\n",
        );
        openbox = openbox.replace(
            "            ob_set_state(OB_STATE_RUNNING);\n",
            "            seele_debug_stage(\"OB: entering running state\");\n            ob_set_state(OB_STATE_RUNNING);\n",
        );
        openbox = openbox.replace(
            "            g_main_loop_run(ob_main_loop);\n",
            "            seele_debug_stage(\"OB: entering main loop\");\n            g_main_loop_run(ob_main_loop);\n",
        );
    }
    fs::write(&openbox_path, openbox)?;

    let instance_path = paths.src.join("obrender/instance.c");
    let instance = fs::read_to_string(&instance_path)?;
    let mut instance = instance;
    if !instance.contains("RR: before pango_xft_get_context") {
        instance = instance.replace(
            "#include \"instance.h\"\n",
            "#include \"instance.h\"\n#include <string.h>\n#include <unistd.h>\n\nstatic void rr_debug_stage(const char *stage)\n{\n    write(2, stage, strlen(stage));\n    write(2, \"\\n\", 1);\n}\n",
        );
        instance = instance.replace(
            "    definst->depth = DefaultDepth(display, screen);\n",
            "    rr_debug_stage(\"RR: instance start\");\n    definst->depth = DefaultDepth(display, screen);\n",
        );
        instance = instance.replace(
            "    definst->colormap = DefaultColormap(display, screen);\n",
            "    definst->colormap = DefaultColormap(display, screen);\n    rr_debug_stage(\"RR: before pango_xft_get_context\");\n",
        );
        instance = instance.replace(
            "    definst->pango = pango_xft_get_context(display, screen);\n",
            "    definst->pango = pango_xft_get_context(display, screen);\n    rr_debug_stage(\"RR: after pango_xft_get_context\");\n",
        );
        instance = instance.replace(
            "    definst->color_hash = g_hash_table_new_full(g_int_hash, g_int_equal,\n                                                NULL, dest);\n",
            "    rr_debug_stage(\"RR: before color_hash\");\n    definst->color_hash = g_hash_table_new_full(g_int_hash, g_int_equal,\n                                                NULL, dest);\n    rr_debug_stage(\"RR: after color_hash\");\n",
        );
        instance = instance.replace(
            "    switch (definst->visual->class) {\n",
            "    rr_debug_stage(\"RR: before visual setup\");\n    switch (definst->visual->class) {\n",
        );
        instance = instance.replace(
            "    return definst;\n",
            "    rr_debug_stage(\"RR: instance ready\");\n    return definst;\n",
        );
    }
    fs::write(&instance_path, instance)?;

    Ok(())
}

fn patch_pango_source(ctx: &crate::types::Context) -> crate::types::Result<()> {
    let paths = Pango.calc_paths(ctx);
    if paths.patch.exists() {
        apply_patch_file(&paths.src, &paths.patch)?;
    }

    let path = paths.src.join("pango/pangoxft-fontmap.c");
    let source = fs::read_to_string(&path)?;
    let mut source = source;

    if !source.contains("PANGOXFT: get_context enter") {
        source = source.replace(
            "#include <string.h>\n",
            "#include <string.h>\n#include <unistd.h>\n\nstatic void pango_xft_debug_stage(const char *stage)\n{\n  write(2, stage, strlen(stage));\n  write(2, \"\\n\", 1);\n}\n",
        );
        source = source.replace(
            "  fontmap = pango_xft_find_font_map (display, screen);\n",
            "  pango_xft_debug_stage(\"PANGOXFT: get_font_map enter\");\n  fontmap = pango_xft_find_font_map (display, screen);\n",
        );
        source = source.replace(
            "  if (fontmap)\n    return fontmap;\n",
            "  if (fontmap)\n    {\n      pango_xft_debug_stage(\"PANGOXFT: get_font_map cache hit\");\n      return fontmap;\n    }\n",
        );
        source = source.replace(
            "  xftfontmap = (PangoXftFontMap *)g_object_new (PANGO_TYPE_XFT_FONT_MAP, NULL);\n",
            "  pango_xft_debug_stage(\"PANGOXFT: before g_object_new font map\");\n  xftfontmap = (PangoXftFontMap *)g_object_new (PANGO_TYPE_XFT_FONT_MAP, NULL);\n  pango_xft_debug_stage(\"PANGOXFT: after g_object_new font map\");\n",
        );
        source = source.replace(
            "  register_display (display);\n",
            "  pango_xft_debug_stage(\"PANGOXFT: before register_display\");\n  register_display (display);\n  pango_xft_debug_stage(\"PANGOXFT: after register_display\");\n",
        );
        source = source.replace(
            "  return PANGO_FONT_MAP (xftfontmap);\n",
            "  pango_xft_debug_stage(\"PANGOXFT: get_font_map return\");\n  return PANGO_FONT_MAP (xftfontmap);\n",
        );
        source = source.replace(
            "  return pango_font_map_create_context (pango_xft_get_font_map (display, screen));\n",
            "  PangoFontMap *font_map;\n\n  pango_xft_debug_stage(\"PANGOXFT: get_context enter\");\n  font_map = pango_xft_get_font_map (display, screen);\n  pango_xft_debug_stage(\"PANGOXFT: before create_context\");\n  {\n    PangoContext *context = pango_font_map_create_context (font_map);\n    pango_xft_debug_stage(\"PANGOXFT: after create_context\");\n    return context;\n  }\n",
        );
    }

    fs::write(&path, source)?;

    let fc_path = paths.src.join("pango/pangofc-fontmap.c");
    let fc_source = fs::read_to_string(&fc_path)?;
    let mut fc_source = fc_source;

    if !fc_source.contains("PANGOFC: init enter") {
        fc_source = fc_source.replace(
            "#include <hb-ft.h>\n",
            "#include <hb-ft.h>\n#include <string.h>\n#include <unistd.h>\n\nstatic void pango_fc_debug_stage(const char *stage)\n{\n  write(2, stage, strlen(stage));\n  write(2, \"\\n\", 1);\n}\n",
        );
        fc_source = fc_source.replace(
            "static void\nstart_fontconfig_thread (PangoFcFontMap *fcfontmap)\n{\n",
            "static void\nstart_fontconfig_thread (PangoFcFontMap *fcfontmap)\n{\n  pango_fc_debug_stage(\"PANGOFC: start_fontconfig_thread enter\");\n",
        );
        fc_source = fc_source.replace(
            "  thread = g_thread_new (\"[pango] fontconfig\", fc_thread_func, g_async_queue_ref (fcfontmap->priv->queue));\n",
            "  pango_fc_debug_stage(\"PANGOFC: before g_thread_new\");\n  thread = g_thread_new (\"[pango] fontconfig\", fc_thread_func, g_async_queue_ref (fcfontmap->priv->queue));\n  pango_fc_debug_stage(\"PANGOFC: after g_thread_new\");\n",
        );
        fc_source = fc_source.replace(
            "      g_async_queue_push (fcfontmap->priv->queue, thread_data_new (FC_INIT, NULL));\n",
            "      pango_fc_debug_stage(\"PANGOFC: before FC_INIT push\");\n      g_async_queue_push (fcfontmap->priv->queue, thread_data_new (FC_INIT, NULL));\n      pango_fc_debug_stage(\"PANGOFC: after FC_INIT push\");\n",
        );
        fc_source = fc_source.replace(
            "  g_mutex_unlock (&fc_init_mutex);\n}\n",
            "  g_mutex_unlock (&fc_init_mutex);\n  pango_fc_debug_stage(\"PANGOFC: start_fontconfig_thread return\");\n}\n",
        );
        fc_source = fc_source.replace(
            "static void\npango_fc_font_map_init (PangoFcFontMap *fcfontmap)\n{\n",
            "static void\npango_fc_font_map_init (PangoFcFontMap *fcfontmap)\n{\n  pango_fc_debug_stage(\"PANGOFC: init enter\");\n",
        );
        fc_source = fc_source.replace(
            "  priv = fcfontmap->priv = pango_fc_font_map_get_instance_private (fcfontmap);\n",
            "  pango_fc_debug_stage(\"PANGOFC: before get_instance_private\");\n  priv = fcfontmap->priv = pango_fc_font_map_get_instance_private (fcfontmap);\n  pango_fc_debug_stage(\"PANGOFC: after get_instance_private\");\n",
        );
        fc_source = fc_source.replace(
            "  priv->font_hash = g_hash_table_new ((GHashFunc)pango_fc_font_key_hash,\n",
            "  pango_fc_debug_stage(\"PANGOFC: before font_hash\");\n  priv->font_hash = g_hash_table_new ((GHashFunc)pango_fc_font_key_hash,\n",
        );
        fc_source = fc_source.replace(
            "  priv->queue = g_async_queue_new ();\n\n  start_fontconfig_thread (fcfontmap);\n",
            "  pango_fc_debug_stage(\"PANGOFC: before async_queue_new\");\n  priv->queue = g_async_queue_new ();\n  pango_fc_debug_stage(\"PANGOFC: before start_fontconfig_thread\");\n\n  start_fontconfig_thread (fcfontmap);\n  pango_fc_debug_stage(\"PANGOFC: init return\");\n",
        );
        fc_source = fc_source.replace(
            "static void\npango_fc_font_map_class_init (PangoFcFontMapClass *class)\n{\n",
            "static void\npango_fc_font_map_class_init (PangoFcFontMapClass *class)\n{\n  pango_fc_debug_stage(\"PANGOFC: class_init enter\");\n",
        );
        fc_source = fc_source.replace(
            "  pclass->add_font_file = pango_fc_font_map_add_font_file;\n}\n",
            "  pclass->add_font_file = pango_fc_font_map_add_font_file;\n  pango_fc_debug_stage(\"PANGOFC: class_init return\");\n}\n",
        );
    }

    fs::write(&fc_path, fc_source)?;
    Ok(())
}

fn patch_openbox_session(sysroot: &std::path::Path) -> crate::types::Result<()> {
    let path = sysroot.join("programs/openbox-session");
    if !path.is_file() {
        return Ok(());
    }

    let content = fs::read_to_string(&path)?;
    let content = content.replace(
        "exec /programs/openbox --startup \"//libexec/openbox-autostart OPENBOX\" \"$@\"",
        "exec /programs/openbox --sm-disable --startup \"/libexec/openbox-autostart OPENBOX\" \"$@\"",
    );
    fs::write(path, content)?;
    Ok(())
}

fn patch_openbox_autostart(sysroot: &std::path::Path) -> crate::types::Result<()> {
    let path = sysroot.join("libexec/openbox-autostart");
    if !path.is_file() {
        return Ok(());
    }

    let content = fs::read_to_string(&path)?;
    let content = content.replace(
        "//libexec/openbox-xdg-autostart \"$@\"",
        "if test -x /programs/python && test -f /libexec/openbox-xdg-autostart; then\n    /programs/python /libexec/openbox-xdg-autostart \"$@\"\nfi",
    );
    fs::write(path, content)?;
    Ok(())
}

fn patch_openbox_environment(sysroot: &std::path::Path) -> crate::types::Result<()> {
    let path = sysroot.join("etc/xdg/openbox/environment");
    if !path.is_file() {
        return Ok(());
    }

    let mut content = fs::read_to_string(&path)?;
    if !content.contains("XMODIFIERS") {
        if !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str(
            "\n# SeeleOS does not provide an input method daemon yet.\n\
: \"${XMODIFIERS:=@im=none}\"\n\
export XMODIFIERS\n",
        );
    }
    fs::write(path, content)?;
    Ok(())
}

fn patch_openbox_global_autostart(sysroot: &std::path::Path) -> crate::types::Result<()> {
    let path = sysroot.join("etc/xdg/openbox/autostart");
    if !path.is_file() {
        return Ok(());
    }

    let mut content = fs::read_to_string(&path)?;
    if !content.contains("SeeleOS debug helpers") {
        if !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str(
            "\n# SeeleOS debug helpers: make it obvious that the session is alive.\n\
if which st >/dev/null 2>/dev/null; then\n\
  st &\n\
elif which xeyes >/dev/null 2>/dev/null; then\n\
  xeyes -geometry 150x100+0+0 &\n\
elif which xclock >/dev/null 2>/dev/null; then\n\
  xclock &\n\
fi\n",
        );
    }
    fs::write(path, content)?;
    Ok(())
}

fn openbox_install_hook(ctx: &crate::types::Context) -> crate::types::Result<()> {
    let sysroot = sysroot_dir(ctx)?;
    for rel in [
        "/programs/openbox-session",
        "/programs/openbox-gnome-session",
        "/programs/openbox-kde-session",
        "/libexec/openbox-autostart",
    ] {
        rewrite_openbox_script(&sysroot, rel)?;
    }
    patch_openbox_session(&sysroot)?;
    patch_openbox_autostart(&sysroot)?;
    patch_openbox_environment(&sysroot)?;
    patch_openbox_global_autostart(&sysroot)?;
    Ok(())
}

make_package!(
    Expat,
    "expat",
    tarball_url =
        "https://github.com/libexpat/libexpat/releases/download/R_2_7_3/expat-2.7.3.tar.xz",
    package_impl = {
        fn configure(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            configure_autotools(
                self,
                ctx,
                Vec::new(),
                vec![
                    "--without-docbook".to_string(),
                    "--without-examples".to_string(),
                    "--without-tests".to_string(),
                    "--without-xmlwf".to_string(),
                ],
                Vec::new(),
            )
        }

        fn build(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            build_autotools_with(self, ctx, Vec::new(), Vec::new())?;

            let paths = self.calc_paths(ctx);
            let lib_dir = paths.src.join("lib");
            let so_name = "libexpat.so.1.11.1";
            run(target_env(
                CommandSpec::new("clang")
                    .cwd(&lib_dir)
                    .arg("--target=x86_64-seele")
                    .arg("-shared")
                    .arg("-Wl,-soname,libexpat.so.1")
                    .arg("-o")
                    .arg(lib_dir.join(".libs").join(so_name))
                    .arg("xmlparse.o")
                    .arg("xmltok.o")
                    .arg("xmlrole.o")
                    .arg("-lm"),
                ctx,
            )?)
        }

        fn install(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            install_autotools(self, ctx)?;

            let paths = self.calc_paths(ctx);
            let sysroot = sysroot_dir(ctx)?;
            let target_lib_dir = sysroot.join(LIB_BINARY_DIR.trim_start_matches('/'));
            let built_lib_dir = paths.src.join("lib").join(".libs");
            let so_name = "libexpat.so.1.11.1";
            let target_so = target_lib_dir.join(so_name);
            let target_soname = target_lib_dir.join("libexpat.so.1");
            let target_link = target_lib_dir.join("libexpat.so");

            ensure_dir(&target_lib_dir)?;
            copy_file(&built_lib_dir.join(so_name), &target_so)?;

            for link in [&target_soname, &target_link] {
                if link.exists() || link.is_symlink() {
                    fs::remove_file(link)?;
                }
            }
            symlink(so_name, &target_soname)?;
            symlink(so_name, &target_link)?;
            Ok(())
        }
    }
);

make_package!(
    LiberationFonts,
    "liberation-fonts",
    tarball_url = "https://github.com/liberationfonts/liberation-fonts/files/7261482/liberation-fonts-ttf-2.1.5.tar.gz",
    package_impl = {
        fn configure(&self, _ctx: &crate::types::Context) -> crate::types::Result<()> {
            Ok(())
        }

        fn build(&self, _ctx: &crate::types::Context) -> crate::types::Result<()> {
            Ok(())
        }

        fn install(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            let paths = self.calc_paths(ctx);
            let sysroot = sysroot_dir(ctx)?;
            let font_dir = sysroot.join(relative_dir("/share/fonts/truetype/liberation"));

            ensure_dir(&font_dir)?;
            for entry in fs::read_dir(&paths.src)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "ttf") {
                    copy_file(&path, &font_dir.join(entry.file_name()))?;
                }
            }

            Ok(())
        }
    }
);

make_package!(
    Fontconfig,
    "fontconfig",
    tarball_url = "https://gitlab.freedesktop.org/api/v4/projects/890/packages/generic/fontconfig/2.17.1/fontconfig-2.17.1.tar.xz",
    dependencies = [Expat, Freetype2],
    package_impl = {
        fn configure(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            configure_autotools(
                self,
                ctx,
                Vec::new(),
                vec![
                    "--disable-docs".to_string(),
                    "--with-expat-includes=/home/elysia/coding-project/elysia-os/packages/work/sysroot-stage/libs/include".to_string(),
                    "--with-expat-lib=/home/elysia/coding-project/elysia-os/packages/work/sysroot-stage/libs/lib_binaries".to_string(),
                ],
                Vec::new(),
            )
        }

        fn build(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            build_autotools_with(self, ctx, Vec::new(), Vec::new())
        }

        fn install(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            install_autotools(self, ctx)?;

            let sysroot = sysroot_dir(ctx)?;
            let fonts_dir = sysroot.join(relative_dir("/etc/fonts"));
            ensure_dir(&fonts_dir)?;
            fs::write(
                fonts_dir.join("fonts.conf"),
                r#"<?xml version="1.0"?>
<!DOCTYPE fontconfig SYSTEM "urn:fontconfig:fonts.dtd">
<fontconfig>
  <dir>/share/fonts</dir>
  <dir>/share/fonts/truetype</dir>
  <dir>/share/fonts/truetype/liberation</dir>

  <cachedir>/tmp/fontconfig</cachedir>

  <match target="pattern">
    <test name="family" qual="any">
      <string>monospace</string>
    </test>
    <edit name="family" mode="prepend" binding="strong">
      <string>Liberation Mono</string>
    </edit>
  </match>

  <alias>
    <family>monospace</family>
    <prefer>
      <family>Liberation Mono</family>
    </prefer>
  </alias>
</fontconfig>
"#,
            )?;

            Ok(())
        }
    }
);

make_autotools_packages!(
    { Gettext, "gettext", tarball_url = "https://ftp.gnu.org/pub/gnu/gettext/gettext-0.26.tar.gz", dependencies = [LibIconv], configure = { args = vec!["--disable-java".to_string(), "--disable-csharp".to_string(), "--disable-openmp".to_string(), "--disable-native-java".to_string(), "--without-emacs".to_string(), "--without-git".to_string(), "--without-cvs".to_string(), "--without-xz".to_string(), "--without-bzip2".to_string()] } },
    { LibFfi, "libffi", tarball_url = "https://github.com/libffi/libffi/releases/download/v3.4.8/libffi-3.4.8.tar.gz", configure = { args = vec!["--disable-docs".to_string()] } },
    { LibIconv, "libiconv", tarball_url = "https://ftp.gnu.org/pub/gnu/libiconv/libiconv-1.19.tar.gz" },
    { Pcre2, "pcre2", tarball_url = "https://github.com/PCRE2Project/pcre2/releases/download/pcre2-10.46/pcre2-10.46.tar.bz2", configure = { args = vec!["--enable-pcre2-8".to_string(), "--disable-pcre2-16".to_string(), "--disable-pcre2-32".to_string(), "--disable-jit".to_string(), "--disable-pcre2grep-jit".to_string(), "--disable-pcre2grep-callout".to_string(), "--disable-pcre2grep-callout-fork".to_string()] } },
    { Fribidi, "fribidi", tarball_url = "https://github.com/fribidi/fribidi/releases/download/v1.0.16/fribidi-1.0.16.tar.xz" },
    { LibXft, "libxft", tarball_url = "https://www.x.org/archive/individual/lib/libXft-2.3.9.tar.xz", dependencies = [XorgProto, LibX11, LibXrender, Freetype2, Fontconfig] },
    { LibXcursor, "libxcursor", tarball_url = "https://www.x.org/archive/individual/lib/libXcursor-1.2.3.tar.xz", dependencies = [XorgProto, LibX11, LibXfixes, LibXrender] },
    { LibXml2, "libxml2", tarball_url = "https://download.gnome.org/sources/libxml2/2.14/libxml2-2.14.6.tar.xz", dependencies = [Zlib], configure = { args = vec!["--without-python".to_string(), "--without-lzma".to_string(), "--without-iconv".to_string()] } },
);

make_package!(
    Openbox,
    "openbox",
    tarball_url = "https://openbox.org/dist/openbox/openbox-3.6.1.tar.gz",
    dependencies = [
        Glib2,
        Pango,
        LibXml2,
        LibXft,
        LibXcursor,
        LibXinerama,
        LibXrandr,
        LibSm,
        LibXext,
        LibX11
    ],
    package_impl = {
        fn patch(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            patch_openbox_source(ctx)
        }

        fn configure(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            configure_autotools(
                self,
                ctx,
                Vec::new(),
                vec![
                    "--disable-imlib2".to_string(),
                    "--disable-startup-notification".to_string(),
                    "--disable-nls".to_string(),
                ],
                Vec::new(),
            )
        }

        fn build(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            build_autotools_with(self, ctx, Vec::new(), Vec::new())
        }

        fn install(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            install_autotools(self, ctx)?;
            openbox_install_hook(ctx)
        }
    }
);

make_autotools_package!(
    Dwm,
    "dwm",
    git_url = "https://github.com/SeeleOS/dwm",
    dependencies = [LibX11, LibXext, LibXrender, LibXft, LibXinerama, Fontconfig, Freetype2, Gettext, Imlib2],
    configure_override = {
    }
);

make_meson_packages!(
    { Glib2, "glib2", tarball_url = "https://download.gnome.org/sources/glib/2.84/glib-2.84.4.tar.xz", dependencies = [Gettext, LibFfi, LibIconv, Pcre2], configure = { args = vec!["-Dtests=false".to_string(), "-Dinstalled_tests=false".to_string(), "-Dintrospection=disabled".to_string(), "-Dnls=disabled".to_string(), "-Dxattr=false".to_string(), "-Dselinux=disabled".to_string(), "-Dlibmount=disabled".to_string(), "-Ddtrace=disabled".to_string(), "-Dsystemtap=disabled".to_string(), "-Dsysprof=disabled".to_string(), "-Dlibelf=disabled".to_string()] } },
    { Harfbuzz, "harfbuzz", tarball_url = "https://github.com/harfbuzz/harfbuzz/releases/download/11.4.4/harfbuzz-11.4.4.tar.xz", dependencies = [Glib2, Freetype2], configure = { args = vec!["-Dtests=disabled".to_string(), "-Ddocs=disabled".to_string(), "-Dbenchmark=disabled".to_string()] } },
);

make_package!(
    Pango,
    "pango",
    tarball_url = "https://download.gnome.org/sources/pango/1.56/pango-1.56.4.tar.xz",
    dependencies = [Glib2, Harfbuzz, Fribidi, Fontconfig, Freetype2, LibXft, Cairo],
    package_impl = {
        fn patch(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            patch_pango_source(ctx)
        }

        fn configure(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            configure_meson(
                self,
                ctx,
                Vec::new(),
                vec![
                    "-Dbuild-testsuite=false".to_string(),
                    "-Dbuild-examples=false".to_string(),
                    "-Dintrospection=disabled".to_string(),
                    "-Dgtk_doc=false".to_string(),
                    "-Dcairo=enabled".to_string(),
                ],
                Vec::new(),
            )
        }

        fn build(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            build_meson(self, ctx)
        }

        fn install(&self, ctx: &crate::types::Context) -> crate::types::Result<()> {
            install_meson(self, ctx)
        }
    }
);

pub struct OpenboxStackPackage;
make_meta_package!(
    "openbox-stack",
    OpenboxStackPackage,
    Glib2,
    LibXml2,
    Pango,
    Openbox
);
