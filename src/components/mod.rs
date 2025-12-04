//! A collection of UI components

#[doc = " Alert banners for info, success, warning, and error messages"]
pub mod alert;
#[doc = " Avatar and AvatarGroup for user images"]
pub mod avatar;
#[doc = " Badge pills for status and labels"]
pub mod badge;
#[doc = " Button with variants, sizes, and loading state"]
pub mod button;
#[doc = " ButtonGroup for segmented button layouts"]
pub mod button_group;
#[doc = " Card container with header, content, and footer"]
pub mod card;
#[doc = " Checkbox input with label"]
pub mod checkbox;
#[doc = " Command palette with search and keyboard navigation"]
pub mod command_palette;
#[doc = " Dropdown menu with trigger and items"]
pub mod dropdown;
#[doc = " Empty state placeholder with icon and action"]
pub mod empty_state;
#[doc = " Generic icon wrapper component"]
pub mod icon;
#[doc = " IconButton for icon-only actions"]
pub mod icon_button;
#[doc = " Lucide icon components"]
pub mod icons;
#[doc = " Text input with label and error state"]
pub mod input;
#[doc = " Modal dialog with header, body, and footer"]
pub mod modal;
#[doc = " Navigation bar with brand and actions"]
pub mod navbar;
#[doc = " Radio button and RadioGroup"]
pub mod radio;
#[doc = " Select dropdown input"]
pub mod select;
#[doc = " Sidebar navigation component"]
pub mod sidebar;
#[doc = " Slider / range input"]
pub mod slider;
#[doc = " Progress bars and circular progress"]
pub mod progress;
#[doc = " Loading spinner with sizes"]
pub mod spinner;
#[doc = " Toast / Snackbar notifications"]
pub mod toast;
#[doc = " Tooltip component for hover info"]
pub mod tooltip;
#[doc = " Toggle for on/off states"]
pub mod toggle;
#[doc = " Tabs with line and pill variants"]
pub mod tabs;
#[doc = " Multi-line text input"]
pub mod textarea;
#[doc = " Tag input with removable chips"]
pub mod tag_input;
#[doc = " Segmented input for OTP/TOTP codes"]
pub mod segmented_input;
#[doc = " Copy input with clipboard button"]
pub mod copy_input;
pub mod file_icons;
pub mod file_path;

// Layout components
#[doc = " Stack - vertical flex layout"]
pub mod stack;
#[doc = " Row - horizontal flex layout"]
pub mod row;
#[doc = " Grid - CSS grid layout"]
pub mod grid;
#[doc = " Container - centered max-width container"]
pub mod container;
#[doc = " Section and SubSection for content organization"]
pub mod section;
#[doc = " SidebarLayout - flex layout for sidebar + main"]
pub mod sidebar_layout;
#[doc = " MainContent - main content area in sidebar layout"]
pub mod main_content;
#[doc = " IconGrid - grid layout for showcasing icons"]
pub mod icon_grid;
#[doc = " Breadcrumb navigation"]
pub mod breadcrumb;
#[doc = " Page navigation (prev/next)"]
pub mod page_nav;
#[doc = " Keyboard key display component"]
pub mod kbd;
#[doc = " Styled link component with external indicator"]
pub mod link;
#[doc = " Skeleton loading placeholders"]
pub mod skeleton;
#[doc = " Collapsible accordion sections"]
pub mod accordion;
#[doc = " Drawer/slide-over panel"]
pub mod drawer;
#[doc = " Table components for tabular data"]
pub mod table;
#[doc = " TimeAgo - relative time display"]
pub mod time_ago;
#[doc = " Popover - click-triggered popup"]
pub mod popover;
#[doc = " Context menu - right-click menu"]
pub mod context_menu;
#[doc = " Code block for displaying code"]
pub mod code_block;
#[doc = " Split pane / resizable panels"]
pub mod split_pane;

// Chat / Messaging components
#[doc = " Notification badge for unread counts"]
pub mod notification_badge;
#[doc = " Day divider for message date separation"]
pub mod day_divider;
#[doc = " Streaming status indicator (thinking, searching, etc.)"]
pub mod streaming_status;
#[doc = " Attachment chip for file attachments"]
pub mod attachment_chip;
#[doc = " Chat bubble for individual messages"]
pub mod chat_bubble;
#[doc = " Message group for consecutive messages from same author"]
pub mod message_group;
#[doc = " Message list container with auto-scroll"]
pub mod message_list;
#[doc = " Message composer with textarea and send button"]
pub mod message_composer;
#[doc = " Thread list for conversation navigation"]
pub mod thread_list;

