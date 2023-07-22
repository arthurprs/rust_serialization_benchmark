// @generated by the capnpc-rust plugin to the Cap'n Proto schema compiler.
// DO NOT EDIT.
// source: src/datasets/log/log.capnp

pub mod address {
    #[derive(Copy, Clone)]
    pub struct Owned(());
    impl ::capnp::traits::Owned for Owned {
        type Reader<'a> = Reader<'a>;
        type Builder<'a> = Builder<'a>;
    }
    impl ::capnp::traits::OwnedStruct for Owned {
        type Reader<'a> = Reader<'a>;
        type Builder<'a> = Builder<'a>;
    }
    impl ::capnp::traits::Pipelined for Owned {
        type Pipeline = Pipeline;
    }

    pub struct Reader<'a> {
        reader: ::capnp::private::layout::StructReader<'a>,
    }
    impl<'a> ::core::marker::Copy for Reader<'a> {}
    impl<'a> ::core::clone::Clone for Reader<'a> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<'a> ::capnp::traits::HasTypeId for Reader<'a> {
        const TYPE_ID: u64 = _private::TYPE_ID;
    }
    impl<'a> ::core::convert::From<::capnp::private::layout::StructReader<'a>> for Reader<'a> {
        fn from(reader: ::capnp::private::layout::StructReader<'a>) -> Self {
            Self { reader }
        }
    }

    impl<'a> ::capnp::traits::FromPointerReader<'a> for Reader<'a> {
        fn get_from_pointer(
            reader: &::capnp::private::layout::PointerReader<'a>,
            default: ::core::option::Option<&'a [::capnp::Word]>,
        ) -> ::capnp::Result<Self> {
            ::core::result::Result::Ok(reader.get_struct(default)?.into())
        }
    }

    impl<'a> ::capnp::traits::IntoInternalStructReader<'a> for Reader<'a> {
        fn into_internal_struct_reader(self) -> ::capnp::private::layout::StructReader<'a> {
            self.reader
        }
    }

    impl<'a> ::capnp::traits::Imbue<'a> for Reader<'a> {
        fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
            self.reader
                .imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
        }
    }

    impl<'a> Reader<'a> {
        pub fn reborrow(&self) -> Reader<'_> {
            Self { ..*self }
        }

        pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
            self.reader.total_size()
        }
        #[inline]
        pub fn get_x0(self) -> u8 {
            self.reader.get_data_field::<u8>(0)
        }
        #[inline]
        pub fn get_x1(self) -> u8 {
            self.reader.get_data_field::<u8>(1)
        }
        #[inline]
        pub fn get_x2(self) -> u8 {
            self.reader.get_data_field::<u8>(2)
        }
        #[inline]
        pub fn get_x3(self) -> u8 {
            self.reader.get_data_field::<u8>(3)
        }
    }

    pub struct Builder<'a> {
        builder: ::capnp::private::layout::StructBuilder<'a>,
    }
    impl<'a> ::capnp::traits::HasStructSize for Builder<'a> {
        const STRUCT_SIZE: ::capnp::private::layout::StructSize =
            ::capnp::private::layout::StructSize {
                data: 1,
                pointers: 0,
            };
    }
    impl<'a> ::capnp::traits::HasTypeId for Builder<'a> {
        const TYPE_ID: u64 = _private::TYPE_ID;
    }
    impl<'a> ::core::convert::From<::capnp::private::layout::StructBuilder<'a>> for Builder<'a> {
        fn from(builder: ::capnp::private::layout::StructBuilder<'a>) -> Self {
            Self { builder }
        }
    }

    impl<'a> ::capnp::traits::ImbueMut<'a> for Builder<'a> {
        fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
            self.builder
                .imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
        }
    }

    impl<'a> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a> {
        fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Self {
            builder
                .init_struct(<Self as ::capnp::traits::HasStructSize>::STRUCT_SIZE)
                .into()
        }
        fn get_from_pointer(
            builder: ::capnp::private::layout::PointerBuilder<'a>,
            default: ::core::option::Option<&'a [::capnp::Word]>,
        ) -> ::capnp::Result<Self> {
            ::core::result::Result::Ok(
                builder
                    .get_struct(
                        <Self as ::capnp::traits::HasStructSize>::STRUCT_SIZE,
                        default,
                    )?
                    .into(),
            )
        }
    }

    impl<'a> ::capnp::traits::SetPointerBuilder for Reader<'a> {
        fn set_pointer_builder(
            mut pointer: ::capnp::private::layout::PointerBuilder<'_>,
            value: Self,
            canonicalize: bool,
        ) -> ::capnp::Result<()> {
            pointer.set_struct(&value.reader, canonicalize)
        }
    }

    impl<'a> Builder<'a> {
        pub fn into_reader(self) -> Reader<'a> {
            self.builder.into_reader().into()
        }
        pub fn reborrow(&mut self) -> Builder<'_> {
            Builder {
                builder: self.builder.reborrow(),
            }
        }
        pub fn reborrow_as_reader(&self) -> Reader<'_> {
            self.builder.as_reader().into()
        }

        pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
            self.builder.as_reader().total_size()
        }
        #[inline]
        pub fn get_x0(self) -> u8 {
            self.builder.get_data_field::<u8>(0)
        }
        #[inline]
        pub fn set_x0(&mut self, value: u8) {
            self.builder.set_data_field::<u8>(0, value);
        }
        #[inline]
        pub fn get_x1(self) -> u8 {
            self.builder.get_data_field::<u8>(1)
        }
        #[inline]
        pub fn set_x1(&mut self, value: u8) {
            self.builder.set_data_field::<u8>(1, value);
        }
        #[inline]
        pub fn get_x2(self) -> u8 {
            self.builder.get_data_field::<u8>(2)
        }
        #[inline]
        pub fn set_x2(&mut self, value: u8) {
            self.builder.set_data_field::<u8>(2, value);
        }
        #[inline]
        pub fn get_x3(self) -> u8 {
            self.builder.get_data_field::<u8>(3)
        }
        #[inline]
        pub fn set_x3(&mut self, value: u8) {
            self.builder.set_data_field::<u8>(3, value);
        }
    }

    pub struct Pipeline {
        _typeless: ::capnp::any_pointer::Pipeline,
    }
    impl ::capnp::capability::FromTypelessPipeline for Pipeline {
        fn new(typeless: ::capnp::any_pointer::Pipeline) -> Self {
            Self {
                _typeless: typeless,
            }
        }
    }
    impl Pipeline {}
    mod _private {
        pub const TYPE_ID: u64 = 0x96d2_eb22_6a8b_d47a;
    }
}

