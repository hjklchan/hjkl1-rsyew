use std::time::SystemTime;

use yew::{function_component, html, use_context, use_effect, use_state, Html};

use crate::app_ctx::AppContext;

#[function_component(Footer)]
pub fn footer() -> Html {
    let app_ctx = use_context::<AppContext>().unwrap();

    let current_date_time = use_state(|| "2024/09/26 11:33 PM");

    html!(
        <footer class="fixed inset-x-0 bottom-0 border-t-2 border-slate-100">
            <div class="flex justify-between items-center px-4 h-8 bg-gray-100 border-b-1 shadow-md bg-gradient-to-b from-gray-50 to-gray-300">
                <div class="flex-none w-14 font-bold text-xs">
                    <div class="inline">
                        <a href="#">{"CwcOS"}</a>
                    </div>
                </div>
                <div class="grow text-xs">
                    <span class="text-xs border border-[#333] px-1 bg-[#E5EDD5]">
                        {"Usage:"}{0}{"MB"}
                    </span>
                    <div class="inline-block ml-2 text-xs text-red-700">
                        if let Some(status_server) = app_ctx.server_status {
                            if !status_server {
                                {"☠ 服务器挂了，部分数据均为假数据"}
                            }
                        } else {
                            {"🔃 检查服务器是否可用..."}
                        }
                    </div>
                </div>
                <div class="flex-none w-32 text-xs text-gray-600">
                    {current_date_time.to_string()}
                </div>
            </div>
        </footer>
    )
}

// fn format_current_time() -> String {
//     let now = SystemTime::now();
//     let datetime: chrono::DateTime<chrono::Utc> = now.into();
//     datetime.format("%Y-%m-%d %H:%M:%S").to_string()
// }