// Additional components
#[doc = " Pulsing dots loading indicator"]
pub mod pulsing_dots;
#[doc = " Sunburst loading indicator"]
pub mod sunburst;
#[doc = " Color swatch for displaying colors"]
pub mod color_swatch;
#[doc = " Git diff statistics display"]
pub mod diff_stats;
#[doc = " Tool call badge for tool execution status"]
pub mod tool_call_badge;
#[doc = " Todo list with dashed circle checkboxes"]
pub mod todo_list;
#[doc = " Retry button for failed actions"]
pub mod retry_button;
#[doc = " Code execution result panel"]
pub mod code_execution_result;
#[doc = " Site footer component"]
pub mod footer;
#[doc = " Application shell layout"]
pub mod app_shell;
#[doc = " Pagination for navigating pages"]
pub mod pagination;
#[doc = " Visual separator/divider"]
pub mod separator;
#[doc = " Form field label with accessibility"]
pub mod label;
#[doc = " Collapsible/expandable content panel"]
pub mod collapsible;
#[doc = " Aspect ratio container"]
pub mod aspect_ratio;
#[doc = " Star rating input"]
pub mod rating;
#[doc = " Statistics display"]
pub mod stat;
#[doc = " Multi-step progress indicator"]
pub mod steps;
#[doc = " File input with drag-and-drop"]
pub mod file_input;
#[doc = " Hover card for rich content on hover"]
pub mod hover_card;
#[doc = " Styled list component"]
pub mod list;
#[doc = " Key-value pairs display"]
pub mod descriptions;
#[doc = " Confirmation dialog"]
pub mod alert_dialog;
#[doc = " Custom scrollbar container"]
pub mod scroll_area;
#[doc = " Calendar date picker"]
pub mod calendar;
#[doc = " Carousel/slider component"]
pub mod carousel;

pub use alert::{Alert, AlertVariant};
pub use avatar::{Avatar, AvatarGroup, AvatarSize};
pub use badge::{Badge, BadgeSize, BadgeVariant};
pub use button::{Button, ButtonSize, ButtonVariant};
pub use button_group::ButtonGroup;
pub use card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
pub use checkbox::{Checkbox, CheckboxSize};
pub use command_palette::{CommandItem, CommandPalette};
pub use dropdown::{
    Dropdown, DropdownAlign, DropdownDivider, DropdownItem, DropdownMenu, DropdownTrigger,
};
pub use empty_state::EmptyState;
pub use icon::{Icon, IconSize};
pub use icon_button::{IconButton, IconButtonSize, IconButtonVariant};
pub use icons::{
    IconAlertCircle, IconArchive, IconBuilding2, IconCheck, IconChevronDown, IconChevronLeft,
    IconChevronRight, IconCircleCheck, IconCircleX, IconClipboardList, IconExternalLink,
    IconFileText, IconFilter, IconGithub, IconGlobe, IconInfo, IconLoader, IconMapPin, IconMenu,
    IconMic, IconMinus, IconPlus, IconSearch, IconServer, IconTriangleAlert, IconUser, IconX,
    IconYoutube,
};
pub use input::{Input, InputSize};
pub use modal::{Modal, ModalBody, ModalFooter, ModalHeader, ModalSize};
pub use navbar::{Navbar, NavbarActions, NavbarBrand, NavbarItem, NavbarNav};
pub use radio::{Radio, RadioGroup, RadioSize};
pub use select::{Select, SelectSize};
pub use sidebar::{Sidebar, SidebarFooter, SidebarGroup, SidebarGroupItems, SidebarHeader, SidebarItem, SidebarNav, SidebarSection};
pub use slider::{Slider, SliderSize};
pub use progress::{Progress, ProgressSize, ProgressVariant, CircularProgress};
pub use spinner::{Spinner, SpinnerSize};
pub use file_path::FilePath;
pub use toast::{Toast, ToastContainer, ToastPosition, ToastVariant};
pub use tooltip::{Tooltip, TooltipPosition};
pub use toggle::{Toggle, ToggleSize};
pub use tabs::{Tab, TabList, TabPanel, TabPanels, Tabs, TabsVariant};
pub use textarea::{Textarea, TextareaSize};
pub use tag_input::{Tag, TagInput, TagInputSize};
pub use segmented_input::{SegmentedInput, SegmentedInputSize};
pub use copy_input::{CopyInput, CopyInputSize};

