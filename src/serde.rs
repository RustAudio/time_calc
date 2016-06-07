extern crate serde;

mod bars {
    use bars::Bars;
    use super::serde;

    impl serde::Serialize for Bars {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer,
        {
            serializer.serialize_newtype_struct("Bars", self.bars())
        }
    }

    impl serde::Deserialize for Bars {
        fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
            where D: serde::Deserializer,
        {
            struct Visitor;

            impl serde::de::Visitor for Visitor {
                type Value = Bars;

                fn visit_i64<E>(&mut self, v: i64) -> Result<Self::Value, E>
                    where E: serde::de::Error,
                {
                    Ok(Bars(v))
                }

                fn visit_newtype_struct<D>(&mut self, deserializer: &mut D) -> Result<Self::Value, D::Error>
                    where D: serde::Deserializer,
                {
                    Ok(Bars(try!(super::serde::de::Deserialize::deserialize(deserializer))))
                }
            }

            deserializer.deserialize_newtype_struct("Bars", Visitor)
        }
    }

    #[test]
    fn test() {
        extern crate serde_json;

        let bars = Bars(4);
        let serialized = serde_json::to_string(&bars).unwrap();

        println!("{}", serialized);
        assert_eq!("4", &serialized);
        
        let deserialized: Bars = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(bars, deserialized);
    }
}

mod beats {
    use beats::Beats;
    use super::serde;

    impl serde::Serialize for Beats {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer,
        {
            serializer.serialize_newtype_struct("Beats", self.beats())
        }
    }

    impl serde::Deserialize for Beats {
        fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
            where D: serde::Deserializer,
        {
            struct Visitor;

            impl serde::de::Visitor for Visitor {
                type Value = Beats;

                fn visit_i64<E>(&mut self, v: i64) -> Result<Self::Value, E>
                    where E: serde::de::Error,
                {
                    Ok(Beats(v))
                }

                fn visit_newtype_struct<D>(&mut self, deserializer: &mut D) -> Result<Self::Value, D::Error>
                    where D: serde::Deserializer,
                {
                    Ok(Beats(try!(super::serde::de::Deserialize::deserialize(deserializer))))
                }
            }

            deserializer.deserialize_newtype_struct("Beats", Visitor)
        }
    }

    #[test]
    fn test() {
        extern crate serde_json;

        let beats = Beats(4);
        let serialized = serde_json::to_string(&beats).unwrap();

        println!("{}", serialized);
        assert_eq!("4", &serialized);
        
        let deserialized: Beats = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(beats, deserialized);
    }
}

mod division {
    use division::Division;
    use super::serde;

    impl serde::Serialize for Division {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer,
        {
            use Division::*;
            match *self {
                Bar                     => serializer.serialize_unit_variant("Division", 0, "Bar"),
                Minim                   => serializer.serialize_unit_variant("Division", 1, "Minim"),
                Beat                    => serializer.serialize_unit_variant("Division", 2, "Beat"),
                Quaver                  => serializer.serialize_unit_variant("Division", 3, "Quaver"),
                SemiQuaver              => serializer.serialize_unit_variant("Division", 4, "SemiQuaver"),
                ThirtySecond            => serializer.serialize_unit_variant("Division", 5, "ThirtySecond"),
                SixtyFourth             => serializer.serialize_unit_variant("Division", 6, "SixtyFourth"),
                OneHundredTwentyEighth  => serializer.serialize_unit_variant("Division", 7, "OneHundredTwentyEighth"),
                TwoHundredFiftySixth    => serializer.serialize_unit_variant("Division", 8, "TwoHundredFiftySixth"),
                FiveHundredTwelfth      => serializer.serialize_unit_variant("Division", 9, "FiveHundredTwelfth"),
                OneThousandTwentyFourth => serializer.serialize_unit_variant("Division", 10, "OneThousandTwentyFourth"),
            }
        }
    }

