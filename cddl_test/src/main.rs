//use cddl;

use serde::Serialize;
use serde::Deserialize;
use serde_cbor::Serializer;
use serde_cbor::ser::SliceWrite;
use serde_cbor::tags::Tagged;

use std::io::Write;

use codegen::*;
use cddl::ast::*;

// #[derive(Deserialize, Serialize)]
// enum Foo {
//     Zero(i32),
//     One(Tagged<(i32, i32)>, i32),
//     Two
// }

// #[derive(Deserialize, Serialize)]
// struct Bar {
//     int: i32,
//     rational: Tagged<(i32, i32)>,
// }

fn group_entry_to_field_name(entry: &GroupEntry) -> String {
    match entry {
        GroupEntry::ValueMemberKey(vmk) => {
            match vmk.member_key.as_ref().unwrap() {
                MemberKey::Value(value) => format!("value_{}", value),
                MemberKey::Bareword(ident) => ident.to_string(),
                _ => "member_key_type1_not_implemented".to_string(),
            }
        },
        GroupEntry::TypeGroupname(tge) => tge.name.to_string(),
        GroupEntry::InlineGroup(_) => panic!("not implemented"),
    }
}

fn group_entry_to_type_name(entry: &GroupEntry) -> String {
    match entry {
        GroupEntry::ValueMemberKey(vmk) => vmk.entry_type.to_string(),
        GroupEntry::TypeGroupname(tge) => tge.name.to_string(),
        GroupEntry::InlineGroup(_) => panic!("not implemented"),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cddl_in = std::fs::read_to_string("supported.cddl").unwrap();
    let cddl = cddl::parser::cddl_from_str(&cddl_in)?;
    //println!("CDDL file: {}", cddl);
    let mut scope = codegen::Scope::new();
    for rule in cddl.rules {
        match rule {
            Rule::Type(tr) => {
                // (1) does not handle optioanl generic parameters
                // (2) does not handle ranges - I think they're the character position in the CDDL
                // (3) is_type_choice_alternate ignored since shelley cddl doesn't need it
                //     It's used, but used for no reason as it is the initial definition
                //     (which is also valid cddl), but it would be fine as = instead of /=
                //let mut s = scope.new_struct(tr.name.to_string().as_ref());
                // TODO: choices (as enums I guess?)
                for choice in &tr.value.0 {
                    // ignores control operators - only used in shelley spec to limit string length for application metadata
                    match &choice.type2 {
                        Type2::Typename((identifier, _generic_arg)) => {
                            // TODO: either replace tstr/ uint etc with str or String / usize, etc
                            // or include type aliases for those.
                            scope.raw(format!("type {} = {};", tr.name, identifier).as_ref());
                        },
                        // TODO: try to re-use/refactor this for arrays
                        Type2::Map(group) => {
                            for group_choice in &group.0 {
                                // handles ValueMemberKey only
                                // TODO: TypeGroupname / InlinedGroup are not supported yet
                                // TODO: handle non-integer keys (all keys in shelley.cddl are uint)
                                
                                let mut s = scope.new_struct(tr.name.to_string().as_ref());
                                // We could re-use this for arrays I guess and add a tag?
                                for (group_entry, _has_comma) in &group_choice.0 {
                                    s.field(
                                        &group_entry_to_field_name(group_entry),
                                        format!("Option<{}>", group_entry_to_type_name(group_entry))
                                    );
                                }

                                // TODO: support multiple choices
                                // this should be refactored into a common area for groups too.
                                break;
                            }
                        },
                        x => {
                            println!("\nignored typename {} -> {:?}\n", tr.name, x);
                            // ignored
                        }
                    }
                    //println!("{} type2 = {:?}\n", tr.name, choice.type2);
                    //s.field("foo", "usize");
                    // remove and implement choices
                    break;
                }
            },
            Rule::Group(gr) => {

            },
        }
    }
    println!("{}", scope.to_string());

    // let foo = Foo::One(Tagged::new(Some(20), (4, 2)), 9);
    // serde_cbor::to_writer(std::fs::File::create("foo.cbor")?, &foo)?;


    // let bar = Bar {
    //     int: 144,
    //     rational: Tagged::new(Some(20), (4, 2)),
    // };
    // //serde_cbor::to_writer(std::fs::File::create("bar.cbor")?, &bar)?;
    // let mut file_bar = std::fs::File::create("bar.cbor")?;
    // let bar_packed = serde_cbor::ser::to_vec_packed(&bar)?;
    // file_bar.write_all(&bar_packed)?;


    Ok(())
}
