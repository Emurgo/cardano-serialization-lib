// use serde::Serialize;
// use serde::Deserialize;
// use serde_cbor::Serializer;
// use serde_cbor::ser::SliceWrite;
// use serde_cbor::tags::Tagged;

use std::io::Write;

use cddl::ast::*;

fn group_entry_to_field_name(entry: &GroupEntry) -> String {
    match entry {
        GroupEntry::ValueMemberKey(vmk) => {
            match vmk.member_key.as_ref().unwrap() {
                MemberKey::Value(value) => format!("value_{}", value),
                MemberKey::Bareword(ident) => ident.to_string(),
                MemberKey::Type1(_) => panic!("Encountered Type1 member key in multi-field map - not supported"),
            }
        },
        GroupEntry::TypeGroupname(tge) => tge.name.to_string(),
        GroupEntry::InlineGroup(_) => panic!("not implemented (define a new struct for this!)"),
    }
}

// TODO: Can we do this, or do we need to be more explicit to match the schema?
fn convert_types(raw: &str) -> &str {
    match raw {
        "uint" => "u32",
        "nint" => "i32",
        "int" => "i32",
        "tstr" | "text" => "String",
        // TODO: Is this right to have it be Vec<u8>?
        "bstr" | "bytes" => "Vec<u8>",
        // What about bingint/other stuff in the standard prelude?
        x => x,
    }
}

fn rust_type_from_type2(type2: &Type2) -> String {
    match type2 {
        // ignoring IntValue/FloatValue/other primitives since they're not in the shelley spec
        // ie Type2::UintValue(value) => format!("uint<{}>", value),
        // generic args not in shelley.cddl
        // TODO: socket plugs (used in hash type)
        Type2::Typename((ident, _generic_arg)) => convert_types(&(ident.0).0).to_owned(),
        // Map(group) not implemented as it's not in shelley.cddl
        Type2::Array(group) => {
            let mut s = String::new();
            for choice in &group.0 {
                // special case for homogenous arrays
                if let Some((entry, _has_comma)) = choice.0.first() {
                    let element_type = match entry {
                        GroupEntry::ValueMemberKey(vmk) => rust_type(&vmk.entry_type),
                        GroupEntry::TypeGroupname(tgn) => tgn.name.to_string(),
                        _ => format!("UNSUPPORTED_ARRAY_ELEMENT<{:?}>", entry),
                    };
                    s.push_str(&format!("Vec<{}>", element_type));
                } else {
                    // TODO: how do we handle this? tuples?
                    // or creating a struct definition and referring to it
                    // by name?
                }
                // TODO: handle group choices (enums?)
                break;
            }
            s
        },
        x => format!("unsupported<{:?}>", x),
    }
}

fn rust_type(t: &Type) -> String {
    for type1 in t.0.iter() {
        // ignoring range control operator here, only interested in Type2
        return rust_type_from_type2(&type1.type2);

        // TODO: how to handle type choices? define an enum for every option?
        //       deserializing would be more complicated since you'd
        //       have to test them until one matches.
    }
    panic!("rust_type() is broken for: '{}'", t)
}

fn group_entry_to_type_name(entry: &GroupEntry) -> String {
    match entry {
        GroupEntry::ValueMemberKey(vmk) => rust_type(&vmk.entry_type),//convert_types(&vmk.entry_type.to_string()).to_owned(),
        GroupEntry::TypeGroupname(tge) => "TGN".to_owned() + &tge.name.to_string(),
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

                                // Here we test if this is a struct vs a table.
                                // struct: { x: int, y: int }, etc
                                // table: { * int => tstr }, etc
                                // this assumes that all maps representing tables are homogenous
                                // and contain no other fields. I am not sure if this is a guarantee in
                                // cbor but I would hope that the cddl specs we are using follow this.

                                // Is there a more concise/readable way of expressing this in rust?
                                let table_types: Option<(&Type2, &Type)> = if group_choice.0.len() == 1 {
                                    if let Some((GroupEntry::ValueMemberKey(vmk), _)) = group_choice.0.first() {
                                        match &vmk.member_key {
                                            // TODO: Do we need to handle cuts for what we're doing?
                                            // Does the range control operator matter?
                                            Some(MemberKey::Type1(type1)) => Some((&type1.0.type2, &vmk.entry_type)),
                                            _ => None
                                        }
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                };
                                match table_types {
                                    Some((domain, range)) => {
                                        s.field("table", format!("std::collections::BTreeMap<{}, {}>", rust_type_from_type2(domain), rust_type(range)));
                                    },
                                    None => {
                                        for (group_entry, _has_comma) in &group_choice.0 {
                                            s.field(
                                                &group_entry_to_field_name(group_entry),
                                                format!("Option<{}>", group_entry_to_type_name(group_entry))
                                            );
                                        }
                                    }
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

    Ok(())
}