    impl serde::Deserialize for Division {
        fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
            where D: serde::Deserializer,
        {
            enum Field {
                Bar,
                Minim,
                Beat,
                Quaver,
                SemiQuaver,
                ThirtySecond,
                SixtyFourth,
                OneHundredTwentyEighth,
                TwoHundredFiftySixth,
                FiveHundredTwelfth,
                OneThousandTwentyFourth,
            }

            impl serde::de::Deserialize for Field {
                fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
                    where D: serde::Deserializer,
                {
                    struct FieldVisitor;

                    impl serde::de::Visitor for FieldVisitor {
                        type Value = Field;

                        fn visit_usize<E>(&mut self, value: usize) -> Result<Field, E>
                            where E: serde::de::Error,
                        {
                            let div = match value {
                                0  => Field::Bar,
                                1  => Field::Minim,
                                2  => Field::Beat,
                                3  => Field::Quaver,
                                4  => Field::SemiQuaver,
                                5  => Field::ThirtySecond,
                                6  => Field::SixtyFourth,
                                7  => Field::OneHundredTwentyEighth,
                                8  => Field::TwoHundredFiftySixth,
                                9  => Field::FiveHundredTwelfth,
                                10 => Field::OneThousandTwentyFourth,
                                _ => return Err(serde::de::Error::unknown_field(&value.to_string())),
                            };
                            Ok(div)
                        }

                        fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                            where E: serde::de::Error,
                        {
                            match value {
                                "Bar"                     => Ok(Field::Bar),
                                "Minim"                   => Ok(Field::Minim),
                                "Beat"                    => Ok(Field::Beat),
                                "Quaver"                  => Ok(Field::Quaver),
                                "SemiQuaver"              => Ok(Field::SemiQuaver),
                                "ThirtySecond"            => Ok(Field::ThirtySecond),
                                "SixtyFourth"             => Ok(Field::SixtyFourth),
                                "OneHundredTwentyEighth"  => Ok(Field::OneHundredTwentyEighth),
                                "TwoHundredFiftySixth"    => Ok(Field::TwoHundredFiftySixth),
                                "FiveHundredTwelfth"      => Ok(Field::FiveHundredTwelfth),
                                "OneThousandTwentyFourth" => Ok(Field::OneThousandTwentyFourth),
                                _ => Err(serde::de::Error::unknown_field(value)),
                            }
                        }
                    }

                    deserializer.deserialize(FieldVisitor)
                }
            }

            struct Visitor;

            impl serde::de::EnumVisitor for Visitor {
                type Value = Division;

                fn visit<V>(&mut self, mut visitor: V) -> Result<Division, V::Error>
                    where V: serde::de::VariantVisitor,
                {
                    let div = match try!(visitor.visit_variant()) {
                        Field::Bar                     => Division::Bar,
                        Field::Minim                   => Division::Minim,
                        Field::Beat                    => Division::Beat,
                        Field::Quaver                  => Division::Quaver,
                        Field::SemiQuaver              => Division::SemiQuaver,
                        Field::ThirtySecond            => Division::ThirtySecond,
                        Field::SixtyFourth             => Division::SixtyFourth,
                        Field::OneHundredTwentyEighth  => Division::OneHundredTwentyEighth,
                        Field::TwoHundredFiftySixth    => Division::TwoHundredFiftySixth,
                        Field::FiveHundredTwelfth      => Division::FiveHundredTwelfth,
                        Field::OneThousandTwentyFourth => Division::OneThousandTwentyFourth,
                    };
                    try!(visitor.visit_unit());
                    Ok(div)
                }
            }

            const VARIANTS: &'static [&'static str] = &[
                "Bar",
                "Minim",
                "Beat",
                "Quaver",
                "SemiQuaver",
                "ThirtySecond",
                "SixtyFourth",
                "OneHundredTwentyEighth",
                "TwoHundredFiftySixth",
                "FiveHundredTwelfth",
                "OneThousandTwentyFourth",
            ];

