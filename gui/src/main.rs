use gui::Button;
use gui::Draw;
use gui::Screen;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to draw a SelectBox
        println!("this would draw a SelectBox");
        println!("the width: {}", self.width);
    }
}

fn main() {
    let my_screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 100,
                height: 32,
                options: vec![String::from("none")],
            }),
            Box::new(Button {
                width: 32,
                height: 32,
                label: String::from("A button"),
            }),
        ],
    };

    my_screen.run();
}
