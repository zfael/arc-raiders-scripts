fn main() {
    #[cfg(windows)]
    {
        // Create a .rc file that references the manifest
        let mut res = embed_resource::compile("resources.rc", embed_resource::NONE);
        res.manifest_optional();
    }
}
