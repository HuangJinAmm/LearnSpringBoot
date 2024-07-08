// use std::{collections::HashMap, any::{Any, TypeId}, any::type_name, sync::Arc};

// /// Class definition
// struct Class {
//     /// The name of the class
//     name: String,
//     /// The corresponding Rust type
//     type_id: TypeId,
//      attributes: HashMap<&'static str, AttributeGetter>
//  }

//  struct AttributeGetter(Arc<dyn Fn(&Instance) -> SomeValue>);

//  struct SomeValue;

//  trait SomeTrait{

//  impl AttributeGetter {
//     pub fn new<T, F, R>(f: F) -> Self
//     where
//         T: 'static,
//         F: Fn(&T) -> R,
//         R: SomeValue,
//     {
//         Self(Arc::new(move |receiver: &Instance, host: &mut Host| {
//             // get back the original type of the receiver
//             let receiver = reveiver.downcast().expect("type error");
//             // call the function; convert the result
//             f(receiver).map(|res| res.to_polar(host))
//         }))
//     }
// }

//  impl Class {
//      /// Create a new class definition for the type `T`
//      fn new<T: 'static>() -> Self {
//          Self {
//              name: type_name::<T>().to_string(),
//              type_id: TypeId::of::<T>(),
//          }
//      }

//  }

//  /// An instance of a class
//  struct Instance {
//      inner: Arc<dyn Any>, // `Arc` because we don't need/want mutability
//  }

//  impl Instance {
//      /// Construct a new `Instance` from a type that
//      /// implements `Any` (i.e. any sized type).
//      fn new(obj: impl Any) -> Self {
//          Self {
//              inner: Arc::new(obj)
//          }
//      }
//  }

//  impl Instance {
//     /// Check whether this is an instance of the provided class
//     fn instance_of(&self, class: &Class) -> bool {
//         self.inner.as_ref().type_id() == class.type_id
//         // self.inner.type_id() == class.type_id
//     }

//     pub fn get_attribute(&self, name: &str, host: &mut Host) -> crate::Result<Term> {
//         let class = self.class(host)?
//         class.get_attribute(self, name)
//     }
// }
fn main() {
    //     struct Foo {}
    //     struct Bar {}

    //     let foo_class: Class = Class::new::<Foo>();
    //     let bar_class: Class = Class::new::<Bar>();
    //     let foo_instance: Instance = Instance::new(Foo {});

    //     println!("{:?}",foo_instance.instance_of(&foo_class));
    //     println!("{:?}",!foo_instance.instance_of(&bar_class));
    //     // let s:String = "hello world".to_string();
    //     // let any: Box<dyn Any> = Box::new(s);
    //     // let mut recovered: Box<String> = any.downcast().expect("failed conversion");
    //     // recovered.make_ascii_uppercase();
    //     // println!("{}", recovered);
}
