/// Type of Hazards
pub enum Hazards {
{%- for hazard in hazards -%}
    {{ hazard }},
{%- endfor -%}
}

/// {{ sho_FireHazard_name }}
/// {{ sho_FireHazard_description }}
pub fn turn_on_light() {
    todo!("Implement using the Wot API");
}

/// {{ sho_FireHazard_name }}
/// {{ sho_FireHazard_description }}
pub fn turn_on_oven() {
    todo!("Implement using the Wot API");
}
