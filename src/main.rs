enum Shape {
    Circle(usize, usize, usize, DrawAPI),
}

impl Shape {
    fn draw(&self) {
	match self {
	    Shape::Circle(radius, x, y, draw_api) => draw_api.draw_circle(*radius, *x, *y),
	}
    }
}

enum DrawAPI {
    GreenCircle,
    RedCircle,
}

impl DrawAPI {
    fn draw_circle(&self, radius: usize, x: usize, y: usize) {
	match &self {
	    DrawAPI::GreenCircle => self.draw_green_circle(radius, x, y),
	    DrawAPI::RedCircle => self.draw_red_circle(radius, x, y),
	}
    }
    fn draw_green_circle(&self, radius: usize, x: usize, y: usize) {
	println!("Drawing Circle [ colour: green, radius: {}, x: {}, y: {} ]", radius, x, y);
    }
    fn draw_red_circle(&self, radius: usize, x: usize, y: usize) {
	println!("Drawing Circle [ colour: red, radius: {}, x: {}, y: {} ]", radius, x, y);
    }
}
fn main() {
    let red_circle = Shape::Circle(100, 100, 10, DrawAPI::RedCircle);
    let green_circle = Shape::Circle(100, 200, 10, DrawAPI::GreenCircle);

    red_circle.draw();
    green_circle.draw();
}
