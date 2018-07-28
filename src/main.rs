extern crate rusty_template;
use rusty_template::*;
use rusty_template::utils::s;
fn main() {
    let mut vhm = VarHashMap::new();
    vhm.insert(s("instance.level"), s("chickenking.rd.9999"));
    vhm.insert(s("context.department_code"), s("anim"));
    vhm.insert(s("context.lod_code"), s("hi"));
    vhm.insert(s("asset.name"), s("robot"));
    vhm.insert(s("instance.name"),s("1"));
    let p = parser::TemplateParser::default();

    let result = p.parse("/dd/shows/SHOW/{ instance.level  | dot_to_slash }/SHARED/{context.department_code?}", &vhm);
    println!("solved {}",result.unwrap());
}
