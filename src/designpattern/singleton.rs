use std::sync::{Arc,Mutex,Once,ONCE_INIT};
use std::time::Duration;
use std::{mem,thread};

#[derive(Debug)]
struct World {
    area: u64,
    population:u64
}

impl World {
    fn set_area( &mut self, area: u64){
        self.area = area;
    }

    fn set_population( &mut self, population: u64){
        self.area =  population;
    }
}

#[derive(Debug)]
struct SingletonReader {
    inner: Arc<Mutex<Wrold>>
}