macro_rules! flag_data {
    ($debug:ident, $name1:ident, $name2:ident) => {
        if $name1.$name2 {
            $debug.abbrev += dubyte(consts::dw_at::$name2)
                .add_comment(concat!("DW_AT_", stringify!($name2)).to_string())
        }
    };
}

macro_rules! string_data {
    ($debug:ident, $name1:ident, $name2:ident, $name3:ident) => {
        if let Some(str) = $name1.$name2 {
            $debug.abbrev += dubyte(consts::dw_at::$name2)
                .add_comment(concat!("DW_AT_", stringify!($name2)).to_string())
                + dubyte(consts::dw_form::STRP).add_comment("DW_FORM_strp".to_string());
            $debug.info += dulong($name3.get_offset(str) as u32)
                .add_comment(concat!("DW_AT_", stringify!($name2)).to_string())
        }
    };
}

// macro_rules! string_data_not_opt {
//     ($debug:ident, $name1:ident, $name2:ident, $name3:ident) => {
//         $debug.abbrev +=
//             dubyte(consts::dw_at::$name2).add_comment(concat!("DW_AT_", stringify!($name2)).to_string())
//             + dubyte(consts::dw_form::STRP).add_comment("DW_FORM_strp".to_string());
//         $debug.info += dulong($name3.get_offset($name1.$name2) as u32).add_comment(concat!("DW_AT_", stringify!($name2)).to_string())
//     };
// }

macro_rules! info_data {
    ($debug:ident, $name1:ident, $name2:ident) => {
        if let Some(id) = $name1.$name2 {
            $debug.abbrev += dubyte(consts::dw_at::$name2)
                .add_comment(concat!("DW_AT_", stringify!($name2)).to_string())
                + dubyte(consts::dw_form::DATA2).add_comment("DW_FORM_data2".to_string());
            $debug.info += dushort(id.as_u16()).add_comment(id.as_str().to_string())
        }
    };
}

macro_rules! info1_data {
    ($debug:ident, $name1:ident, $name2:ident) => {
        if let Some(id) = $name1.$name2 {
            $debug.abbrev += dubyte(consts::dw_at::$name2)
                .add_comment(concat!("DW_AT_", stringify!($name2)).to_string())
                + dubyte(consts::dw_form::DATA1).add_comment("DW_FORM_data1".to_string());
            $debug.info += dubyte(id.as_u8()).add_comment(id.as_str().to_string())
        }
    };
}

macro_rules! data4 {
    ($debug:ident, $name1:ident, $name2:ident) => {
        if let Some(id) = $name1.$name2 {
            $debug.abbrev += dubyte(consts::dw_at::$name2)
                .add_comment(concat!("DW_AT_", stringify!($name2)).to_string())
                + dubyte(consts::dw_form::DATA4).add_comment("DW_FORM_data4".to_string());
            $debug.info += dulong(id).add_comment(concat!("DW_AT_", stringify!($name2)).to_string())
        }
    };
}

macro_rules! sdata4 {
    ($debug:ident, $name1:ident, $name2:ident) => {
        if let Some(id) = $name1.$name2 {
            $debug.abbrev += dubyte(consts::dw_at::$name2)
                .add_comment(concat!("DW_AT_", stringify!($name2)).to_string())
                + dubyte(consts::dw_form::DATA4).add_comment("DW_FORM_data4".to_string());
            $debug.info += dlong(id).add_comment(concat!("DW_AT_", stringify!($name2)).to_string())
        }
    };
}

macro_rules! low_high_pc {
    ($debug:ident, $name:ident, $str:ident) => {
        if let Some((low, high)) = $name.low_high_pc {
            $debug.abbrev += dubyte(consts::dw_at::low_pc).add_comment("DW_AT_low_pc".to_string())
                + dubyte(consts::dw_form::ADDR).add_comment("DW_FORM_addr".to_string())
                + dubyte(consts::dw_at::high_pc).add_comment("DW_AT_high_pc".to_string())
                + dubyte(consts::dw_form::DATA4).add_comment("DW_FORM_data4".to_string());
            let id = $str.new_tmp();
            $debug.info += daddress(low.clone()).add_comment("DW_AT_low_pc".to_string())
                + Segment::directive(crate::directives::set_sub(id.clone(), high, low))
                + dlong_label(id).add_comment("DW_AT_high_pc".to_string())
        };
    };
}

