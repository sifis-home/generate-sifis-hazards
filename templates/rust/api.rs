#![allow(clippy::enum_variant_names)]

use crate::thing::Hazard as ThingHazard;

/// Hazards type.
pub enum Hazard {
{%- for hazard in hazards -%}
    /// {{ hazard.description }}
    {{ hazard.name }},
{%- endfor -%}
}

impl ToString for Hazard {
    fn to_string(&self) -> String {
        match self {
            {%- for hazard in hazards -%}
                Self::{{ hazard.name }} => "{{ hazard.name }}",
            {%- endfor -%}
        }.to_owned()
    }
}

impl Hazard {

    /// Returns the `Hazard` type associated to a Thing.
    ///
    /// It returns `None` if any type has been found for the given
    /// Thing.
    pub fn has_hazard(thing_hazard: &ThingHazard) -> Option<Self> {
        match thing_hazard.name.as_str() {
            {%- for hazard in hazards -%}
                "{{ hazard.name }}" => Some(Self::{{ hazard.name }}),
            {%- endfor -%}
            _ => None,
        }
    }

    /// Returns the description associated to an `Hazard` type.
    pub fn get_description(&self) -> &str {
        match self {
            {%- for hazard in hazards -%}
                Self::{{ hazard.name }} => "{{ hazard.description }}",
            {%- endfor -%}
        }
    }

    /// Returns the `Category`s associated to an `Hazard`.
    pub fn has_category(&self) -> Category {
        match self {
            {%- for hazard in hazards -%}
                Self::{{ hazard.name }} => Category::{{ hazard.category }},
            {%- endfor -%}
        }
    }

    /// Returns all `Hazard` types as immutable strings.
    pub fn all_hazards() -> &'static [&'static str] {
        &[
            {%- for hazard in hazards -%}
                "{{ hazard.name }}",
            {%- endfor -%}
        ]
    }
}

/// Categories associated to an hazard.
pub enum Category {
{%- for category in categories -%}
    /// {{ category.description }}
    {{ category.name }},
{%- endfor -%}
}

impl ToString for Category {
    fn to_string(&self) -> String {
        match self {
            {%- for category in categories -%}
                Self::{{ category.name }} => "{{ category.name }}",
            {%- endfor -%}
        }.to_owned()
    }
}

impl Category {
    /// Returns all `Hazard`s associated to a `Category`.
    pub fn all_hazards(&self) -> &[Hazard] {
        match self {
            {%- for category in categories -%}
                Self::{{ category.name }} => &[
                    {%- for hazard in category.hazards -%}
                        Hazard::{{ hazard }},
                    {%- endfor -%}
                ],
            {%- endfor -%}
        }
    }

    /// Returns all `Category` as immutable strings.
    pub fn all_categories() -> &'static [&'static str] {
        &[
            {%- for category in categories -%}
                "{{ category.name }}",
            {%- endfor -%}
        ]
    }
}
