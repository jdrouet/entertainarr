static ASSETS: include_dir::Dir<'_> = include_dir::include_dir!("$CARGO_MANIFEST_DIR/assets");

#[derive(Clone, Debug, Default)]
pub struct ClientService;

impl entertainarr_adapter_http::server::ClientService for ClientService {
    fn get_file<P: AsRef<std::path::Path>>(&self, path: P) -> Option<&[u8]> {
        ASSETS.get_file(path).map(|file| file.contents())
    }
}
