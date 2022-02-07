mod text_input;

use crate::text_input::TextInput;
use yew::prelude::*;

enum Msg {
    SetDec(String),
}

struct Model {
    bin_value: String,
    dec_value: u32,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            bin_value: String::new(),
            dec_value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetDec(value) => {
                match u32::from_str_radix(value.trim(), 2) {
                    Ok(result) => {
                        self.dec_value = result;
                        self.bin_value = value;
                    }
                    Err(_) => self.bin_value = self.bin_value.clone(),
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = ctx.link().callback(Msg::SetDec);
        let value = self.bin_value.clone();
        html! {
            <div>
                <div class="w3-container w3-blue">
                    <h1>{"bin2dec"}</h1>
                    <p>{"WebAssembly - Rust"}</p>
                </div>
                <div class="w3-container">
                    <div class="w3-small w3-text-gray">{"\u{00a9}2022 sumomo-99"}</div>
                    <div>
                        <a href="https://github.com/sumomo-99"><i class="w3-xlarge w3-text-gray w3-margin-right fab fa-github"></i></a>
                        <a href="https://twitter.com/sumomo_99"><i class="w3-xlarge w3-text-gray fab fa-twitter"></i></a>
                    </div>
                    <p>{"2進数を10進数に変換します。"}</p>
                </div>
                <div class="w3-container">
                    <form class="w3-margin-top">
                        <label class="w3-text-blue">{"Binary Number"}</label>
                        <TextInput
                            class="w3-input w3-border" {on_change} {value} />
                    </form>

                    <div class="w3-margin-top">
                        <label class="w3-text-blue">{"Decimal Number"}</label>
                        <div class="w3-jumbo">{ self.dec_value }</div>
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
