mod Move{

    pub struct Move {
        pub name: String,
        pub description: String,
        pub move_type: Type,
        pub power: i32,
        pub accuracy: i32,
        pub power_points: i32,
        pub effect: fn(),
        pub priority: i32,
        pub physical: bool,
        pub contact: bool,
    }
}