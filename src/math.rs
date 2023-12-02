pub struct Point{
    pub x: f32,
    pub y: f32,
}

impl Point{
    pub fn distance(&self, other: &Point) -> f32{
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
    
        return (x_diff.powf(2.0) + y_diff.powf(2.0)).sqrt();
    }
    pub fn get_unit_vector(&self, other: &Point) -> (f32,f32){
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;

        let magnitude = (x_diff.powf(2.0) + y_diff.powf(2.0)).sqrt();
        return (x_diff/magnitude, y_diff/magnitude);
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

pub fn get_circle_point(center: &Point, angle_rad: f32, radius: f32) -> Point{
    let x = center.x + radius * angle_rad.cos();
    let y = center.y + radius * angle_rad.sin();
    return Point{x,y};
}