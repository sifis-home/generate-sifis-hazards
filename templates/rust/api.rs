/// Hazards type.
pub enum Hazards {
{%- for hazard in hazards -%}
    {{ hazard }},
{%- endfor -%}
}

/// Categories.
pub enum Categories {
{%- for category in categories -%}
    {{ category }},
{%- endfor -%}
}

/// {{ FireHazard_name }}.
/// {{ FireHazard_description }}.
pub fn turn_on_light() {
    todo!("Implement using the Wot API");
}

/// {{ FireHazard_name }}.
/// {{ FireHazard_description }}.
///
/// {{ AudioVideoStream_name }}.
/// {{ AudioVideoStream_description }}.
pub fn turn_on_oven() {
    todo!("Implement using the Wot API");
}
