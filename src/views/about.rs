use typed_html::elements::FlowContent;
use typed_html::html;

pub fn about() -> Box<dyn FlowContent<String>> {
    html!(
        <div class="container">
            <h1>"About Rusflix"</h1>
            <p>
                "Rusflix is a streaming service."
            </p>
        </div>
    )
}