pub mod log {
    #[derive(Copy, Clone)]
    pub struct Owned(());
    impl ::capnp::traits::Owned for Owned {
        type Reader<'a> = Reader<'a>;
        type Builder<'a> = Builder<'a>;
    }
    impl ::capnp::traits::OwnedStruct for Owned {
        type Reader<'a> = Reader<'a>;
        type Builder<'a> = Builder<'a>;
    }
    impl ::capnp::traits::Pipelined for Owned {
        type Pipeline = Pipeline;
    }

    pub struct Reader<'a> {
        reader: ::capnp::private::layout::StructReader<'a>,
    }
    impl<'a> ::core::marker::Copy for Reader<'a> {}
    impl<'a> ::core::clone::Clone for Reader<'a> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<'a> ::capnp::traits::HasTypeId for Reader<'a> {
        const TYPE_ID: u64 = _private::TYPE_ID;
    }
    impl<'a> ::core::convert::From<::capnp::private::layout::StructReader<'a>> for Reader<'a> {
        fn from(reader: ::capnp::private::layout::StructReader<'a>) -> Self {
            Self { reader }
        }
    }

    impl<'a> ::capnp::traits::FromPointerReader<'a> for Reader<'a> {
        fn get_from_pointer(
            reader: &::capnp::private::layout::PointerReader<'a>,
            default: ::core::option::Option<&'a [::capnp::Word]>,
        ) -> ::capnp::Result<Self> {
            ::core::result::Result::Ok(reader.get_struct(default)?.into())
        }
    }

    impl<'a> ::capnp::traits::IntoInternalStructReader<'a> for Reader<'a> {
        fn into_internal_struct_reader(self) -> ::capnp::private::layout::StructReader<'a> {
            self.reader
        }
    }

    impl<'a> ::capnp::traits::Imbue<'a> for Reader<'a> {
        fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
            self.reader
                .imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
        }
    }

    impl<'a> Reader<'a> {
        pub fn reborrow(&self) -> Reader<'_> {
            Self { ..*self }
        }

        pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
            self.reader.total_size()
        }
        #[inline]
        pub fn get_address(
            self,
        ) -> ::capnp::Result<crate::datasets::log::log_capnp::address::Reader<'a>> {
            ::capnp::traits::FromPointerReader::get_from_pointer(
                &self.reader.get_pointer_field(0),
                ::core::option::Option::None,
            )
        }
        #[inline]
        pub fn has_address(&self) -> bool {
            !self.reader.get_pointer_field(0).is_null()
        }
        #[inline]
        pub fn get_identity(self) -> ::capnp::Result<::capnp::text::Reader<'a>> {
            ::capnp::traits::FromPointerReader::get_from_pointer(
                &self.reader.get_pointer_field(1),
                ::core::option::Option::None,
            )
        }
        #[inline]
        pub fn has_identity(&self) -> bool {
            !self.reader.get_pointer_field(1).is_null()
        }
        #[inline]
        pub fn get_userid(self) -> ::capnp::Result<::capnp::text::Reader<'a>> {
            ::capnp::traits::FromPointerReader::get_from_pointer(
                &self.reader.get_pointer_field(2),
                ::core::option::Option::None,
            )
        }
        #[inline]
        pub fn has_userid(&self) -> bool {
            !self.reader.get_pointer_field(2).is_null()
        }
        #[inline]
        pub fn get_date(self) -> ::capnp::Result<::capnp::text::Reader<'a>> {
            ::capnp::traits::FromPointerReader::get_from_pointer(
                &self.reader.get_pointer_field(3),
                ::core::option::Option::None,
            )
        }
        #[inline]
        pub fn has_date(&self) -> bool {
            !self.reader.get_pointer_field(3).is_null()
        }
        #[inline]
        pub fn get_request(self) -> ::capnp::Result<::capnp::text::Reader<'a>> {
            ::capnp::traits::FromPointerReader::get_from_pointer(
                &self.reader.get_pointer_field(4),
                ::core::option::Option::None,
            )
        }
        #[inline]
        pub fn has_request(&self) -> bool {
            !self.reader.get_pointer_field(4).is_null()
        }
        #[inline]
        pub fn get_code(self) -> u16 {
            self.reader.get_data_field::<u16>(0)
        }
        #[inline]
        pub fn get_size(self) -> u64 {
            self.reader.get_data_field::<u64>(1)
        }
    }

    pub struct Builder<'a> {
        builder: ::capnp::private::layout::StructBuilder<'a>,
    }
    impl<'a> ::capnp::traits::HasStructSize for Builder<'a> {
        const STRUCT_SIZE: ::capnp::private::layout::StructSize =
            ::capnp::private::layout::StructSize {
                data: 2,
                pointers: 5,
            };
    }
    impl<'a> ::capnp::traits::HasTypeId for Builder<'a> {
        const TYPE_ID: u64 = _private::TYPE_ID;
    }
    impl<'a> ::core::convert::From<::capnp::private::layout::StructBuilder<'a>> for Builder<'a> {
        fn from(builder: ::capnp::private::layout::StructBuilder<'a>) -> Self {
            Self { builder }
        }
    }

    impl<'a> ::capnp::traits::ImbueMut<'a> for Builder<'a> {
        fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
            self.builder
                .imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
        }
    }

    impl<'a> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a> {
        fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Self {
            builder
                .init_struct(<Self as ::capnp::traits::HasStructSize>::STRUCT_SIZE)
                .into()
        }
        fn get_from_pointer(
            builder: ::capnp::private::layout::PointerBuilder<'a>,
            default: ::core::option::Option<&'a [::capnp::Word]>,
        ) -> ::capnp::Result<Self> {
            ::core::result::Result::Ok(
                builder
                    .get_struct(
                        <Self as ::capnp::traits::HasStructSize>::STRUCT_SIZE,
                        default,
                    )?
                    .into(),
            )
        }
    }

    impl<'a> ::capnp::traits::SetPointerBuilder for Reader<'a> {
        fn set_pointer_builder(
            mut pointer: ::capnp::private::layout::PointerBuilder<'_>,
            value: Self,
            canonicalize: bool,
        ) -> ::capnp::Result<()> {
            pointer.set_struct(&value.reader, canonicalize)
        }
    }

    impl<'a> Builder<'a> {
        pub fn into_reader(self) -> Reader<'a> {
            self.builder.into_reader().into()
        }
        pub fn reborrow(&mut self) -> Builder<'_> {
            Builder {
                builder: self.builder.reborrow(),
            }
        }
        pub fn reborrow_as_reader(&self) -> Reader<'_> {
            self.builder.as_reader().into()
        }

        pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
            self.builder.as_reader().total_size()
        }
        #[inline]
        pub fn get_address(
            self,
        ) -> ::capnp::Result<crate::datasets::log::log_capnp::address::Builder<'a>> {
            ::capnp::traits::FromPointerBuilder::get_from_pointer(
                self.builder.get_pointer_field(0),
                ::core::option::Option::None,
            )
        }
        #[inline]
        pub fn set_address(
            &mut self,
            value: crate::datasets::log::log_capnp::address::Reader<'_>,
        ) -> ::capnp::Result<()> {
            ::capnp::traits::SetPointerBuilder::set_pointer_builder(
                self.builder.reborrow().get_pointer_field(0),
                value,
                false,
            )
        }
        #[inline]
        pub fn init_address(self) -> crate::datasets::log::log_capnp::address::Builder<'a> {
            ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(0), 0)
        }
        #[inline]
        pub fn has_address(&self) -> bool {
            !self.builder.is_pointer_field_null(0)
        }
        #[inline]
        pub fn get_identity(self) -> ::capnp::Result<::capnp::text::Builder<'a>> {
            ::capnp::traits::FromPointerBuilder::get_from_pointer(
                self.builder.get_pointer_field(1),
                ::core::option::Option::None,
            )
        }
        #[inline]
        pub fn set_identity(&mut self, value: ::capnp::text::Reader<'_>) {
            self.builder.reborrow().get_pointer_field(1).set_text(value);
        }
        #[inline]
        pub fn init_identity(self, size: u32) -> ::capnp::text::Builder<'a> {
            self.builder.get_pointer_field(1).init_text(size)
        }
        #[inline]
        pub fn has_identity(&self) -> bool {
            !self.builder.is_pointer_field_null(1)
        }
        #[inline]
        pub fn get_userid(self) -> ::capnp::Result<::capnp::text::Builder<'a>> {
            ::capnp::traits::FromPointerBuilder::get_from_pointer(
                self.builder.get_pointer_field(2),
                ::core::option::Option::None,
            )
        }
        #[inline]
        pub fn set_userid(&mut self, value: ::capnp::text::Reader<'_>) {
            self.builder.reborrow().get_pointer_field(2).set_text(value);
        }
        #[inline]
        pub fn init_userid(self, size: u32) -> ::capnp::text::Builder<'a> {
            self.builder.get_pointer_field(2).init_text(size)
        }
        #[inline]
        pub fn has_userid(&self) -> bool {
            !self.builder.is_pointer_field_null(2)
        }
        #[inline]
        pub fn get_date(self) -> ::capnp::Result<::capnp::text::Builder<'a>> {
            ::capnp::traits::FromPointerBuilder::get_from_pointer(
                self.builder.get_pointer_field(3),
                ::core::option::Option::None,
            )
        }
        #[inline]
        pub fn set_date(&mut self, value: ::capnp::text::Reader<'_>) {
            self.builder.reborrow().get_pointer_field(3).set_text(value);
        }
        #[inline]
        pub fn init_date(self, size: u32) -> ::capnp::text::Builder<'a> {
            self.builder.get_pointer_field(3).init_text(size)
        }
        #[inline]
        pub fn has_date(&self) -> bool {
            !self.builder.is_pointer_field_null(3)
        }
        #[inline]
        pub fn get_request(self) -> ::capnp::Result<::capnp::text::Builder<'a>> {
            ::capnp::traits::FromPointerBuilder::get_from_pointer(
                self.builder.get_pointer_field(4),
                ::core::option::Option::None,
            )
        }
        #[inline]
        pub fn set_request(&mut self, value: ::capnp::text::Reader<'_>) {
            self.builder.reborrow().get_pointer_field(4).set_text(value);
        }
        #[inline]
        pub fn init_request(self, size: u32) -> ::capnp::text::Builder<'a> {
            self.builder.get_pointer_field(4).init_text(size)
        }
        #[inline]
        pub fn has_request(&self) -> bool {
            !self.builder.is_pointer_field_null(4)
        }
        #[inline]
        pub fn get_code(self) -> u16 {
            self.builder.get_data_field::<u16>(0)
        }
        #[inline]
        pub fn set_code(&mut self, value: u16) {
            self.builder.set_data_field::<u16>(0, value);
        }
        #[inline]
        pub fn get_size(self) -> u64 {
            self.builder.get_data_field::<u64>(1)
        }
        #[inline]
        pub fn set_size(&mut self, value: u64) {
            self.builder.set_data_field::<u64>(1, value);
        }
    }

    pub struct Pipeline {
        _typeless: ::capnp::any_pointer::Pipeline,
    }
    impl ::capnp::capability::FromTypelessPipeline for Pipeline {
        fn new(typeless: ::capnp::any_pointer::Pipeline) -> Self {
            Self {
                _typeless: typeless,
            }
        }
    }
    impl Pipeline {
        pub fn get_address(&self) -> crate::datasets::log::log_capnp::address::Pipeline {
            ::capnp::capability::FromTypelessPipeline::new(self._typeless.get_pointer_field(0))
        }
    }
    mod _private {
        pub const TYPE_ID: u64 = 0x87f6_c7ab_2de3_81ff;
    }
}

