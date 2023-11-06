use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;

use crate::routes::Route;

#[derive(Properties, PartialEq, Clone)]
pub struct SidebarItemProps {
    pub text: String,
    pub icon: IconId,
    pub path: Route,
    #[prop_or_default]
    pub sub_text: Option<String>,
    #[prop_or_default]
    pub number: Option<u32>,
}

#[function_component]
fn SidebarItem(props: &SidebarItemProps) -> Html {
    let SidebarItemProps {
        text,
        sub_text,
        icon,
        path,
        number,
    } = props.clone();

    html! {
        <li>
            <Link<Route> to={path} classes="flex items-center p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700 group">
                <Icon icon_id={icon}/>
                <span class="flex-1 ml-3 whitespace-nowrap">{text}</span>
                if let Some(s) = sub_text {
                    <span class="inline-flex items-center justify-center px-2 ml-3 text-sm font-medium text-gray-800 bg-gray-100 rounded-full dark:bg-gray-700 dark:text-gray-300">{s}</span>
                }
                if let Some(n) = number {
                    <span class="inline-flex items-center justify-center w-3 h-3 p-3 ml-3 text-sm font-medium text-blue-800 bg-blue-100 rounded-full dark:bg-blue-900 dark:text-blue-300">{n}</span>
                }
            </Link<Route>>
        </li>
    }
}

#[function_component]
fn Sidebar() -> Html {
    // https://flowbite.com/docs/components/sidebar/
    html! {
        <>
            <button data-drawer-target="default-sidebar" data-drawer-toggle="default-sidebar" aria-controls="default-sidebar" type="button" class="inline-flex items-center p-2 mt-2 ml-3 text-sm text-gray-500 rounded-lg sm:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600">
                <span class="sr-only">{"Open sidebar"}</span>
                <svg class="w-6 h-6" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                    <path clip-rule="evenodd" fill-rule="evenodd" d="M2 4.75A.75.75 0 012.75 4h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 4.75zm0 10.5a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5h-7.5a.75.75 0 01-.75-.75zM2 10a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 10z"></path>
                </svg>
            </button>

            <aside id="default-sidebar" class="fixed top-0 left-0 z-40 w-64 h-screen transition-transform -translate-x-full sm:translate-x-0" aria-label="Sidebar">
                <div class="h-full px-3 py-4 overflow-y-auto bg-gray-50 dark:bg-gray-800">
                    <ul class="space-y-2 font-medium">
                        <SidebarItem text={"Dashboard"} icon={IconId::LucideLayoutDashboard} path={Route::Dashboard} />
                        <SidebarItem text={"X(Twitter)"} icon={IconId::BootstrapTwitter} path={Route::X} />
                    </ul>
                </div>
            </aside>
        </>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct MainLayoutProps {
    pub children: Children,
}

#[function_component]
pub fn MainLayout(props: &MainLayoutProps) -> Html {
    let MainLayoutProps { children } = props.clone();

    html! {
        <div>
            <Sidebar />
            <div class="p-4 sm:ml-64">
                <main>{children}</main>
            </div>
        </div>
    }
}
