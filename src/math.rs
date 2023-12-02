use std::ops::{Mul, Add, AddAssign};
#[derive(Debug, Copy, Clone)]
pub struct Point{
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug)]
pub struct Vector{
    pub x: f32,
    pub y: f32,
}

impl Vector{
    pub fn get_magnitude(&self) -> f32{
        return (self.x.powf(2.0) + self.y.powf(2.0)).sqrt();
    }
    pub fn get_unit_vector(&self) -> Vector{
        let magnitude = self.get_magnitude();
        return Vector{x:self.x/magnitude, y:self.y/magnitude};
    }

    pub fn tan(&self) -> f32{
        return (self.y/self.x).atan();
    }

    pub fn cross(&self, other: &Vector) -> f32{
        return self.x * other.y - self.y * other.x;
    }

    pub fn rotate(&self, angle: f32) -> Vector{
        let x = self.x;
        let y = self.y;
        return Vector{x:x*angle.cos() - y*angle.sin(), y:x*angle.sin() + y*angle.cos()};
    }

    pub fn dot(&self, other: &Vector) -> f32{
        return self.x * other.x + self.y * other.y;
    }

    // Yelds the smallest angle between the two vectors
    pub fn smallest_angle(&self, other: &Vector) -> f32{
        return (self.dot(&other)/(self.get_magnitude() * other.get_magnitude())).acos();
    }


    pub fn angle(&self, other: &Vector) -> f32{
        let dot_product = self.dot(other);
        let cross_product = self.cross(other);

        return -cross_product.atan2(dot_product);
    }

}
impl<'a> Mul<f32> for &'a Vector {
    type Output = Vector;

    fn mul(self, f: f32) -> Vector {
        Vector {
            x: self.x * f,
            y: self.y * f
        }
    }
}

impl Add<& Vector> for Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign<&Vector> for Vector {
    fn add_assign(&mut self, v: &Vector){
        self.x += v.x;
        self.y += v.y;
    }
}


impl Point{
    pub fn new((x,y): (f32, f32)) -> Point{
        Point{
            x: x,
            y: y,
        }
    }

    pub fn distance(&self, other: &Point) -> f32{
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
    
        return (x_diff.powf(2.0) + y_diff.powf(2.0)).sqrt();
    }
    pub fn get_unit_vector(&self, other: &Point) -> Vector{
        let x_diff = other.x - self.x;
        let y_diff = other.y - self.y;

        let magnitude = (x_diff.powf(2.0) + y_diff.powf(2.0)).sqrt();
        return Vector{x:x_diff/magnitude, y:y_diff/magnitude};
    }
    pub fn get_middle_point(&self, b: &Point) -> Point{
        let x = (self.x + b.x)/2.0;
        let y = (self.y + b.y)/2.0;
        return Point{x,y};
    }
    pub fn set_bazier_next_point(&mut self, a: &Point,b: &Point, c: &Point, t: f32){
        //https://math.stackexchange.com/questions/1360891/find-quadratic-bezier-curve-equation-based-on-its-control-points
        let x=( a.x - 2.0*b.x + c.x)*t.powf(2.0) + 2.0*(b.x-a.x)*t+a.x;
        let y=( a.y - 2.0*b.y + c.y)*t.powf(2.0) + 2.0*(b.y-a.y)*t+a.y;
        
        self.x = x;
        self.y = y;
    }
}

impl Add<&Vector> for Point {
    type Output = Point;

    fn add(self, p: &Vector) -> Point {
        Point {
            x: self.x + p.x,
            y: self.y + p.y
        }
    }
}

impl AddAssign<&Vector> for Point {
    fn add_assign(&mut self, v: &Vector){
        self.x += v.x;
        self.y += v.y;
    }
}

pub fn convert_bytes_str(bytes: u64) -> String{
    let mut bytes = bytes as f32;
    let power = bytes.log(1024.0).floor() as u32;
    bytes /= 1024.0_f32.powi(power as i32);

    match power{
        0 => return format!("{:.0} B", bytes),
        1 => return format!("{:.2} KB", bytes),
        2 => return format!("{:.2} MB", bytes),
        3 => return format!("{:.2} GB", bytes),
        4 => return format!("{:.2} TB", bytes),
        5 => return format!("{:.2} PB", bytes),
        6 => return format!("{:.2} EB", bytes),
        7 => return format!("{:.2} ZB", bytes),
        _ => return format!("{:.2} YB", bytes),
    }
}

