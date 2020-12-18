
struct Point {
    x: i32,
    y: i32
}

enum State {
    AcceptingInput,
    Executing
}

pub struct  Contour <'a> {

    points: Vec<Point>,
    state:  State,
    alpha:  f32,
    beta:   u16,
    gamma:  u16,
    window_name: &'a str
}

impl <'a> Contour <'a> {

    //! Create a new contour - The window should be made prior to instantiation 
    //! as it will modify the window to add sliders for the ABG parameters
    pub fn new(window_name: &'a str) -> Self {
        Self {
            points: Vec::new(),
            state:  State::AcceptingInput,
            alpha:  0.2,
            beta:   1,
            gamma:  100,
            window_name: window_name
        }
    }

    pub fn start(&mut self) -> bool {
        match self.state {
            State::AcceptingInput => {
                self.state = State::Executing;
            },
            State::Executing => {
                return false;
            }
        }
        true
    }

    pub fn add_point(&mut self, x: i32, y: i32) -> bool {

        match self.state {
            State::Executing => {
                return false;
            }
            _ => {}
        }

        self.points.push(Point{ x: x, y: y});

        return true;
    }

    pub fn step(&mut self) {

        // Block until ready
        // Step each thing minimizing the snakaroni

    }
}