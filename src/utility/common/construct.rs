pub enum LateConstruct<T> {
    Placeholder,
    Constructed(T),
}
