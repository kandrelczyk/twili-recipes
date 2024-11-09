use leptos::prelude::*;

use thaw::*;

#[component]
pub fn LLMInfo() -> impl IntoView {
    view! {
        <DialogSurface class="my-9 mx-2">
            <DialogBody class="pb-4">
                <DialogTitle>"Supported LLM Services"</DialogTitle>
                <DialogContent>

                    <h2 class="mt-2 text-lg">Free</h2>
                    <span>
                        "Free option uses rate limited OpenAI API. The token is shared between all users so it may not work if the limit is reached. "
                    </span>

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

                    <h2 class="mt-3 text-lg">Perplexity</h2>
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

                </DialogContent>
            </DialogBody>
        </DialogSurface>
    }
}
