import { createEffect, createSignal } from 'solid-js';
import styles from './App.module.css';
import Chat from './componants/Chat';
import Prompt from './componants/Prompt';

function App() {
  const[getChat, setChat] = createSignal([{"text":"What is your name ?", "user": true}, {"text":"My name is AI", "user": false}]);
  const [Loading, setLoading] = createSignal(false);
  return (
    <div class={styles.App}>
    <h1 class="text-3xl font-bold underline">
      AI ChatBot
    </h1>
    <Chat getChat={getChat} Loading={Loading}></Chat>
    <Prompt setChat={setChat} getChat={getChat} setLoading={setLoading}></Prompt>
    </div>
  );
}

export default App;
