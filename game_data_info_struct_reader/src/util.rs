pub(crate) fn strip_name(name: &str) -> String {
    let mut ret = name.to_string();
    let rm = vec!["SPEC_", "SPEC", "_system", "_planet", "NAME"];
    for r in rm {
        ret = ret.replace(r, "");
    }
    ret = ret.replace("_", " ");
    ret
}
