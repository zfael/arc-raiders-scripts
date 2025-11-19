fn main() {
    #[cfg(windows)]
    {
        // Embed the resource file that references the manifest
        embed_resource::compile("resources.rc", embed_resource::NONE);
    }
}
