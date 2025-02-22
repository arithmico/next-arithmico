use leptos::prelude::*;
use translate::FormattedMessage;

use crate::components::{HistoryIcon, ListIcon, MenuIcon, PageTitle};

#[component]
pub fn CalculatorHeader() -> impl IntoView {
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

            <button class="calculator-header-action-element">
                <FormattedMessage id="calculator.toolbar.actions" />
                <MenuIcon class="rotate-180 icon-hover" />
            </button>

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
