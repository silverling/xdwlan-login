extern crate embed_resource;

fn main() {
    embed_resource::compile("app.rc", embed_resource::NONE);
}
