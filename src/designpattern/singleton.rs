use std::sync::{Arc,Mutex,Once};
use std::{mem};

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

#[derive(Debug,Clone)]
struct SingletonReader {
    inner: Arc<Mutex<World>>
}

fn singleton() -> SingletonReader {
    static mut SINGLETON: *const SingletonReader = 0 as *const SingletonReader;
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(||{
            let singleton = SingletonReader{
                inner: Arc::new(Mutex::new(World{area:0,population:0}))
            };

            SINGLETON = mem::transmute(Box::new(singleton));
        });
        (*SINGLETON).clone()
    }
}