use hashbrown::HashMap;

#[path = "blocks.rs"]
mod blocks;

const CHUNK_SIZE:usize = 32;

pub struct ChunkGrid {
    //chunk_pointer: [*mut Chunks; 25]
    chunks: HashMap<[u32; 3], Chunks>, // 2,147,483,647
}

impl ChunkGrid {
    // You need a hash table to index the chunks in 3D space
    //
    // e.g. given coordinate 200, 1, 70, find the proper index?
    //
    // a: 200/64.truncade() = 3 <- Chunk Relative to Origin
    // b: 1/64.truncade() = 0 <- Chunk Relative to Origin
    // c: 70/64.truncade() = 1 <- Chunk Relative to Origin
    //
    // x: 200%64 = 8 <- Location in side Chunk
    // y: 1%64 = 1
    // z: 70%64 = 6
    //
    // dictArray["3,0,1"] returns 18
    // chunkpointer[18].blockpointer[8][1][6] //Quadrent 1
    // if x < 0: 64-8

    // Or use 1 massive dictionary to store all possible coordinates?
    // And then linked-list blocks to know their neighbors?

    pub fn new() -> ChunkGrid{//chunk_pointer: [*mut Chunks; 25]) -> ChunkGrid {
        //ChunkGrid { chunk_pointer: chunk_pointer}
        ChunkGrid { chunks: HashMap::new() }
        //chunks = HashMap::new();
    }

    pub fn insert(&mut self){
        //let mut map = HashMap::new();
        //map.insert(1, "one");
        self.chunks.insert([0,0,0], Chunks::new());
    }

    /*fn list(&self){
        for (key, val) in self.chunks.iter() {
            println!("key: {} val: {}", key, val);
        }
    }*/
}

pub struct Chunks {
    block: [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]
}

impl Chunks {
// when running redstone wire, have a pointer point from the two endpoints of the wire so it is a fast track,
// therefore if the wire expands past multiple chunks, it can skip the lazy chunks
    fn new() -> Chunks{
        Chunks {block: [[[Block::new(); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]}
    }
}

// Use mod to reach blocks in another chunk

//#[path = "grid.rs"]
//mod grid;

#[derive(Copy, Clone)]
pub struct Block {
    //parent_chunk: *mut Chunks,
    block_id: u16, //pointer
    orientation: u8, // 256
    block_state: u8, // 256
    health: u8,
    block_type: u8,
    //data: *mut AdditionalBlockData,
}

impl Block {
    // Based Off Of Celluar Automata
    // Must move in the direction of gravity
    // Celluar Automata randomly combines and forms entities?
    //
    // when converting to vertex, keep vertex data after to only modify location where change has occured
    // we can assume only blocks that touch air can/ will be destroyed?
    fn new() -> Block{
        Block {block_id: 0, orientation: 0, block_state: 0, health: 0, block_type: 0}
    }

    fn updateGridBasedPhysics() { 

    }
}

pub struct AdditionalBlockData {
    data: u64,
}

impl AdditionalBlockData {
    fn decodeData(block_type: u8) {
        println!("Decode Based On Block Type");
    }
}

    // Only Updates if Neighboors Update
    // Hitting a Non Block would stop propagation

trait PhysicsBlock2 { 
    fn updateBlocxk();
}

impl PhysicsBlock2 for Block {
    fn updateBlocxk() {
        println!("Hello, world!");
    }
}
trait RuntimeBlock { // Always Updates On Tick

}

impl RuntimeBlock for Block {

}

trait UpdateBlock { 
    // Only Updates if Neighboors Update
    // Hitting a Non Block would stop propagation
    fn updateBlock();
}

impl UpdateBlock for Block {
    fn updateBlock() {
        println!("Hello, world!");
    }
}

//traits apply to multiple functions e.g.
//impl UpdateBlock for BlockV2 {
//    fn updateBlock() {
//        println!("Hello, world!");
//    }
//}

// block where if its powered and gets destroyed it causes explosions... battery?
// logic gates, classical and quantum

pub fn queue_load_chunk(){

}

pub fn queue_release_chunk(){

}

pub fn check_chunk(){
    
}