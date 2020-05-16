
use typed_html::dom::DOMTree;
use typed_html::elements::FlowContent;
use typed_html::{html, text};

pub fn main(title: String, content: Box<dyn FlowContent<String>>) -> String {
    let menu: Box<dyn FlowContent<String>> = html!(
        <nav class="navbar navbar-expand-lg navbar-light bg-light">
            <a class="navbar-brand" href="/">
                <span class="b-logo">
                    <img src="/public/img/logo.svg" class="b-logo__img" width="50" alt="Rust logo" />
                    <span class="b-logo__text">"Rusflix"</span>
                </span>
            </a>
            <button
                class="navbar-toggler"
                type="button"
                data-toggle="collapse"
                data-target="#navbarSupportedContent"
            >
                <span class="navbar-toggler-icon"></span>
            </button>
            <div class="collapse navbar-collapse" id="navbarSuportedContent">
                <ul class="navbar-nav mr-auto">
                    <li class="nav-item"><a class="nav-link" href="/">"Home"</a></li>
                    <li class="nav-item"><a class="nav-link" href="/about">"About"</a></li>
                </ul>
            </div>
        </nav>);
    let page: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("{}", title)}</title>
                <link rel="stylesheet"
                    href="https://stackpath.bootstrapcdn.com/bootstrap/4.5.0/css/bootstrap.min.css" 
                    crossorigin="anonymous" 
                />
                <link rel="stylesheet" href="/public/style.css" />
            </head>
            <body>
                <header>
                    { menu }
                </header>
                { content }
            </body>
        </html>
    );
    page.to_string()
}