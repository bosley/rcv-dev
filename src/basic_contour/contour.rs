
use opencv::{
    prelude::{
        Mat,
        MatTrait
    },
    imgproc,
    core
};


pub struct Point {
    pub x: i32,
    pub y: i32
}

enum State {
    AcceptingInput,
    Executing
}

pub struct  Contour {

    points: Vec<Point>,
    state:  State,
    alpha:  f32,
    beta:   i32,
    gamma:  i32,
}

impl Contour {

    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            state:  State::AcceptingInput,
            alpha:  0.2,
            beta:   1,
            gamma:  100
        }
    }

    pub fn get_params(&self) -> (i32, i32, i32) {
        ((self.alpha * 100.00) as i32, self.beta, self.gamma)
    }

    pub fn update_alpha(&mut self, alpha: i32) {
        self.alpha = alpha as f32 / 100.00;
    }

    pub fn update_beta(&mut self, beta: i32) {
        self.beta = beta;
    }

    pub fn update_gamma(&mut self, gamma: i32) {
        self.gamma = gamma;
    }

    pub fn start(&mut self) {
        match self.state {
            State::AcceptingInput => {
                self.state = State::Executing;
            },
            State::Executing => {
                return;
            }
        }
    }

    pub fn reset(&mut self) {
        self.alpha = 0.2;
        self.beta  = 1;
        self.gamma = 100;
        self.points.clear();
        self.state = State::AcceptingInput;
    }

    pub fn add_point(&mut self, x: i32, y: i32)  {

        match self.state {
            State::Executing => {
                return;
            }
            _ => {}
        }

        self.points.push(Point{ x: x, y: y});
    }

    pub fn step(&mut self, sobel_image: &mut Mat) {

        match self.state {
            State::AcceptingInput => {
                return;
            }
            _ => {}
        }
        
        let height = sobel_image.rows();
        let width  = sobel_image.cols();

        let mut idx: u64 = 0;
        for point in self.points.iter_mut() {

            let mut start = Point{ x: 0, y: 0 };
            let mut end   = Point{ x: 0, y: 0 };

            if point.x - 3 > 0 {
                start.x = point.x - 3;
            }

            if point.x + 4 > width {
                end.x = width;
            } else {
                end.x = point.x + 4;
            }

            if point.y - 3 > 0 {
                start.y = point.y - 3;
            }

            if point.y + 4 > height {
                end.y = height;
            } else {
                end.y = point.y + 4;
            }

            let np = self.calculate_new_pos(idx, start, end);

            point.x = np.x; point.y = np.y;

            idx = idx + 1;
        }

    }

    fn average_distance(&mut self) -> f32 {

        let mut sum = 0.00;

        for idx in 0..self.points.len()-1 {
            sum = sum + ((
            ( (self.points[idx].x - self.points[idx + 1].x) *  (self.points[idx].x - self.points[idx + 1].x) ) + 
            ( (self.points[idx].y - self.points[idx + 1].y) *  (self.points[idx].y - self.points[idx + 1].y) )) as f32 ).sqrt();
        }

        sum = sum + ((
        ( (self.points[self.points.len()-1].x - self.points[0].x) *  (self.points[self.points.len()-1].x - self.points[0].x) ) + 
        ( (self.points[self.points.len()-1].y - self.points[0].y) *  (self.points[self.points.len()-1].y - self.points[0].y) )) as f32 ).sqrt();

        return sum / self.points.len() as f32;
    }

    fn calculate_new_pos(&self, idx: u64, start: Point, end: Point ) -> Point {


        Point{ x: 0, y: 0}
    }
}