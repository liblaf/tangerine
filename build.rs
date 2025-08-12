fn main() {
    minijinja_embed::embed_templates!("templates");
    shadow_rs::ShadowBuilder::builder()
        .deny_const(Default::default())
        .build()
        .unwrap();
}
