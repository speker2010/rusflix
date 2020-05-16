use typed_html::elements::FlowContent;
use typed_html::html;

pub fn index() -> Box<dyn FlowContent<String>> {
    html!(
        <div class="container">
            <h1>"Welcome to Rusflix"</h1>
        </div>
    )
}