            deserializer.deserialize_enum("Division", VARIANTS, Visitor)
        }
    }

    #[test]
    fn test() {
        extern crate serde_json;

        let div = Division::SemiQuaver;
        let serialized = serde_json::to_string(&div).unwrap();

        println!("{}", serialized);
        assert_eq!("{\"SemiQuaver\":[]}", &serialized);
        
        let deserialized: Division = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(div, deserialized);
    }
}

mod div_type {
    use division::DivType;
    use super::serde;

    impl serde::Serialize for DivType {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer,
        {
            match *self {
                DivType::Whole     => serializer.serialize_unit_variant("DivType", 0, "Whole"),
                DivType::TwoThirds => serializer.serialize_unit_variant("DivType", 1, "TwoThirds"),
            }
        }
    }

    impl serde::Deserialize for DivType {
        fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
            where D: serde::Deserializer,
        {
            enum Field {
                Whole,
                TwoThirds,
            }

            impl serde::de::Deserialize for Field {
                fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
                    where D: serde::Deserializer,
                {
                    struct FieldVisitor;

                    impl serde::de::Visitor for FieldVisitor {
                        type Value = Field;

                        fn visit_usize<E>(&mut self, value: usize) -> Result<Field, E>
                            where E: serde::de::Error,
                        {
                            let div = match value {
                                0  => Field::Whole,
                                1  => Field::TwoThirds,
                                _ => return Err(serde::de::Error::unknown_field(&value.to_string())),
                            };
                            Ok(div)
                        }

                        fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                            where E: serde::de::Error,
                        {
                            match value {
                                "Whole"     => Ok(Field::Whole),
                                "TwoThirds" => Ok(Field::TwoThirds),
                                _ => Err(serde::de::Error::unknown_field(value)),
                            }
                        }
                    }

                    deserializer.deserialize(FieldVisitor)
                }
            }

            struct Visitor;

            impl serde::de::EnumVisitor for Visitor {
                type Value = DivType;

                fn visit<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
                    where V: serde::de::VariantVisitor,
                {
                    let div = match try!(visitor.visit_variant()) {
                        Field::Whole     => DivType::Whole,
                        Field::TwoThirds => DivType::TwoThirds,
                    };
                    try!(visitor.visit_unit());
                    Ok(div)
                }
            }

            const VARIANTS: &'static [&'static str] = &["Whole", "TwoThirds"];

            deserializer.deserialize_enum("DivType", VARIANTS, Visitor)
        }
    }

    #[test]
    fn test() {
        extern crate serde_json;

        let div = DivType::TwoThirds;
        let serialized = serde_json::to_string(&div).unwrap();

        println!("{}", serialized);
        assert_eq!("{\"TwoThirds\":[]}", &serialized);
        
        let deserialized: DivType = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(div, deserialized);
    }

}

mod measure {
    use measure::Measure;
    use super::serde;

    impl serde::Serialize for Measure {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer,
        {
            struct Visitor<'a> {
                t: &'a Measure,
                field_idx: u8,
            }

            impl<'a> serde::ser::SeqVisitor for Visitor<'a> {
                fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
                    where S: serde::Serializer,
                {
                    match self.field_idx {
                        0 => {
                            self.field_idx += 1;
                            Ok(Some(try!(serializer.serialize_tuple_struct_elt(self.t.0))))
                        },
                        1 => {
                            self.field_idx += 1;
                            Ok(Some(try!(serializer.serialize_tuple_struct_elt(self.t.1))))
                        },
                        2 => {
                            self.field_idx += 1;
                            Ok(Some(try!(serializer.serialize_tuple_struct_elt(self.t.2))))
                        },
                        _ => Ok(None),
                    }
                }

                fn len(&self) -> Option<usize> {
                    Some(3)
                }
            }

