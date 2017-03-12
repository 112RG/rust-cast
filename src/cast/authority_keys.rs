// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default,Debug)]
pub struct AuthorityKeys {
    // message fields
    keys: ::protobuf::RepeatedField<AuthorityKeys_Key>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthorityKeys {}

impl AuthorityKeys {
    pub fn new() -> AuthorityKeys {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthorityKeys {
        static mut instance: ::protobuf::lazy::Lazy<AuthorityKeys> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthorityKeys,
        };
        unsafe {
            instance.get(AuthorityKeys::new)
        }
    }

    // repeated .extensions.api.cast_channel.proto.AuthorityKeys.Key keys = 1;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<AuthorityKeys_Key>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<AuthorityKeys_Key> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<AuthorityKeys_Key> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[AuthorityKeys_Key] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<AuthorityKeys_Key> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AuthorityKeys_Key> {
        &mut self.keys
    }
}

impl ::protobuf::Message for AuthorityKeys {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.keys)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.keys {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.keys {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AuthorityKeys {
    fn new() -> AuthorityKeys {
        AuthorityKeys::new()
    }
}

impl ::protobuf::Clear for AuthorityKeys {
    fn clear(&mut self) {
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

#[derive(PartialEq,Clone,Default,Debug)]
pub struct AuthorityKeys_Key {
    // message fields
    fingerprint: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    public_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthorityKeys_Key {}

impl AuthorityKeys_Key {
    pub fn new() -> AuthorityKeys_Key {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthorityKeys_Key {
        static mut instance: ::protobuf::lazy::Lazy<AuthorityKeys_Key> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthorityKeys_Key,
        };
        unsafe {
            instance.get(AuthorityKeys_Key::new)
        }
    }

    // required bytes fingerprint = 1;

    pub fn clear_fingerprint(&mut self) {
        self.fingerprint.clear();
    }

    pub fn has_fingerprint(&self) -> bool {
        self.fingerprint.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fingerprint(&mut self, v: ::std::vec::Vec<u8>) {
        self.fingerprint = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fingerprint(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.fingerprint.is_none() {
            self.fingerprint.set_default();
        };
        self.fingerprint.as_mut().unwrap()
    }

    // Take field
    pub fn take_fingerprint(&mut self) -> ::std::vec::Vec<u8> {
        self.fingerprint.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_fingerprint(&self) -> &[u8] {
        match self.fingerprint.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_fingerprint_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.fingerprint
    }

    fn mut_fingerprint_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.fingerprint
    }

    // required bytes public_key = 2;

    pub fn clear_public_key(&mut self) {
        self.public_key.clear();
    }

    pub fn has_public_key(&self) -> bool {
        self.public_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_public_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.public_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.public_key.is_none() {
            self.public_key.set_default();
        };
        self.public_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_public_key(&mut self) -> ::std::vec::Vec<u8> {
        self.public_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_public_key(&self) -> &[u8] {
        match self.public_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_public_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.public_key
    }

    fn mut_public_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.public_key
    }
}

impl ::protobuf::Message for AuthorityKeys_Key {
    fn is_initialized(&self) -> bool {
        if self.fingerprint.is_none() {
            return false;
        };
        if self.public_key.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.fingerprint)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.public_key)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.fingerprint.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.public_key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fingerprint.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.public_key.as_ref() {
            os.write_bytes(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AuthorityKeys_Key {
    fn new() -> AuthorityKeys_Key {
        AuthorityKeys_Key::new()
    }
}

impl ::protobuf::Clear for AuthorityKeys_Key {
    fn clear(&mut self) {
        self.clear_fingerprint();
        self.clear_public_key();
        self.unknown_fields.clear();
    }
}
