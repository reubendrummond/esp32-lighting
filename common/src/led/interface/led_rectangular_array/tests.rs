use super::{LedArray, LedRectangularArray, Pixel};

#[test]
fn ordered_iter_mapping_correct() {
    //    0  1  2  3
    // 0: 17 18 19 20
    // 1: 16 15 14 13
    // 2: 9  10 11 12
    // 3: 8  7  6  5
    // 4: 1  2  3  4

    let correct_order: Vec<(u8, u8)> = vec![
        (4, 0),
        (4, 1),
        (4, 2),
        (4, 3),
        (3, 3),
        (3, 2),
        (3, 1),
        (3, 0),
        (2, 0),
        (2, 1),
        (2, 2),
        (2, 3),
        (1, 3),
        (1, 2),
        (1, 1),
        (1, 0),
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
    ];

    let width = 4;
    let height = 5;

    let mut rectangular_array = LedRectangularArray::new(width, height);

    for (x, y) in correct_order.iter() {
        rectangular_array.set_pixel(
            x.to_owned() as usize,
            y.to_owned() as usize,
            Pixel::new(*x, *y, 0),
        );
    }

    for (i, pixel) in rectangular_array.ordered_iter().enumerate() {
        let (x, y) = correct_order[i];
        assert_eq!(pixel, &Pixel::new(x, y, 0));
    }

    assert_eq!(1, 2);
}
