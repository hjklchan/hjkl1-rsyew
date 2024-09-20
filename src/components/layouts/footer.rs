use yew::{function_component, html, Html};

#[function_component(Footer)]
pub fn footer() -> Html {
    let datetime = "2024/09/20 10:24 AM";
    
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
                </div>
                <div class="flex-none w-32 text-xs text-gray-600">
                    {datetime}
                </div>
            </div>
        </footer>
    )
}
