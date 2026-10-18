//! Give memory the system allocator keeps after frees back to the system
//! ([`hazavana_engine::memory::set_release_hook`]). The platform call lives in
//! `hazavana-sysmem`, the workspace's only crate allowed `unsafe`.

/// Install the hook.
pub fn install() {
    hazavana_engine::memory::set_release_hook(release);
}

fn release() {
    let _ = hazavana_sysmem::release_free_memory();
}
