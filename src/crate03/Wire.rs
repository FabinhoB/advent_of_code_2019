use crate::crate03::point::Point;

pub struct Wire {
    points: Vec<Point>,
}

impl Wire {
    pub fn new(directions: &Vec<String>) -> Wire {
        let mut points: Vec<Point> = Vec::with_capacity(0);
        points.push(Point::new(0,0));
        for direction_with_length in directions {
            let direction = direction_with_length.chars().nth(0).unwrap();
            let length = direction_with_length.replace(direction, "").parse::<i32>().unwrap();
            match direction {
                'U' => Wire::add_points(&mut points, length, 0, 1),
                'D' => Wire::add_points(&mut points, length, 0, -1),
                'L' => Wire::add_points(&mut points, length, -1, 0),
                'R' => Wire::add_points(&mut points, length, 1, 0),
                _ => println!("Error !"),
            }
        }
        Wire{points}
    }

    fn add_points(points:&mut Vec<Point>, length: i32, x_increment: i32, y_increment: i32) {
        let p = points[points.len() - 1];
        for i in 1..length+1 {
            points.push(Point::new(p.get_x()+ i * x_increment, p.get_y() + i * y_increment));
        }
    }

    pub fn get_points(&self) -> &Vec<Point> {
        &(self.points)
    }
}