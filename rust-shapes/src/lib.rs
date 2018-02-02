
extern crate num;

#[derive(Clone, Copy, Debug, PartialEq)]
struct CartesianPoint {
    x: f64,
    y: f64
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct PolarPoint {
    phi: f64,
    r: f64
}


trait Point: std::fmt::Debug { // : Clone + Copy + std::fmt::Debug + PartialEq
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn phi(&self) -> f64;
    fn r(&self) -> f64;
}

impl Point for CartesianPoint {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn phi(&self) -> f64 {
        if self.x == num::traits::Zero::zero() {return 0.0f64;}
        (self.y/self.x).atan()
    }

    fn r(&self) -> f64 {
        (self.x.powf(2.0)+self.y.powf(2.0)).sqrt()
    }
}

impl Point for PolarPoint {
    fn x(&self) -> f64 {
        self.r * self.phi.cos()
    }

    fn y(&self) -> f64 {
        self.r * self.phi.sin()
    }

    fn phi(&self) -> f64 {
        self.phi
    }

    fn r(&self) -> f64 {
        self.r
    }
}

// https://users.rust-lang.org/t/assert-eq-for-float-numbers/7034/4
pub fn nearly_equal<T: num::Float>(a: T, b: T) -> bool {
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = (a - b).abs();

    if a == b { // Handle infinities.
        true
    } else if a == num::traits::Zero::zero() || b == num::traits::Zero::zero() || diff < T::min_positive_value() {
        // One of a or b is zero (or both are extremely close to it,) use absolute error.
        diff < (T::epsilon() * T::min_positive_value())
    } else { // Use relative error.
        (diff / T::min(abs_a + abs_b, T::max_value())) < T::epsilon()
    }
}

#[test]
fn test_point() {
    let cartesian_point = Box::new(CartesianPoint {x: 1.0, y: 1.0});

    assert_eq!(cartesian_point.x(), 1.0f64);
    assert_eq!(cartesian_point.y(), 1.0f64);
    assert!(nearly_equal(cartesian_point.phi(), 45.0f64/180.0f64*std::f64::consts::PI));

    let polar_point = PolarPoint { phi: 30.0/180.0*std::f64::consts::PI, r: 1.0};
    assert!(nearly_equal(polar_point.x(), 0.5 * 3.0f64.sqrt()));
    assert!(nearly_equal(polar_point.y(), 0.5 ));


}

// #[derive(Clone, Copy, Debug, PartialEq)]
#[derive(Clone, Debug)]
struct Circle {
    center: std::rc::Rc<Point>,
    radius: f64
}

trait TwoDimensionalShape: std::fmt::Debug {
    fn area(&self) -> f64;
}

impl TwoDimensionalShape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius *  std::f64::consts::PI
    }
}

#[derive(Clone, Debug)]
struct Rectangle {
    p1: std::rc::Rc<Point>,
    p2: std::rc::Rc<Point>
}

impl TwoDimensionalShape for Rectangle {
    fn area(&self) -> f64 {
        let delta_x = (self.p1.x() - self.p2.x()).abs();
        let delta_y = (self.p1.y() - self.p2.y()).abs();
        delta_x * delta_y
    }
}

#[cfg(test)]
use std::rc::Rc;

#[test]
fn test_circle() {
    let center = std::rc::Rc::new(CartesianPoint {x: 0.0,y: 0.0});
    println!("{:?}", center);
    let unit_circle = Circle { center: center.clone(), radius: 1.0f64 };
    println!("{:?}", unit_circle);

    assert_eq!(unit_circle.area(), std::f64::consts::PI);

    let unit_circle1 = Circle {
        center: center.clone(),
        radius: 1.0f64
    };
    assert_eq!(unit_circle1.area(), std::f64::consts::PI);

    let rectangle = Rectangle { p1: Rc::new(CartesianPoint {x: 0.0, y:0.0}), p2: Rc::new(CartesianPoint {x: 1.0, y:1.0})};
    assert!(nearly_equal(rectangle.area(), 1.0f64));

    {
        let shapes: [&TwoDimensionalShape; 2] = [&unit_circle, &rectangle];
        println!("{:?}", shapes);
        println!("-- iterating over slice --");
        for shape in &shapes {
            println!("shape: {:?} area: {:?}", shape, shape.area());
        }
    }

    { // required to define the life time of the two references: &unit_circle, &rectangle
        let shapes: Vec<&TwoDimensionalShape> = vec![&unit_circle, &rectangle];
        println!("-- iterating over vector of references --");
        for shape in shapes {
            println!("shape: {:?} area: {:?}", shape, shape.area());
        }
    }

    let shapes1: Vec<Rc<TwoDimensionalShape>> = vec![Rc::new(unit_circle), Rc::new(rectangle)];
    println!("-- iterating over vector of reference counted pointers --");
    for shape in &shapes1 {
        println!("shape: {:?} area: {:?}", shape, shape.area());
    }

    let shapes2 = shapes1.clone();
    println!("-- iterating over a copy of the previous vector of reference counted pointers --");
    for shape in &shapes2 {
        println!("shape: {:?} area: {:?}", shape, shape.area());
    }

}