            serializer.serialize_tuple_struct("Measure", Visitor { t: self, field_idx: 0 })
        }
    }

    impl serde::Deserialize for Measure {
        fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
            where D: serde::Deserializer,
        {
            struct Visitor;

            impl serde::de::Visitor for Visitor {
                type Value = Measure;

                fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Measure, V::Error>
                    where V: serde::de::SeqVisitor,
                {
                    let num = try!(visitor.visit());
                    let div = try!(visitor.visit());
                    let divt = try!(visitor.visit());

                    let num = match num {
                        Some(num) => num,
                        None => return Err(serde::de::Error::missing_field("num")),
                    };

                    let div = match div {
                        Some(div) => div,
                        None => return Err(serde::de::Error::missing_field("div")),
                    };

                    let divt = match divt {
                        Some(divt) => divt,
                        None => return Err(serde::de::Error::missing_field("divt")),
                    };

                    try!(visitor.end());

                    Ok(Measure(num, div, divt))
                }
            }

            deserializer.deserialize_tuple_struct("Measure", 3, Visitor)
        }
    }

    #[test]
    fn test() {
        use division::{Division, DivType};
        extern crate serde_json;

        let measure = Measure(4, Division::Bar, DivType::Whole);
        let serialized = serde_json::to_string(&measure).unwrap();

        println!("{}", serialized);
        assert_eq!("[4,{\"Bar\":[]},{\"Whole\":[]}]", &serialized);
        
        let deserialized: Measure = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(measure, deserialized);
    }
}

mod ms {
    use ms::Ms;
    use super::serde;

    impl serde::Serialize for Ms {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer,
        {
            serializer.serialize_newtype_struct("Ms", self.ms())
        }
    }

    impl serde::Deserialize for Ms {
        fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
            where D: serde::Deserializer,
        {
            struct Visitor;

            impl serde::de::Visitor for Visitor {
                type Value = Ms;

                fn visit_f64<E>(&mut self, v: f64) -> Result<Self::Value, E>
                    where E: serde::de::Error,
                {
                    Ok(Ms(v))
                }

                fn visit_newtype_struct<D>(&mut self, deserializer: &mut D) -> Result<Self::Value, D::Error>
                    where D: serde::Deserializer,
                {
                    Ok(Ms(try!(super::serde::de::Deserialize::deserialize(deserializer))))
                }
            }

            deserializer.deserialize_newtype_struct("Ms", Visitor)
        }
    }

    #[test]
    fn test() {
        extern crate serde_json;

        let ms = Ms(16.0);
        let serialized = serde_json::to_string(&ms).unwrap();

        println!("{}", serialized);
        assert_eq!("16", &serialized);
        
        let deserialized: Ms = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(ms, deserialized);
    }
}

mod samples {
    use samples::Samples;
    use super::serde;

    impl serde::Serialize for Samples {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer,
        {
            serializer.serialize_newtype_struct("Samples", self.samples())
        }
    }

    impl serde::Deserialize for Samples {
        fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
            where D: serde::Deserializer,
        {
            struct Visitor;

            impl serde::de::Visitor for Visitor {
                type Value = Samples;

                fn visit_i64<E>(&mut self, v: i64) -> Result<Self::Value, E>
                    where E: serde::de::Error,
                {
                    Ok(Samples(v))
                }

                fn visit_newtype_struct<D>(&mut self, deserializer: &mut D) -> Result<Self::Value, D::Error>
                    where D: serde::Deserializer,
                {
                    Ok(Samples(try!(super::serde::de::Deserialize::deserialize(deserializer))))
                }
            }

            deserializer.deserialize_newtype_struct("Samples", Visitor)
        }
    }

    #[test]
    fn test() {
        extern crate serde_json;

        let samples = Samples(44100);
        let serialized = serde_json::to_string(&samples).unwrap();

        println!("{}", serialized);
        assert_eq!("44100", &serialized);
        
        let deserialized: Samples = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(samples, deserialized);
    }
}

mod ticks {
    use ticks::Ticks;
    use super::serde;

    impl serde::Serialize for Ticks {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer,
        {
            serializer.serialize_newtype_struct("Ticks", self.ticks())
        }
    }

