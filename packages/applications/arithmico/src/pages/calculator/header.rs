use leptos::prelude::*;
use translate::FormattedMessage;
use ui::{
    common::page_title::PageTitle,
    form::menu::{Menu, MenuDefinition},
    icon::{
        history_icon::HistoryIcon, list_icon::ListIcon, menu_icon::MenuIcon,
    },
};

#[component]
pub fn CalculatorHeader() -> impl IntoView {
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
            <button>
                <FormattedMessage id="calculator.toolbar.actions.clear-input" />
            </button>
        })
        .item(|| view! {
            <button>
                <FormattedMessage id="calculator.toolbar.actions.clear-output" />
            </button>
        });

    view! {
        <div class="calculator-header">
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

        </div>
    }
}

#[component]
fn ToolbarNavigation(children: Children) -> impl IntoView {
    view! {
        <nav>
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
