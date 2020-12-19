
use opencv::{
    prelude::{
        Mat,
        MatTrait
    }
};

use std::os::raw::c_uchar;

#[derive(Copy, Clone)]
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

    pub fn get_points(&self) -> Vec<Point> {
        self.points.clone()
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

        let mut idx: usize = 0;

        for point in self.points.clone().iter() {

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

            let np = self.calculate_new_pos(sobel_image, idx, start, end);

            self.points[idx] = np;

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

    fn calculate_new_pos(&mut self, img: &mut Mat, idx: usize, start: Point, end: Point ) -> Point {

        let cols = end.x - start.x;
        let rows = end.y - start.y;

        let mut location = self.points[idx].clone();

        let mut flag = true;
        let mut local_min = 1000.00;

        let average_distance = self.average_distance();

        let mut ngmax: i32 = 0;
        let mut ngmin: i32 = 0;

        for y in 0..rows {
            for x in 0..cols {

                let cval = *img.at_2d::<c_uchar>(y, x).unwrap() as i32;

                if flag {
                    ngmax = cval;
                    ngmin = cval;
                    flag = false;
                } else if cval > ngmax {
                    ngmax = cval;
                } else if cval < ngmin {
                    ngmin = cval;
                }
            }
        }

        if ngmax == 0 {
            ngmax = 1;
        }
        if ngmin == 0 {
            ngmin = 1;
        }

        flag = true;

        for y in 0..rows-1 {
            for x in 0..cols-1 {

                // E = ∫(α(s)Econt + β(s)Ecurv + γ(s)Eimage)ds

                let mut parent_x = x + start.x;
                let mut parent_y = y + start.y;

                let height = img.rows();
                let width  = img.cols();

                if parent_x >= width {
                    parent_x = width-1;
                }

                if parent_y >= height {
                    parent_y = height-1;
                }

                // Econt
                // (δ- (x[i] - x[i-1]) + (y[i] - y[i-1]))^2
                // δ = avg dist between snake points

                let previous_point : Point;

                if idx == 0 {
                    previous_point = self.points[idx].clone();
                } else {
                    previous_point = self.points[idx-1].clone();
                }

                // Econt
                let mut econt = ( (parent_x - previous_point.x) + (parent_y - previous_point.y) )as f32;
                econt = (econt as i64).pow(2) as f32;
                econt = average_distance - econt;
                econt = econt * self.alpha;

                // Ecurv
                // (x[i-1] - 2x[i] + x[i+1])^2 + (y[i-1] - 2y[i] + y[i+1])^2

                let next_point : Point;
                if idx == self.points.len()-1 {
                    next_point = self.points[0].clone();
                } else {
                    next_point = self.points[idx+1].clone();
                }

                let mut ecurv = (previous_point.x - (parent_x * 2) + next_point.x).pow(2) as f32;
                ecurv = ecurv + (previous_point.y - (parent_y * 2) + next_point.y).pow(2) as f32;
                ecurv = ecurv * self.beta as f32;

                // Eimage
                // -||∇||

                let mut eimg = *img.at_2d::<c_uchar>(parent_y, parent_x).unwrap() as i32;
                eimg = eimg * self.gamma;

                // Normalize

                econt = econt / ngmax as f32;
                ecurv = ecurv / ngmax as f32;

                let mut divisor = ngmax - ngmin;
                if divisor <= 0 {
                    divisor = 1;
                }

                eimg = (eimg - ngmin) / divisor;

                // Energy = ∫(α(s)Econt + β(s)Ecurv + γ(s)Eimage)ds
                let energy = econt + ecurv + eimg as f32;

                if flag {
                    flag = false;
                    local_min = energy;
                    location  = Point{ x: parent_x, y: parent_y};
                } else if energy < local_min {
                    local_min = energy;
                    location  = Point{ x: parent_x, y: parent_y};
                }
            }
        }

        return location;
    }
}