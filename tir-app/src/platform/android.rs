//! Android-specific entry point and JNI bindings.

use jni::objects::{JObject, JValue};
use jni::refs::Reference as _;
use jni::{JavaVM, jni_sig, jni_str};
use winit::platform::android::activity::AndroidApp;

use crate::app::FontScale;

pub static ANDROID_APP: std::sync::OnceLock<AndroidApp> = std::sync::OnceLock::new();

/// Android entry point, invoked by NativeActivity via android-activity.
#[unsafe(no_mangle)]
pub fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );
    let font_scale = android_font_scale(&app);
    let _ = ANDROID_APP.set(app.clone());
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        android_app: Some(app),
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "Tessellations",
        options,
        Box::new(move |cc| create_app(cc, font_scale)),
    ) {
        log::error!("eframe failed: {e}");
    }
}

type AppResult = Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>>;

fn create_app(cc: &eframe::CreationContext<'_>, font_scale: f32) -> AppResult {
    let _ = cc;
    let font_scale = FontScale::from_raw(font_scale);
    log::info!(
        "font scale: {} (x{:.2})",
        font_scale.label(),
        font_scale.factor()
    );
    let app = crate::app::TirApp {
        font_scale,
        ..crate::app::TirApp::default()
    };
    Ok(Box::new(app))
}

/// `resources.configuration.fontScale` of the activity, via JNI.
fn android_font_scale(app: &AndroidApp) -> f32 {
    // SAFETY: android-activity guarantees a valid JavaVM pointer.
    let vm = unsafe { JavaVM::from_raw(app.vm_as_ptr().cast()) };
    let res = vm.attach_current_thread(|env| -> jni::errors::Result<f32> {
        // SAFETY: borrowed (not owned) activity reference; JObject has no Drop.
        let activity = unsafe { JObject::from_raw(env, app.activity_as_ptr().cast()) };
        let resources = env
            .call_method(
                &activity,
                jni_str!("getResources"),
                jni_sig!("()Landroid/content/res/Resources;"),
                &[],
            )?
            .l()?;
        let config = env
            .call_method(
                &resources,
                jni_str!("getConfiguration"),
                jni_sig!("()Landroid/content/res/Configuration;"),
                &[],
            )?
            .l()?;
        env.get_field(&config, jni_str!("fontScale"), jni_sig!("F"))?
            .f()
    });
    match res {
        Ok(s) if s > 0.0 => s,
        Ok(_) => 1.0,
        Err(e) => {
            log::warn!("could not read fontScale: {e}");
            1.0
        }
    }
}

/// System bar + display cutout insets of the activity window, in physical
/// pixels `[left, top, right, bottom]`. `None` until the view is attached.
pub fn android_safe_area(app: &AndroidApp) -> Option<[i32; 4]> {
    // SAFETY: android-activity guarantees a valid JavaVM pointer.
    let vm = unsafe { JavaVM::from_raw(app.vm_as_ptr().cast()) };
    let sdk = AndroidApp::sdk_version();
    let res = vm.attach_current_thread(|env| -> jni::errors::Result<Option<[i32; 4]>> {
        env.with_local_frame(16, |env| {
            // SAFETY: borrowed (not owned) activity reference; JObject has no Drop.
            let activity = unsafe { JObject::from_raw(env, app.activity_as_ptr().cast()) };
            let window = env
                .call_method(
                    &activity,
                    jni_str!("getWindow"),
                    jni_sig!("()Landroid/view/Window;"),
                    &[],
                )?
                .l()?;
            let decor = env
                .call_method(
                    &window,
                    jni_str!("getDecorView"),
                    jni_sig!("()Landroid/view/View;"),
                    &[],
                )?
                .l()?;
            let insets = env
                .call_method(
                    &decor,
                    jni_str!("getRootWindowInsets"),
                    jni_sig!("()Landroid/view/WindowInsets;"),
                    &[],
                )?
                .l()?;
            if insets.is_null() {
                return Ok(None); // not attached yet
            }

            if sdk >= 30 {
                // insets.getInsets(Type.systemBars() | Type.displayCutout())
                let mut mask = 0;
                for name in [jni_str!("systemBars"), jni_str!("displayCutout")] {
                    mask |= env
                        .call_static_method(
                            jni_str!("android/view/WindowInsets$Type"),
                            name,
                            jni_sig!("()I"),
                            &[],
                        )?
                        .i()?;
                }
                let i = env
                    .call_method(
                        &insets,
                        jni_str!("getInsets"),
                        jni_sig!("(I)Landroid/graphics/Insets;"),
                        &[JValue::Int(mask)],
                    )?
                    .l()?;
                let mut out = [0; 4];
                for (o, f) in out.iter_mut().zip([
                    jni_str!("left"),
                    jni_str!("top"),
                    jni_str!("right"),
                    jni_str!("bottom"),
                ]) {
                    *o = env.get_field(&i, f, jni_sig!("I"))?.i()?;
                }
                Ok(Some(out))
            } else {
                // API 26-29: deprecated per-side getters (system bars only).
                let mut out = [0; 4];
                for (o, m) in out.iter_mut().zip([
                    jni_str!("getSystemWindowInsetLeft"),
                    jni_str!("getSystemWindowInsetTop"),
                    jni_str!("getSystemWindowInsetRight"),
                    jni_str!("getSystemWindowInsetBottom"),
                ]) {
                    *o = env.call_method(&insets, m, jni_sig!("()I"), &[])?.i()?;
                }
                Ok(Some(out))
            }
        })
    });
    res.unwrap_or_else(|e| {
        log::warn!("could not query window insets: {e}");
        None
    })
}
