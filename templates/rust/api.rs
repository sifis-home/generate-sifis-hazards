/// Hazards type.
pub enum Hazards {
{%- for hazard in hazards -%}
    /// {{ hazard.description }}
    {{ hazard.name }},
{%- endfor -%}
}

/// Categories.
pub enum Categories {
{%- for category in categories -%}
    /// {{ category.description }}
    {{ category.name }},
{%- endfor -%}
}
