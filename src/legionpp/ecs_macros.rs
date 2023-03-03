use crate::comp_store::*;
use std::marker::PhantomData;
use crate::GlobalEntityID;
use std::slice::Iter;
use crate::superb_ecs::SuperbEcs;
use crate::pending_entity::PendingEntity;

use crate::utils::gett;

// T :

use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

// 'static + Serialize + Clone + DeserializeOwned + Send + Debug
#[macro_export] // Can remove.
macro_rules! comp_iter_def {
	($query_name:ident, $get_name:ident, $get_name_unwrap:ident, $new_name:ident, $($type_name:ident),+) => {
        #[allow(non_snake_case)]
        pub struct $query_name<'a, $($type_name: 'static + Serialize + Clone + DeserializeOwned + Send + Debug,)+> {
            $($type_name: PhantomData<$type_name>,)+
            ecs: &'a CompStorage,
            vec: Vec<GlobalEntityID>
        }// C:/Users/tomul/.rustup/toolchains/nightly-x86_64-pc-windows-gnu/lib/rustlib/src/rust/library/core/src/slice/iter.rs:66
        impl<'a, $($type_name: 'static + Serialize + Clone + DeserializeOwned + Send + Debug,)+> $query_name<'a, $($type_name,)+>{
            pub fn new(ecs: &'a CompStorage) -> Self{
                let mut my_vec = ecs.query(vec![$(gett::<$type_name>()),+]).iter().as_slice().to_vec();
                Self{
                    $($type_name: Default::default(),)+
                    ecs,
                    vec: my_vec,
                }
            }
        }
        impl<'a, $($type_name: 'static + Serialize + Clone + DeserializeOwned + Send + Debug,)+> Iterator for $query_name<'a, $($type_name,)+>{
            type Item = (GlobalEntityID, $(&'a mut $type_name),+);
            fn next(&mut self) -> Option<Self::Item> {
                let entity_id = self.vec.pop()?;

                return Some((entity_id,
                            $(self.ecs.get_mut::<$type_name>(entity_id).unwrap()),+
                )
                );
            }
        }
        #[allow(unused_parens)]
        impl CompStorage{
            pub fn $get_name<$($type_name : 'static + Serialize + Clone + DeserializeOwned + Send + Debug),+>(&self, entity_id: GlobalEntityID) -> ($(Option<&mut $type_name>),+){
                return ($(self.get_mut::<$type_name>(entity_id)),+ );
            }
            pub fn $get_name_unwrap<$($type_name : 'static + Serialize + Clone + DeserializeOwned + Send + Debug),+>(&self, entity_id: GlobalEntityID) -> ($(&mut $type_name),+){
                return ($(self.get_mut::<$type_name>(entity_id).unwrap()),+ );
            }
        }
        #[allow(unused_parens, non_snake_case)]
        impl PendingEntity{
            pub fn $new_name<$($type_name : 'static + Serialize + Clone + DeserializeOwned + Send + Debug),+>($($type_name: $type_name),+) -> Self{
                let mut pending = Self::new();
                $(
                    pending.add_comp($type_name);
                )+
                return pending;
            }
        }
    };
}

comp_iter_def!(CompIter1, get1, get1_unwrap, new1, A);
comp_iter_def!(CompIter2, get2, get2_unwrap, new2, A, B);
comp_iter_def!(CompIter3, get3, get3_unwrap, new3, A, B, C);
comp_iter_def!(CompIter4, get4, get4_unwrap, new4, A, B, C, D);
comp_iter_def!(CompIter5, get5, get5_unwrap, new5, A, B, C, D, E);
comp_iter_def!(CompIter6, get6, get6_unwrap, new6, A, B, C, D, E, F);
comp_iter_def!(CompIter7, get7, get7_unwrap, new7, A, B, C, D, E, F, G);
comp_iter_def!(CompIter8, get8, get8_unwrap, new8, A, B, C, D, E, F, G, H);
comp_iter_def!(CompIter9, get9, get9_unwrap, new9, A, B, C, D, E, F, G, H, I);
comp_iter_def!(CompIter10, get10, get10_unwrap, new10, A, B, C, D, E, F, G, H, I, J);













// #[allow(non_snake_case)]
// pub struct CompIter3<'a, A : 'static, B : 'static, C : 'static> {
//     A: PhantomData<A>,
//     B: PhantomData<B>,
//     C: PhantomData<C>,
//     ecs: &'a CompStorage,
//     vec: Vec<GlobalEntityID>
// }// C:/Users/tomul/.rustup/toolchains/nightly-x86_64-pc-windows-gnu/lib/rustlib/src/rust/library/core/src/slice/iter.rs:66
// impl<'a, A : 'static, B : 'static, C : 'static> CompIter3<'a, A, B, C>{
//     pub fn new(ecs: &'a CompStorage) -> Self{
//         let mut my_vec = ecs.query(vec![gett::<A>(), gett::<B>(), gett::<C>()]).iter().as_slice().to_vec();
//         my_vec.reverse();
//         Self{
//             A: Default::default(),
//             B: Default::default(),
//             C: Default::default(),
//             ecs,
//             vec: my_vec,
//         }
//     }
// }
// impl<'a, A, B, C> Iterator for CompIter3<'a, A, B, C>{
//     type Item = (GlobalEntityID, &'a mut A, &'a mut B, &'a mut C);
//     fn next(&mut self) -> Option<Self::Item> {
//         let entity_id = self.vec.pop()?;
//
//         return Some((entity_id,
//                      self.ecs.get_mut::<A>(entity_id).unwrap(),
//                      self.ecs.get_mut::<B>(entity_id).unwrap(),
//                      self.ecs.get_mut::<C>(entity_id).unwrap()
//         )
//         );
//     }
// }

// create_system!( render_system | secret_render_system
// 	| my_position: PositionComp, my_render: RenderComp, my_size: SizeComp, my_wasdmover_comp: WasdMoverComp
// 	|
// 	| player_names: &BTreeMap<PlayerID, String>, ctx:&mut Context
// );