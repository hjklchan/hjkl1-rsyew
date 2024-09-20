use crate::components::category2::Category2;
use crate::router::Router;
use yew::{function_component, html, Html};
use yew_router::prelude::Link;

#[function_component(Posts)]
pub fn posts() -> Html {
    html! {
        <>
            // Category
            <Category2 items={None} />
            // Posts - Header
            <div class="mt-6">
                <table
                    class="w-full table-fixed"
                    cellSpacing={0}
                    cellPadding={0}
                >
                    <tbody>
                        <tr class="border-b border-[#hC2D5E3] bg-[#F2F2F2] border-[#C2D5E3] text-xs">
                            <td
                                colSpan={2}
                                class="text-left pl-2 py-3 space-x-3"
                            >
                                <div class="inline-block space-x-1">
                                    <input
                                        type="checkbox"
                                        // TODO
                                        // defaultChecked
                                        // onChange={onNewTab}
                                    />
                                    <label>{"New Tab"}</label>
                                </div>
                                <div class="inline-block space-x-1">
                                    <input
                                        type="checkbox"
                                        // onChange={onShowTop}
                                    />
                                    <label>{"Show Top"}</label>
                                </div>
                                <button class="hover:cursor-pointer text-[#369]">
                                    {"All"}
                                </button>
                                <button class="hover:cursor-pointer text-[#369]">
                                    {"Newest"}
                                </button>
                                <button class="hover:cursor-pointer text-[#369]">
                                    {"Popular"}
                                </button>
                            </td>
                            // TODO Hide by special device
                            <td class="w-28">{"Author"}</td>
                            <td class="w-24">
                                <div class="flex space-x-1">
                                    // TODO
                                    {"c"}
                                    // <HiOutlineChatBubbleLeftEllipsis />
                                    <span>{"/"}</span>
                                    // TODO
                                    {"v"}
                                    // <HiOutlineEye />
                                </div>
                            </td>
                            <td class="w-28">{"Last Updated"}</td>
                        </tr>
                    </tbody>
                </table>
                // Posts - List
                <table
                    class="table-fixed w-full text-sm text-[#334]"
                    cellspacing={0}
                    cellpadding={0}
                >
                    // Using local css
                    <tbody class={""}>
                        <tr class="table-row align-middle hover:bg-[#F2F2F2] border-b border-[#C2D5E3]">
                            <td class="w-6 py-1">
                                // TODO
                                {"???"}
                                // <HiOutlineChatBubbleBottomCenterText className="w-full block" />
                            </td>
                            <td>
                                // Link component
                                <span class="hover:cursor-pointer text-[#369] pl-1 pr-2">
                                    {"[Laravel]"}
                                </span>
                                <Link<Router>
                                    to={Router::PostDetail { id: "1".to_string() }}
                                    classes="text-[#333] hover:cursor-pointer hover:border-b border-[#333]"
                                    // TODO
                                    // {...newTabProps}
                                >
                                    {"最新DNF万能输入法注入器开源了（带易语言源码,带外挂源码）解决部分内存错误"}
                                </Link<Router>>
                            </td>
                            <td class="w-28">
                                <cite>
                                    <button
                                        class="hover:cursor-pointer text-[#369]"
                                        title="(TODO)"
                                    >
                                        {"(TODO)"}
                                    </button>
                                </cite>
                            </td>
                            <td class="w-24">{"1.2k / 5k"}</td>
                            <td class="w-28">{"N/A"}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </>
    }
}
