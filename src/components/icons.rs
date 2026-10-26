use leptos::prelude::*;

/// Semantic names for the project's locally maintained SVG icon set.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconName {
    Home,
    Tools,
    Market,
    Games,
    FileText,
    Network,
    Search,
    Bold,
    Italic,
    Strikethrough,
    Heading,
    Link,
    Image,
    Code,
    Braces,
    Quote,
    List,
    ListOrdered,
    Table,
    HorizontalRule,
    Eye,
    RotateCcw,
    Trash,
    Copy,
    Download,
    Upload,
    Refresh,
    ExternalLink,
    ChevronDown,
    Menu,
    Close,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Bomb,
    Snake,
    Grid,
    Cards,
    Keyboard,
    Word,
    Hangman,
    Puzzle,
    Lightbulb,
    BrickWall,
    Paddle,
    Bird,
    Blocks,
    LockOpen,
    AlertTriangle,
    Check,
    Info,
    ShieldCheck,
    Clock,
    Sun,
    Moon,
    Sparkles,
    ArrowUpDown,
    Play,
}

impl IconName {
    /// Returns the stable identifier used by the SVG sprite.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::Tools => "tools",
            Self::Market => "market",
            Self::Games => "games",
            Self::FileText => "file-text",
            Self::Network => "network",
            Self::Search => "search",
            Self::Bold => "bold",
            Self::Italic => "italic",
            Self::Strikethrough => "strikethrough",
            Self::Heading => "heading",
            Self::Link => "link",
            Self::Image => "image",
            Self::Code => "code",
            Self::Braces => "braces",
            Self::Quote => "quote",
            Self::List => "list",
            Self::ListOrdered => "list-ordered",
            Self::Table => "table",
            Self::HorizontalRule => "horizontal-rule",
            Self::Eye => "eye",
            Self::RotateCcw => "rotate-ccw",
            Self::Trash => "trash",
            Self::Copy => "copy",
            Self::Download => "download",
            Self::Upload => "upload",
            Self::Refresh => "refresh",
            Self::ExternalLink => "external-link",
            Self::ChevronDown => "chevron-down",
            Self::Menu => "menu",
            Self::Close => "close",
            Self::ArrowUp => "arrow-up",
            Self::ArrowDown => "arrow-down",
            Self::ArrowLeft => "arrow-left",
            Self::ArrowRight => "arrow-right",
            Self::Bomb => "bomb",
            Self::Snake => "snake",
            Self::Grid => "grid",
            Self::Cards => "cards",
            Self::Keyboard => "keyboard",
            Self::Word => "word",
            Self::Hangman => "hangman",
            Self::Puzzle => "puzzle",
            Self::Lightbulb => "lightbulb",
            Self::BrickWall => "brick-wall",
            Self::Paddle => "paddle",
            Self::Bird => "bird",
            Self::Blocks => "blocks",
            Self::LockOpen => "lock-open",
            Self::AlertTriangle => "alert-triangle",
            Self::Check => "check",
            Self::Info => "info",
            Self::ShieldCheck => "shield-check",
            Self::Clock => "clock",
            Self::Sun => "sun",
            Self::Moon => "moon",
            Self::Sparkles => "sparkles",
            Self::ArrowUpDown => "arrow-up-down",
            Self::Play => "play",
        }
    }
}

/// Renders a project-owned SVG icon from the shared sprite.
///
/// Icons inherit the current text color and are decorative by default. Put the
/// accessible name on the surrounding button or link.
#[component]
pub fn Icon(
    name: IconName,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let href = format!("/icons.svg#{}", name.as_str());

    view! {
        <svg
            class=class
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1.8"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
            focusable="false"
        >
            <use href=href />
        </svg>
    }
}
