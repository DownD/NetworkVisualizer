use std::ops::{Mul, Add, Sub, AddAssign};
#[derive(Debug)]
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

pub fn get_circle_point(center: &Point, angle_rad: f32, radius: f32) -> Point{
    let x = center.x + radius * angle_rad.cos();
    let y = center.y + radius * angle_rad.sin();
    return Point{x,y};
}

pub fn get_gravity_force(a: &Point, b: &Point, m1: f32, m2: f32, g:f32) -> Vector{
    let force = g * m1 * m2 / (a.distance(b).powf(2.0));
    let unit_vector = a.get_unit_vector(b);
    return &unit_vector * force;
}