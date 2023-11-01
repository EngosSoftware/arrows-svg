use domrs::{HtmlDocument, HtmlElement};
use std::fs;

const PI_2: f64 = std::f64::consts::PI * 2.0;

pub fn generate_html(file_name: &str) {
  let mut svg = create_svg(400.0, 400.0);

  svg.add_child(create_svg_line_with_arrow(200.0, 200.0, 300.0, 300.0));
  svg.add_child(create_svg_line_with_arrow(200.0, 200.0, 300.0, 230.0));
  svg.add_child(create_svg_line_with_arrow(200.0, 200.0, 210.0, 300.0));
  svg.add_child(create_svg_line_with_arrow(200.0, 200.0, 300.0, 100.0));
  svg.add_child(create_svg_line_with_arrow(200.0, 200.0, 300.0, 190.0));
  svg.add_child(create_svg_line_with_arrow(200.0, 200.0, 210.0, 10.0));
  svg.add_child(create_svg_line_with_arrow(200.0, 200.0, 100.0, 300.0));
  svg.add_child(create_svg_line_with_arrow(200.0, 200.0, 100.0, 210.0));
  svg.add_child(create_svg_line_with_arrow(200.0, 200.0, 100.0, 100.0));
  svg.add_child(create_svg_line_with_arrow(200.0, 200.0, 100.0, 10.0));

  let mut body = HtmlElement::new("body");
  body.add_child(svg);

  let doc = HtmlDocument::new("Arrows", "en", &[], body);
  fs::write(file_name, doc.to_string()).expect("writing HTML file failed");
}

fn create_svg(width: f64, height: f64) -> HtmlElement {
  let mut svg = HtmlElement::new("svg");
  svg.set_attr("width", width);
  svg.set_attr("height", height);
  svg
}

fn create_svg_line(x1: f64, y1: f64, x2: f64, y2: f64) -> HtmlElement {
  let mut line = HtmlElement::new("line");
  line.set_attr("x1", x1);
  line.set_attr("y1", y1);
  line.set_attr("x2", x2);
  line.set_attr("y2", y2);
  line.set_attr("stroke", "black");
  line.set_attr("stroke-width", "1");
  line
}

fn create_svg_arrow(x2: f64, y2: f64) -> HtmlElement {
  let mut path = HtmlElement::new("path");
  path.set_attr("d", format!("M {} {} l 10 -3 l 0 6 l -10 -3", x2, y2));
  path.set_attr("fill", "black");
  path.set_attr("stroke-linecap", "round");
  path
}

fn create_svg_line_with_arrow(x1: f64, y1: f64, x2: f64, y2: f64) -> HtmlElement {
  let mut sub_group = HtmlElement::new("g");
  let a = rotation(x1, y1, x2, y2);
  sub_group.set_attr("transform", format!("rotate({} {} {})", a, x2, y2));
  sub_group.add_child(create_svg_arrow(x2, y2));
  let mut group = HtmlElement::new("g");
  group.add_child(create_svg_line(x1, y1, x2, y2));
  group.add_child(sub_group);
  group
}

fn rotation(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
  let x = x2 - x1;
  if x == 0.0 {
    return 0.0;
  }
  let y = y2 - y1;
  let angle = ((y / x).atan() * 360.0) / PI_2;
  if x > 0.0 {
    if y >= 0.0 {
      angle - 180.0
    } else {
      angle + 180.0
    }
  } else {
    angle
  }
}