    impl serde::Deserialize for Ticks {
        fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
            where D: serde::Deserializer,
        {
            struct Visitor;

            impl serde::de::Visitor for Visitor {
                type Value = Ticks;

                fn visit_i64<E>(&mut self, v: i64) -> Result<Self::Value, E>
                    where E: serde::de::Error,
                {
                    Ok(Ticks(v))
                }

                fn visit_newtype_struct<D>(&mut self, deserializer: &mut D) -> Result<Self::Value, D::Error>
                    where D: serde::Deserializer,
                {
                    Ok(Ticks(try!(super::serde::de::Deserialize::deserialize(deserializer))))
                }
            }

            deserializer.deserialize_newtype_struct("Ticks", Visitor)
        }
    }

    #[test]
    fn test() {
        extern crate serde_json;

        let ticks = Ticks(1960);
        let serialized = serde_json::to_string(&ticks).unwrap();

        println!("{}", serialized);
        assert_eq!("1960", &serialized);
        
        let deserialized: Ticks = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(ticks, deserialized);
    }
}

mod time_sig {
    use time_sig::TimeSig;
    use super::serde;

    impl serde::Serialize for TimeSig {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer,
        {
            struct Visitor<'a> {
                t: &'a TimeSig,
                field_idx: u8,
            }

            impl<'a> serde::ser::MapVisitor for Visitor<'a> {
                fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
                    where S: serde::Serializer,
                {
                    match self.field_idx {
                        0 => {
                            self.field_idx += 1;
                            Ok(Some(try!(serializer.serialize_struct_elt("top", self.t.top))))
                        },
                        1 => {
                            self.field_idx += 1;
                            Ok(Some(try!(serializer.serialize_struct_elt("bottom", self.t.bottom))))
                        },
                        _ => Ok(None),
                    }
                }

                fn len(&self) -> Option<usize> {
                    Some(2)
                }
            }

            serializer.serialize_struct("TimeSig", Visitor { t: self, field_idx: 0 })
        }
    }

    impl serde::Deserialize for TimeSig {
        fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
            where D: serde::Deserializer,
        {
            struct Visitor;

            impl serde::de::Visitor for Visitor {
                type Value = TimeSig;

                fn visit_map<V>(&mut self, mut visitor: V) -> Result<TimeSig, V::Error>
                    where V: serde::de::MapVisitor,
                {
                    let mut top = None;
                    let mut bottom = None;

                    enum Field {
                        Top,
                        Bottom,
                    }

                    impl serde::Deserialize for Field {
                        fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
                            where D: serde::de::Deserializer,
                        {
                            struct FieldVisitor;

                            impl serde::de::Visitor for FieldVisitor {
                                type Value = Field;

                                fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                                    where E: serde::de::Error,
                                {
                                    match value {
                                        "top" => Ok(Field::Top),
                                        "bottom" => Ok(Field::Bottom),
                                        _ => Err(serde::de::Error::custom("expected top or bottom")),
                                    }
                                }
                            }

                            deserializer.deserialize(FieldVisitor)
                        }
                    }

                    loop {
                        match try!(visitor.visit_key()) {
                            Some(Field::Top) => { top = Some(try!(visitor.visit_value())); },
                            Some(Field::Bottom) => { bottom = Some(try!(visitor.visit_value())); },
                            None => { break; }
                        }
                    }

                    let top = match top {
                        Some(top) => top,
                        None => return Err(serde::de::Error::missing_field("top")),
                    };

                    let bottom = match bottom {
                        Some(bottom) => bottom,
                        None => return Err(serde::de::Error::missing_field("bottom")),
                    };

                    try!(visitor.end());

                    Ok(TimeSig { top: top, bottom: bottom })
                }
            }

            static FIELDS: &'static [&'static str] = &["top", "bottom"];

            deserializer.deserialize_struct("TimeSig", FIELDS, Visitor)
        }
    }

    #[test]
    fn test() {
        extern crate serde_json;

        let ts = TimeSig { top: 4, bottom: 4 };
        let serialized = serde_json::to_string(&ts).unwrap();

        println!("{}", serialized);
        assert_eq!("{\"top\":4,\"bottom\":4}", serialized);
        
        let deserialized: TimeSig = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(ts, deserialized);
    }
}
