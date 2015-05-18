use room::Room;
use hallway::Hallway;

pub struct Dungeon {
    rooms: Vec<Room>,
    hallways: Vec<Hallway>
}

impl Dungeon {
    pub fn new() -> Dungeon {
        Dungeon { rooms: Vec::new(), hallways: Vec::new() }
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }

    pub fn add_hallway(&mut self, hallway: Hallway) {
        self.hallways.push(hallway);
    }
}
