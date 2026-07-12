struct Car {
    number_of_doors: i8,
    vehicle: Option<Vehicle>,
    wheel: Option<Wheel>,
}

struct Vehicle {
    number_of_seats: i16,
    color: i8
}

struct Wheel {
    size: i8
}

#[derive(o2o::o2o)]
#[ref_into(Car)]
#[child_parents(vehicle: |v: Vehicle| Some(v), wheel: |w: Wheel| Some(w))]
struct CarDto {
    number_of_doors: i8,
    #[child(vehicle)]
    number_of_seats: i16,
    #[child(vehicle)]
    color: i8,
    #[child(wheel)]
    size: i8,
}


#[test]
fn basics() {
    let dto = &CarDto { number_of_doors: 4, number_of_seats: 5, color: 6, size: 7 };
    let car: Car = dto.into();

    assert_eq!(car.number_of_doors, 4);
    assert_eq!(car.vehicle.as_ref().unwrap().number_of_seats, 5);
    assert_eq!(car.vehicle.unwrap().color, 6);
    assert_eq!(car.wheel.unwrap().size, 7);
}
