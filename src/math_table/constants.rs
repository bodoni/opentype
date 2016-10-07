use truetype::{Result, Tape, Value};

table! {
    #[doc = "math constants"]
    pub Constants {
        script_percent_scale_down (Percentage),
        script_script_percent_scale_down (Percentage),
        delimited_sub_formula_min_height (DesignUnits),
        display_operator_min_height (DesignUnits),
        math_leading (MathValueRecord),
        axis_height (MathValueRecord),
        accent_base_height (MathValueRecord),
        flattened_accent_base_height (MathValueRecord),
        subscript_shift_down (MathValueRecord),
        subscript_top_max (MathValueRecord),
        subscript_baseline_drop_min (MathValueRecord),
        superscript_shift_up (MathValueRecord),
        superscript_shift_up_cramped (MathValueRecord),
        superscript_bottom_min (MathValueRecord),
        superscript_baseline_drop_max (MathValueRecord),
        sub_superscript_gap_min (MathValueRecord),
        superscript_bottom_max_with_subscript (MathValueRecord),
        space_after_script (MathValueRecord),
        upper_limit_gap_min (MathValueRecord),
        upper_limit_baseline_rise_min (MathValueRecord),
        lower_limit_gap_min (MathValueRecord),
        lower_limit_baseline_drop_min (MathValueRecord),
        stack_top_shift_up (MathValueRecord),
        stack_top_display_style_shift_up (MathValueRecord),
        stack_bottom_shift_down (MathValueRecord),
        stack_bottom_display_style_shift_down (MathValueRecord),
        stack_gap_min (MathValueRecord),
        stack_display_style_gap_min (MathValueRecord),
        stretch_stack_top_shift_up (MathValueRecord),
        stretch_stack_bottom_shift_down (MathValueRecord),
        stretch_stack_gap_above_min (MathValueRecord),
        stretch_stack_gap_below_min (MathValueRecord),
        fraction_numerator_shift_up (MathValueRecord),
        fraction_numerator_display_style_shift_up (MathValueRecord),
        fraction_denominator_shift_down (MathValueRecord),
        fraction_denominator_display_style_shift_down (MathValueRecord),
        fraction_numerator_gap_min (MathValueRecord),
        fraction_num_display_style_gap_min (MathValueRecord),
        fraction_rule_thickness (MathValueRecord),
        fraction_denominator_gap_min (MathValueRecord),
        fraction_denom_display_style_gap_min (MathValueRecord),
        skewed_fraction_horizontal_gap (MathValueRecord),
        skewed_fraction_vertical_gap (MathValueRecord),
        overbar_vertical_gap (MathValueRecord),
        overbar_rule_thickness (MathValueRecord),
        overbar_extra_ascender (MathValueRecord),
        underbar_vertical_gap (MathValueRecord),
        underbar_rule_thickness (MathValueRecord),
        underbar_extra_descender (MathValueRecord),
        radical_vertical_gap (MathValueRecord),
        radical_display_style_vertical_gap (MathValueRecord),
        radical_rule_thickness (MathValueRecord),
        radical_extra_ascender (MathValueRecord),
        radical_kern_before_degree (MathValueRecord),
        radical_kern_after_degree (MathValueRecord),
        radical_degree_bottom_raise_percent (Percentage),
    }
}

table! {
    #[doc = "Math value record"]
    pub MathValueRecord {
        value (DesignUnits),
        device_table (u16),
    }
}

// FIXME: Figure out default parameters??
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Percentage(pub i16);

impl Value for Percentage {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(Percentage(try!(tape.take::<i16>())))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DesignUnits(pub i16);

impl Value for DesignUnits {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(DesignUnits(try!(tape.take::<i16>())))
    }
}