macro_rules! addr_data {
    ($debug:ident, $name:ident, $name2:ident) => {
        if let Some(lab) = $name.$name2 {
            $debug.abbrev += dubyte(consts::dw_at::$name2)
                .add_comment(concat!("DW_AT_", stringify!($name2)).to_string())
                + dubyte(consts::dw_form::ADDR).add_comment("DW_FORM_addr".to_string());
            $debug.info += daddress(lab.clone())
                .add_comment(concat!("DW_AT_", stringify!($name2)).to_string());
        };
    };
}

macro_rules! end {
    ($debug:ident) => {
        $debug.abbrev += dubyte(0).add_comment("EOM(0)".to_string());
        $debug.abbrev += dubyte(0).add_comment("EOM(1)".to_string());
    };
}

macro_rules! not_done {
    ($name1:ident, $name2:ident) => {
        assert_eq!($name1.$name2, ());
    };
}

#[allow(unused_macros)]
macro_rules! line_ptr {
    ($debug:ident, $name1:ident, $field:ident, $str:ident) => {
        if let Some(lab) = $name1.$field {
            if let Some(start_ptr) = $str.get_line_start() {
                $debug.abbrev += dubyte(consts::dw_at::$field)
                    .add_comment(concat!("DW_AT_", stringify!($field)).to_string())
                    + dubyte(consts::dw_form::SEC_OFFSET)
                        .add_comment("DW_FORM_sec_offset".to_string());
                let start_ptr = start_ptr.clone();
                let id = $str.new_tmp();
                $debug.info +=
                    Segment::directive(crate::directives::set_sub(id.clone(), lab, start_ptr))
                        + daddress(id).add_comment(stringify!($field).to_string());
            } else {
                panic!("Required a .debug_line start label but none was given")
            }
        }
    };
}

macro_rules! loclist_ptr {
    ($debug:ident, $name1:ident, $field:ident, $str:ident) => {
        if let Some(lab) = $name1.$field {
            if let Some(start_ptr) = $str.get_loc_start() {
                $debug.abbrev += dubyte(consts::dw_at::$field)
                    .add_comment(concat!("DW_AT_", stringify!($field)).to_string())
                    + dubyte(consts::dw_form::SEC_OFFSET)
                        .add_comment("DW_FORM_sec_offset".to_string());
                let start_ptr = start_ptr.clone();
                let id = $str.new_tmp();
                $debug.info +=
                    Segment::directive(crate::directives::set_sub(id.clone(), lab, start_ptr))
                        + daddress(id).add_comment(stringify!($field).to_string());
            } else {
                panic!("Required a .debug_loc start label but none was given")
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! mac_ptr {
    ($debug:ident, $name1:ident, $field:ident, $str:ident) => {
        if let Some(lab) = $name1.$field {
            if let Some(start_ptr) = $str.get_macinfo_start() {
                $debug.abbrev += dubyte(consts::dw_at::$field)
                    .add_comment(concat!("DW_AT_", stringify!($field)).to_string())
                    + dubyte(consts::dw_form::SEC_OFFSET)
                        .add_comment("DW_FORM_sec_offset".to_string());
                let start_ptr = start_ptr.clone();
                let id = $str.new_tmp();
                $debug.info +=
                    Segment::directive(crate::directives::set_sub(id.clone(), lab, start_ptr))
                        + daddress(id.clone()).add_comment(stringify!($field).to_string());
            } else {
                panic!("Required a .debug_macinfo start label but none was given")
            }
        }
    };
}

macro_rules! rangelistptr {
    ($debug:ident, $name1:ident, $field:ident, $str:ident) => {
        if let Some(lab) = $name1.$field {
            if let Some(start_ptr) = $str.get_ranges_start() {
                $debug.abbrev += dubyte(consts::dw_at::$field)
                    .add_comment(concat!("DW_AT_", stringify!($field)).to_string())
                    + dubyte(consts::dw_form::SEC_OFFSET)
                        .add_comment("DW_FORM_sec_offset".to_string());
                let start_ptr = start_ptr.clone();
                let id = $str.new_tmp();
                $debug.info +=
                    Segment::directive(crate::directives::set_sub(id.clone(), lab, start_ptr))
                        + daddress(id).add_comment(stringify!($field).to_string());
            } else {
                panic!("Required a .debug_ranges start label but none was given")
            }
        }
    };
}