// Layout exports
pub use stack::{Stack, StackGap};
pub use row::{Row, RowAlign, RowGap};
pub use grid::{Grid, GridColumns, GridGap};
pub use container::{Container, ContainerSize};
pub use section::{Section, SubSection};
pub use sidebar_layout::SidebarLayout;
pub use main_content::MainContent;
pub use icon_grid::{IconGrid, IconGridItem};
pub use breadcrumb::{Breadcrumb, BreadcrumbItem};
pub use page_nav::PageNav;

// New components
pub use kbd::{Kbd, KbdSize};
pub use link::{Link, LinkVariant};
pub use skeleton::{
    Skeleton, SkeletonAvatar, SkeletonCard, SkeletonCodeBlock, SkeletonMessage,
    SkeletonMessageList, SkeletonText, SkeletonThreadItem, SkeletonThreadList, SkeletonVariant,
};
pub use accordion::{Accordion, AccordionItem};
pub use drawer::{Drawer, DrawerBody, DrawerFooter, DrawerHeader, DrawerPosition, DrawerSize};
pub use table::{
    Table, TableBody, TableCell, TableEmpty, TableFooter, TableHeader, TableHeaderCell, TableRow,
    TableSize, TableVariant,
};
pub use time_ago::TimeAgo;
pub use popover::{Popover, PopoverContent, PopoverPosition};
pub use context_menu::{
    ContextMenu, ContextMenuContent, ContextMenuDivider, ContextMenuItem, ContextMenuTrigger,
};
pub use code_block::{CodeBlock, CodeBlockSize, InlineCode, Language};
pub use split_pane::{Panel, SplitDirection, SplitPane};

// Chat / Messaging exports
pub use notification_badge::{NotificationBadge, NotificationBadgeSize};
pub use day_divider::{DayDivider, NewMessagesDivider};
pub use streaming_status::{StreamingStatus, StreamingStatusVariant};
pub use attachment_chip::{AttachmentChip, AttachmentType};
pub use chat_bubble::{ChatBubble, ChatBubbleAlign, ChatBubbleVariant};
pub use message_group::{GroupMessage, MessageGroup};
pub use message_list::MessageList;
pub use message_composer::{ComposerButton, MessageComposer};
pub use thread_list::{ThreadList, ThreadListHeader, ThreadListItem, ThreadListSection};

// Additional component exports
pub use pulsing_dots::{PulsingDots, PulsingDotsSize};
pub use sunburst::{Sunburst, SunburstSize};
pub use color_swatch::{ColorFormat, ColorSwatch};
pub use diff_stats::{DiffFileSummary, DiffStats, DiffStatsStyle};
pub use tool_call_badge::{ToolCallBadge, ToolCallStatus};
pub use todo_list::{TodoItem, TodoList, TodoSize};
pub use retry_button::{RetryButton, RetryButtonState};
pub use code_execution_result::{CodeExecutionResult, ExecutionStatus};
pub use footer::{Footer, FooterBottom, FooterCopyright, FooterLink, FooterLinks, FooterSection};
pub use app_shell::{AppContent, AppHeader, AppShell};
pub use pagination::{Pagination, PaginationSize, SimplePagination};
pub use separator::{Separator, SeparatorOrientation};
pub use label::{FormField, Label, LabelSize};
pub use collapsible::{Collapsible, CollapsiblePanel};
pub use aspect_ratio::{AspectRatio, AspectRatioPreset};
pub use rating::{Rating, RatingDisplay, RatingSize};
pub use stat::{Stat, StatCard, StatGroup, StatSize, StatTrend};
pub use steps::{Step, StepButton, StepStatus, Steps, StepsOrientation, StepsSize};
pub use file_input::{FileInput, FileInputButton, FileInputSize};
pub use hover_card::{HoverCard, HoverCardPosition, ProfileHoverCard};
pub use list::{List, ListItem, ListItemContent, ListSection, ListSize, ListVariant};
pub use descriptions::{DescriptionItem, Descriptions, DescriptionsLayout, DescriptionsSize};
pub use alert_dialog::{AlertDialog, AlertDialogVariant};
pub use scroll_area::{ScrollArea, ScrollDirection, ScrollbarVisibility};
pub use calendar::{Calendar, CalendarSize};
pub use carousel::{Carousel, CarouselNavigation, CarouselSlide};
