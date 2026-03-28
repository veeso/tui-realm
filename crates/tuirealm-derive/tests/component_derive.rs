#![allow(dead_code)] // practically just compile tests

use tui_realm_stdlib::components::Label;
use tuirealm_derive::Component;

#[test]
fn should_allow_default_field_name() {
    #[derive(Component)]
    struct Test1 {
        component: Label,
    }
}

#[test]
fn should_allow_specific_names() {
    #[derive(Component)]
    #[component = "non_default_name"]
    struct Test1 {
        non_default_name: Label,
    }

    #[derive(Component)]
    #[component("non_default_name")]
    struct Test2 {
        non_default_name: Label,
    }
}

#[test]
fn should_allow_specific_names_via_field_attr() {
    #[derive(Component)]
    struct Test1 {
        #[component]
        non_default_name: Label,
    }
}

#[test]
fn should_allow_unnamed_fields() {
    #[derive(Component)]
    struct Test1(Label);

    #[derive(Component)]
    struct Test2(String, #[component] Label);
}
