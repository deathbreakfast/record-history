//! Static preview registrations exported for host catalog merge.

use crate::preview::PreviewRegistration;

#[cfg(feature = "preview")]
use crate::components::{HISTORYTIMELINE_DOC, HISTORYTIMELINE_PROPS};
#[cfg(feature = "preview")]
use crate::preview::OrbitalComponentView;
#[cfg(feature = "preview")]
use leptos::prelude::*;

/// `HistoryTimeline` loads through a server function, so the catalog page documents
/// it (`preview = "manual"` on the component) and does not mount a live timeline.
#[cfg(feature = "preview")]
fn history_timeline_preview() -> AnyView {
    view! {
        <OrbitalComponentView
            component_name="History Timeline"
            component_description_md=HISTORYTIMELINE_DOC
            component_props=HISTORYTIMELINE_PROPS
            default=view! { <p>"Live timeline requires a running host with a Valence session."</p> }
        />
    }
    .into_any()
}

#[cfg(feature = "preview")]
static HISTORYTIMELINE_PREVIEW_REGISTRATION: PreviewRegistration = PreviewRegistration {
    slug: "history-timeline",
    label: "History Timeline",
    section: "History",
    section_priority: 100,
    category: "Unified Field",
    category_priority: 100,
    category_default_collapsed: false,
    group: "",
    group_priority: 0,
    nav_item: false,
    icon: icondata::AiHistoryOutlined,
    render: history_timeline_preview,
};

orbital_macros::preview_registrations! {
    &HISTORYTIMELINE_PREVIEW_REGISTRATION,
}
