#[macro_export]
macro_rules! actor {
    (
        $name:ident {
            $( $field:ident : $ty:ty = $init:expr ),* $(,)?
        }
    ) => {
        pub struct $name {
            // engine-owned id storage (not public)
            id: $crate::actor::Property<$crate::actor::ActorId>,

            pub transform: $crate::actor::Property<$crate::math::Transform>,

            // user gameplay fields
            $( pub $field : $crate::actor::Property<$ty> ),*
        }

        impl $name {
            pub fn new(transform: $crate::math::Transform) -> Self {
                Self {
                    id: $crate::actor::Property::new($crate::actor::ActorId::INVALID),
                    transform: $crate::actor::Property::new(transform),
                    $( $field : $crate::actor::Property::new($init) ),*
                }
            }

            pub fn get_id(&self) -> $crate::actor::ActorId{
                // delegate to engine trait method so the logic stays in one place
                <Self as $crate::actor::base::ActorBase>::get_id(self)
            }

            pub fn get_transform(&self) -> $crate::math::Transform {
                // delegate to engine trait method so the logic stays in one place
               <Self as $crate::actor::base::ActorBase>::get_transform(self)
            }
        }

        impl $crate::actor::base::HasTransform for $name {
            fn transform(&self) -> &$crate::actor::Property<$crate::math::Transform> {
                &self.transform
            }
        }

        impl $crate::actor::base::HasId for $name {
            fn id_prop(&self) -> &$crate::actor::Property<$crate::actor::ActorId> {
                &self.id
            }
        }
    };
}
