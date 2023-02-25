mod renderer;
mod tests;

use renderer::html_parcer;
use renderer::css_parcer;


fn main() {

    let src = String::from("<div id=\"jkl\">
    hello world
</div><h1>hello</h1>
<!-- test -->");
    let nodes = html_parcer::parce(src);
    nodes.display();

    println!("");

    let css_src = String::from("div {
        color: #ffffff;
        width: 265px;
    }");
    let mut styles = css_parcer::parce(css_src);
    styles.display();
}
