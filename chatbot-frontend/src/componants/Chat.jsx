function Chat (props) {
    let user_cls = "max-w-md p-4 mb-5 rounded-lg self-end bg-blue-500 text-white";
    let ai_cls = "max-w-md p-4 mb-5 rounded-lg self-start bg-gray-200 text-black";
    return (
        <div class="h-screen pb-24 w-full flex flex-col overflow-y-auto border border-gray-300 rounded p-5 bg-gray-100">
            <For each={props.getChat()} fallback={<div>Loading...</div>}>
                {(item) => <div class={item.user? user_cls : ai_cls}>{item.text}</div>}
            </For>
            <Show when={props.Loading() == true}>
                <div class={ai_cls}>Loading...</div>
            </Show>
        </div>
    )
}

export default Chat;