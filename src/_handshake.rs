use super::*;

macro_rules! packets {
    (
        $(
            $packet:ident {
                $(
                    $field:ident $typ:ident $(<$generics:ident>)?
                );* $(;)?
            } $(,)?
        )*
    ) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $packet {
                $(
                    pub $field: user_type!($typ $(<$generics>)?),
                )*
            }

            #[allow(unused_imports, unused_variables)]
            impl crate::Readable for $packet {
                fn read(buffer: &mut ::std::io::Cursor<&[u8]>, version: crate::ProtocolVersion) -> anyhow::Result<Self>
                where
                    Self: Sized
                {
                    use anyhow::Context as _;
                    $(
                        let $field = <$typ $(<$generics>)?>::read(buffer, version)
                            .context(concat!("failed to read field `", stringify!($field), "` of packet `", stringify!($packet), "`"))?
                            .into();
                    )*

                    Ok(Self {
                        $(
                            $field,
                        )*
                    })
                }
            }

            #[allow(unused_variables)]
            impl crate::Writeable for $packet {
                fn write(&self, buffer: &mut Vec<u8>, version: crate::ProtocolVersion) -> anyhow::Result<()> {
                    $(
                        user_type_convert_to_writeable!($typ $(<$generics>)?, &self.$field).write(buffer, version)?;
                    )*
                    Ok(())
                }
            }
        )*
    };
}

macro_rules! def_enum {
    (
        $ident:ident ($discriminant_type:ident) {
            $(
                $discriminant:literal = $variant:ident
                $(
                    {
                        $(
                            $field:ident $typ:ident $(<$generics:ident>)?
                        );* $(;)?
                    }
                )?
            ),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub enum $ident {
            $(
                $variant
                $(
                    {
                        $(
                            $field: user_type!($typ $(<$generics>)?),
                        )*
                    }
                )?,
            )*
        }

        impl crate::Readable for $ident {
            fn read(buffer: &mut ::std::io::Cursor<&[u8]>, version: crate::ProtocolVersion) -> anyhow::Result<Self>
                where
                    Self: Sized
            {
                use anyhow::Context as _;
                let discriminant = <$discriminant_type>::read(buffer, version)
                    .context(concat!("failed to read discriminant for enum type ", stringify!($ident)))?;

                match discriminant_to_literal!($discriminant_type, discriminant) {
                    $(
                        $discriminant => {
                            $(
                                $(
                                    let $field = <$typ $(<$generics>)?>::read(buffer, version)
                                        .context(concat!("failed to read field `", stringify!($field),
                                            "` of enum `", stringify!($ident), "::", stringify!($variant), "`"))?
                                            .into();
                                )*
                            )?

                            Ok($ident::$variant $(
                                {
                                    $(
                                        $field,
                                    )*
                                }
                            )?)
                        },
                    )*
                    _ => Err(anyhow::anyhow!(
                        concat!(
                            "no discriminant for enum `", stringify!($ident), "` matched value {:?}"
                        ), discriminant
                    ))
                }
            }
        }

        impl crate::Writeable for $ident {
            fn write(&self, buffer: &mut Vec<u8>, version: crate::ProtocolVersion) -> anyhow::Result<()> {
                match self {
                    $(
                        $ident::$variant $(
                            {
                                $($field,)*
                            }
                        )? => {
                            let discriminant = <$discriminant_type>::from($discriminant);
                            discriminant.write(buffer, version)?;

                            $(
                                $(
                                    user_type_convert_to_writeable!($typ $(<$generics>)?, $field).write(buffer, version)?;
                                )*
                            )?
                        }
                    )*
                }
                Ok(())
            }
        }
    };
}


def_enum! {
    HandshakeState (VarInt) {
        1 = Status,
        2 = Login,
    }
}

packets! {
    Handshake {
        protocol_version VarInt;
        server_address String;
        server_port u16;
        next_state HandshakeState;
    }
}
