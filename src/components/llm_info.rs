use leptos::prelude::*;

use thaw::*;

#[component]
pub fn LLMInfo() -> impl IntoView {
    view! {
        <DialogSurface>
            <DialogBody class="pb-4">
                <DialogTitle>"Supported LLM Services"</DialogTitle>
                <DialogContent>
                    <h2 class="mt-3 text-lg">Perplexity (recommended)</h2>
                    <div class="mb-2">
                        "To create the API token sign up for "
                        <a href="https://www.perplexity.ai/">Perplexity</a> " account and visit "
                        <a href="https://www.perplexity.ai/settings/api">
                            "https://www.perplexity.ai/settings/api"
                        </a>
                    </div>
                    <div class="mb-2">
                        "The minimum amount of money that can be added to the balance is $3.
                        Credits don't expire."
                    </div>
                    <div class="mb-2">"Perplexity option uses llama-3.1-70b-instruct model."</div>


                    <h2 class="mt-3 text-lg">Anthropic</h2>
                    <div class="mb-2">
                        "To create the API token sign up for "
                        <a href="https://console.anthropic.com/">Claude console</a> " account and visit "
                        <a href="https://console.anthropic.com/settings/keys">
                            "https://console.anthropic.com/settings/keys"
                        </a>
                    </div>
                    <div class="mb-2">
                        "The minimum amount of money that can be added to the balance is $5.
                        Credits expire after a year."
                    </div>
                    <div class="mb-2">"Anthropic option uses sonnet-3.5 model."</div>

                    <h2 class="mt-3 text-lg">OpenAI</h2>
                    <span>

                        <div class="mb-2">
                            "To create the API token sign up for "
                            <a href="https://platform.openai.com/signup">OpenAI</a>
                            " account and visit "
                            <a href="https://platform.openai.com/api-keys">
                                "https://platform.openai.com/api-keys"
                            </a>
                        </div>
                        <div class="mb-2">
                            "The minimum amount of money that can be added to the balance is $5. Credits expire after 1 year"
                        </div>
                        <div class="my-b">"OpenAI option uses gpt-4o model."</div>

                    </span>


                </DialogContent>
            </DialogBody>
        </DialogSurface>
    }
}
