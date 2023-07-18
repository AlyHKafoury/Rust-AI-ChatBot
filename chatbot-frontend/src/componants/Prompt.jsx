import { createSignal } from "solid-js";

const disabled_class =
  "h-full p-4 bg-gray-500 text-white rounded-full cursor-pointer";
const button_class =
  "h-full p-4 bg-orange-500 text-white rounded-full cursor-pointer";
const url = "http://localhost:5000/api/chat";

function Prompt(props) {
  const [getPrompt, setPrompt] = createSignal("");
  const [getDisabled, setDisabled] = createSignal(false);
  const handle_submit = () => {
    props.setChat((c) => {
      c.push({
        user: true,
        text: getPrompt(),
      });
      setPrompt("");
      props.setLoading(true);
      console.log(c);
      return [...c];
    });
    setDisabled(true);
    const message = JSON.stringify({ messages: props.getChat() });
    console.log(message);
    fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: message,
    })
      .then((res) => res.json())
      .then((data) => {
        console.log(data);
        props.setLoading(false);
        setDisabled(false);
        props.setChat((c) => {
          c.push({
            user: false,
            text: data.result,
          });
          return [...c];
        });
      });
  };

  return (
    <div class="h-24 w-full fixed bottom-0 flex justify-center items-center p-5 bg-white border-t border-gray-300">
      <form
        class="w-full flex justify-center items-center gap-4"
        onsubmit={(e) => {
          e.preventDefault();
          if (getPrompt() == "") {
            return;
          }
          handle_submit();
        }}
      >
        <input
          class="w-2/3 p-4 border border-gray-300 rounded-full"
          type="text"
          placeholder="Enter your prompt"
          value={getPrompt()}
          onChange={(event) => setPrompt(event.target.value)}
        />
        <input
          class={getDisabled() ? disabled_class : button_class}
          type="submit"
          disabled={getDisabled()}
        />
      </form>
    </div>
  );
}

export default Prompt;
