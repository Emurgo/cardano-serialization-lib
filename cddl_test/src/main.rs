// We need to figure how out we handle integers as they can only be serialized as
// Unsigned or Negative. Do we do an enum for the int type?
// It's only used in transaction_metadata as one choice.
// How it has to be serialized (assuming number: i32):
// if number >= 0 {
//     serializer.write_unsigned_integer(number as u64)
// } else {
//     serializer.write_negative_integer(number as i64)
// }

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
        "uint" => "u64",
        // Not sure on this one, I think they can be bigger than i64 can fit
        // but the cbor_event serialization takes the argument as an i64
        "nint" => "i64",
        // TODO: define enum or something as otherwise it can overflow i64
        // and also we can't define the serialization traits for types
        // that are defined outside of this crate (includes primitives)
        //"int" => "i64",
        "tstr" | "text" => "String",
        // TODO: Is this right to have it be Vec<u8>?
        // the serialization library for bytes takes type [u8]
        // so we'll have to put some logic in there I guess?
        // it might be necessary to put a wrapper type..
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

// Separate function for when we support multiple choices as an enum
fn codegen_group(scope: &mut codegen::Scope, group: &Group, name: &str) {
    for group_choice in &group.0 {
        codegen_group_choice(scope, group_choice, name);

        // TODO: support multiple choices
        // this should be refactored into a common area for groups too.
        break;
    }
}

fn codegen_group_choice(scope: &mut codegen:: Scope, group_choice: &GroupChoice, name: &str) {
    // handles ValueMemberKey only
    // TODO: TypeGroupname / InlinedGroup are not supported yet
    // TODO: handle non-integer keys (all keys in shelley.cddl are uint)

    let mut s = scope.new_struct(name);
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
            let mut ser_impl = codegen::Impl::new(name);
            ser_impl.impl_trait("cbor_event::se::Serialize");
            let mut s_impl = codegen::Impl::new(name);
            let mut ser_func = ser_impl.new_fn("serialize")
                .generic("'se, W: Write")
                .ret("cbor_event::Result<&'se mut Serializer<W>>")
                .arg_ref_self()
                .arg("serializer", "&'se mut Serializer<W>");
            // can't use _has_comma to detect last element as you can have a trailing
            // comma on the last line in valid CDDL
            for (i, (group_entry, _has_comma)) in group_choice.0.iter().enumerate() {
                let field_name = group_entry_to_field_name(group_entry);
                s.field(
                    &field_name,
                    format!("Option<{}>", group_entry_to_type_name(group_entry))
                );
                // TODO: support conditional members (100% necessary for heterogenous maps (not tables))
                // TODO: proper support since this assumes all members implement the trait
                //       maybe we could put a special case for primitives or Maps/Vecs?
                // TODO: remove clone()? Without it String gets moved out.
                ser_func.line(
                    format!(
                        "self.{}.clone().unwrap().serialize(serializer){}",
                        field_name,
                        if i == group_choice.0.len() - 1 { "" } else { ";" } ));
            }
            scope.push_impl(ser_impl);
            // TODO: write code for serializing as map vs array
            //scope.push_impl(s_impl);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cddl_in = std::fs::read_to_string("supported.cddl").unwrap();
    let cddl = cddl::parser::cddl_from_str(&cddl_in)?;
    //println!("CDDL file: {}", cddl);
    let mut scope = codegen::Scope::new();
    // Can't generate groups of imports with codegen::Import so we just output this as raw text
    // since we don't need it to be dynamic so it's fine. codegen::Impl::new("a", "{z::b, z::c}")
    // does not work.
    scope.raw("use cbor_event::{self, de::Deserializer, se::Serializer};");
    scope.import("std::io", "Write");
    let mut group_module = codegen::Module::new("groups");
    let mut group_scope = group_module.scope();
    group_scope.import("super", "*");
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
                            scope.raw(format!("type {} = {};", tr.name, convert_types(&identifier.to_string())).as_ref());
                        },
                        // TODO: try to re-use/refactor this for arrays
                        Type2::Map(group) => {
                            codegen_group(group_scope, group, tr.name.to_string().as_ref());
                        },
                        x => {
                            println!("\nignored typename {} -> {:?}\n", tr.name, x);
                            // ignored
                        }
                    }
                    //println!("{} type2 = {:?}\n", tr.name, choice.type2);
                    //s.field("foo", "usize");
                    // remove and implement type choices
                    break;
                }
            },
            Rule::Group(group_rule) => {
                // Freely defined group - no need to generate anything outside of group module
                match &group_rule.entry {
                    GroupEntry::InlineGroup((_occur, inline_group)) => {
                        codegen_group(group_scope, inline_group, &group_rule.name.to_string());
                    },
                    x => panic!("Group rule with non-inline group? {:?}", x),
                }
            },
        }
    }
    scope.push_module(group_module);
    println!("{}", scope.to_string());

    Ok(())
}
