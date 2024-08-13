use dynamics::joints::{Joint, JointDef, JointType};
use dynamics::world::{JointHandle, World};
use user_data::{RawUserData, UserDataTypes};
use wrap::*;

pub struct GearJointDef {
    pub collide_connected: bool,
    pub joint_1: JointHandle,
    pub joint_2: JointHandle,
    pub ratio: f32,
}

impl GearJointDef {
    pub fn new(joint_1: JointHandle, joint_2: JointHandle) -> GearJointDef {
        GearJointDef {
            collide_connected: false,
            joint_1: joint_1,
            joint_2: joint_2,
            ratio: 1.,
        }
    }
}

impl JointDef for GearJointDef {
    fn joint_type() -> JointType
    where
        Self: Sized,
    {
        JointType::Gear
    }

    unsafe fn create<U: UserDataTypes>(&self, world: &mut World<U>) -> *mut ffi::Joint {
        self.try_create(world)
            .expect("joint create failed: invalid joint handle")
    }

    unsafe fn try_create<U: UserDataTypes>(&self, world: &mut World<U>) -> Option<*mut ffi::Joint> {
        Some(ffi::World_create_gear_joint(
            world.mut_ptr(),
            self.collide_connected,
            world.try_joint_mut(self.joint_1)?.mut_base_ptr(),
            world.try_joint_mut(self.joint_2)?.mut_base_ptr(),
            self.ratio,
        ))
    }
}

wrap_joint! {
    ffi::GearJoint => GearJoint (JointType::Gear)
    < ffi::GearJoint_as_joint
    > ffi::Joint_as_gear_joint
}

impl GearJoint {
    pub fn ratio(&self) -> f32 {
        unsafe { ffi::GearJoint_get_ratio(self.ptr()) }
    }

    pub fn joint_1(&self) -> JointHandle {
        //unsafe { ffi::GearJoint_get_joint_1(self.ptr() as *mut ffi::GearJoint).handle() }
        panic!()
    }

    pub fn joint_2(&self) -> JointHandle {
        //unsafe { ffi::GearJoint_get_joint_2(self.ptr() as *mut ffi::GearJoint).handle() }
        panic!()
    }

    pub fn set_ratio(&mut self, ratio: f32) {
        unsafe { ffi::GearJoint_set_ratio(self.mut_ptr(), ratio) }
    }
}

#[doc(hidden)]
pub mod ffi {
    pub use dynamics::joints::ffi::Joint;
    pub use dynamics::world::ffi::World;

    pub enum GearJoint {}

    pub fn World_create_gear_joint(
        world: *mut World,
        collide_connected: bool,
        joint_a: *mut Joint,
        joint_b: *mut Joint,
        ratio: f32,
    ) -> *mut Joint {
        todo!()
    }
    pub fn GearJoint_as_joint(slf: *mut GearJoint) -> *mut Joint {
        todo!()
    }
    pub fn Joint_as_gear_joint(slf: *mut Joint) -> *mut GearJoint {
        todo!()
    }
    pub fn GearJoint_get_joint_1(slf: *mut GearJoint) -> *mut Joint {
        todo!()
    }
    pub fn GearJoint_get_joint_2(slf: *mut GearJoint) -> *mut Joint {
        todo!()
    }
    pub fn GearJoint_set_ratio(slf: *mut GearJoint, ratio: f32) {
        todo!()
    }
    pub fn GearJoint_get_ratio(slf: *const GearJoint) -> f32 {
        todo!()
    }
}
