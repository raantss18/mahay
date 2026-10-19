//! The entry points of Mahay HUB's background update job (`org.mahay.shell.UpdateJobService`).
//!
//! The job runs in the hub's process without an activity: `init` never ran, so there is no
//! `AndroidApp`. Everything it needs comes in as arguments, and the answer goes back as strings
//! for Java to install. The work itself is `mahay_update::managed::headless`, the same signed,
//! hash-checked download the screen uses.

use jni::objects::{JClass, JObjectArray, JString};
use jni::sys::{jboolean, jint};
use jni::JNIEnv;
use mahay_update::managed::{self, Network};

use crate::imp::{jstring, string_array};

fn empty<'a>(env: &mut JNIEnv<'a>) -> JObjectArray<'a> {
    string_array(env, &[]).unwrap_or_else(|_| JObjectArray::from(jni::objects::JObject::null()))
}

/// `String[] nativeApps()`: the sub-apps' ids, so Java doesn't keep a second list.
#[unsafe(no_mangle)]
pub extern "system" fn Java_org_mahay_shell_UpdateJobService_nativeApps<'a>(mut env: JNIEnv<'a>, _class: JClass<'a>) -> JObjectArray<'a> {
    string_array(&mut env, &managed::SUB_APPS).unwrap_or_else(|_| empty(&mut env))
}

/// `String[] nativeRun(String dataDir, String[] installed, String only, int network, boolean explicit)`.
#[unsafe(no_mangle)]
pub extern "system" fn Java_org_mahay_shell_UpdateJobService_nativeRun<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    data_dir: JString<'a>,
    installed: JObjectArray<'a>,
    only: JString<'a>,
    network: jint,
    explicit: jboolean,
) -> JObjectArray<'a> {
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info).with_tag("MahayHub"));
    let Some(dir) = jstring(&mut env, &data_dir) else { return empty(&mut env) };
    let count = env.get_array_length(&installed).unwrap_or(0);
    let mut lines = Vec::new();
    for i in 0..count {
        if let Ok(item) = env.get_object_array_element(&installed, i)
            && let Some(line) = jstring(&mut env, &JString::from(item))
        {
            lines.push(line);
        }
    }
    let only = jstring(&mut env, &only);
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let out = managed::headless(std::path::Path::new(&dir), &lines, only.as_deref(), Network::from_code(network), explicit != 0, now);
    let refs: Vec<&str> = out.iter().map(String::as_str).collect();
    string_array(&mut env, &refs).unwrap_or_else(|_| empty(&mut env))
}