pub mod logs {
    #[derive(Copy, Clone)]
    pub struct Owned(());
    impl ::capnp::traits::Owned for Owned {
        type Reader<'a> = Reader<'a>;
        type Builder<'a> = Builder<'a>;
    }
    impl ::capnp::traits::OwnedStruct for Owned {
        type Reader<'a> = Reader<'a>;
        type Builder<'a> = Builder<'a>;
    }
    impl ::capnp::traits::Pipelined for Owned {
        type Pipeline = Pipeline;
    }

    pub struct Reader<'a> {
        reader: ::capnp::private::layout::StructReader<'a>,
    }
    impl<'a> ::core::marker::Copy for Reader<'a> {}
    impl<'a> ::core::clone::Clone for Reader<'a> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<'a> ::capnp::traits::HasTypeId for Reader<'a> {
        const TYPE_ID: u64 = _private::TYPE_ID;
    }
    impl<'a> ::core::convert::From<::capnp::private::layout::StructReader<'a>> for Reader<'a> {
        fn from(reader: ::capnp::private::layout::StructReader<'a>) -> Self {
            Self { reader }
        }
    }

    impl<'a> ::capnp::traits::FromPointerReader<'a> for Reader<'a> {
        fn get_from_pointer(
            reader: &::capnp::private::layout::PointerReader<'a>,
            default: ::core::option::Option<&'a [::capnp::Word]>,
        ) -> ::capnp::Result<Self> {
            ::core::result::Result::Ok(reader.get_struct(default)?.into())
        }
    }

    impl<'a> ::capnp::traits::IntoInternalStructReader<'a> for Reader<'a> {
        fn into_internal_struct_reader(self) -> ::capnp::private::layout::StructReader<'a> {
            self.reader
        }
    }

    impl<'a> ::capnp::traits::Imbue<'a> for Reader<'a> {
        fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
            self.reader
                .imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
        }
    }

    impl<'a> Reader<'a> {
        pub fn reborrow(&self) -> Reader<'_> {
            Self { ..*self }
        }

        pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
            self.reader.total_size()
        }
        #[inline]
        pub fn get_logs(
            self,
        ) -> ::capnp::Result<
            ::capnp::struct_list::Reader<'a, crate::datasets::log::log_capnp::log::Owned>,
        > {
            ::capnp::traits::FromPointerReader::get_from_pointer(
                &self.reader.get_pointer_field(0),
                ::core::option::Option::None,
            )
        }
        #[inline]
        pub fn has_logs(&self) -> bool {
            !self.reader.get_pointer_field(0).is_null()
        }
    }

    pub struct Builder<'a> {
        builder: ::capnp::private::layout::StructBuilder<'a>,
    }
    impl<'a> ::capnp::traits::HasStructSize for Builder<'a> {
        const STRUCT_SIZE: ::capnp::private::layout::StructSize =
            ::capnp::private::layout::StructSize {
                data: 0,
                pointers: 1,
            };
    }
    impl<'a> ::capnp::traits::HasTypeId for Builder<'a> {
        const TYPE_ID: u64 = _private::TYPE_ID;
    }
    impl<'a> ::core::convert::From<::capnp::private::layout::StructBuilder<'a>> for Builder<'a> {
        fn from(builder: ::capnp::private::layout::StructBuilder<'a>) -> Self {
            Self { builder }
        }
    }

    impl<'a> ::capnp::traits::ImbueMut<'a> for Builder<'a> {
        fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
            self.builder
                .imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
        }
    }

    impl<'a> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a> {
        fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Self {
            builder
                .init_struct(<Self as ::capnp::traits::HasStructSize>::STRUCT_SIZE)
                .into()
        }
        fn get_from_pointer(
            builder: ::capnp::private::layout::PointerBuilder<'a>,
            default: ::core::option::Option<&'a [::capnp::Word]>,
        ) -> ::capnp::Result<Self> {
            ::core::result::Result::Ok(
                builder
                    .get_struct(
                        <Self as ::capnp::traits::HasStructSize>::STRUCT_SIZE,
                        default,
                    )?
                    .into(),
            )
        }
    }

    impl<'a> ::capnp::traits::SetPointerBuilder for Reader<'a> {
        fn set_pointer_builder(
            mut pointer: ::capnp::private::layout::PointerBuilder<'_>,
            value: Self,
            canonicalize: bool,
        ) -> ::capnp::Result<()> {
            pointer.set_struct(&value.reader, canonicalize)
        }
    }

    impl<'a> Builder<'a> {
        pub fn into_reader(self) -> Reader<'a> {
            self.builder.into_reader().into()
        }
        pub fn reborrow(&mut self) -> Builder<'_> {
            Builder {
                builder: self.builder.reborrow(),
            }
        }
        pub fn reborrow_as_reader(&self) -> Reader<'_> {
            self.builder.as_reader().into()
        }

        pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
            self.builder.as_reader().total_size()
        }
        #[inline]
        pub fn get_logs(
            self,
        ) -> ::capnp::Result<
            ::capnp::struct_list::Builder<'a, crate::datasets::log::log_capnp::log::Owned>,
        > {
            ::capnp::traits::FromPointerBuilder::get_from_pointer(
                self.builder.get_pointer_field(0),
                ::core::option::Option::None,
            )
        }
        #[inline]
        pub fn set_logs(
            &mut self,
            value: ::capnp::struct_list::Reader<'a, crate::datasets::log::log_capnp::log::Owned>,
        ) -> ::capnp::Result<()> {
            ::capnp::traits::SetPointerBuilder::set_pointer_builder(
                self.builder.reborrow().get_pointer_field(0),
                value,
                false,
            )
        }
        #[inline]
        pub fn init_logs(
            self,
            size: u32,
        ) -> ::capnp::struct_list::Builder<'a, crate::datasets::log::log_capnp::log::Owned>
        {
            ::capnp::traits::FromPointerBuilder::init_pointer(
                self.builder.get_pointer_field(0),
                size,
            )
        }
        #[inline]
        pub fn has_logs(&self) -> bool {
            !self.builder.is_pointer_field_null(0)
        }
    }

    pub struct Pipeline {
        _typeless: ::capnp::any_pointer::Pipeline,
    }
    impl ::capnp::capability::FromTypelessPipeline for Pipeline {
        fn new(typeless: ::capnp::any_pointer::Pipeline) -> Self {
            Self {
                _typeless: typeless,
            }
        }
    }
    impl Pipeline {}
    mod _private {
        pub const TYPE_ID: u64 = 0x9658_6a57_8e4a_5409;
    }
}
