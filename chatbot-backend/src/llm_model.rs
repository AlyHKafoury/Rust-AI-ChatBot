use super::model::Conversation;
use llm::models::Llama;
use llm::KnownModel;
use rand;
use std::convert::Infallible;
use std::path::PathBuf;

pub fn language_model() -> Llama {
    let model_path = "../wizardlm-7b-v1.0-uncensored.ggmlv3.q6_K.bin";

    llm::load::<Llama>(
        &PathBuf::from(&model_path),
        llm::TokenizerSource::Embedded,
        Default::default(),
        llm::load_progress_callback_stdout,
    )
    .unwrap_or_else(|err| panic!("Failed to llm : {err}"))
}

pub fn gen_conversation(mdl: &Llama, chat: Conversation) -> String {
    let ai_name = "<# AI #>";
    let human_name = "<# Human #>";
    let persona = "A chat between a human and an AI model";

    let mut history = String::new();
    let mut res = String::new();
    let mut rng = rand::thread_rng();
    let mut buf = String::new();

    for message in chat.messages.into_iter() {
        let msg = message.text;
        let curr_line = if message.user {
            format!("{human_name}:{msg}\n")
        } else {
            format!("{ai_name}:{msg}\n")
        };

        history.push_str(&curr_line);
    }

    let mut session = mdl.start_session(Default::default());
    session
        .infer(
            mdl,
            &mut rng,
            &llm::InferenceRequest {
                prompt: format!("{persona}\n{history}\n{ai_name}:").as_str().into(),
                parameters: &llm::InferenceParameters::default(),
                play_back_previous_tokens: false,
                maximum_token_count: None,
            },
            &mut Default::default(),
            inference_callback(human_name.to_string(), &mut buf, &mut res),
        )
        .unwrap_or_else(|e| panic!("{e}"));
    res
}

fn inference_callback<'a>(
    stop_sequence: String,
    buf: &'a mut String,
    out_str: &'a mut String,
) -> impl FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, Infallible> + 'a {
    use llm::InferenceFeedback::Continue;
    use llm::InferenceFeedback::Halt;

    move |resp| match resp {
        llm::InferenceResponse::InferredToken(t) => {
            let mut reverse_buf = buf.clone();
            reverse_buf.push_str(t.as_str());
            if stop_sequence.as_str().eq(reverse_buf.as_str()) {
                buf.clear();
                return Ok::<llm::InferenceFeedback, Infallible>(Halt);
            } else if stop_sequence.as_str().starts_with(reverse_buf.as_str()) {
                buf.push_str(t.as_str());
                return Ok(Continue);
            }

            if buf.is_empty() {
                out_str.push_str(&t);
            } else {
                out_str.push_str(&reverse_buf);
            }

            Ok(Continue)
        }
        llm::InferenceResponse::EotToken => Ok(Halt),
        _ => Ok(Continue),
    }
}
