fn main() {
    #[cfg(windows)]
    {
        // Embed Windows manifest for UAC elevation
        embed_resource::compile("app.manifest", embed_resource::NONE);
    }
}
