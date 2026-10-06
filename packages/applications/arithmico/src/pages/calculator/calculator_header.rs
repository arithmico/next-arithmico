use leptos::prelude::*;
use translate::{FormattedMessage, use_translate};
use ui::{
    common::page_title::PageTitle,
    container::page_header::PageHeader,
    form::menu::{Menu, MenuDefinition},
    icon::{
        history_icon::HistoryIcon, list_icon::ListIcon, menu_icon::MenuIcon,
    },
};
use web_state::WebState;

use crate::state::{
    ClearInputAction, ClearOutputAction, ResetSessionAction, State,
};

#[component]
pub fn CalculatorHeader() -> impl IntoView {
    let state = State::expect_state();

    let menu_definition = MenuDefinition::new()
        .button(|| {
            view! {
                <>
                    <FormattedMessage id="calculator.toolbar.actions" />
                    <MenuIcon class="rotate-180 icon-hover" />
                </>
            }
        })
        .item(|| view! {
            <>
                <FormattedMessage id="calculator.toolbar.actions.clear-input" />
            </>
        }, move || state.dispatch(&ClearInputAction::new()))
        .item(|| view! {
            <>
                <FormattedMessage id="calculator.toolbar.actions.clear-output" />
            </>
        }, move || state.dispatch(&ClearOutputAction::new()))
        .item(|| view! {
            <>
                <FormattedMessage id="calculator.toolbar.actions.reset-session" />
            </>
        }, move || state.dispatch(&ResetSessionAction::new()));

    view! {
        <PageHeader>
            <PageTitle>
                <FormattedMessage id="calculator.title" />
            </PageTitle>

            <ToolbarNavigation>
                <ToolbarNavigationItem to="/history">
                    <HistoryIcon class="icon-hover" />
                    <FormattedMessage id="calculator.toolbar.history" />
                </ToolbarNavigationItem>
                <ToolbarNavigationItem to="/definitions">
                    <ListIcon class="icon-hover" />
                    <FormattedMessage id="calculator.toolbar.definitions" />
                </ToolbarNavigationItem>
            </ToolbarNavigation>

            <Menu definition=menu_definition />
        </PageHeader>
    }
}

#[component]
fn ToolbarNavigation(children: Children) -> impl IntoView {
    let translate = use_translate();

    view! {
        <nav
            class="calculator-toolbar"
            aria-label=move || {
                translate("calculator.toolbar.label", None).unwrap_or_default()
            }
        >
            <ul>{children()}</ul>
        </nav>
    }
}

#[component]
fn ToolbarNavigationItem(
    children: Children,
    to: impl ToString,
) -> impl IntoView {
    view! {
        <li class="flex toolbar-item">
            <a class="calculator-header-action-element" href=to.to_string()>
                {children()}
            </a>
        </li>
    }
}
